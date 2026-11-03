# Cost model for a large-prime variation of the cover attack

Status labels: **measured** (a recorded run), **derived** (computed from measured values, formula shown), **proposed** (not built).

## Setting

- `F` is the factor base of `jv_cover`: one element per `x ∈ F_p` with `h(x)` a non-zero square in `F_{p²}`; `N = |F| ≈ p/2` (measured: 30, 32, 52, 49, 129, 123, 238, 484 at p = 53, 53, 101, 101, 251, 251, 503, 1009).
- One *test* solves six quadrics over `F_p` (`C_cov` counted multiplications). Every `F_p`-solution is a decomposition `R = Σ ε_i B_i` over six distinct base elements. A test yields a decomposition with probability `1/T`; `T = 720` is the heuristic `6!`.
- One relation is a row `Σ ε_i log B_i = a + b·d` (columns: the six points and the unknown `d`).
- Plain method (as run): collect solutions, take the square core, solve by Wiedemann. `S_full = X·T·C_test + LA(N)`, with `X = N + 1`.

## Proposition 1 (the test cannot be restricted to a sub-base)

Let `F' ⊂ F`, `|F'| = f·N`, `α = log|F'| / log N`. The unknowns of the system are `x_1..x_6 ∈ F_p`; `F'` is an arbitrary subset of `F_p` and not the zero set of a low-degree polynomial, so no system equivalent to "all six abscissae in `F'`" is cheaper. Adding the constraint as equations makes the system over-determined. Every test therefore returns decompositions over the full `F` at the same cost `C_test`, and the sub-base is applied afterwards as a filter. This is the difference from Gaudry-type attacks, where the *test* (a smoothness test of a divisor) is also independent of `F'`, **but where `T·C_test` is cheap against the linear algebra**. Here it is the other way round.

*Status: argued, not proved; the structural claim is that no test whose cost falls with `f` is known. Open.*

## Yield probabilities (derived; checked by measurement in `yields.json`)

A solution is modelled as a uniform 6-subset of `F` (hypergeometric; binomial if points are independent). With `L = (1-f)N` large primes:

| type | symbol | probability |
|:--|:--|:--|
| full (0 outside `F'`) | `P0` | `C(fN,6)/C(N,6) ≈ f^6` |
| 1-partial | `P1` | `fN-choose-5 · L / C(N,6) ≈ 6 f^5 (1-f)` |
| 2-partial | `P2` | `≈ 15 f^4 (1-f)^2` |
| ≥ 3 outside | rest | |

## Relation counts

`X` solutions, `c = X/N`.

- **Single large prime.** A 1-partial is an edge from its large prime to the "1" node; two partials on one large prime combine into one relation. With Poisson degrees, `λ1 = 6 c f^5` partials per large prime: `R1/N = c f^6 + (1-f)(λ1 - 1 + e^{-λ1})`. For small `f`, `R1/N ≈ c f^6 + 18 c² f^10`, and `R1 ≥ fN` gives `c1 ≈ f^{-9/2}/√18`.
- **Double large prime.** Relations with ≤ 2 outside points are edges of a graph on `L` large primes plus the "1" node; the number of relations is the cycle rank `E - V + C` (union-find in the code).
- **Information bound (any elimination, generic coefficients).** The large-prime columns form an `X × L` matrix `A_LP`; the relations over `F' ∪ {d}` number `X - rank(A_LP)`, and rank is at most the term rank (a matching). Solving for `d` needs these to be at least the touched `F'` columns plus one. Hence `X ≥ touched_all + 1` whatever `f` is: **the large primes cost exactly the rank they remove.** With Poisson column degree `6c`, `touched_all = N(1 - e^{-6c})`, so `c* = 1 - e^{-6c*}`, `c* = 0.99753`. The plain method's schedule (first attempt at `N + 1`) is `0.25 %` above this bound; the bound is reached by any scheme (and by the plain method with a core filter and earlier attempts), not only by large primes.

## Proposition 2 (derived)

`S(f) ≥ c*·N·T·C_test + LA(f)`, where `LA(f)` is the cost of the reduced system. Hence

`S_full - S(f) ≤ (N + 1 - c*N)·T·C_test + LA(N) ≤ 0.25 % · (1-s) · S_full + s · S_full`, with `s` the LA share of `S_full`.

The saving is at most `s + 0.25 %`, reached only if the reduced system costs nothing and an elimination exists that attains the rank bound (Lean: `lean/LpBound.lean`, the conditional arithmetic form: given `x0 ≤ x`, the saving is at most the plain LA cost.)

## LA cost (measured constant)

`la_ops = 26.2·u²` Wiedemann operations for `u` unknowns at row weight 7 (`wiedemann_constant`, pooled), 16 multiplications each: **419 u²**. The reduced system is priced at the *plain* row weight, which favours large primes (merged rows are denser); the graph is priced at 0.

## Total

`S(f) = X(f)·T·C_test + 419·(fN)² (+ graph cost, priced 0)`, with `X(f)/N` from the Monte Carlo (`curves.json`). LA share of the plain method:

`s(p) = 419 N² / ((N+1)·T·C_test + 419 N²)`, `N = p/2`.

## Asymptotic exponents (derived, single large prime)

Balance `N^{1+4.5β}` (relations, `f = N^{-β}`) against `N^{2-2β}` (LA): `β = 1/6.5`, `S ∝ N^{2 - 4/13} = N^{1.69}`, against `N^2` for the plain method once LA dominates. This is the same form as Thériault's single-large-prime exponent. It applies only above the p at which `s(p)` is large (p ~ 10^6 to 10^7, table), where the plain method is already hundreds of times below rho's p^3 growth.
