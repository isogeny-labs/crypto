# The exact q16 genus-four GHS frontier

## Claim and scope

This experiment closes the **documented GHS parameter sweep** for the 66
ordinary curves over `K = F_(16^3)` that the earlier three genus-three V4
screens miss even after arbitrary `K`-isogeny. All 66 have an evaluated
nonzero prime-subgroup map to a genus-four hyperelliptic Jacobian over
`k = F_16`, using Magma's standard `WeilDescent(E,k,1)` intrinsic. Every
nonzero Magma parameter was checked on every source: none produces genus
three. The three source-order representatives also have complete rational
factor-base relation solves and matched source-group rho runs. Rho wins on
all three measured representatives.

This is a new **finite census and exact-equation transfer certificate** for
the specified frontier, not a new GHS method, a new geometric family, or a
demonstrated faster attack. The genus-three isogeny routes already suggested
by the catalogued Jacobians are still unimplemented.

## Exact source equations and exhaustive bound

Write `K = F_2[w]/(w^12+w^7+w^6+w^5+w^3+w+1)` and embed `k = F_16`
into `K` as in the retained Magma scripts. Every model is

```text
E_b/K : y^2 + xy = x^3 + w^5 x^2 + b,   b != 0.
```

An integer `b` below is the polynomial-basis bit mask in `w`. These are
the exact 66 models in the prior [frontier certificate](../ghs_nonsplit_order4222_20261028/Q16_FRONTIER.md),
which proves that their source orders are the three orders shown and that
the three named V4-screen isogeny classes exclude them.

| `#E_b(K)` | Number | Exact `b` masks |
|---:|---:|---|
| 3974 | 12 | 306, 691, 2082, 2268, 2628, 2896, 2978, 2996, 3347, 3412, 3505, 3978 |
| 3978 | 30 | 11, 69, 250, 347, 418, 508, 657, 716, 723, 866, 910, 1155, 1317, 1498, 1539, 1605, 1717, 1774, 2215, 2282, 2328, 2384, 2495, 2766, 2770, 3130, 3467, 3596, 3766, 3870 |
| 4178 | 24 | 31, 341, 445, 489, 500, 553, 629, 1062, 1872, 1884, 2125, 2279, 2319, 2486, 2488, 2614, 2692, 2772, 2872, 3199, 3591, 3815, 3914, 4028 |

For each of these 66 equations, `sweep66.py` tested all 4,095 nonzero
parameters `c=w^i`, `0<=i<4095`, with `WeilDescentGenus(E,k,c)` in Magma
2.30-1. All 66 histograms were identical: genus four 30, genus six 450,
genus seven 3,585, and 30 intrinsic rejections. Thus 270,270 exact
source/parameter pairs were accounted for and **zero returned genus
three**. The 30 rejected parameters are reported as rejections, not assigned
a genus. The three full server receipts are `sweep_*_receipt.json`; the
generated Magma inputs are `sweep_*.magma`. This is an exhaustive statement
about that intrinsic and parameterization, not about all covers or all
isogenies.

## Evaluated subgroup maps

For each of the 66 models, `map66.py` constructs `C_b,h_b :=
WeilDescent(E_b,k,1)` and selects `G_b` of order `r`. The Magma assertions
check

```text
genus(C_b)=4,  G_b != O,  [r]G_b=O,
h_b(G_b) != 0,  [r]h_b(G_b)=0,
h_b([7]G_b)=[7]h_b(G_b).
```

Since `r` is prime, the nonzero image has exact order `r`. The source
order, chosen `G_b`, reduced Mumford image, Jacobian order, and assertion
result are retained **for every `b`** in `map_*_receipt.json`; `summary.json`
indexes those records. The selected primes are `1987`, `17`, and `2089`
for source orders `3974`, `3978`, and `4178`, respectively. At `c=1`,
the genus-four Jacobian orders are `55,636` for all 12 order-3974 models,
`75,204` for all 24 order-4178 models, and `71,604` for 24 plus `39,780`
for six order-3978 models. Every listed order is divisible by the
corresponding source order and selected prime. The exact target curves
are instantiated by the retained Magma inputs and the representative
hyperelliptic equations appear in `order_*_receipt.json`.

The map is [Magma's published characteristic-two GHS divisor-class
map](https://magma.maths.usyd.edu.au/magma/handbook/text/1561), which
implements the conorm/trace transfer. These experiments evaluate it on
the selected subgroup; they do not derive a new map formula.

## Complete representative attacks and matched rho

`run.py` fixes the earlier selected `(G,Q=[7]G)` for one representative
per order, constructs the genus-four map, collects rational factor-base
relations from reduced Mumford divisors, solves the resulting linear
system over `F_r`, and verifies the result on both source and target.
It retains the relation matrix, right-hand side, source/image points, and
full Magma stdout. The order-3978 subgroup has no nonzero rationally
split reduced divisor for **any** of its 30 genus-four parameters; its
complete relation solve therefore uses known rational-point offsets.
Every accepted relation has a distinct Jacobian point, and its divisor
equality is asserted before the row enters linear algebra.

The same Magma invocation then runs 1,000 rho solves on the same source
`(G,Q)`, including degenerate-collision restarts. CPU figures are Magma
`Cputime` on the calculator backend, with hundredth-second reporting
granularity. `FULL_CPU` includes map construction, relation collection,
and linear algebra, excluding input parsing, Jacobian-order reporting,
and the later rho trials.

| Source `#E` (`b`) | `r` | Distinct relations | GHS relation steps | Full GHS CPU | Rho mean steps | Rho mean CPU | GHS / rho CPU |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 3974 (306) | 1987 | 13 | 1,188 | 0.22 s | 75.112 | 0.00102 s | 216× |
| 3978 (11) | 17 | 17 | 4,172 | 0.33 s | 6.811 | 0.00010 s | 3,300× |
| 4178 (31) | 2089 | 17 | 479 | 0.20 s | 72.153 | 0.00101 s | 198× |

All three recovered the checked value `7`. The ratios are same-backend,
same-target observations at this finite field size; the short timing
intervals limit precision. The other 63 models have verified maps and
genus sweeps but **their full attack costs have not been measured**.

## Rigorous necessary genus bound

There is a bound for **all 66 equations** under any algebraic transfer.
For source order `N`, let `t=4097-N`. The Frobenius polynomial of the
Weil restriction `Res_(K/k)(E_b)` is

```text
P_t(X) = X^6 - t X^3 + 4096.
```

`irreducibility.py` gives finite Rabin irreducibility certificates:
`t=123` and `t=119` remain irreducible modulo 11, while `t=-81`
remains irreducible modulo 5. Hence all three polynomials are
irreducible over the rationals by Gauss's lemma. The three Weil
restrictions are therefore simple abelian threefolds over `k`.
Any nonzero algebraic homomorphism from one of them into a Jacobian
has a three-dimensional image, so its target curve must have genus
**at least three**. This covers every model in the three source-order
classes, including the order-17 subgroup class. It is a necessary
bound for algebraic cover/isogeny transfers; it says nothing by itself
about whether a useful map or a faster algorithm exists.

An independent, subgroup-specific bound uses only cardinalities.
For any prime-order source subgroup `r`, if a homomorphism to a Jacobian
`J(C)(F_q)` is nonzero on that subgroup, Lagrange's theorem gives
`r | #J(C)(F_q)`. The Weil bound gives

```text
r <= #J(C)(F_q) <= (sqrt(q)+1)^(2g),
g >= ceil(log(r)/(2 log(sqrt(q)+1))).
```

This necessary condition applies to *every* subgroup-preserving
curve-to-Jacobian transfer, regardless of the cover or isogeny used.
For `q=16`, a genus-at-most-two Jacobian has order at most `5^4=625`.
The two selected larger primes, `1987` and `2089`, therefore require
`g>=3`. `GenusBound.lean` kernel-checks the finite divisibility
contradiction and the exact census arithmetic. It takes the Weil bound,
Lagrange's theorem, the Magma results, and the finite-field census as
external inputs; it does not formalize their proofs or coordinate maps.

The lower bound is attainable *at the isogeny-class level*: LMFDB already
catalogues genus-three hyperelliptic Jacobians in the corresponding
classes. The exact genus-three source maps and their costs remain open.
The necessary genus bound does not characterize all vulnerable curves.

## Prior work and novelty audit

The [original GHS paper](https://www.kantmath.de/publications/papers/weil_descent.pdf)
already treats characteristic-two cubic descent and genus-three/genus-four
outcomes. [Magma's Weil-descent documentation](https://magma.maths.usyd.edu.au/magma/handbook/text/1561)
provides the parameter and the divisor-class map used here. The
[Murai et al. abstract](https://www.iwsec.org/scis/2022/_abst/1A3-2.pdf)
reports a complete even-characteristic cover classification under the
minimal-genus isogeny condition `g(C)=d*g(C0)`; the full case table was
unavailable in this review. [Tian's cover-isogeny
work](https://arxiv.org/abs/2012.07173) predates this experiment.

Every exact source model has a source order in one of three already
catalogued LMFDB isogeny classes:
[3974](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_aet),
[3978](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_aep), and
[4178](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_dd).
The earlier `prior_work_frontier_q16.json` records the class-level checks,
including the prior repository hit for order 3978. Exact equation/order
web queries returned no indexed matches, which cannot prove publication
novelty. `summary.json` names the prior class for each of the 66 models.

The only new claim supported here is the **exact finite frontier
classification and evaluated genus-four maps**. No measured representative
beats rho, and 63 full attack costs are unmeasured. No new vulnerable
family is established.

## Replay

```sh
python3 research/ghs_q16_genus4_frontier_20261028/sweep66.py --order 3974
python3 research/ghs_q16_genus4_frontier_20261028/sweep66.py --order 3978
python3 research/ghs_q16_genus4_frontier_20261028/sweep66.py --order 4178
python3 research/ghs_q16_genus4_frontier_20261028/map66.py --order 3974
python3 research/ghs_q16_genus4_frontier_20261028/map66.py --order 3978
python3 research/ghs_q16_genus4_frontier_20261028/map66.py --order 4178
python3 research/ghs_q16_genus4_frontier_20261028/run.py --order 3974
python3 research/ghs_q16_genus4_frontier_20261028/run.py --order 3978
python3 research/ghs_q16_genus4_frontier_20261028/run.py --order 4178
python3 research/ghs_q16_genus4_frontier_20261028/smooth17.py
sage -python research/ghs_q16_genus4_frontier_20261028/irreducibility.py
python3 research/ghs_q16_genus4_frontier_20261028/audit.py
lean research/ghs_q16_genus4_frontier_20261028/GenusBound.lean
```
