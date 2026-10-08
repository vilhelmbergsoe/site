#let post = (
  title: "Notes on portfolio optimization",
  date: datetime(year: 2025, month: 8, day: 7),
  archived: false,
  tags: ("economics", "investment", "optimization"),
)

#set document(title: post.title, date: post.date, keywords: post.tags)
#metadata(post) <post-meta>

#import "/typst/post.typ": anchored-headings
#show: anchored-headings

= Starting with a Single Asset
<starting-with-a-single-asset>
Suppose we have some asset with a random return $r$. Its expected
return is $mu = bb(E)[r]$, and its variance is $sigma^2 =
upright("Var")(r)$. Our goal is to find the optimal allocation of
capital $w$ that balances expected return against variance.

This is described using our #strong[objective function]:

$ max_w quad w mu - lambda / 2 sigma^2 w^2 $

There's quite a lot happening here, so let's break it down.

The squared exposure $w^2$ confused me quite a bit the first time
'round, but here's how it pops up: When we scale the random return $r$
by $w$, we get a new random return $w r$. The variance of this scaled
return is given by:

$ upright("Var") \( w r \) = bb(E) [(w r - bb(E) [w r])^2] $

Since $w$ is a constant, we can factor it out:
$ = bb(E) [(w (r - bb(E) [r]))^2] $
$ = w^2 dot.op bb(E) [(r - bb(E) [r])^2] $

The last term is simply the variance of $r$: $ = w^2 sigma^2 $

The expected return scales linearly instead:

$ bb(E)[w r] = w bb(E)[r] = w mu $

So doubling $w$ doubles our expected return but quadruples
variance. In our model the reward grows linearly while the variance
penalty grows quadratically which means that, at some point, taking on
more exposure hurts the objective more than it helps.

#quote(block: true)[Using variance as our measure of risk is a modelling choice
 from mean-variance optimization; it makes this a quadratic
 optimization problem with a nice solution]

Now back to our objective function: the term $lambda / 2 sigma^2 w^2$
represents our variance penalty with a risk-aversion parameter
$lambda$ which controls how heavily variance is penalized.

#quote(block: true)[
The $1 / 2$ in front of the risk term is really just to give us a
cleaner derivative, as we'll see later.
]

Now to find the optimal exposure $w$ we need to take the derivative
of the objective function with respect to $w$, set it to zero, and
solve for $w$:

$ frac(partial , partial w) \( w mu - lambda / 2 sigma^2 w^2 \) = 0 $

$ mu - lambda sigma^2 w = 0 $

$ w^* = mu / (lambda sigma^2) $

So the optimal exposure to our single asset is given by
$frac(mu,lambda sigma^2)$. This makes good intuitive sense, as we want more
exposure to assets with high expected returns and low variance, and
less exposure when we are more risk-averse.

But what if we instead have multiple assets to choose from? How do we optimize
the allocation across multiple assets?

= Adding a Second Asset
<adding-a-second-asset>
We've seen how to optimize the allocation to a #strong[single asset];,
where the trade-off is between expected return and variance. The optimal
allocation was determined by balancing the expected return $mu$ against
the variance $sigma^2$ of the asset. But what happens when we have two
or more assets to choose from?

Let's start small, with just two assets, Asset A and Asset B. Their
random returns are $r_A$ and $r_B$, with expected returns $mu_A$ and
$mu_B$ and variances $sigma_A^2$ and $sigma_B^2$, just like the
single-asset case. If we want to allocate our capital between these two
assets, we'll assign weights $w_A$ and $w_B$. The random portfolio return
is the weighted sum:

$ r_p = w_A r_A + w_B r_B $

Its expected return is therefore:

$ bb(E)[r_p] = w_A mu_A + w_B mu_B $

This is pretty straightforward: the more you allocate to an asset, the
more its expected return contributes to the portfolio's overall
expected return.

Here's where things start to get fun. You might assume that the
portfolio variance is just the sum of the individual variance terms:

$ sigma_p^2 = w_A^2 sigma_A^2 + w_B^2 sigma_B^2 $

And at first, this does seems reasonable. But it doesn't tell us
whether the two assets tend to move in relation to eachother.

Imagine that Asset A and Asset B are positively correlated: when Asset
A goes up, Asset B will tend to go up too. This will result in a
portfolio with a higher risk profile as both assets are likely to
experience both gains, and losses at the same time, reinforcing each
other's variance.

Now imagine that Asset A and Asset B tend to move in opposite
directions (i.e., they have negative correlation), the portfolio's
overall risk will be lower. This is because the losses in one asset
might be offset by gains in the other, reducing the total variance
of the portfolio.

To derive the formula for the variance of a portfolio with multiple
assets, we begin with the definition of variance. For any random
variable, the variance measures the spread of its possible outcomes
around its mean. In the case of a portfolio, the variance of the random
portfolio return $r_p$ is given by:

$ sigma_p^2 = bb(E) [(r_p - bb(E) \[ r_p \])^2] $

If our portfolio consists of two assets, the total return of the
portfolio is a weighted sum of the individual random returns.
Specifically, if $w_A$ and $w_B$ represent the weights of Asset A and
Asset B respectively, then the portfolio return is, as described
above:

$ r_p = w_A r_A + w_B r_B $

Substituting this into the variance formula, we get:

$ sigma_p^2 = bb(E) [(w_A r_A + w_B r_B - bb(E) \[ w_A r_A + w_B r_B \])^2] $

Since $w_A$ and $w_B$ are constants, we can factor them out of the
expectation, giving us:

$ sigma_p^2 = bb(E) [(w_A \( r_A - bb(E) \[ r_A \] \) + w_B \( r_B - bb(E) \[ r_B \] \))^2] $

At this point, we can expand the square inside the expectation, which
gives us three terms:

$ sigma_p^2 = bb(E) [w_A^2 \( r_A - bb(E) \[ r_A \] \)^2 + w_B^2 \( r_B - bb(E) \[ r_B \] \)^2 + 2 w_A w_B \( r_A - bb(E) \[ r_A \] \) \( r_B - bb(E) \[ r_B \] \)] $

The first two terms correspond to our individual variances of each
asset scaled by the square of their respective weights like we showed
earlier with the single-asset case.

The third term, $2 w_A w_B \( r_A - bb(E) \[ r_A \] \) \( r_B - bb(E)
\[ r_B \] \)$, measures how the returns of the two assets move
together. This is the covariance between the returns of Asset A and
Asset B. By definition:

$ upright("Cov") \( A \, B \) = bb(E) [\( r_A - bb(E) \[ r_A \] \) \( r_B - bb(E) \[ r_B \] \)] $

Therefore our cross-term becomes:

$ 2 w_A w_B upright("Cov") \( A \, B \) $

Now combining everything we get:

$ sigma_p^2 = w_A^2 sigma_A^2 + w_B^2 sigma_B^2 + 2 w_A w_B upright("Cov") \( A \, B \) $

This formula shows that our portfolio variance depends on both the
individual variances of each asset but also on how the returns of the
assets interact, as captured by our covariance term.

This is why diversification works: If the assets are not perfectly
correlated, the covariance term can reduce (or increase) overall
portfolio risk, even while the portfolio maintains a positive expected
return.

= Generalizing to N Assets
<generalizing-to-n-assets>
Our formula for just two assets is already getting a bit long. Imagine
trying to write this out for a portfolio of 100 assets! We'll use some
basic linear algebra to express these concepts in a much cleaner way.

For $N$ assets we collect their random returns and our chosen weights
into two column vectors:

$ upright(bold(r)) = mat(delim: "[", r_1; r_2; dots.v; r_N), quad upright(bold(w)) = mat(delim: "[", w_1; w_2; dots.v; w_N) $

Taking the expected value of each return gives us the expected-return
vector:

$ upright(bold(mu)) = bb(E)[upright(bold(r))] = mat(delim: "[", mu_1; mu_2; dots.v; mu_N) $

We then collect all the variances and covariances in the covariance
matrix $upright(bold(Sigma))$, whose entries are:

$ upright(bold(Sigma))_(i j) = upright("Cov") \( r_i, r_j \) $

The diagonal entries are the individual asset variances, while the
off-diagonal entries show how each pair of assets moves together. Since
$upright("Cov") \( r_i, r_j \) = upright("Cov") \( r_j, r_i \)$, the
covariance matrix is symmetric.

From this, our portfolio equations follow:

$ r_p = upright(bold(w))^T upright(bold(r)), quad bb(E)[r_p] = upright(bold(w))^T upright(bold(mu)) $

$ upright("Var")(r_p) = upright(bold(w))^T upright(bold(Sigma)) upright(bold(w)) $

Expanding $upright(bold(w))^T upright(bold(Sigma)) upright(bold(w))$
gives us the same individual variance and pairwise covariance terms we
derived above, only now the notation works for any number of assets.

== Solving for the Weights
<solving-for-the-weights>

Now we can write the $N$-asset optimization problem:

$ max_(upright(bold(w))) quad upright(bold(w))^T upright(bold(mu)) - lambda / 2 upright(bold(w))^T upright(bold(Sigma)) upright(bold(w)) $

This should look familiar! It's the multi-asset version of our earlier
single-asset objective function. Just like the single-asset problem,
$lambda$ controls how heavily we penalize variance.

If we leave the weights unrestricted, we can differentiate the
objective and set the result to zero. Since $upright(bold(Sigma))$ is
symmetric, this gives:

$ upright(bold(mu)) - lambda upright(bold(Sigma)) upright(bold(w)) = 0 $

Assuming the covariance matrix is invertible, solving for the weights
gives:

$ upright(bold(w))^* = 1 / lambda upright(bold(Sigma))^(-1) upright(bold(mu)) $

This is the multi-asset equivalent of our single-asset solution $w^* =
mu / (lambda sigma^2).$

This solution, however, is unrestricted meaning the weights can add up
to more or less than one. That can be useful when forms of borrowing
is allowed, but suppose we instead want a fully invested portfolio:

$ upright(bold(1))^T upright(bold(w)) = 1 $

To include this constraint, we introduce a Lagrange multiplier $gamma$:

$ cal(L)(upright(bold(w)), gamma) = upright(bold(w))^T upright(bold(mu)) - lambda / 2 upright(bold(w))^T upright(bold(Sigma)) upright(bold(w)) + gamma (1 - upright(bold(1))^T upright(bold(w)) upright(bold(1))) $

We then differentiate with respect to both $upright(bold(w))$ and
   $gamma$:

   $ frac(partial cal(L), partial upright(bold(w)))
     &=
       upright(bold(mu))
       - lambda upright(bold(Sigma)) upright(bold(w))
       - gamma upright(bold(1))
       = 0 \

   frac(partial cal(L), partial gamma)
     &=
       1 - upright(bold(1))^T upright(bold(w))
       = 0 $

The first equation gives us:

   $ upright(bold(w))^*
     = 1 / lambda
       upright(bold(Sigma))^(-1)
       \( upright(bold(mu)) - gamma upright(bold(1)) \) $

   We choose $gamma$ so that the weights satisfy our full-investment
   constraint:

   $ gamma
     = frac(
         upright(bold(1))^T
         upright(bold(Sigma))^(-1)
         upright(bold(mu))
         - lambda,
         upright(bold(1))^T
         upright(bold(Sigma))^(-1)
         upright(bold(1)),
       ) $

Together, these equations give us the portfolio that maximizes our
mean-variance objective while investing exactly 100% of our capital.

The constraint still allows individual weights to be negative, meaning
that short positions are permitted. Preventing short selling requires
the additional constraints $w_i >= 0$, which generally means solving
the problem numerically.

Now, let's put some numbers into this and see how the calculation works.

== Try It Yourself
<try-it-yourself>
#import "/typst/live.typ": live, input, formula, view, module
#show: live

Let's use two imaginary stocks. TechCorp has an
expected return of #input.number("mu_a", 0.10, min: -0.10, max: 0.30, step: 0.005, format: "percent", digits: 1)
and a standard deviation of #input.number("sigma_a", 0.20, min: 0.01, max: 0.50, step: 0.01, format: "percent", digits: 0).
GlobalGoods has an expected return of #input.number("mu_b", 0.06, min: -0.10, max: 0.30, step: 0.005, format: "percent", digits: 1)
and a standard deviation of #input.number("sigma_b", 0.15, min: 0.01, max: 0.50, step: 0.01, format: "percent", digits: 0).
Their correlation is #input.number("rho", 0.30, min: -0.95, max: 0.95, step: 0.05, digits: 2),
and we'll use a risk-aversion parameter of
#input.number("lambda", 2.0, min: 0.5, max: 10, step: 0.1, digits: 1).

Drag any of the blue values to see the calculation update.

#module("portfolio", ```js
export function calculate(mu_a, sigma_a, mu_b, sigma_b, rho, lambda) {
  const inputs = [mu_a, sigma_a, mu_b, sigma_b, rho, lambda];
  if (!inputs.every(Number.isFinite)) throw new Error("every input must be finite");
  if (sigma_a <= 0 || sigma_b <= 0) throw new Error("standard deviations must be positive");
  if (rho <= -1 || rho >= 1) throw new Error("correlation must lie between -1 and 1");
  if (lambda <= 0) throw new Error("risk aversion must be positive");

  const covariance = rho * sigma_a * sigma_b;
  const variance_a = sigma_a * sigma_a;
  const variance_b = sigma_b * sigma_b;
  const determinant = variance_a * variance_b - covariance * covariance;
  if (!(determinant > 0)) throw new Error("the covariance matrix must be invertible");

  const sigma = [
    [variance_a, covariance],
    [covariance, variance_b],
  ];
  const inverse = [
    [variance_b / determinant, -covariance / determinant],
    [-covariance / determinant, variance_a / determinant],
  ];
  const sigma_inv_mu = [
    inverse[0][0] * mu_a + inverse[0][1] * mu_b,
    inverse[1][0] * mu_a + inverse[1][1] * mu_b,
  ];
  const sigma_inv_one = [
    inverse[0][0] + inverse[0][1],
    inverse[1][0] + inverse[1][1],
  ];
  const denominator = sigma_inv_one[0] + sigma_inv_one[1];
  if (!(denominator > 0)) throw new Error("the full-investment constraint has no solution");

  const gamma = (sigma_inv_mu[0] + sigma_inv_mu[1] - lambda) / denominator;
  const weights = [
    (sigma_inv_mu[0] - gamma * sigma_inv_one[0]) / lambda,
    (sigma_inv_mu[1] - gamma * sigma_inv_one[1]) / lambda,
  ];
  const weight_sum = weights[0] + weights[1];
  if (!weights.every(Number.isFinite)) throw new Error("the portfolio weights are not finite");
  if (Math.abs(weight_sum - 1) > 1e-9) throw new Error("the portfolio weights do not sum to one");

  const expected_return = weights[0] * mu_a + weights[1] * mu_b;
  const variance =
    weights[0] * weights[0] * variance_a
    + 2 * weights[0] * weights[1] * covariance
    + weights[1] * weights[1] * variance_b;
  if (!Number.isFinite(expected_return)) throw new Error("the portfolio return is not finite");
  if (!Number.isFinite(variance) || variance < 0) throw new Error("the portfolio variance is invalid");

  return [
    [mu_a, mu_b],
    covariance,
    sigma,
    inverse,
    sigma_inv_mu,
    sigma_inv_one,
    gamma,
    weights,
    expected_return,
    variance,
  ];
}
```)

#formula("calculation", "portfolio.calculate")

First, the covariance and our two input matrices are:

$ upright("Cov") \( A, B \) = #view("calculation", index: 1, digits: 4) $

$ upright(bold(mu)) = #view("calculation", index: 0, format: "percent", digits: 1) quad
  upright(bold(Sigma)) = #view("calculation", index: 2, digits: 4) $

The inverse covariance matrix is:

$ upright(bold(Sigma))^(-1) = #view("calculation", index: 3, digits: 2) $

The remaining values in the constrained solution are:

$ upright(bold(Sigma))^(-1) upright(bold(mu))
  = #view("calculation", index: 4, digits: 3) $

$ upright(bold(Sigma))^(-1) upright(bold(1))
  = #view("calculation", index: 5, digits: 3) quad
  gamma = #view("calculation", index: 6, digits: 4) $

Finally, the fully invested portfolio is:

$ upright(bold(w))^* = #view("calculation", index: 7, format: "percent", digits: 1) $

Its expected return is #view("calculation", index: 8, format: "percent", digits: 2)
and its variance is #view("calculation", index: 9, digits: 4).

This puts #view("calculation", index: (7, 0), format: "percent",
digits: 1) in TechCorp and #view("calculation", index: (7, 1), format:
"percent", digits: 1) in GlobalGoods. Changing any of the assumptions
above runs the same calculation again immediately.

And that's pretty much the core of it! We started with a single asset,
introduced covariance for multiple assets, and ended up with a compact
way to optimize an entire portfolio. Thanks for reading.
