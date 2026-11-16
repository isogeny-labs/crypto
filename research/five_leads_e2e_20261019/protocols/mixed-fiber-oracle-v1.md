# Small three-summand mixed-fiber oracle census

This is an exploratory first test for L3, recorded after implementation and
before interpreting the retained `v1` output. An earlier development run used
unmatched fiber point counts and was replaced; it is not a measured control.
No F4/msolve or DLP claim is licensed by this protocol.

For `(p,d)=(97,8),(193,12),(257,16)` use the three curves
`y^2=x^3+a*x+b` with `(a,b)=(2,3),(5,7),(8,11)` at each prime. For each
curve enumerate all multiplicative cosets of the order-`d` subgroup of
`F_p^*`, group them by counts of x coordinates with zero, one or two rational
lifts, and choose the highest rational point count group with at least three
cosets. The first coset defines a homogeneous triple; the first three define
the mixed triple. Generate an independent random x-coordinate set for each
slot with identical zero/one/two-lift counts, seeded by the formula in
`native/mixed_fiber_census.rs`. All choice rules depend only on the curve,
never on held-out targets.

Choose 20 distinct public target points per curve by seeded lift sampling.
For each of the nine curves, 20 targets and three arms, build the fibers cold,
lift every x, build a complete ordered pair-sum table for the first two
summands, query every third-summand point and verify every lifted triple.
Compare the sorted complete solution list to a separate exhaustive
three-loop enumerator on a fresh curve object. Preserve zero-solution cases.
Output one JSON line per target/arm with the exact sets, solutions, setup and
total M/S/I, pair insertions, lookups, lifts and random draws. The target
generation and exhaustive verifier are excluded from solver cost but
reported separately. `BTreeMap` implementation work is represented by the
logical counters, not measured CPU instructions or memory traffic.

The twelve arithmetic prices are `S/M={0,0.5,1}` and
`I/M={1,10,100,1000}`. Report the separate `M+0.75S+10I` conversion.
Native sensitivity is `0,0.1,1` M per logical table/random operation, applied
symmetrically. Ratios are sums of one-target cold costs on matched curves and
targets. Differences in solution lists across arms are expected because
the factor-base sets differ; each arm must agree with its own exhaustive
solution set. Repeating the native binary must give a byte-identical JSONL.

This pair-sum oracle does not implement the proposed mixed-fiber polynomial
system, a complete F4/msolve reference, relation collection, linear algebra
or source rho. Those remain the explicit next gates in
[`problems/mixed-fiber-pkm-full-solver.md`](../../../problems/mixed-fiber-pkm-full-solver.md).
