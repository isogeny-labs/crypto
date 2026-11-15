# Working status, 2026-11-14

## What was done
Previous phase: principal-row pilot55 gated and330 paired solves passed, but the candidate
lost to best-native control at every size. Table-factor oracle passed50624 exhaustive small
polynomials (independent root division) and48000 randomized/constructed fixtures. Lean checked
the cubic/projective/Ferrari algebra. Cold table pipeline passed220 exact stream/solver gates
and660 solves, best total gain1.15..1.97x. These do not complete the10x goal.

New anchor.rs: target-seeded anchors, tabulated residual factorization, normalized row dedup,
weighted forest with signed/scaled coefficients, balanced/unbalanced cycles, small anchor solve.
Four release tests passed, including12 fully group-checked target solves and planted forest
invariants. This is a new cost screen, not a breakthrough declaration.

## Decisions made
Keep every cold construction and solve retry inside total. Gate raw principal rows in the full
Jacobian and emitted modular rows under a catalogue-certified ell projection. Compare to both
best native and best cold table controls from the previous pilot, including branch count1.

## Open threads
The first55-case gate and55-case cold timing are complete; results below. Retain rank/cap failures. Package prior3 open
benchmark records and the new one with source-linked reports, plots and formal receipts.

## Errors
Coordination service restarted and changed local transport. The new task was successfully
claimed before editing these files; repeated coordination work stopped at the user's request.
No correctness failure has occurred in the anchor unit gates.

## First anchor pilot result and next decision
55 gated and55 timed solves pass;15890 raw and1638 projected identities checked. Most total
ratios versus prior cold table controls are<=1.2x, with large L0 timing outliers retained.
The next variant shares work across all sign choices via the exact Gray-code norm identity,
uses cached anchor inverses and dense reusable forest potentials, and removes repeated small
matrix rebuilding. This remains a falsification attempt toward the original10x goal.

## Signed-orbit integration gate
Implemented orbit.rs: barycentric basis, cached cross terms and reflected Gray-code updates.
The local release test compares5600(h,Q) pairs with independent fresh Newton interpolation
and norm division across six genus2/3/4 instances. All comparisons passed.
Implemented dense.rs incremental echelon elimination;60 planted systems compare target
determination against solve_fast at every inserted prefix, including guaranteed rank completion.
Integrated both into solve_orbit with the reference sparse forest and raw-row dedup retained.
Twelve fully group-gated target solves passed. All seven example tests passed locally
(release,9.23seconds test execution). These are producer self-checks, not signed independent
verification or performance evidence. No new timing arm was run.

Next: implement dense affine forest, batch anchor inverses and bounded tuple schedule, then
run full group gates before measuring the frozen variant. Retain the old solve path unchanged
as an explicit reference. Current integration intentionally does not represent the entire
signed-orbit protocol amendment.
