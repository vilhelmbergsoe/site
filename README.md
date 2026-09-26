# site

My personal website with blog functionality.

## Usage

```console
git clone https://github.com/vilhelmbergsoe/site
cargo run --release
# or with nix flake
nix run
```

## Endpoints

`/` home page

`/archive/` writing archive

`/archive/{url}` writing page

`/tag/{tag}` tagged posts page

`/assets/{file}` compile-time embedded static asset

`/rss.xml` rss feed

`/sitemap.xml` sitemap

`/robots.txt` robots.txt

`/cv.pdf` curriculum vitae

## License

[MIT](https://choosealicense.com/licenses/mit)
