# Cost gate for the next PKM map sweep

This is a premeasurement screen for new factor-base maps. It does not replace
the frozen cold panel or establish a general ECDLP lower bound. Keep every
screened-out map and the reason for rejection in the sweep ledger.

## Map placement alone

Let the target subgroup have order `n`, and let a proposed map produce `B`
factor-base points. Count ordered `m`-tuples, with repetition, before quotienting
by signs or permutations. For a target selected uniformly and independently of
the map, the expected number of tuples summing to it is exactly `B^m/n`:
each tuple has one sum and the target equals that sum with probability `1/n`.
Consequently the direct-relation hit probability is at most
`min(1, B^m/n)`. This statement is about the independently selected target;
it does not assume that the map points themselves look random. A map may
redistribute hits among targets, but it cannot increase this average at fixed
`B`. The same calculation applies to an equal-size interval control.

Suppose each query target remains uniform and independent of the map, a query
returns at most one independent rank row, rank collection requires
`C = Θ(B)` rows, and the method enumerates `Θ(B^(m-1))` tuple prefixes per
query. For `B^m <= n`, the optimistic expected query count is at least
`Ω(C n/B^m)` and the resulting enumerative work is `Ω(n)` when `C = Θ(B)`.
For `B^m >= n`, at least `Ω(C)` queries remain and enumerative work is again
`Ω(B^m) >= Ω(n)`. These are conditional cost bounds for this query/rank model,
not for every algebraic algorithm. Multiple independent rows per query, a
sub-enumerative certified solver, or a proved reduction in required rank can
change the calculation; their extraction, setup, joins, and linear algebra
must then be charged explicitly.

At the natural `B ≈ n^(1/m)` scale, ordinary two-sum scanning costs `Θ(B)`
per call and ordinary three-sum pair scanning costs `Θ(B^2)` per call. Either
route therefore has an optimistic `Θ(n)` relation-collection term before
map construction, failed systems, linear algebra, and verification. A fixed
translation, a different coset, or a different isogeny chain with the same
`B` needs a **measured solver-cost, multi-row, or rank change**, not merely a
favorable screen count, to survive this gate.

## Existing evidence and the next falsifier

The [local PKM map sweep](../pkm_map_sweep_20261123/REPORT.md) screened 16
affine placements, 32 cosets, and eight distinct auxiliary chains at
`p=1000003`; none passed its prespecified paired relation-yield gate
`[pcc:unverified]`. Those are component screens on one field and one target
curve. The separate
[16-bit `m=3` engine sweep](RESULT_RESGCD_M3_20261126.md) reports F4
multiply-adds per call rising from 267,148,734 at `t=3` to 17,464,131,718
at `t=4` (about 65 times), with its `t=5` call censored. That report is
`[pcc:unverified]` and its operation counts are not a cold same-target cost.
An unmerged [half-GCD and NTT screen](https://github.com/isogeny-labs/crypto/blob/b50fdeb1ba7d0cac9383cd02129bc294721e722c/research/pkm_adaptive_sweep_20261123/RESULT_RESGCD_HALFGCD_NTT_20261128.md)
reports a lower `m=3` resultant–GCD call cost at `t=10`, but still 3.8 times
the grid's counted operations and `B^(2+o(1))` work per target at the natural
`m=3` base size. That result is also `[pcc:unverified]` and does not supply a
complete-cost crossover.

For each new map lead, record a concrete mechanism that could beat the
conditional `Ω(n)` enumerative term: a certified sub-enumerative relation
solver, a charged multi-row query, or a changed rank requirement. First
falsify the mechanism on equal-size controls with independently selected
curves and targets, recording every failed relation call, map search and
construction, peak memory, and measured wall and CPU. A lead advances only
when its complete cold cost improves against equal-size interval and existing
PKM controls on at least three curves at each of two increasing sizes, and
source-pinned independent replay agrees. Report an advantage over matched
same-target rho only if the measured total cost crosses it.
