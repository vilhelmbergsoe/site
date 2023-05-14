---
title: "Nix is pretty awesome ❄️"
date: 05-04-2023
archived: false
tags: [nix, nixos, learning]
---

## Introduction

I have known of Nix and NixOS for a while, and I've always found them both very
neat. I even used NixOS in a pretty naive way on my Thinkpad x220, which works
as my home server.

I would effectively manage most things very imperatively, which kind of defeats
the purpose of using Nix in the first place. I had told myself I would do it the
proper way once I started moving my desktops to NixOS.

Recently my desktop drive went kapoot, and so I thought to myself, "This is the
perfect opportunity to move to NixOS and get a proper config up and running"
and so I did just that.

This post is more a intro / quick showcase of what nix can do and how I use it.
Not so much a detailed guide on learning Nix.

For that I suggest you look at the bottom of the blog post for some learning
resources to help you get started.

## What is Nix?

First off, what even is Nix?

According to their [webpage](https://nixos.org), _"Nix is a tool that takes a
unique approach to package management and system configuration."_

This is a good one-line summary of what Nix is, but I'll try my best to get a
bit more detailed here.

When people refer to Nix, they usually really refer to one of three things:

- **Nix - the programming language**

  Nix - the programming language is a purely functional and declarative
  programming language.

- **Nix - the package manager**

  Nix - the Package Manager is a cross-platform package manager that has access to
  the Nix Packages collection ([nixpkgs](https://github.com/NixOS/nixpkgs)) which
  has a set of over 80,000 packages and can be installed on Linux and other
  Unix-like systems like macOS.

- **NixOS - the Nix based linux distro**

  NixOS is a Linux distro built around the Nix package manager and the declarative
  Nix programming language. It is designed to provide a consistent and
  reproducible system from one source of truth configuration.

## Diving into Nix

All packages, which are called **derivations** in Nix-land, are stored in the
`/nix/store`, which is an immutable, read-only directory that stores all of a
system's derivations.

Each derivation and its dependencies are stored in a unique directory within
`/nix/store`, with the directory name being a cryptographic hash of the package
contents and its dependencies.

This helps ensure that each package and its dependencies are isolated and
independent of other packages, which is important for reproducibility.

Here is an example path to GNU hello:

`/nix/store/1pry7pnxqig0n2pkl4mnhl76qlmkk6vi-hello-2.12.1/`

Inside we find a `bin` and `share` directory, which each hold files associated
with this derivation.

```console
[vb@buckbeak:~]$ tree -L 2 /nix/store/1pry7pnxqig0n2pkl4mnhl76qlmkk6vi-hello-2.12.1
/nix/store/1pry7pnxqig0n2pkl4mnhl76qlmkk6vi-hello-2.12.1
├── bin
│   └── hello
└── share
    ├── info
    ├── locale
    └── man

5 directories, 1 file
vb@buckbeak:~]$ 
```

Under `share`, we find the typical `/usr/share` entries for man pages and such.
Under `bin`, we actually find the binary `hello`, and we can execute it just
fine:

```console
[vb@buckbeak:~]$ /nix/store/1pry7pnxqig0n2pkl4mnhl76qlmkk6vi-hello-2.12.1/bin/hello 
Hello, world!
```

## Nix as a development environment

One really cool use for Nix is its ability to create reproducible development
environments.

For example, let's say we have the following C program that relies on SDL:

```c
#include <SDL2/SDL.h>
#include <stdio.h>

#define SCREEN_WIDTH 640
#define SCREEN_HEIGHT 480

int main(int argc, char* args[]) {
  SDL_Window* window = NULL;
  SDL_Renderer* renderer = NULL;
  if (SDL_Init(SDL_INIT_VIDEO) < 0) {
    fprintf(stderr, "could not initialize sdl2: %s\n", SDL_GetError());
    return 1;
  }
  window = SDL_CreateWindow(
      "simple_c",
      SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED,
      SCREEN_WIDTH, SCREEN_HEIGHT,
      SDL_WINDOW_SHOWN
  );
  if (window == NULL) {
    fprintf(stderr, "could not create window: %s\n", SDL_GetError());
    return 1;
  }
  renderer = SDL_CreateRenderer(window, -1, 0);


  // Event loop
  SDL_Event event;
  int quit = 0;
  while (!quit) {
    while (SDL_PollEvent(&event)) {
      if (event.type == SDL_QUIT) {
        quit = 1;
        break;
      }
    }

    SDL_SetRenderDrawColor(renderer, 18, 18, 18, 255);
    SDL_RenderClear(renderer);
    SDL_RenderPresent(renderer);

    SDL_Delay(10);
  }

  SDL_DestroyWindow(window);
  SDL_Quit();
  return 0;
}
```

If we wanted a guarantee that this program would compile, we would have to know
for sure that we have all the dependencies **and** the right version of the
dependencies.

This is a bothersome task, and everyone who has had to compile software on their
own machines knows the struggle of getting the dependencies right.

Luckily, Nix comes to the rescue!

We can simply create a Nix flake, which is an experimental standard schema for
defining both inputs and outputs of your application.

This `flake.nix` file in the project root defines the flake, which specifies how
to build the derivation, as well as the necessary dependencies for building and
development shell environments:

```js
{
  description = "A simple C program that uses SDL";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in rec {
      packages = {
        default = pkgs.stdenv.mkDerivation {
          name = "simple_c";
          src = ./.;

          buildInputs = with pkgs; [
            pkgconfig
            SDL2
            gcc
          ];

          buildPhase = ''
            gcc ./main.c -o simple_c `pkg-config --libs --cflags sdl2`
          '';

          installPhase = ''
            mkdir -p $out/bin
            cp simple_c $out/bin
          '';
        };
      };
    });
}
```

If we then want to build the derivation, we can simply run:

```console
[vb@buckbeak:~/simple_c_example]$ nix build
[vb@buckbeak:~/simple_c_example]$ # or
[vb@buckbeak:~/simple_c_example]$ nix develop
```

which will build the project and create a shell with all dependencies in path,
respectively.

`nix build` will create a directory called `result` in the working directory,
and in it, there is a `bin` directory that contains our binary. The keen among
you, you might have figured out that `result` is actually a symlink to the
`/nix/store`.

```console
[vb@buckbeak:~/simple_c_example]$ ls -lah result
lrwxrwxrwx 1 vb users 52 Apr  5 20:25 result -> /nix/store/3mjvzschwmxivpmhm54x43djja35mim3-simple_c
```

A really neat piece of software that works super well with Nix is
[direnv](https://direnv.net/).

With direnv installed and setup on your system, you can add the following to a
`.envrc` file in your project root:

```console
[vb@buckbeak:~/simple_c_example]$ echo "use flake" >> .envrc
[vb@buckbeak:~/simple_c_example]$ direnv allow
```

This will automagically ✨ trigger the Nix shell and have your dependencies
loaded in your path when you change your working directory to the project root.

The `flake.lock` file is an important aspect of Nix's reproducibility. Acting
as a lockfile for all dependencies, it ensures that the input defined in the
flake, such as `inputs.nixpkgs`, is locked to a specific revision/commit using
a sha256 hashsum.

In the flake, the `inputs.nixpkgs.url` is the link to the actual nixpkgs repo,
which contains all the packages and dependencies. By locking the input, Nix
guarantees that everything will build exactly the same way on your and your
coworker's machines.

There is so much more cool stuff you can do with Nix alone, but I think this
gives you a pretty good idea of how useful it is.

Let's take a look at NixOS..

## A look at NixOS

NixOS is really exciting because it takes the same principles of declarative
structure of a project and applies it to your entire system.

Here is an example of how you can enable PostgreSQL and Nginx declaratively on
your NixOS machine.

In your NixOS configuration, you simply add:

```c
services.postgresql.enable = true;
services.nginx.enable = true;
```

Really, it's that simple.

NixOS's declarativeness isn't limited to services though. You can configure
quite literally everything. Here is a snippet from my own configuration:

```c
# Configure user
users.users = {
  vb = {
    initialPassword = "nixos";
    isNormalUser = true;
    openssh.authorizedKeys.keys = [
      "REDACTED"
    ];
    extraGroups = ["wheel" "networkmanager" "docker"];

    shell = pkgs.bash;
  };
};
# ...
# Configure networking
networking = {
  hostName = "buckbeak";
  networkmanager.enable = true;
  nameservers = ["1.1.1.1" "1.0.0.1"];
};
```

NixOS's configurability even extends to custom modules.

Here is the same Nix module used to host this
[site](https://github.com/vilhelmbergsoe/site) on my server:

```c
{inputs, ...}: {
  systemd.services.site = {
    enable = true;

    description = "my site";
    wantedBy = ["multi-user.target"];
    after = ["network.target"];

    serviceConfig = {
      Type = "simple";
      ExecStart = "${inputs.site.packages.x86_64-linux.default}/bin/site";
      Restart = "on-failure";
    };
  };
}
```

It uses the `inputs.site` which is an input defined in my configuration's
`flake.nix` that refers to the GitHub repo for the site:

```c
inputs = {
  # Nixpkgs
  nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  # ...

  # Site
  site.url = "github:vilhelmbergsoe/site";
};
```

In the site repository there is a `flake.nix` in the root, which uses
[crane](https://github.com/ipetkov/crane) for handling source fetching from the
Cargo.lock file and incremental building.

It's really quite something 😁

## Conclusion

I have had a blast learning Nix for the past month, and I think this is an
amazing tool. I have moved my entire desktop configuration to NixOS as well as
my server.

And while there is so much I haven't covered here, I hope this gave
someone the itch to pick up Nix and try it out for themselves.

If it did and you want to learn more, here are some learning resources to help
you get started with:

- Zero2Nix: <https://zero-to-nix.com/>
- Nix Pills: <https://nixos.org/guides/nix-pills/>
- NixOS wiki: <https://nixos.wiki/>

If you're interested, you can also find my configuration
[here](https://github.com/vilhelmbergsoe/dotfiles).

Thanks for reading!
