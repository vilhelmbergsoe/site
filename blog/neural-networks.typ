#let post = (
  title: "Introduction to Neural Networks and Backpropagation",
  date: datetime(year: 2024, month: 4, day: 30),
  archived: false,
  tags: ("machine learning", "mathematics"),
)

#set document(title: post.title, date: post.date, keywords: post.tags)
#metadata(post) <post-meta>

#import "/typst/post.typ": anchored-headings
#import "@preview/cetz:0.5.0"

#show: anchored-headings

#let diagram-stroke = 0.55pt
#let node-text-size = 9.5pt
#let arrowhead = (end: ">", fill: black, scale: 0.72)

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

#quote(block: true)[
I originally wrote these notes as a way to make the concepts more
concrete for myself.
]

Neural networks are a family of function approximators built by
composing layers of relatively simple mathematical operations. Their
name comes from a loose analogy with biological neurons, but modern
neural networks should not be mistaken for realistic models of the
brain.

Despite the simplicity of their individual parts, neural networks can
approximate complicated relationships in data. They are used for tasks
such as image classification, recommendation systems, sequence
modelling and many other kinds of function approximation.

In this post, I'll build up a small fully connected neural network from
individual neurons, describe how information moves through it and then
derive the backpropagation algorithm used to calculate its gradients.
Finally, we'll use those gradients to train the network with gradient
descent.

= An introduction to Neural Networks
<an-introduction-to-neural-networks>
== Perceptron (single-neuron function)
<perceptron-single-neuron-function>
One influential early model was Frank Rosenblatt's perceptron,
introduced in the late 1950s in the paper #emph["The Perceptron: A
probabilistic model for information storage and organization in the
brain"] #footnote[Rosenblatt, F. 1958. #emph[“The perceptron: A
probabilistic model for information storage and organization in the
brain”];. Psychological Review 65 (6): 386--408.
#link("https://doi.org/10.1037/h0042519");.];. A perceptron combines
several inputs into a weighted sum and applies a threshold to produce
its output. Modern artificial neurons use the same broad pattern,
although they commonly use differentiable activation functions instead
of a hard threshold.

A perceptron receives $n$ inputs $x_1, x_2, dots.h, x_n$. Each input
$x_i$ has a corresponding adjustable weight $w_i$. Together with a bias
$b$, these values form a weighted sum of the inputs.

#figure(
  drawing(
    "Inputs pass through their weights into a weighted sum together with a bias, followed by an activation function",
    length: 1.25cm,
    {
      import cetz.draw: *

      // Draw connections first so that the nodes cover their endpoints.
      line((0.42, 3), (2.18, 3), mark: arrowhead, stroke: diagram-stroke)
      line((0.42, 2), (2.18, 2), mark: arrowhead, stroke: diagram-stroke)
      line((0.42, 0.4), (2.18, 0.4), mark: arrowhead, stroke: diagram-stroke)
      line((2.97, 2.8), (4.93, 1.7), mark: arrowhead, stroke: diagram-stroke)
      line((3.01, 1.92), (4.89, 1.58), mark: arrowhead, stroke: diagram-stroke)
      line((2.99, 0.56), (4.91, 1.34), mark: arrowhead, stroke: diagram-stroke)
      line((2.9, -0.9), (5, 1.2), mark: arrowhead, stroke: diagram-stroke)
      line((5.72, 1.5), (7.58, 1.5), mark: arrowhead, stroke: diagram-stroke)

      for node in (
        ((0, 3), [$x_1$]),
        ((0, 2), [$x_2$]),
        ((0, 0.4), [$x_n$]),
      ) {
        circle(node.at(0), radius: 0.42, fill: white, stroke: diagram-stroke)
        content(node.at(0), text(size: node-text-size, node.at(1)))
      }
      content((0, 1.2), text(size: 16pt)[$dots.v$])

      for node in (
        ((2.6, 3), [$w_1$]),
        ((2.6, 2), [$w_2$]),
        ((2.6, 0.4), [$w_n$]),
        ((2.6, -1.2), [$b$]),
        ((5.3, 1.5), [$sum$]),
        ((8, 1.5), [$sigma$]),
      ) {
        circle(node.at(0), radius: 0.42, fill: white, stroke: diagram-stroke)
        content(node.at(0), text(size: node-text-size, node.at(1)))
      }
      content((2.6, 1.2), text(size: 16pt)[$dots.v$])
    },
  ),
  caption: [The weighted sum and activation of a single neuron.],
)

This means multiplying every input $x_i$ by its corresponding weight
$w_i$, summing the results and adding the bias. The resulting scalar is
the pre-activation $z$:

$ z = sum_(i = 1)^n w_i x_i + b $

where $x_i$ is the $i$-th input, $w_i$ is its weight, $b$ is the bias
and $n$ is the number of inputs.

In Rosenblatt's formulation, this value is passed through a step
function. The perceptron outputs $1$ if $z$ is greater than $0$ and $0$
otherwise:

$ a = cases(delim: "{", 1 & upright(" if ") z > 0, 0 & upright("else")) $

Modern artificial neurons keep the same weighted sum but replace the
hard threshold with a more general activation function $sigma$:

$ a = sigma \( z \) $

Here, $a$ is the neuron's scalar output, or activation. Common choices
for $sigma$ include ReLU, sigmoid and hyperbolic tangent. They all
introduce nonlinearity, but they have different ranges and behave
differently during training.

This becomes cumbersome notation for when we need to describe larger
networks with multiple layers or multiple neurons per layer. For this
reason we can make use of vector and matrix notation to better organise
our data.

Now we can instead represent both $x$ and $w$ as column-vectors.

$ x = mat(delim: "[", x_1; x_2; dots.v; x_n) quad w = mat(delim: "[", w_1; w_2; dots.v; w_n) $

Because both are column vectors, the weighted sum can be written as
$w^T x$. We then add the bias and apply the activation function:

$ z = w^T x + b $

$ a = sigma \( z \) $

So, to summarise, a neuron takes an input vector $x$ and produces a
scalar. It calculates a weighted sum of the inputs, adds a bias and
applies a nonlinear activation function.

The nonlinearity is important because it allows a network to model
nonlinear relationships. Without nonlinear activation functions, a
stack of layers would still be equivalent to a single linear
transformation. For example, composing these two linear functions only
produces another linear function:

$ f \( x \) & = 2 x + 1\
g \( x \) & = 3 x - 2\
g \( f \( x \) \) & = g \( 2 x + 1 \)\
 & = 3 \( 2 x + 1 \) - 2\
 & = 6 x + 3 - 2\
 & = 6 x + 1 $

= Multiple neurons and the MLP
<multiple-neurons-and-the-mlp>
A multi-layer perceptron (also called an MLP) is built by arranging
our neurons into layers. In what's called a fully connected layer,
every neuron receives every activation from the previous layer as
input. Each neuron has its own weights and bias so each can produces a
distinct scalar output from the same input vector.

We can collect these individual scalar outputs into a column vector
$a^l$, where $l$ identifies the layer. For example, the two outputs
$s_1$ and $s_2$ in the diagram below form the activation vector
$a^l = mat(delim: "[", s_1; s_2)$.

#figure(
  drawing(
    "Two inputs connect to two neurons whose scalar outputs form the activation vector for the layer",
    display-width: "22.5rem",
    length: 1.5cm,
    {
      import cetz.draw: *

      line((0.46, 2), (1.54, 2), mark: arrowhead, stroke: diagram-stroke)
      line((0.33, 1.67), (1.67, 0.33), mark: arrowhead, stroke: diagram-stroke)
      line((0.33, 0.33), (1.67, 1.67), mark: arrowhead, stroke: diagram-stroke)
      line((0.46, 0), (1.54, 0), mark: arrowhead, stroke: diagram-stroke)
      line((2.4, 1.77), (3.15, 1.35), mark: arrowhead, stroke: diagram-stroke)
      line((2.4, 0.23), (3.15, 0.65), mark: arrowhead, stroke: diagram-stroke)

      for node in (
        ((0, 2), [$x_1$]),
        ((0, 0), [$x_2$]),
      ) {
        circle(node.at(0), radius: 0.46, fill: white, stroke: diagram-stroke)
        content(node.at(0), text(size: node-text-size, node.at(1)))
      }

      for position in ((2, 2), (2, 0)) {
        circle(position, radius: 0.46, fill: white, stroke: diagram-stroke)
      }

      line((3.75, 2.05), (3.55, 2.05), (3.55, -0.05), (3.75, -0.05), stroke: diagram-stroke)
      line((4.15, 2.05), (4.35, 2.05), (4.35, -0.05), (4.15, -0.05), stroke: diagram-stroke)
      content((3.95, 2.5), [$a^l$])
      content((3.95, 1.55), [$s_1$])
      content((3.95, 0.45), [$s_2$])
    },
  ),
  caption: [Two neuron outputs collected into the activation vector $a^l$.],
)

Suppose layer $l - 1$ contains $n$ activations and layer $l$ contains
$m$ neurons. The weights for the whole layer can be represented by a
matrix $W^l$ with $m$ rows and $n$ columns:

$ W^l = mat(delim: "[", w_11^l, w_12^l, dots.h.c, w_(1 n)^l; w_21^l, w_22^l, dots.h.c, w_(2 n)^l; dots.v, dots.v, dots.down, dots.v; w_(m 1)^l, w_(m 2)^l, dots.h.c, w_(m n)^l) $

The entry $w_(j k)^l$ is the weight connecting activation $k$ in the
previous layer to neuron $j$ in layer $l$. The first index therefore
selects a destination neuron, while the second selects an input to that
neuron. Each neuron also has its own bias, collected in the vector

$ b^l = mat(delim: "[", b_1^l; b_2^l; dots.v; b_m^l) . $

This gives us the dimensions

$ a^(l - 1) in RR^n, quad W^l in RR^(m times n), quad b^l in RR^m. $

#figure(
  drawing(
    "A network with two input activations, two hidden activations and one output activation",
    length: 1.25cm,
    {
      import cetz.draw: *

      line((0.42, 2), (2.08, 2), mark: arrowhead, stroke: diagram-stroke)
      line((0.33, 0.26), (2.17, 1.74), mark: arrowhead, stroke: diagram-stroke)
      line((0.33, 1.74), (2.17, 0.26), mark: arrowhead, stroke: diagram-stroke)
      line((0.42, 0), (2.08, 0), mark: arrowhead, stroke: diagram-stroke)
      line((2.89, 1.84), (4.61, 1.16), mark: arrowhead, stroke: diagram-stroke)
      line((2.89, 0.16), (4.61, 0.84), mark: arrowhead, stroke: diagram-stroke)

      for node in (
        ((0, 2), [$x_1$]),
        ((0, 0), [$x_2$]),
        ((2.5, 2), [$a_1^1$]),
        ((2.5, 0), [$a_2^1$]),
        ((5, 1), [$a_1^2$]),
      ) {
        circle(node.at(0), radius: 0.42, fill: white, stroke: diagram-stroke)
        content(node.at(0), text(size: node-text-size, node.at(1)))
      }

      content((1.25, 2.25), text(size: 10pt)[$w_11^1$])
      content((0.78, 0.78), text(size: 10pt)[$w_12^1$])
      content((1.72, 0.78), text(size: 10pt)[$w_21^1$])
      content((1.25, -0.25), text(size: 10pt)[$w_22^1$])
      content((3.75, 1.68), text(size: 10pt)[$w_11^2$])
      content((3.75, 0.32), text(size: 10pt)[$w_12^2$])
    },
  ),
  caption: [A two-layer fully connected network.],
)

The layer first computes a vector of pre-activations $z^l$ and then
applies the activation function elementwise:

$ z^l = W^l a^(l - 1) + b^l $

$ a^l = sigma^l \( z^l \) $

Both $z^l$ and $a^l$ are vectors in $RR^m$. In our network pictured
above, the input vector is $a^0 = x$, the hidden layer produces $a^1$,
and the output layer produces $a^2$.

More generally, an MLP repeats this calculation for layers
$l = 1, dots.h, L$, using the activation from one layer as the input to
the next. Evaluating those layers from $a^0$ through to $a^L$ is called
the forward pass. Training the network requires working in the opposite
direction to determine how each parameter influenced the final error.
That is the purpose of backpropagation.

= Backpropagation and optimization (training)
<backpropagation-and-optimization-training>
Neural networks are trained by adjusting these weights and biases so
that their predictions produce a smaller loss. We do this by first
determining how each parameter contributes to said loss.

Backpropagation utilises the chain rule from differential calculus to
compute the gradient of the loss function, also known as an objective
function, with respect to every weight and bias in the
network. It starts from the output and propagates this information
back through each layer.

An optimization algorithm such as gradient descent then uses the
resulting gradients to update the parameters.

== Loss calculation
<loss-calculation>

A loss function measures how far the network's prediction $a^L$ is
from the desired target $y$, producing a single scalar $C(a^L, y)$. #link("https://en.wikipedia.org/wiki/Mean_squared_error")[Mean squared error] (MLE) is a common choice for regression while classification problems often use a form of #link("https://en.wikipedia.org/wiki/Cross-entropy")[cross entropy].

The choice of loss function determines the gradient at the output
layer. From there, backpropagation then uses the chain rule to
propagate that gradient through the rest of the network.

For each layer $l$, define the pre-activation gradient

$ delta^l := frac(partial C, partial z^l) $

The vector $delta^l$ has the same dimensions as $z^l$. At the output
layer, the chain rule gives

$ delta^L = frac(partial C, partial z^L) = frac(partial C, partial
a^L) dot.o (sigma^L)^' (z^L) $

where $dot.o$ denotes elementwise multiplication.

In order to calculate $delta^l$ for an earlier layer, we follow the computational path from the loss $C$ back to the pre-activations $z^l$, applying the chain rule at each step.

Starting with $delta^L$ at the output layer, we can work backwards through the network to obtain the pre-activation gradient for each preceding layer.

$ delta^l & = frac(partial C, partial z^l)\
 & = frac(partial C, partial a^L) frac(partial a^L, partial z^L) frac(partial z^L, partial a^(L - 1)) dots.h frac(partial z^(l + 1), partial a^l) frac(partial a^l, partial z^l)
 $

#figure(
  drawing(
    "A curved backpropagation path from the cost through a chain of intermediate activations and pre-activations",
    display-width: "34rem",
    length: 1.2cm,
    {
      import cetz.draw: *

      let positions = ((0, 1), (1.8, 1), (3.8, 1), (5.8, 1), (9, 1))
      for index in range(positions.len() - 1) {
        let start = positions.at(index)
        let end = positions.at(index + 1)
        bezier(
          (end.at(0) - 0.43, end.at(1)),
          (start.at(0) + 0.43, start.at(1)),
          ((start.at(0) + end.at(0)) / 2, 0.68),
          mark: arrowhead,
          stroke: (thickness: diagram-stroke, dash: "dashed"),
        )
      }

      bezier(
        (8.7, 1.32),
        (0.3, 1.32),
        (7.6, 3.35),
        (1.4, 3.35),
        mark: arrowhead,
        stroke: diagram-stroke,
      )

      for node in (
        ((0, 1), [$z^l$]),
        ((1.8, 1), [$a^l$]),
        ((3.8, 1), [$z^(l + 1)$]),
        ((5.8, 1), [$a^(l + 1)$]),
        ((9, 1), [$C$]),
      ) {
        circle(node.at(0), radius: 0.43, fill: white, stroke: diagram-stroke)
        content(node.at(0), text(size: node-text-size, node.at(1)))
      }

      content((0.9, 0.18), [$frac(partial a^l, partial z^l)$])
      content((2.8, 0.18), [$frac(partial z^(l + 1), partial a^l)$])
      content((4.8, 0.18), [$frac(partial a^(l + 1), partial z^(l + 1))$])
      content((6.85, 0.18), [$dots.h$])
      content((7.4, 1.22), [$dots.h$])
      content((8.15, 0.18), [$frac(partial C, partial a^(l + 1))$])
      content(
        (4.5, 2.55),
        box(fill: white, inset: 2pt)[$frac(partial C, partial z^l)$],
      )
    },
  ),
  caption: [Backpropagation follows the computational path in reverse.],
)

Rather than expanding the entire chain for every layer, we can simplify
the calculation by only looking at layer $l$ and the following layer
$l + 1$. Suppose the two layers contain $n$ and $m$ neurons,
respectively. Starting with the definition of the next layer's
pre-activation and working backwards through the linear transformation
and activation function gives

$ z^(l + 1)
  & = W^(l + 1) a^l + b^(l + 1) && in RR^m \
frac(partial C, partial a^l)
  & = (W^(l + 1))^T delta^(l + 1) && in RR^n \
delta^l
  & = frac(partial C, partial a^l) dot.o (sigma^l)'(z^l) && in RR^n \
  & = ((W^(l + 1))^T delta^(l + 1)) dot.o (sigma^l)'(z^l) && in RR^n. $

This gives us the backward step: once we know
$delta^(l + 1)$, we can calculate $delta^l$ and continue backwards
through the network. The transposed weight matrix reverses the mapping
from $RR^n arrow.r RR^m$ to $RR^m arrow.r RR^n$.

Finally, the parameter gradients are computed, which are the changes in the loss
function $C$ with respect to the parameters $W^l$ and $b^l$. Looking at
an individual weight and bias first, and then collecting the results,
we get:

$ frac(partial C, partial w_(j k)^l)
  & = frac(partial z_j^l, partial w_(j k)^l) frac(partial C, partial z_j^l) \
  & = a_k^(l - 1) delta_j^l \
frac(partial C, partial W^l)
  & = delta^l (a^(l - 1))^T \
frac(partial C, partial b_j^l)
  & = frac(partial z_j^l, partial b_j^l) frac(partial C, partial z_j^l) \
  & = 1 dot.op delta_j^l = delta_j^l \
frac(partial C, partial b^l)
  & = delta^l. $

Now that we know how each parameter in our neural net influences the
loss, we just need to figure out how to optimize our network to minimise
the loss. Luckily, there is a simple and effective way of doing that as
we've already done the hard part.

= Gradient descent and optimization
<gradient-descent-and-optimization>
The gradient points in the direction in which the loss increases most
quickly. #link("https://en.wikipedia.org/wiki/Gradient_descent")[Gradient descent]
therefore updates each parameter in the opposite direction. Using the
parameter gradients derived above, the
updates for layer $l$ are

$ W^l & arrow.l W^l - eta frac(partial C, partial W^l) \
b^l & arrow.l b^l - eta frac(partial C, partial b^l), $

where $eta$ is the learning rate, which controls the size of each
update.

Ordinary gradient descent calculates these gradients over the entire
training set before performing an update.
#link("https://en.wikipedia.org/wiki/Stochastic_gradient_descent")[Stochastic gradient descent]
(SGD) instead uses a randomly selected training example or, more
commonly, a small batch of examples. Each update is then cheaper to
compute, although the resulting gradient is a noisier estimate of the
full-dataset gradient.

Training basically just consists of repeating these steps: perform a
forward pass, calculate the loss, use backpropagation to calculate the
parameter gradients and finally update the parameters. Repeating this
process allows the network to gradually find parameters that produce a
smaller loss on the training data.
