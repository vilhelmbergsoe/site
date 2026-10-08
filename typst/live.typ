#import "live/common.typ": enabled, in-math, formats, format-number, assert-name
#import "live/input.typ"

#let live(body) = {
  assert(type(body) == content)
  enabled.update(true)
  // This is the only point where Typst exposes whether a view is inside math.
  show math.equation: equation => {
    in-math.update(true)
    equation
    in-math.update(false)
  }
  body
}

#let source-text(source) = {
  if type(source) == str {
    assert(source.trim().len() > 0)
    source
  } else {
    assert(type(source) == content, message: "source must be a string or raw block")
    assert(source.func() == raw, message: "source must be a string or raw block")
    assert(source.at("lang", default: "js") == "js", message: "source must be JavaScript")
    assert(source.text.trim().len() > 0, message: "source must not be empty")
    source.text
  }
}

// Templates preserve JavaScript source without rendering or executing it.
#let code(kind, name, source) = {
  assert(kind in ("formula", "module"))
  assert-name(name)
  let text = source-text(source)
  assert(text.trim().len() > 0)
  html.elem("template", attrs: (data-live: kind, data-name: name, data-source: text))
}

#let formula(name, source) = code("formula", name, source)

#let module(name, source) = code("module", name, source)

#let view(name, index: none, format: "fixed", digits: 2, delim: "[") = {
  assert-name(name)
  assert(format in formats)
  assert(type(digits) == int)
  assert(digits >= 0)
  assert(delim in ("[", "(", none))

  let path = if index == none { () } else if type(index) == int { (index,) } else { index }
  assert(type(path) == array)
  assert(path.all(index => type(index) == int))

  let attrs = (
    class: "live-view",
    data-live: "view",
    data-name: name,
    data-index: json.encode(path, pretty: false),
    data-format: format,
    data-digits: str(digits),
    data-delim: if delim == none { "" } else { delim },
  )

  context {
    assert(enabled.get(), message: "add `#show: live` before using view")
    if in-math.get() {
      html.elem("mrow", attrs: attrs, [requires JavaScript])
    } else {
      html.elem("span", attrs: attrs, [requires JavaScript])
    }
  }
}
