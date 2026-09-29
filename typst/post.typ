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
