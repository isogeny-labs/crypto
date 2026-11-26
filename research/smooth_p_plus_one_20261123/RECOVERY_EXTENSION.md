# Fixed extension of the F8191 recovery check

2026-11-24. Preregistered development control. [pcc:unverified]

The original seed-781815 check stopped after 30 draws with nine selected
Refined rows and no scalar. Its raw record remains in
`evidence/refinement_v1/f8191_recovery.jsonl`. The Direct control recovered
4356 in 13 calls. Do not replace or reclassify those outcomes.

The new trial keeps the same F8191 fixture, degree-32 selector, seed, ordering
of returned rows, first-row selection rule, linear algebra and three-second
cooperative deadline per query. It allows at most 128 draws from the start of
the same deterministic mask sequence. The driver exposes this as `draws=128`.
Both its automatic Direct control and Refined arm run. A scalar is accepted
only if multiplication by the fixture generator reproduces its public target.
The Refined prefix must match all 30 earlier attempt records after excluding
only the new summary metadata. Failure or a null scalar remains in the result.

This is a separately frozen extension, not a success under the original
30-draw allowance. There is one trial, no retry, and an outer 420-second watchdog
with a kill grace of five seconds. The worst permitted oracle budget is
128 times three seconds, plus fixture construction, direct control and output.
The internal deadline is checked between native phases; expensive native
phases may overrun it, in which case the query is undetermined. The outer
watchdog prevents an unbounded overall invocation.

After a successful scalar check, the fixed five-target panel, including the
already tested planted fixture, emits full signed rows for replay. It is an output
qualification, not another search for a favorable target. The original five
target deadline comparison remains in refinement_v1.

No timings from this shared host are admitted as performance measurements.
Setup and linear-algebra timing, the strongest production IC controls and
independent verification remain requirements of the original goal.

    dickson-four-summand 8191 5 "$MSOLVE" recover=781815 draws=128 refined

The new source is copied to an immutable executable, its source and binary
hashes are recorded before the first call, and all stdout also passes through
the local JSONL capture hook.
