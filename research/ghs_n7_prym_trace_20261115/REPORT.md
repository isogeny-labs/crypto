# n=7 GHS Prym trace-class screen

15 November 2026. Source and protocol: [PROTOCOL.md](PROTOCOL.md), [THEOREM.md](THEOREM.md). Raw results: [successful CSV](reports/run-20261115T0821Z/curves.csv) and [summary](reports/run-20261115T0821Z/summary.txt). The first run stopped on a signed-remainder assertion error; see [AMENDMENT_1.md](AMENDMENT_1.md) and its preserved [failure output](reports/run-20261115T0820Z/summary.txt). All derived and measured statements in this report await an independent PCC verdict. [pcc:unverified]

## What the bound says

For ordinary E over F128 with trace other than ±13, its seven-dimensional Weil restriction is simple. Every nonzero algebraic map from that variety into a Jacobian needs target genus at least seven, by [Diem's Theorem 3](https://www.esaga.uni-due.de/f/ulrich.goertz/iem-archiv/dissertation_diem.pdf). This applies to cover correspondences and excludes a genus-six algebraic target for these source curves. It is a lower bound on target dimension, not on attack cost.

Hess's [Theorems 14–15 and Section 8.3](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf) put the source factor in the complementary seven-dimensional Prym of the genus-fourteen W1 cover; the norm through its natural genus-seven deck quotient is zero. [Achter–Casalaina-Martin's Theorem A](https://www.math.colostate.edu/~achter/math/prym1224.pdf) supplies the characteristic-two principal polarization for the ramification type, subject to confirming this cover's divisor locally. A principally polarized Prym need not be a Jacobian. The broad search for genus-n Jacobians in the Weil-restriction class is already an internal prior lead (`LEAD-TRF-weil-restriction-jacobian-class`).

## Exact q=2 result

The fixed field representation is F2[z]/(z^7+z+1), with E(a,b): `y^2+xy=x^3+a x^2+b`. The census contains 56 W1 coefficients and both a twists, 112 exact curves. Fourteen coefficients lie in the classical cubic kernels. The other 42 give six trace classes absent from this classical coefficient slice, plus two shared trace classes. The table lists one exact nonclassical representative for each class.

| trace t | E(a,b) | #E(F128) | degree-7 places if genus-7 Y exists | classical W1 trace class? |
|---:|:---|---:|---:|:---|
| -17 | (1,26) | 146 | 35 | no |
| -9 | (1,8) | 138 | 27 | no |
| -7 | (0,14) | 136 | 25 | no |
| -1 | (1,28) | 130 | 19 | yes |
| 1 | (0,28) | 128 | 17 | yes |
| 7 | (1,14) | 122 | 11 | no |
| 9 | (0,8) | 120 | 9 | no |
| 17 | (0,26) | 112 | 1 | no |

![Trace classes of classical and nonclassical W1 coefficients](trace-classes.svg){width=65%}

The classical W1 slice has traces `{-15,-1,1,15}`. Every observed trace is ordinary and outside the ±13 exceptional subfield-isogeny traces, so the genus-seven floor applies to all 112 curves. The degree-seven place bound `18-t>=0` is met in every row. The six nonclassical-only trace classes are a concrete *arithmetic search subset* of the **published** W1 family. The equations and subset do not constitute a new vulnerable family; the orders here are small and this run has no attack-cost measurement.

A second native program [replayed all 112 orders](reports/replay-20261115T0833Z/RUN.md) by directly testing every (x,y) pair in the fixed field, with a separate carryless multiplication routine. It matched the CSV orders, traces and place-count values. This is a local algorithmic cross-check; a PCC signed verdict and independent geometric review remain open.

[`TraceExceptions.lean`](TraceExceptions.lean) checks the five q=2 base-trace evaluations in Lean 4.35.0; `#print axioms` reports none. This finite theorem does not cover the geometric transfer or Diem's theorem.

## Hyperelliptic target geometry

The [characteristic-two parity lemmas of Borodin–May](https://arxiv.org/pdf/2508.16886) give a sharper necessary screen: an ordinary genus-seven hyperelliptic Y with this sparse L-polynomial must have exactly one F2-rational branch point and one Frobenius orbit of seven branch points. Up to `PGL_2(F_2)`, there are [nine possible branch supports](BRANCH_SUPPORT_BOUND.md). This is a finite search reduction, not a count of curves or an existence result. Nonhyperelliptic genus-seven targets remain outside this screen.

![One rational branch point and one orbit of seven](branch-support.svg){width=75%}

## Next discriminating step

For a nonclassical-only trace class, search the nine hyperelliptic branch-support classes for a genus-seven Y with L-polynomial `1-tT^7+128T^14`, or exclude that hyperelliptic route; also examine nonhyperelliptic Jacobians. If Y exists, construct an explicit k-rational algebraic correspondence from A to J_Y and evaluate it on a chosen prime-order source subgroup. Then run complete relation/decomposition, linear algebra, target recovery and matched rho with setup charged. A point-count match alone does not supply a map or a lower-cost logarithm algorithm.
