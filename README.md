# site

My personal website and blog.

## Usage

```console
git clone https://github.com/vilhelmbergsoe/site
cargo run --release
# or with nix flake
nix run
```

## Endpoints

`/` home page

`/blog/` blog index

`/blog/{url}` blog post

`/tag/{tag}` tagged posts page

`/assets/{file}` compile-time embedded static asset

`/rss.xml` rss feed

`/sitemap.xml` sitemap

`/robots.txt` robots.txt

`/cv.pdf` curriculum vitae

## License

[MIT](https://choosealicense.com/licenses/mit)
