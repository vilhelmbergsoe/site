{
  description = "My personal site";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-utils.url = "github:numtide/flake-utils";
    crate2nix = {
      url = "github:nix-community/crate2nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    crate2nix,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
      };

      cargoNix = crate2nix.tools.${system}.appliedCargoNix {
        name = "site";
        src = ./.;
      };
    in rec {
      checks.site = cargoNix.rootCrate.build.override {
        runTests = true;
      };

      packages = rec {
        site = cargoNix.rootCrate.build;
        default = site;
      };

      apps.default = {
        type = "app";
        program = "${packages.site}/bin/site";
      };

      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          cargo
          rustc
          rust-analyzer
          rustfmt
        ];
      };
    });
}
