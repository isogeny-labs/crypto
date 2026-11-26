# Amendment 2: factor-base coset with valid planted subgroup targets

**Filed 2026-11-25 after the correctness preflight failed at `g=1`, before any F4 solver row.**

The protocol fixed `x = lo + w` (`g=1`). The native correctness preflight
found zero distinct planted target abscissae in the order-`r` subgroup at
`d=7`, so that fixture cannot supply the frozen four planted relations.

The instrument then searched positive integers `g` in ascending order. For
each `g`, it enumerated every unordered distinct factor-base triple and all
eight sign patterns, retained sums lying in the committed prime subgroup, and
deduplicated them by target abscissa. The first `g` with at least four target
abscissae at every qualification order was `g=3`:

| d | distinct planted subgroup target abscissae |
|--:|--:|
| 7 | 4 |
| 9 | 10 |
| 19 | 34 |

Read every protocol occurrence of factor-base `g=1` as `g=3`. The same `g`
is used by both arms and every target. The search rule and failed `g=1`
preflight are retained under `preflight/`.

No curve, root order, arm, equation, exponentiation schedule, seed, target
count, budget, metric, qualification threshold, or scaling rule changes. No
solver cost was observed before this amendment.
