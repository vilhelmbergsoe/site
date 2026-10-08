const MAX_FORMULAS = 256;
const IDENTIFIER = /^[A-Za-z_][A-Za-z0-9_]*$/;
const MATHML = "http://www.w3.org/1998/Math/MathML";
const DRAG_PIXELS = 300;
const PAGE_STEPS = 10;

async function main() {
  const modules = await loadModules(document.querySelectorAll('[data-live="module"]'));
  const inputs = collectInputs(document.querySelectorAll('[data-live="input"]'), modules);
  const formulas = collectFormulas(
    document.querySelectorAll('[data-live="formula"]'),
    inputs,
    modules,
  );
  const order = evaluationOrder(formulas);
  const views = Array.from(document.querySelectorAll('[data-live="view"]'));

  assert(order.length === formulas.size, "every formula is ordered");
  assert(views.every((view) => inputs.has(view.dataset.name) || formulas.has(view.dataset.name)), "every view has a value");

  const values = new Map();
  for (const input of inputs.values()) values.set(input.name, input.default);

  // Coalesce rapid drag events into one recalculation per frame.
  let scheduled = false;
  const update = () => {
    scheduled = false;
    evaluate(order, values);
    render(views, values);
  };
  const schedule = () => {
    if (!scheduled) {
      scheduled = true;
      requestAnimationFrame(update);
    }
  };

  for (const input of inputs.values()) {
    const show = mountNumber(input.element, input.config, (value) => {
      assert(value !== undefined, `input "${input.name}" was set to undefined`);
      values.set(input.name, value);
      show(value);
      schedule();
    });
    assert(typeof show === "function", `input "${input.name}" has no update function`);
    show(input.default);
  }
  update();

  assert(values.size === inputs.size + formulas.size, "every name has a value");
  assert(!scheduled, "the initial render is synchronous");
}

async function loadModules(elements) {
  const modules = new Map();
  for (const element of elements) {
    const name = element.dataset.name;
    assertName(name);
    assert(!modules.has(name), `duplicate module "${name}"`);

    // Blob imports give post code module semantics without exposing globals.
    const url = URL.createObjectURL(new Blob([element.dataset.source], { type: "text/javascript" }));
    try {
      modules.set(name, await import(url));
    } finally {
      URL.revokeObjectURL(url);
    }
  }

  assert(modules.size === elements.length, "every module loaded");
  assert(Array.from(modules.values()).every((module) => typeof module === "object"), "every module has exports");
  return modules;
}

function collectInputs(elements, modules) {
  const inputs = new Map();
  for (const element of elements) {
    const name = element.dataset.name;
    assertName(name);
    assert(!inputs.has(name), `duplicate input "${name}"`);
    assert(!modules.has(name), `input "${name}" shadows a module`);
    assert(element.dataset.kind === "number", `unknown input kind "${element.dataset.kind}"`);

    const input = {
      name,
      element,
      default: JSON.parse(element.dataset.default),
      config: JSON.parse(element.dataset.config),
    };
    assert(typeof input.default === "number", `input "${name}" has a non-number default`);
    assert(typeof input.config === "object" && input.config !== null, `input "${name}" has invalid config`);
    inputs.set(name, input);
  }

  assert(inputs.size === elements.length, "every input was collected");
  assert(Array.from(inputs.values()).every((input) => input.element instanceof Element), "every input has an element");
  return inputs;
}

function collectFormulas(elements, inputs, modules) {
  assert(elements.length <= MAX_FORMULAS, "too many formulas on one page");
  assert(inputs instanceof Map, "formula inputs are collected");

  const scopeNames = Array.from(modules.keys());
  const scopeValues = Array.from(modules.values());
  const formulas = new Map();
  for (const element of elements) {
    const name = element.dataset.name;
    assertName(name);
    assert(!formulas.has(name), `duplicate formula "${name}"`);
    assert(!inputs.has(name), `formula "${name}" shadows an input`);
    assert(!modules.has(name), `formula "${name}" shadows a module`);

    const fn = new Function(
      ...scopeNames,
      `"use strict"; return (${element.dataset.source});`,
    )(...scopeValues);
    assert(typeof fn === "function", `formula "${name}" is not a function`);
    assert(fn.constructor.name !== "AsyncFunction", `formula "${name}" must be synchronous`);
    formulas.set(name, { name, fn, reads: parameterNames(fn, name) });
  }

  for (const formula of formulas.values()) {
    for (const read of formula.reads) {
      assert(inputs.has(read) || formulas.has(read), `formula "${formula.name}" reads unknown name "${read}"`);
    }
  }
  assert(formulas.size === elements.length, "every formula was collected");
  return formulas;
}

// A formula's parameter names are its dependencies.
function parameterNames(fn, name) {
  assert(typeof fn === "function", `formula "${name}" is callable`);
  assertName(name);

  const source = fn.toString();
  const parenthesized = source.match(/^(?:function\b[^(]*)?\(([^)]*)\)/);
  const single = source.match(/^([A-Za-z_$][\w$]*)\s*=>/);
  assert(parenthesized !== null || single !== null, `formula "${name}" has unreadable parameters`);

  const names = parenthesized === null
    ? [single[1]]
    : parenthesized[1].split(",").map((part) => part.trim()).filter(Boolean);
  assert(names.length === fn.length, `formula "${name}" uses default or rest parameters`);
  assert(new Set(names).size === names.length, `formula "${name}" repeats a parameter`);
  for (const parameter of names) {
    assert(IDENTIFIER.test(parameter), `formula "${name}" has invalid parameter "${parameter}"`);
  }
  return names;
}

function evaluationOrder(formulas) {
  assert(formulas instanceof Map, "formulas are collected");
  assert(formulas.size <= MAX_FORMULAS, "formula count is bounded");

  const pending = new Map();
  for (const formula of formulas.values()) {
    pending.set(formula.name, formula.reads.filter((read) => formulas.has(read)).length);
  }

  const order = [];
  const ready = Array.from(formulas.values()).filter((formula) => pending.get(formula.name) === 0);
  for (let index = 0; index < ready.length; index += 1) {
    assert(index < formulas.size, "evaluation order is bounded");
    const formula = ready[index];
    order.push(formula);
    for (const dependent of formulas.values()) {
      if (dependent.reads.includes(formula.name)) {
        const remaining = pending.get(dependent.name) - 1;
        pending.set(dependent.name, remaining);
        if (remaining === 0) ready.push(dependent);
      }
    }
  }

  const ordered = new Set(order.map((formula) => formula.name));
  const cycle = Array.from(formulas.keys()).filter((name) => !ordered.has(name));
  assert(cycle.length === 0, `formula cycle: ${cycle.join(", ")}`);
  assert(order.length === formulas.size, "every formula has an evaluation position");
  return order;
}

function evaluate(order, values) {
  assert(Array.isArray(order), "evaluation order is an array");
  assert(values instanceof Map, "values are stored in a map");

  for (const formula of order) {
    const args = formula.reads.map((read) => values.get(read));
    assert(args.every((value) => value !== undefined), `formula "${formula.name}" ran before its inputs`);

    const failed = args.find((value) => value instanceof Error);
    if (failed instanceof Error) {
      values.set(formula.name, failed);
    } else {
      let result;
      try {
        result = formula.fn(...args);
      } catch (error) {
        console.error(`live: formula "${formula.name}" failed`, error);
        result = error instanceof Error ? error : new Error(String(error));
      }
      values.set(
        formula.name,
        result === undefined ? new Error(`formula "${formula.name}" returned undefined`) : result,
      );
    }
    assert(values.get(formula.name) !== undefined, `formula "${formula.name}" has a value`);
  }

  assert(order.every((formula) => values.has(formula.name)), "every formula was evaluated");
}

function render(views, values) {
  assert(Array.isArray(views), "views are collected");
  assert(values instanceof Map, "render values are stored in a map");

  for (const view of views) {
    const value = pick(values.get(view.dataset.name), JSON.parse(view.dataset.index));
    const node = view.namespaceURI === MATHML
      ? mathNode(value, view.dataset)
      : document.createTextNode(htmlText(value, view.dataset));
    view.replaceChildren(node);
    view.classList.toggle("live-error", value instanceof Error);
  }

  assert(views.every((view) => view.childNodes.length === 1), "every view rendered one value");
}

function pick(value, path) {
  assert(Array.isArray(path), "a view index is an array");
  assert(path.every(Number.isInteger), "view indices are integers");

  let picked = value;
  for (const index of path) {
    if (picked instanceof Error) return picked;
    if (!Array.isArray(picked) || index < 0 || index >= picked.length) {
      return new Error(`index ${path.join(", ")} is out of range`);
    }
    picked = picked[index];
  }
  return picked;
}

function htmlText(value, options) {
  assert(options.format === "fixed" || options.format === "percent", "view format is valid");
  assert(Number.isInteger(Number(options.digits)), "view digits are an integer");

  if (value instanceof Error) return `[${value.message}]`;
  if (typeof value === "number") return formatNumber(value, options.format, Number(options.digits));
  if (typeof value === "string" || typeof value === "boolean") return String(value);
  if (Array.isArray(value)) return value.map((item) => htmlText(item, options)).join(", ");
  return `[cannot show ${typeof value} as text]`;
}

function mathNode(value, options) {
  assert(options.format === "fixed" || options.format === "percent", "math view format is valid");
  assert(Number.isInteger(Number(options.digits)), "math view digits are an integer");

  if (value instanceof Error) return mathElement("mtext", [`[${value.message}]`]);
  if (typeof value === "number") return mathNumber(value, options);
  if (typeof value === "string" || typeof value === "boolean") {
    return mathElement("mtext", [String(value)]);
  }
  if (Array.isArray(value) && value.length > 0) {
    const rows = value.every(Array.isArray) ? value : value.map((item) => [item]);
    assert(rows.every((row) => row.length === rows[0].length), "matrix rows have equal lengths");
    return mathMatrix(rows, options);
  }
  return mathElement("mtext", [`[cannot show ${typeof value} in math]`]);
}

function mathNumber(value, options) {
  assert(typeof value === "number", "math number is numeric");
  assert(options !== null, "math number has options");

  const text = formatNumber(Math.abs(value), options.format, Number(options.digits));
  if (value < 0 && Number(text.replace("%", "")) !== 0) {
    return mathElement("mrow", [mathElement("mo", ["−"]), mathElement("mn", [text])]);
  }
  return mathElement("mn", [text]);
}

function mathMatrix(rows, options) {
  assert(rows.length > 0, "matrix has rows");
  assert(rows.every(Array.isArray), "matrix rows are arrays");

  const table = mathElement("mtable", rows.map((row) =>
    mathElement("mtr", row.map((item) => mathElement("mtd", [mathNode(item, options)])))));
  const closing = { "[": "]", "(": ")" }[options.delim];
  if (closing === undefined) {
    assert(options.delim === "", `unknown delimiter "${options.delim}"`);
    return table;
  }
  return mathElement("mrow", [
    mathElement("mo", [options.delim]),
    table,
    mathElement("mo", [closing]),
  ]);
}

function mathElement(tag, children) {
  assert(typeof tag === "string" && tag !== "", "MathML tag is non-empty");
  assert(Array.isArray(children), "MathML children are an array");

  const element = document.createElementNS(MATHML, tag);
  element.append(...children);
  assert(element.childNodes.length === children.length, "every MathML child was appended");
  return element;
}

function formatNumber(value, format, digits) {
  assert(format === "fixed" || format === "percent", `unknown format "${format}"`);
  assert(Number.isInteger(digits) && digits >= 0, "digits are non-negative");

  if (!Number.isFinite(value)) return value > 0 ? "∞" : value < 0 ? "−∞" : "NaN";
  const scaled = format === "percent" ? value * 100 : value;
  let text = scaled.toFixed(digits);
  if (Number(text) === 0) text = text.replace("-", "");
  text = text.replace("-", "−");

  assert(text.length > 0, "formatted number is non-empty");
  assert(!text.endsWith("."), "formatted number has no trailing point");
  return format === "percent" ? `${text}%` : text;
}

function mountNumber(element, config, set) {
  assert(element instanceof Element, "number input has an element");
  assert(typeof set === "function", "number input has a setter");
  assert(config.min < config.max, "number input range is non-empty");
  assert(config.step > 0, "number input step is positive");

  const steps = Math.round((config.max - config.min) / config.step);
  const pixelsPerStep = Math.min(20, Math.max(2, DRAG_PIXELS / steps));
  assert(steps > 0, "number input has at least one step");
  assert(Number.isFinite(pixelsPerStep), "drag scale is finite");

  let current = config.min;
  const change = (value) => {
    const clamped = Math.min(config.max, Math.max(config.min, value));
    const step = Math.round((clamped - config.min) / config.step);
    // Trim floating-point drift after snapping to the step grid.
    const snapped = Number((config.min + step * config.step).toFixed(10));
    if (snapped !== current) set(snapped);
  };

  element.tabIndex = 0;
  element.setAttribute("role", "slider");
  element.setAttribute("aria-valuemin", String(config.min));
  element.setAttribute("aria-valuemax", String(config.max));
  element.classList.add("is-live");

  let drag = null;
  element.addEventListener("pointerdown", (event) => {
    element.setPointerCapture(event.pointerId);
    drag = { x: event.clientX, value: current };
    element.classList.add("is-dragging");
    event.preventDefault();
  });
  element.addEventListener("pointermove", (event) => {
    if (drag !== null) {
      const offset = Math.round((event.clientX - drag.x) / pixelsPerStep);
      change(drag.value + offset * config.step);
    }
  });
  const release = () => {
    drag = null;
    element.classList.remove("is-dragging");
  };
  element.addEventListener("pointerup", release);
  element.addEventListener("pointercancel", release);

  const keySteps = {
    ArrowRight: 1,
    ArrowUp: 1,
    ArrowLeft: -1,
    ArrowDown: -1,
    PageUp: PAGE_STEPS,
    PageDown: -PAGE_STEPS,
    Home: -steps,
    End: steps,
  };
  element.addEventListener("keydown", (event) => {
    const delta = keySteps[event.key];
    if (delta !== undefined) {
      change(current + delta * config.step);
      event.preventDefault();
    }
  });

  return (value) => {
    assert(typeof value === "number", "number input value is numeric");
    assert(value >= config.min && value <= config.max, "number input value is in range");

    current = value;
    const text = formatNumber(value, config.format, config.digits);
    element.textContent = text;
    element.setAttribute("aria-valuenow", String(value));
    element.setAttribute("aria-valuetext", text);
  };
}

function assertName(name) {
  assert(typeof name === "string", "live name is a string");
  assert(IDENTIFIER.test(name), `invalid live name "${name}"`);
}

function assert(condition, message) {
  if (!condition) throw new Error(`live: ${message}`);
}

main().catch((error) => {
  console.error(error);
  const message = error instanceof Error ? error.message : String(error);
  const views = document.querySelectorAll('[data-live="view"]');
  for (const view of views) {
    view.textContent = `[${message}]`;
    view.classList.add("live-error");
  }
});
