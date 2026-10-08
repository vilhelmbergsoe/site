#import "common.typ": enabled, in-math, formats, format-number, assert-name

#let number(name, default, min: none, max: none, step: none, format: "fixed", digits: 2) = {
  assert-name(name)
  assert(type(default) in (int, float), message: "a number input needs a number")
  assert(type(min) in (int, float), message: "a number input needs min")
  assert(type(max) in (int, float), message: "a number input needs max")
  assert(type(step) in (int, float), message: "a number input needs step")
  assert(min < max)
  assert(step > 0)
  assert(min <= default)
  assert(default <= max)
  assert(format in formats)
  assert(type(digits) == int)
  assert(digits >= 0)

  let attrs = (
    class: "live-input live-number",
    data-live: "input",
    data-name: name,
    data-kind: "number",
    data-default: json.encode(default, pretty: false),
    data-config: json.encode((
      min: min,
      max: max,
      step: step,
      format: format,
      digits: digits,
    ), pretty: false),
  )

  context {
    assert(enabled.get(), message: "add `#show: live` before using input")
    assert(not in-math.get(), message: "inputs cannot be placed inside equations")
    html.elem("span", attrs: attrs, format-number(default, format, digits))
  }
}
