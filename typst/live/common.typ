#let in-math = state("live-in-math", false)
#let enabled = state("live-enabled", false)
#let formats = ("fixed", "percent")

#let format-number(value, format, digits) = {
  assert(type(value) in (int, float))
  assert(format in formats)
  assert(type(digits) == int)
  assert(digits >= 0)

  let scaled = if format == "percent" { value * 100 } else { value }
  let parts = str(calc.round(float(scaled), digits: digits)).split(".")
  assert(parts.len() <= 2)

  let fraction = parts.at(1, default: "")
  let text = if digits == 0 {
    parts.at(0)
  } else {
    parts.at(0) + "." + fraction + "0" * (digits - fraction.len())
  }

  assert(text.len() > 0)
  assert(not text.ends-with("."))
  text + if format == "percent" { "%" } else { "" }
}

#let assert-name(name) = {
  assert(type(name) == str)
  assert(name.match(regex("^[A-Za-z_][A-Za-z0-9_]*$")) != none, message: "invalid live name: " + repr(name))
}
