#import "@preview/cetz:0.5.0"

#let anchored-headings(body) = {
  assert(type(body) == content)
  assert(repr(body).len() > 0)

  show heading: it => {
    let fields = it.fields()
    assert("body" in fields)
    assert(type(it.body) == content)

    // The replacement heading is realized by this show rule a second time.
    // Its label remains on the original element, so only the first pass needs
    // to insert the self-link that makes Typst emit the label as an HTML ID.
    if "label" not in fields {
      it
    } else {
      let heading-label = fields.label
      let _ = fields.remove("body")
      let _ = fields.remove("label")
      let anchored-body = [
        #link(heading-label)[
          #html.elem("span", attrs: (
            class: "anchor",
            aria-label: "Link to section",
          ))
        ];#it.body
      ]
      it.func()(anchored-body, ..fields)
    }
  }

  body
}

// A cetz canvas embedded as inline SVG in HTML output.
#let drawing(alt, display-width: "30rem", ..args) = {
  assert(type(alt) == str)
  assert(alt.trim() != "")
  assert(type(display-width) == str)
  assert(display-width.trim() != "")

  let canvas = pad(x: 3pt, y: 3pt, cetz.canvas(..args))
  context if target() == "html" {
    html.elem("div", attrs: (
      class: "typst-diagram",
      style: "--diagram-width: " + display-width,
      role: "img",
      aria-label: alt,
    ))[
      #html.frame(canvas)
    ]
  } else {
    canvas
  }
}
