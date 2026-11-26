# PKM three-summand complete-cost scaling gate

Frozen 2026-11-25 before native F4 panel timing. This adds a complete-cost
prediction to `research/pkm_nonenum_decomp_20261125/PROTOCOL.md` and
`AMENDMENT_NATIVE_F4_20261125.md`; it does not replace their fixed roster,
controls, caps, or witness requirements. The model applies to the measured
`p-1` Kummer construction only. Applying it to `p+1` Dickson assumes a
separately verified norm-one factor base and witness solver.

## Quantities to record for every cold target

Let `B=2^t` be the map degree, `n` the actual number of independent
factor-base log columns, `s` the number of signed base points, `r` the prime
subgroup order, and `h` the measured probability that an independently
chosen relation query yields a checked, usable three-point row. The random
three-summand proxy is `lambda=s^3/(6r)` for unordered distinct summands,
with `h≈1-exp(-lambda)` only when the sum distribution and rank behavior
support it. Do not substitute `B` for `s` or `n` in a measured result.

For each target retain the exact relation-attempt count `A`, successful
rows, independent rank, F4 refutations, undecided calls, basis extraction,
point lifts, full-rank solve, and target descent. The charged cost is

`W_cold = W_setup + W_all_relation_queries + W_rank + W_linear_solve + W_target_descent + W_verification`.

`W_all_relation_queries` includes failed and censored calls and all
specialization, F4, extraction and point-lift work. Record process wall,
CPU, peak RSS and calibrated counted work by stage. The control arms and
rho receive the identical curve and target, with their own cold setup.
Do not replace `A` by its expectation in a reported measurement.

The basis extractor is a DFS over quadratic tower roots. Before a timed
candidate row, instrument and retain its visited nodes, root-split branches,
terminal leaves, and verified tuples separately from F4's elimination
counts. Record the largest live basis/cache state and branch-copy work.
The current extractor reports field multiplications but does not expose
these DFS counts, so its existing output alone cannot establish that the
high-degree search was pruned rather than enumerated in extraction. Report
visited leaves against `B^3` and against the equal-size grid control; a
small leaf ratio is supporting evidence only when total charged query work
also falls. Include failed and censored queries in these totals.

## Pre-measurement scaling prediction and falsifier

Conditionally assume `s=Theta(B)`, `n=Theta(B)`, independently distributed
three-sums, and a full-rank relation matrix requiring `Theta(n)` usable
rows. When `lambda` is small, the expected query count is
`Theta(n/h)=Theta(r/B^2)`. If a query costs `T(B)` in one common unit,
relation collection costs `Theta(r T(B)/B^2)` before rank failures and
descent. The selected panel has `r=Theta(B^3)`, so this becomes
`Theta(B T(B))` under those assumptions.

A conventional sparse Krylov solve on `Theta(B)` rows with three nonzeros
per row takes `Theta(B)` matrix-vector products of `Theta(B)` field work:
`Theta(B^2)` field operations, with at least `Theta(B)` live coefficients
or vector entries. This is a model for the current full-rank pipeline, not
a lower bound on every possible structured linear solver. At `r=Theta(B^3)`,
same-target rho has expected `Theta(B^(3/2))` group steps. Thus under this
sparse-solve model, a decreasing `T(B)/B^2` establishes only a component
gain; the `B^2` matrix term eventually exceeds rho's `B^(3/2)` growth.
The panel must report both operation types and calibrated cost, rather than
divide field operations by group steps without calibration.

At `t=3,4,5`, compare measured `A` with `n/h`, query work with
`T(B)`, matrix work with `n^2`, and total cold work with same-target rho.
If the actual columns, yield, rank, or solver cost violate an assumption,
show the measured replacement in the model. A complete-cost advancement
requires verified witnesses, full rank, a decreasing charged query ratio
under the original protocol, and a cold same-target advantage over both
equal-size controls and rho at the largest measured size. Otherwise retain
the component finding and identify the measured bottleneck. Censored arms
retain spent work and cannot be treated as wins.

## Wide-field implications under the same conditional model

At the `m=3` constant-density heights in `WIDE_PM1_DECISIONS_20261125.md`,
a standard sparse solve scales as `B^2` field operations. This gives
approximately `2^128` at P-192 `p+1` (`B=2^64`), `2^152` at P-224 `p-1`
(`B=2^76`), `2^174` at P-256 `p+1` (`B=2^87`), and `2^350` at P-521
`p+1` (`B=2^175`). The corresponding rho exponents in group steps are
about 96, 112, 128, and 260. These are model exponents, not measured
cost ratios or universal impossibility results. P-384 `p+1` has only
`t=32`; its relation-query density screen fails before linear algebra.

The s59 `p+1` height 18 has about `2^6.6` queries per usable relation by
the existing proxy, so roughly `2^24.6` attempted queries for `Theta(B)`
rows and a `2^36` field-operation sparse-solve term. The s59 `p-1`
height 16 similarly gives about `2^28.6` attempts and `2^32` sparse-solve
field operations. Their comparison with roughly `2^29.5` rho group steps
depends on measured per-query, field/group calibration, rank, and memory.
Neither s59 row is admitted from these exponents alone.

Source of curve valuations and density proxies:
`research/pkm_nonenum_decomp_20261125/WIDE_M3_GATE.md` and
`WIDE_PM1_DECISIONS_20261125.md`. The rank and sparse-solve model above
is a stated inference from the required full-rank three-sparse system.
