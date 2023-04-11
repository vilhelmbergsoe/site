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

`/blog/{url}` blog post page

`/assets/{file}` static file serve directory

`/rss.xml` rss feed

## License

[MIT](https://choosealicense.com/licenses/mit)
