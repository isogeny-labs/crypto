# Tapered Hermite recovery identity

This note isolates the algebraic identity implemented by
`contraction::recursive::recover` and separates it from the 2-adic precision
obligation. It is a proof target and producer audit, not a kernel-checked
theorem. `[pcc:unverified]`

## Algebraic statement

Let `T` be the active nonzero nodes in the integer grid and let `m_t` be the
source multiplicity at `t`. Put

```text
Q = sum_(t in T) m_t,
alpha = 2^(a+1),
E_t(X) = product_(q in T, q != t) ((X-q)/(t-q))^m_q.
```

For a polynomial `F` of degree below `Q`, define the retained jet and the
local basis series

```text
J_t(z) = F(t+alpha*z) mod z^m_t,
B_t(z) = E_t(t+alpha*z),
C_t(z) = J_t(z) / B_t(z) mod z^m_t.
```

Over the fraction field of the unramified 2-adic extension, the candidate

```text
H(X) = sum_(t in T) E_t(X) C_t((X-t)/alpha)
```

has degree below `Q` and has the same order-`m_t` jet as `F` at every node.
Therefore `H=F`, and evaluation at zero gives `[pcc:unverified]`

```text
F(0) = sum_(t in T) E_t(0)
         * sum_(0 <= j < m_t) coefficient(C_t,j) * (-t/alpha)^j.
```

The divisibility argument is included to make the obligation reviewable. For
`q != t`, `E_t` has a zero of order `m_q` at `q`; at `t`, `E_t(t)=1` and the
definition of `C_t` matches the retained jet. Hence `H-F` is divisible by
`product_t (X-t)^m_t`. Both polynomials have degree below its degree `Q`, so
their difference is zero. `[pcc:unverified]`

For the source grid, the number of nodes with valuation `i<u` is `L/2^i`.
With `h=ceil(3M/u)` and `m_t=h(u-i)`, already the `i=0` term gives

```text
Q >= h*L*u >= 3*M*L > M*(2*L-1),
```

which covers the deformation-degree bound used by `active`.
`[pcc:unverified]`

## Exact correspondence with Rust

| Identity object | Rust object |
| --- | --- |
| `m_t` | `multiplicity(edges, parameters, t)` |
| `J_t` | retained `Jet::coefficients` |
| `E_t(0)` | `e_zero` |
| `B_t(z)` | `e_series` |
| `B_t(z)^(-1)` | `reciprocal` |
| `C_t(z)` | `quotient` |
| `(-t/alpha)^j` | repeated multiplication by `ratio` |

Every factor of `B_t(z)` is

```text
(1 + alpha*z/(t-q))^m_q.
```

The grid check enforces `v2(t-q)<=a`, so each nonconstant ratio has positive
2-adic valuation. Consequently the truncated reciprocal has integral
coefficients. `[pcc:unverified]`

## Precision obligation

The fraction-field identity alone does not justify computation modulo a
finite power of two. Write `i=v2(t)`. The direct rational multiplier on
coefficient `j` has valuation

```text
v2(E_t(0)) + j*(i-a-1),
v2(E_t(0)) = sum_(q != t) m_q*(v2(q)-v2(t-q)).
```

Thus a sufficient recovery-only loss allowance is

```text
ell_t = max_(0 <= j < m_t)
          max(0, -v2(E_t(0)) - j*(i-a-1)).
```

The remaining quantified lemma is to connect this exact expression, plus the
live child-evaluation losses, to the production allowance `[pcc:unverified]`

```text
8*h*(L/2^i) + 8*M*(a+1).
```

The native small-grid control checks the direct multiplier inequality for
source-shaped `a>=2u`. A separate mutation computes the same `u=2` jets only
at root precision and then canonically lifts them: recovery returns `7` while
the true constant is `3`. This is not a live-path counterexample, because
`active` computes at `child_precision`; it demonstrates that the precision
premise is load-bearing and cannot be dropped. `[pcc:unverified]`

## Acceptance gate

C1 item 5 closes only after an independent proof or kernel refinement covers
both the fraction-field identity and the all-parameter precision inequality,
and links them to the live Rust objects. The finite positive and negative
controls remain counterexample searches, not substitutes for that gate.
`[pcc:unverified]`
