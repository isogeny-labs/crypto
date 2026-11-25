# PKM prefix-sieve root admission diagnostic — 2026-11-25

The uncommitted `research/pkm_nonenum_decomp_20261125/prefix_sieve.py` prototype
uses SymPy `groebner(..., modulus=97, order="grevlex")` at each prefix node.
This diagnostic is read-only with respect to that prototype. It checks whether
the generic backend can pass the frozen protocol's first `t=3`, 60-second
per-target gate. It is not a relation-cost comparison.

Workload: `p=97`, target curve `y²=x³+2x+3`, `g=1`, target `(92,16)`, and
power-of-two Kummer factor base. The target is on the curve. The runner chose
primitive roots of the required order for each depth.

| Depth `t` | Cap (s) | Status | Root nodes | Certificates | Leaves | Solver wall (s) |
|---:|---:|:--|---:|---:|---:|---:|
| 1 | 10 | complete | 1 | 1 | 0 | 0.0124 |
| 2 | 10 | complete | 1 | 1 | 0 | 1.3470 |
| 3 | 3 | censored | 1 | 0 | 0 | 3.0040 |
| 3 | 15 | censored | 1 | 0 | 0 | 15.0048 |
| 3 | 60 | censored | 1 | 0 | 0 | 60.0044 |

An independent exhaustive group-law check at `t=3` found 8 base abscissae,
9 lifting points, and 18 ordered three-point witnesses among 729 triples.
One checked witness is `(1,43)+(96,0)+(50,19)=(92,16)` over `F_97`.
Thus the root ideal is satisfiable. The timeout says that this generic
Gröbner call did not reach a witness within the frozen cap; it is not a
mathematical rejection of the PKM map or prefix pruning.

The prefix-class equations themselves were checked independently at `p=97`,
`t=5`, primitive `zeta=19`, `g=3`: all 63 prefix classes matched their exact
root sets, and each parent was the union of its two children.

Next admission step: replace or accelerate the root solve while preserving
checkable inconsistency certificates and group-law-checked witnesses. Freeze
that backend and its accounting before a timed `t=3,4,5` panel. The current
prototype does not report field-operation counts, child CPU, or peak memory,
which the protocol requires for a complete-cost claim.
