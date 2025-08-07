---
title: "Notes on portfolio optimization"
date: 07-08-2025
archived: false
tags: [economics, investment, optimization]
---

In the realm of finance, portfolio optimization is the process of selecting the
optimal allocation of assets to maximize returns while minimizing risk. This
blog post shares my notes on the mathematics of portfolio optimization,
starting with single-asset optimization and progressing to more complex
scenarios.

> **Just a heads-up!**
> 
> While I've explored portfolio optimization and have gained a decent grasp of
> the maths involved, these notes are more for my understanding than expert
> advice. So, take it all with a grain of salt! ;)

Let's begin with the simplest problem:

## Optimizing the allocation to a single asset.

Suppose we have an asset with a expected return of $R$ and a
variance of $\sigma^2$. Our goal is to find the optimal allocation
$w$ that minimizes the variance while maximizing the return. This
is described using an **objective function** as follows:

$$\min \quad \frac{1}{2}\sigma^2 w^2 - R w$$

There’s a lot happening here, so let’s break it down.

You might notice the quadratic function form $ax^2+bx+c$; this is intentional.
The $ax^2$ term represents our risk, scaled by the square of the allocation
$w$.

> The squared allocation $w^2$ can be a bit confusing, but here’s why it
> matters: When we scale the asset return $R$ by $w$, we get a new return $wR$.
> The variance of this scaled return is given by:
> 
> $$\text{Var}(wR) = \mathbb{E}\left[\left(wR - \mathbb{E}\left[wR\right]\right)^2\right]$$
> 
> Since $w$ is a constant, we can factor it out:
> $$ = \mathbb{E}\left[\left(w\left(R - \mathbb{E}\left[R\right]\right)\right)^2\right] $$
> $$ = w^2 \cdot \mathbb{E}\left[\left(R - \mathbb{E}\left[R\right]\right)^2\right] $$
> 
> The last term is simply the variance of $R$:
> $$ = w^2 \sigma^2 $$
> 
> Thus, we see that the variance of the scaled return is proportional to the
> square of the allocation.
> 
> **In short:** Increasing your investment in an asset not only boosts returns
> but also raises your exposure to return variability. For example, doubling
> your allocation doesn’t just double the return; it quadruples your risk. This
> quadratic relationship explains the squared term in the variance calculation.


Now, back to our objective function: the coefficient $a$ is
$\frac{1}{2}\sigma^2$, representing the asset's risk.

> The $\frac{1}{2}$ in front of the risk term is really just to give us a
> cleaner derivative as we'll see later

The $bx$ term, on the other hand, represents the return of the
asset, scaled by the allocation $w$. Here, $b$ is equal to $−R$,
which is the negative of the return of the asset. The negation is
used because we want to maximize the return, and the minimization
problem will try to minimize the negative return, effectively
maximizing the actual return.

The $c$ term is absent in this case, as there is no constant term in our
objective function.

Now, to find the optimal allocation $w$, we need to minimize this quadratic
function. We can do this by taking the derivative of the function with respect
to $w$, setting it to zero, and solving for $w$. This will give us the value of
$w$ that minimizes the function.

Let's do that:

$$\frac{\partial{}}{\partial{w}} (\frac{1}{2}\sigma^2w^2 - Rw) = 0$$

$$\sigma^2w - R = 0$$

$$w = \frac{R}{\sigma^2}$$

So, the optimal allocation to the single asset is given by the ratio of the
return to the variance. This makes intuitive sense, as we want to allocate more
to assets with high returns and low risk, and less to assets with low returns
and high risk.

But, what if we have multiple assets to choose from? How do we optimize the
allocation across multiple assets?

## Optimizing a Portfolio of multiple assets

We’ve seen how to optimize the allocation to a **single asset**, where the
trade-off is between return and risk. The optimal allocation was determined by
balancing the expected return $R$ against the variance $\sigma2$ (the risk) of
the asset. But what happens when we have two or more assets to choose from?

### Extending to Two Assets

Let's start small, with just two assets, Asset A and Asset B. Each
asset has its own expected return $R_{A}$, $R_{B}$ and variance
$\sigma^2_{A}$, $\sigma^2_{B}$, just like the single asset case.
If we want to allocate our capital between these two assets, we'll
assign weights $w_{A}$ and $w_{B}$, where the expected return of
the portfolio is a simple weighted average of the expected returns
of the individual assets:

$$R_{p} = w_{A}R_{A} + w_{B}R_{B}$$

This is straightforward: the more you allocate to an asset, the more its return
contributes to the portfolio's overall return.

Here's where things start to get interesting. You might assume that the
portfolio risk is also just the weighted average of the individual risks:

$$\sigma^2_{p} = w^2_{A}\sigma^2_{A} + w^2_{B}\sigma^2_{B}$$

At first glance, this seems reasonable. But it's incomplete.

Why? Because it doesn't take into account how the **two assets
interact**. We need to consider how the returns of Asset A and
Asset B move in relation to each other. This is where
**covariance** comes in.

### Why Covariance Matters

Imagine that Asset A and Asset B are highly **correlated** -- when
Asset A goes up, Asset B tends to go up too. In this case, the
portfolio's risk will be higher because both assets are likely to
experience gains and losses together, reinforcing each other's
volatility.

On the other hand, if Asset A and Asset B tend to move in
**opposite directions** (i.e., they have negative correlation),
the portfolio's overall risk will be lower. This is because the
losses in one asset might be offset by gains in the other,
reducing the total volatility of the portfolio.

### Deriving the formula for variance of multiple assets

To derive the formula for the variance of a portfolio with multiple assets, we
begin with the definition of variance. For any random variable, the variance
measures the spread of its possible outcomes around its mean. In the case of a
portfolio, the variance of the portfolio return, $R_p$, is given by:

$$\sigma^2_p = \mathbb{E}\left[\left(R_p - \mathbb{E}[R_p]\right)^2\right]$$

If our portfolio consists of two assets, the total return of the portfolio is a
weighted sum of the individual asset returns. Specifically, if $w_A$ and $w_B$
represent the weights of Asset A and Asset B, respectively, then the portfolio
return is, like described before:

$$R_p = w_A R_A + w_B R_B$$

Substituting this into the variance formula, we get:

$$\sigma^2_p = \mathbb{E}\left[\left(w_A R_A + w_B R_B - \mathbb{E}[w_A R_A +
w_B R_B]\right)^2\right]$$

Since $w_A$ and $w_B$ are constants, we can factor them out of the expectation,
giving us:

$$\sigma^2_p = \mathbb{E}\left[\left(w_A (R_A - \mathbb{E}[R_A]) + w_B (R_B -
\mathbb{E}[R_B])\right)^2\right]$$

At this stage, we expand the square inside the expectation, which gives us
three terms:

$$\sigma^2_p = \mathbb{E}\left[w_A^2 (R_A - \mathbb{E}[R_A])^2 + w_B^2 (R_B -
\mathbb{E}[R_B])^2 + 2 w_A w_B (R_A - \mathbb{E}[R_A])(R_B -
\mathbb{E}[R_B])\right]$$

The first two terms correspond to the individual variances of each asset,
scaled by the square of their respective weights like we showed earlier with
the single-asset case. Specifically, $w^2 (R - \mathbb{E}[R])^2$ simplifies to
$w^2 \sigma^2$.

The third term, $2 w_A w_B (R_A - \mathbb{E}[R_A])(R_B - \mathbb{E}[R_B])$,
measures how the returns of the two assets move together—specifically, this is
the covariance between the returns of Asset A and Asset B. By definition:

<!--The interesting part comes from the third term, $2 w_A w_B (R_A --->
<!--\mathbb{E}[R_A])(R_B - \mathbb{E}[R_B])$. This term measures how the returns of-->
<!--the two assets move together, and it is precisely the **covariance** between-->
<!--the returns of Asset A and Asset B. By definition, the covariance between two-->
<!--random variables $R_A$ and $R_B$ is:-->

$$\text{Cov}(A, B) = \mathbb{E}\left[(R_A - \mathbb{E}[R_A])(R_B - \mathbb{E}[R_B])\right]$$

The factor of 2 comes from the expansion of $(w_A R_A + w_B R_B)^2$, where the
covariance term appears twice: once for $R_A$ paired with $R_B$ and once for
$R_B$ paired with $R_A$.

Thus, the cross-term becomes:

$$2 w_A w_B \text{Cov}(A, B)$$

Finally, combining all the terms, the variance of the portfolio is:

$$\sigma^2_p = w_A^2 \sigma^2_A + w_B^2 \sigma^2_B + 2 w_A w_B \text{Cov}(A,B)$$

This formula shows that the portfolio variance depends not only on the
individual variances of each asset but also on how the returns of the assets
interact, as captured by the covariance term. This is why diversification works
— if the assets are not perfectly correlated, the covariance term can reduce
overall portfolio risk, even while the portfolio maintains a positive return.



This formula for two assets is already getting a bit long. Imagine trying to write it for a portfolio of 100 assets! This is where the elegance of linear algebra comes to our rescue, allowing us to express these concepts in a much cleaner way.

### Generalizing to N Assets with Matrix Notation

To handle a portfolio with any number of assets, we switch to vectors and matrices. Let's define the key components for a portfolio of $N$ assets:

1. **The Weights Vector ($\mathbf{w}$):** A column vector containing the allocation to each asset.
   $$\mathbf{w} = \begin{bmatrix} w_1 \\ w_2 \\ \vdots \\ w_N \end{bmatrix}$$

2. **The Returns Vector ($\mathbf{R}$):** A column vector of the expected returns for each asset.
   $$\mathbf{R} = \begin{bmatrix} R_1 \\ R_2 \\ \vdots \\ R_N \end{bmatrix}$$

3. **The Covariance Matrix ($\mathbf{\Sigma}$):** An $N \times N$ matrix that captures the variance of each asset and the covariance between each pair of assets.
   $$\mathbf{\Sigma} = \begin{bmatrix}
   \sigma_1^2 & \text{Cov}(1,2) & \cdots & \text{Cov}(1,N) \\
   \text{Cov}(2,1) & \sigma_2^2 & \cdots & \text{Cov}(2,N) \\
   \vdots & \vdots & \ddots & \vdots \\
   \text{Cov}(N,1) & \text{Cov}(N,2) & \cdots & \sigma_N^2
   \end{bmatrix}$$
   > Note that the diagonal of $\mathbf{\Sigma}$ contains the individual asset variances ($\sigma_i^2$), and the off-diagonal elements contain the covariances. Since $\text{Cov}(i,j) = \text{Cov}(j,i)$, the matrix is symmetric.

With this notation, the portfolio's expected return and variance are expressed very neatly:

- **Portfolio Return:** $R_p = \mathbf{w}^T \mathbf{R}$
- **Portfolio Variance:** $\sigma_p^2 = \mathbf{w}^T \mathbf{\Sigma} \mathbf{w}$

This is the exact same math as before, just generalized for $N$ assets. The term $\mathbf{w}^T \mathbf{R}$ is the dot product of the weights and returns, giving us a weighted average. The term $\mathbf{w}^T \mathbf{\Sigma} \mathbf{w}$ is a quadratic form that perfectly captures all the variance and covariance terms.

## The N-Asset Optimization Problem

Now we can formulate the core problem of portfolio optimization. We want to find the best weight vector $\mathbf{w}$. A common way to frame this is as maximizing a utility function that balances return and risk:

$$\max_{\mathbf{w}} \quad \mathbf{w}^T \mathbf{R} - \frac{\lambda}{2} \mathbf{w}^T \mathbf{\Sigma} \mathbf{w}$$

This should look familiar! It's the multi-asset version of our single-asset objective function. The new term, $\lambda$ (lambda), is a **risk-aversion parameter**.

> **What is $\lambda$?**
>
> $\lambda$ represents how much you dislike risk.
>
> - A **high $\lambda$** means you are very risk-averse. The optimization will heavily penalize variance, leading to a safer, lower-return portfolio.
> - A **low $\lambda$** means you are more willing to take on risk for potentially higher returns.
>
> By varying $\lambda$, we can trace out a whole set of optimal portfolios.

We also typically add the constraint that all weights must sum to one: $\sum_{i=1}^{N} w_i = 1$.

### Solving the Optimization Problem

So we have our objective function, but how do we actually find the optimal weight vector $\mathbf{w}$ that maximizes it?

The process is remarkably similar to the single-asset case. We take the derivative with respect to our variable ($\mathbf{w}$), set it to zero, and solve. The only difference is that we're now using matrix calculus.

Our objective function is:
$$\max_{\mathbf{w}} \quad \mathbf{w}^T \mathbf{R} - \frac{\lambda}{2} \mathbf{w}^T \mathbf{\Sigma} \mathbf{w}$$

Taking the derivative with respect to the vector $\mathbf{w}$ and setting it to zero gives us:
$$\mathbf{R} - \lambda\mathbf{\Sigma}\mathbf{w} = 0$$

Now, we just need to solve for $\mathbf{w}$. Rearranging the equation, we get:
$$\lambda\mathbf{\Sigma}\mathbf{w} = \mathbf{R}$$
$$\mathbf{w} = \frac{1}{\lambda} \mathbf{\Sigma}^{-1} \mathbf{R}$$

Let's pause and appreciate how elegant this solution is. It tells us that the optimal allocation $\mathbf{w}$ is proportional to $\mathbf{\Sigma}^{-1}\mathbf{R}$.

- This is the direct multi-asset equivalent of our single-asset solution, $w = R/\sigma^2$.
- Instead of dividing by the variance ($\sigma^2$), we multiply by the **inverse of the covariance matrix** ($\mathbf{\Sigma}^{-1}$). This matrix inversion is the magic step that accounts for all the complex interactions between every asset in the portfolio.
- The result is then scaled by our risk aversion, $1/\lambda$.

> **A note on constraints:** The solution above is for the unconstrained case. Adding the constraint that all weights must sum to one ($\mathbf{w}^T \mathbf{1} = 1$) makes the math a bit more involved (it requires a technique called Lagrange multipliers), but the core concept remains the same: we are finding the weights that give us the best risk-return trade-off.

By solving this problem for different values of $\lambda$, we can generate a whole family of optimal portfolios. This whole set of optimal portfolios is actually called the **efficient frontier**. It's something that I'll dive into in a future post, but for now, let's solve an example portfolio.

### A Simple Worked Example

Let's make this real with a simple two-asset portfolio. Suppose we have two stocks:

- **TechCorp (T):** A high-growth tech stock.
- **GlobalGoods (G):** A stable, global consumer goods company.

We've estimated their financial characteristics as follows:

- **Expected Returns ($\mathbf{R}$):** TechCorp: 10%, GlobalGoods: 6%
- **Standard Deviations ($\sigma$):** TechCorp: 20%, GlobalGoods: 15%
- **Correlation ($\rho$):** The returns have a low positive correlation of 0.3.

First, let's assemble our vectors and matrices.

The **returns vector** $\mathbf{R}$ is straightforward:
$$\mathbf{R} = \begin{bmatrix} 0.10 \\ 0.06 \end{bmatrix}$$

For the **covariance matrix** $\mathbf{\Sigma}$, we need the variances and the covariance.
- Variance of TechCorp: $\sigma_T^2 = (0.20)^2 = 0.04$
- Variance of GlobalGoods: $\sigma_G^2 = (0.15)^2 = 0.0225$
- Covariance: $\text{Cov}(T,G) = \rho_{TG} \sigma_T \sigma_G = 0.3 \times 0.20 \times 0.15 = 0.009$

So, our covariance matrix $\mathbf{\Sigma}$ is:
$$\mathbf{\Sigma} = \begin{bmatrix} 0.04 & 0.009 \\ 0.009 & 0.0225 \end{bmatrix}$$

Now, let's solve for the optimal weights $\mathbf{w}$ using our formula: $\mathbf{w} = \frac{1}{\lambda} \mathbf{\Sigma}^{-1} \mathbf{R}$. We'll assume a moderate risk-aversion parameter of $\lambda = 2$.

1. **Find the inverse of the covariance matrix ($\mathbf{\Sigma}^{-1}$):**
   For a 2x2 matrix, this is a standard procedure. The result is:
   $$\mathbf{\Sigma}^{-1} \approx \begin{bmatrix} 27.47 & -10.99 \\ -10.99 & 48.84 \end{bmatrix}$$

2. **Multiply by the returns vector ($\mathbf{\Sigma}^{-1} \mathbf{R}$):**
   $$\begin{bmatrix} 27.47 & -10.99 \\ -10.99 & 48.84 \end{bmatrix} \begin{bmatrix} 0.10 \\ 0.06 \end{bmatrix} = \begin{bmatrix} (27.47 \times 0.10) + (-10.99 \times 0.06) \\ (-10.99 \times 0.10) + (48.84 \times 0.06) \end{bmatrix} = \begin{bmatrix} 2.088 \\ 1.831 \end{bmatrix}$$

3. **Scale by risk aversion ($1/\lambda$):**
   With $\lambda = 2$, our scaling factor is $1/2 = 0.5$.
   $$\mathbf{w} = 0.5 \times \begin{bmatrix} 2.088 \\ 1.831 \end{bmatrix} = \begin{bmatrix} 1.044 \\ 0.915 \end{bmatrix}$$

So our solution is to allocate 104.4% of our capital to TechCorp and 91.5% to GlobalGoods.

> **Wait, 104.4% + 91.5% = 195.9%? How is that possible?**
>
> Welcome to the concept of **leverage**. The unconstrained solution we just calculated doesn't assume our weights must sum to 100%. A total allocation over 100% implies borrowing money to invest more than you have. In this case, you would borrow 95.9% of your capital to achieve this high-return (and high-risk!) portfolio. This is a strategy an investor with very low risk aversion might take.

### What if you can't borrow?

Most investors operate under the constraint that their weights must sum to 1 ($\mathbf{w}^T \mathbf{1} = 1$). While the formal solution involves more complex math (Lagrange multipliers), the unconstrained solution we found still gives us something incredibly useful.

If we **normalize** our unconstrained weights so they sum to 1, we get the portfolio with the best possible risk-return trade-off.

- Total Weight = $1.044 + 0.915 = 1.959$
- Normalized weight for TechCorp: $w_T = 1.044 / 1.959 \approx 53.3\%$
- Normalized weight for GlobalGoods: $w_G = 0.915 / 1.959 \approx 46.7\%$

This particular portfolio (53.3% TechCorp, 46.7% GlobalGoods) is special. It's the portfolio that maximizes our return for the risk we are taking.

And that’s pretty much the core of it! We've walked through the math, starting with a single asset and building up to a neat way of handling a whole portfolio with matrices. By solving for the weights and then normalizing them, we landed on a specific mix: about 53% in TechCorp and 47% in GlobalGoods.

It's interesting that no matter what our personal risk aversion `λ` was, the *ratio* between the assets stayed the same. Normalizing them just gives us a tangible portfolio to look at.

But this definitely opens up more questions. What's so special about this specific 53/47 mix? And what about the constraint that most of us have: that our investments must add up to 100%? We kind of sidestepped that with our normalization trick.

These are deeper topics in portfolio theory, and we'll definitely dig into them in a future post. For now, hopefully, this gives a solid feel for the fundamental math involved. Thanks for reading
