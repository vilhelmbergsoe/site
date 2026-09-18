#set page(
  paper: "a4",
  margin: (left: 30.5mm, right: 32.5mm, top: 33mm, bottom: 24mm),
)

#set text(
  font: "New Computer Modern",
  size: 9pt,
)

#set par(
  justify: false,
  leading: 0.55em,
)

#let section(title) = {
  assert(type(title) == str)
  assert(title != "")

  block(above: 1.1em, below: 0.35em)[
    #text(size: 12pt, smallcaps(title))
    #v(-0.28em)
    #line(length: 100%, stroke: 0.6pt)
  ]
}

#let heading(org, place, roles) = {
  assert(type(org) == str)
  assert(roles.len() > 0)

  grid(
    columns: (1fr, auto),
    column-gutter: 1em,
    row-gutter: 0.55em,
    [*#org*], align(right)[#place],
    ..roles.map(((role, dates)) => (
      [#emph(role)], align(right)[#emph(dates)]
    )).flatten(),
  )
}

#let detail-list(items) = {
  assert(type(items) == array)
  assert(items.len() > 0)

  list(
    marker: text(size: 5pt, sym.circle.stroked),
    indent: 0.45em,
    body-indent: 0.45em,
    spacing: 0.63em,
    ..items,
  )
}

#grid(
  columns: (1fr, auto),
  column-gutter: 1em,
  row-gutter: 0.55em,
  [#text(size: 18pt, weight: "bold")[Vilhelm Bergsøe]], [],
  [#link("https://bergsoe.net")[Portfolio: bergsoe.net]],
  [Email: #link("mailto:vilhelm@bergsoe.net")[vilhelm\@bergsoe.net]],
  [#link("https://github.com/vilhelmbergsoe")[GitHub: github.com/vilhelmbergsoe]], [],
)

#section("Experience")

#set list(
  marker: sym.bullet,
  indent: 1em,
  body-indent: 0.45em,
  spacing: 1.35em,
)

#list(
  [
    #heading(
      "Factbird",
      "In-Office–Remote",
      (
        ("Student Assistant – Cloud Development (Part-time)", "2025 – Present"),
        ("Full Stack Engineer (Full-time)", "2024 – 2025"),
      ),
    )

    #block(above: 1.45em, below: 1.6em)[
      Most of my work has been on the backend and infrastructure side. I ported the core statistics pipeline and a number of GraphQL resolvers from TypeScript to Rust, and spent quite a bit of time on performance work around the gateway, authentication, and data processing.

      #v(0.55em)
      I also built and maintained large parts our Nix-based development and CI infrastructure, including developer tooling, improving build times and observability.
    ]
  ],
  [
    #heading(
      "TestaViva",
      "In-Office",
      (("Developer Support (Part-time)", "Nov 2019 – Feb 2020"),),
    )
  ],
)

#section("Education")

#list(
  [#heading(
    "University of Copenhagen",
    "Copenhagen, Denmark",
    (("B.Sc. in Mathematics", "2024 – Present"),),
  )],
  [#heading(
    "Niels Brock Innovationsgymnasiet",
    "Copenhagen, Denmark",
    (("Project management track", "2021 – 2024"),),
  )],
)

#section("Projects")

#list(
  spacing: 1.4em,
  [*thread:* My (very work-in-progress) native code debugger.],
  [*nod:* Nix Observability Daemon which monitors builds, substitutions using Nix's structured JSON logs.],
  [*brainybishop:* Simple little chess engine.],
  [*asciicam:* ASCII webcam for the console.],
)
