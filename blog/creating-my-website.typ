#let post = (
  title: "Creating my website",
  date: datetime(year: 2022, month: 8, day: 28),
  archived: false,
  tags: ("go", "website", "blog"),
)

#set document(title: post.title, date: post.date, keywords: post.tags)
#metadata(post) <post-meta>

#import "/typst/post.typ": anchored-headings
#show: anchored-headings

= IMPORTANT NOTICE
<important-notice>
#quote(block: true)[
#emph[This blog post is outdated. The website has been migrated to Rust,
and the new codebase can be found
#link("https://github.com/vilhelmbergsoe/site")[here];. The old code is
still available #link("https://github.com/vilhelmbergsoe/sb")[here];.]

Oh, and here is a blog post about the updated site:
#link("/blog/new-website")[Migrating my site to Rust]
]

= The Beginning
<the-beginning>
I've had a domain for a while now, but haven't got around to building a
portfolio site until now.

I wanted to do something a little more unique than just finding a nice
#link("https://gohugo.io/")[Hugo] theme, generating a static HTML page and
calling it a day.

Therefore, I decided to create
#link("https://github.com/vilhelmbergsoe/sb/")[what you're looking at now];.
A nice little personal website with blog functionality and an admin
panel for creating, deleting and updating blog posts.

= The logistics
<the-logistics>
I wanted the website to be very minimal and found
#link("https://minwiz.com/")[this] nice template for a minimal
responsive website.

I made minor changes to it, including Go templating, simple JavaScript
functions for CRUD functionality, and a few color changes.

The entire website is hosted through Go's
#link("https://pkg.go.dev/net/http/")[net/http] with the
#link("https://github.com/gorilla/mux/")[gorilla HTTP router];.

The application's architecture is inspired by
#link("https://pace.dev/blog/2018/05/09/how-I-write-http-services-after-eight-years.html")[Matt Ryer's blog post];.

I added Markdown functionality and HTML sanitizing with
#link("https://github.com/russross/blackfriday/")[blackfriday] and
#link("https://github.com/microcosm-cc/bluemonday")[bluemonday]
respectively.

= Usage
<usage>
The actual template is very easy to use. Example instructions are on the
GitHub README #link("https://github.com/vilhelmbergsoe/sb/")[here];.

There are only two endpoints: `/` and `/admin`. The admin endpoint is
protected by basic authentication using nothing but Go's standard
library, gorilla's HTTP router and SQLite!

When you start out, you need to add a user to the SQLite database for
administration purposes, and you can do that through a simple shell
script included in the repository under `/tools/createuser`.

After that, you can start customizing the HTML pages and creating blog
posts through the admin panel.

I hope to showcase some of my future projects on this website and that
this was an interesting read 😀

You can check out the repository
#link("https://github.com/vilhelmbergsoe/sb/")[here];.
