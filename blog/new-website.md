---
title: New website update ⭐
date: 23-06-2023
archived: false
tags: [website, rust, nix, programming]
---

In August of last year, I published a blog post titled "Creating my website",
but since then, significant changes have been made in the implementation of my
site and the blog post is outdated. I thought I would share the process of
migrating my website to Rust and the implementation details in this blog post.

# Background and Motivation

First off, I should establish the motivation for the move from Go to Rust. I
should clarify that performance and safety was not the primary concern with my
original implementation in Go. In fact I love Go and the old codebase but
there was a separate reason for my switch.

The reason for my migration to Rust is... Nix! In my previous blog post, "Nix is
pretty awesome❄️", I expressed my excitement with Nix as a development and
deployment tool so naturally I wanted to deploy my site with Nix as well. There
is, however, a problem with "nixifying" go applications as the hashing method
used by Go for dependency management is fundamentally incompatible with Nix.

There is a way to get over this hurdle, by using a code generation tool like
[gomod2nix](https://github.com/nix-community/gomod2nix). This, however, is a bit
of a pain and I'd rather not need to generate new Nix expressions everytime I
update dependencies. Rust, however, doesn't have this problem and works
exceptionally well with Nix.

For this reason, combined with my new interest in the Rust language I came to
the conclusion that I wanted to rebuild my site in Rust, and maybe get more
comfortable with the language.

# Implementation

At the time of rewriting my site, I discovered that
[axum](https://github.com/tokio-rs/axum) recently had a major release and I
heard good things about its seamless integration with the tokio runtime, so I
decided to go with axum as my choice of web framework.

In line with my previous site, I required most of the same endpoints for my
revamped version:

- The root or homepage, accessible via `/`
- Individual blog posts, reachable through `/blog/{url}`
- A directory for serving static files, located at `/assets`

However, I took the opportunity to introduce a new feature: an RSS Feed endpoint
accessible via `/rss.xml`.

In axum, the routing for my webpage looks like the following:

```rust
let app = Router::new()
    .route("/", get(root))
    .route("/blog/:url", get(handle_blog))
    .route("/rss.xml", get(handle_rss))
    .with_state(state)
    .nest_service(
        "/assets",
        ServeDir::new(path_prefix.join(Path::new("assets"))),
    )
    .fallback(get(handle_404));
```

So far everything looks pretty familiar.

The `.with_state(state)` refers to the common state for all the endpoints. This
state contains a `Vec` of `BlogPost`'s:

``` rust
pub struct AppState {
    blogposts: Vec<BlogPost>,
}
```

The `BlogPost` structure looks like this:

```rust
pub struct BlogPost {
    url: String,
    title: String,
    date: DateTime<Utc>,
    archived: bool,
    tags: Vec<String>,
    content: String,
    estimated_read_time: usize,
}
```

In this struct, we have the following fields:

- `url`: Represents the URL used to access the blog post via the /blog/
  endpoint.
- `title`: Holds the title of the blog post.
- `date`: Represents the date and time when the blog post was created or
  published.
- `archived`: A boolean value indicating whether the post should be displayed or
  not.
- `tags`: Contains a list of tags associated with the blog post, such as "rust"
  or "nix".
- `content`: Stores the parsed and processed HTML content of the blog post.
- `estimated_read_time`: Provides a rough estimation of the read time for the
  post.
  
This structure might not be as comprehensive a representation of a blog post as
I'd have liked it to be. But it does the job for now 😀

I decided to load and parse the blog posts at startup so as to minimize runtime
overhead. This is all done in the `new_state()` function:

```rust
async fn new_state(path_prefix: &Path) -> Result<AppState> {
    // Create an empty vector to store the blog posts
    let mut blogposts: Vec<BlogPost> = Vec::new();

    // Read the contents of the "blog" directory
    let mut blog_dir = match tokio::fs::read_dir(path_prefix.join(Path::new("blog"))).await {
        Ok(dir) => dir,
        Err(err) => return Err(eyre!(format!("Error reading blog directory: {err}"))),
    };

    // Set the options and plugins for parsing Markdown content
    let adapter = SyntectAdapter::new("base16-eighties.dark");
    let options = ComrakOptions::default();
    let mut plugins = ComrakPlugins::default();

    // Iterate over each entry (file or directory) in the "blog" directory
    while let Some(entry) = blog_dir.next_entry().await? {
        let path = entry.path();

        // Check if the entry is a file
        if path.is_file() {
            // Check for invalid file extensions
            let ext = path.extension();
            if ext != Some(std::ffi::OsStr::new("md"))
                && ext != Some(std::ffi::OsStr::new("markdown"))
                || ext.is_none()
            {
                // Skip files with invalid extensions
                tracing::warn!("skipping non markdown file: {}", path.display());
                continue;
            }

            if let Some(stem) = path.file_stem() {
                let url = stem.to_str().unwrap();

                // Check if a blog post with the same URL already exists
                if blogposts.par_iter().any(|b| b.url == url) {
                    // Skip duplicate blog posts
                    tracing::warn!("skipping duplicate blogpost: {}", url);
                    continue;
                }

                // Set the syntax highlighter adapter for code fences
                plugins.render.codefence_syntax_highlighter = Some(&adapter);

                // Parse the blog post content and metadata
                let start_time = Instant::now();
                let blogpost = parse_blog(url, &path, &options, &plugins).await?;
                let elapsed = start_time.elapsed().as_millis();

                // Add the parsed blog post to the vector
                blogposts.push(blogpost);
                tracing::info!("loaded blogpost - {} in {} ms", url, elapsed);
            }
        }
    }

    // Sort the blog posts by date in descending order
    blogposts.sort_by(|a, b| b.date.cmp(&a.date));

    // Create and return the application state with the loaded blog posts
    Ok(AppState { blogposts })
}
```

This is quite boring though, and all the interesting stuff is happening in the
call to `parse_blog()` which looks like this:

```rust
async fn parse_blog(
    url: &str,
    path: &PathBuf,
    options: &ComrakOptions,
    plugins: &ComrakPlugins<'_>,
) -> Result<BlogPost, Report> {
    // Read the file contents as bytes
    let bytes = tokio::fs::read(path).await?;
    
    // Convert the bytes to a UTF-8 encoded string
    let text = String::from_utf8_lossy(&bytes);

    // Parse the frontmatter and content sections of the blog post
    let (frontmatter, content) = match parse_frontmatter(&text) {
        Ok((frontmatter, content)) => (frontmatter, content),
        Err(_) => {
            return Err(eyre!(format!(
                "Error parsing frontmatter ({url}). Most likely missing delimiter \"---\\n\""
            )))
        }
    };

    // Deserialize the frontmatter YAML into a Frontmatter struct
    let frontmatter: Frontmatter = match serde_yaml::from_str(frontmatter) {
        Ok(fm) => fm,
        Err(err) => return Err(eyre!(format!("Error parsing blog ({url}): {err}"))),
    };

    // Parse the date from the frontmatter and convert it to UTC DateTime
    let naive_date = NaiveDate::parse_from_str(&frontmatter.date, "%d-%m-%Y").unwrap();
    let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
    let date: DateTime<Utc> = Utc.from_utc_datetime(&naive_datetime);

    // Convert the Markdown content to HTML using the provided options and plugins
    let html = markdown_to_html_with_plugins(content, &options, &plugins);

    // Create a new BlogPost struct with the parsed data and return it
    Ok(BlogPost {
        url: url.to_string(),
        title: frontmatter.title,
        date,
        archived: frontmatter.archived,
        tags: frontmatter.tags,
        content: html,
        estimated_read_time: content.split_whitespace().count() / 200,
    })
}
```

So let's dissect what's happening here.

First, we're reading the contents of the file into `text`. Next we make a call
to `parse_frontmatter()`, which is my dodgy frontmatter parser written using the
parser combinator library [nom](https://docs.rs/nom/latest/nom/). The parsing
logic itself is straightforward: It searches for a pair of delimiters "---" and
extracts the text between them as the frontmatter. The remaining part of the
file is considered the main content.

```rust
fn parse_frontmatter(input: &str) -> IResult<&str, &str> {
    let delimiter = "---";

    let (input, frontmatter) =
        delimited(tag(delimiter), take_until(delimiter), tag(delimiter))(input)?;
    let content = input.trim_start();

    Ok((frontmatter, content))
}
```

The frontmatter, which is just YAML code, is then parsed by
[serde_yaml](https://docs.rs/serde_yaml/latest/serde_yaml/) into a nice
`Frontmatter` struct:

```rust
struct Frontmatter {
    title: String,
    date: String,
    archived: bool,
    tags: Vec<String>,
}
```

After parsing the date from the string literal in the Frontmatter, we convert
the markdown to html with the [comrak](https://docs.rs/comrak/latest/comrak/)
markdown parser, using a nice one liner:

```rust
let html = markdown_to_html_with_plugins(content, &options, &plugins);
```

Finally, we construct a `BlogPost` with all of the parsed data and return it:

```rust
Ok(BlogPost {
    url: url.to_string(),
    title: frontmatter.title,
    date,
    archived: frontmatter.archived,
    tags: frontmatter.tags,
    content: html,
    estimated_read_time: content.split_whitespace().count() / 200,
})
```

## Handlers

Now that we have an understanding of the project's structural skeleton, let's
explore how we handle requests. The good news is that Axum makes this process
remarkably simple and convenient.

In conjunction with Axum, I've incorporated a template engine called
[maud](https://maud.lambda.xyz/). Maud provides an `html!` macro that compiles
pseudo HTML into efficient Rust code, resulting in exceptional performance. This
combination of Axum and Maud enables seamless and efficient rendering of HTML
responses for our web application.

```rust
pub async fn handle_blog (
    Path(url): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let blogpost = state
        .blogposts
        .par_iter()
        .find_first(|blogpost| blogpost.url == url);

    match blogpost {
        Some(blogpost) => (
            StatusCode::OK,
            html! {
                (header(&format!("Vilhelm Bergsøe - {}", blogpost.title), "Vilhelm Bergsøe - Blog"))
                main {
                    section #h {
                        div .blogpost {
                            h2 .blogtitle { (blogpost.title) }
                            span style="opacity: 0.7;" {
                                (blogpost.date.format("%a %d %b %Y"))
                                // 200 words per minute estimate
                                (format!(" - {} min read", blogpost.estimated_read_time))
                            }
                            br;
                            p {
                                (PreEscaped(&blogpost.content))
                            }
                        }

                        em { (format!("tags: [{}]", blogpost.tags.join(", "))) }
                    }
                }

                (footer())
            },
        ),
        None => handle_404().await,
    }
}
```

This is the entirety of the blog handler for the `/blog/{url}` endpoint.

Essentially we find the first blog post that matches the url requested and
return the default blog page with the contents of the blog post integrated
otherwise we pass control to the 404 Not Found handler.

I'm aware that searching through a `Vec` isn't very efficient and I should look
into using a `HashSet` og `HashMap` for the lookups the problem with this is sorting
for dates isn't possible and I have yet to do benchmarks to find out which
really has the biggest effect on performance.

The root endpoint looks kind of the same, with the only dynamic part being the
list of blog posts:

```rust
// ...
h2 { "Blog " a href="/rss.xml" title="RSS Feed" { img .rss-icon src="/assets/rss.png" alt="rss"; } }
ul {
    @for blogpost in &state.blogposts {
        @if !blogpost.archived {
            li {
                (blogpost.date.format("D%d-%m-%Y "))
                a href=(format!("/blog/{}", blogpost.url)) { (blogpost.title) }
            }
        }
    }
}
// ...
```

For my RSS Feed endpoint I chose to go with the
[ructe](https://docs.rs/ructe/latest/ructe/) template engine as maud doesn't
have explicit support for XML.

Here the template for the rss feed looks like this:

```xml
@use crate::BlogPost;

@(posts: Vec<BlogPost>)
<?xml version="1.0" encoding="UTF-8" ?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
    <channel>
        <title>Vilhelm's Blog</title>
        <link>https://bergsoe.net/</link>
        <description>My Blog RSS Feed</description>
        @for post in posts {
            <item>
                <guid>https://bergsoe/blog/@post.url</guid>
                <title>@post.title</title>
                <link>https://bergsoe.net/blog/@post.url</link>
                <description>tags: @post.tags.join(", ")</description>
                <pubDate>@post.date.to_rfc2822()</pubDate>
            </item>
        }
    </channel>
</rss>
```

## Nix deployment

As mentioned, the main drive behind my move from Rust is ease of deployment with
Nix. So let's look into how it is done:

In the project root we define a Nix flake `flake.nix`. Here I utilize the
[crane](https://crane.dev/) library for building the project. Crane provides
other niceties such as automatic source fetching and incremental builds.

One problem you run into is having relative paths work correctly when the
service is run from the nix store. There are probably many ways of solving this
problem, but I opted for an environment variable with the path to the project
directory:

``` rust
let site_root = std::env::var("SITE_ROOT").unwrap_or_else(|_| "./".to_string());
let path_prefix = Path::new(&site_root);
```

Here we load the environment variable if it exists and if it doesn't it just
defaults to the current directory.

In the Nix flake we then make sure to define this environment variable:

``` nix
# ...
default = pkgs.symlinkJoin {
    inherit (site) name pname version;
    nativeBuildInputs = [pkgs.makeWrapper];
    paths = [site];
    postBuild = ''
        wrapProgram $out/bin/site --set-default SITE_ROOT ${./.}
    '';
};
# ...
```

We can then build and run the project just fine with Nix:

``` sh
$ nix run
2023-05-17T09:20:16.398678Z  INFO site: site root: /nix/store/z04g8kmpmkvbf0kxf81aigjbx61b5i4q-40kgjvsccc7ny75r4wfd4gi98kp7l004-source
2023-05-17T09:20:16.403586Z  INFO site: loaded blogpost - ascii-webcam in 0 ms
2023-05-17T09:20:16.422079Z  INFO site: loaded blogpost - nix-is-pretty-awesome in 18 ms
2023-05-17T09:20:16.422582Z  INFO site: loaded blogpost - creating-my-website in 0 ms
2023-05-17T09:20:16.423409Z  INFO site: listening on 0.0.0.0:8080
```

On my server I then use the following Nix module for serving the website:

``` nix
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

Here we define a systemd service called `site` which uses the `site` input
`github:vilhelmbergsoe/site` from the Nix flake.

Deployment is then as simple as importing the module in my host configuration
and it runs!

If I have to update the site in the future all I have to do is push my changes
and run

```sh
$ nix flake lock --update-input site
$ # and
$ sudo nixos-rebuild switch --flake .#clifton
```

And that's it!

# Conclusion

Overall, it has been an awesome learning experience migrating my site to Rust +
Nix and I hope it at least was an interesting read. I learned a lot about both
Rust and Nix during this process.

If you're interested in looking at the full code you can find the code
[here](https://github.com/vilhelmbergsoe/site).

Also if you're interested in seeing the deployment code in it's entirety you can
find it
[here](https://github.com/vilhelmbergsoe/dotfiles/blob/master/hosts/clifton/modules/site.nix).

Thanks for reading!
