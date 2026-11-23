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

## Completed signed-orbit pilot
Source88a969080d implements the dense affine forest, batch inverse differences, shuffled
genus2 triples, all Gray sign classes and incremental target elimination. The original Horner
catalogue remains in both arms; a pre-timing protocol amendment records that choice.
All55 fully gated solves passed, with15115 raw principal identities and1290 projected anchor
identities checked. All110 interleaved cold solves passed. Five-seed table/orbit median ratios:
g2 p1011.27,p2511.32,p5031.58,p10092.59,p20032.86,p40012.94;
g3 p611.55,p2111.38,p4012.21; g4 p311.34,p611.69.
These are local L0 producer assertions [pcc:unverified], not independently signed results.
Record20261115T0602Z-hec-signed-orbit and run/orbit-* retain all outputs.
The shared report preserves the preceding negative pilots and separate comparison boundaries.
The next research step is frozen-arm confirmation on held-out families and a phase-cost
analysis of the remaining relation work. The10x mathematical goal remains active.

## Compact high-coefficient quotient pilot
Frozen protocol1a115ad302; sourceb2fddfd8b2. Local gates:22128exhaustive sign cases including
branch places,8000random cases,110reference/compact solves with identical14field fingerprints.
Every generated compact orbit equals reference;55compact cases check15115principal and1290
projected anchor identities. All165timed solves pass. [pcc:unverified]
Complete-cost gain vs orbit is0.83..1.32, with g2p101 regression; vs frozen table in this run
1.22..2.98. Raw record20261115T0625Z-hec-compact-orbit, compact-run/ and separate report preserve
all arms and losses.12library+10anchor tests passed in release;2compact tests passed in debug.
Decision: retain optional compact kernel, change no defaults; stop equivalent arithmetic rewrites.
Next discriminating action: freeze held-out dense polynomial families and a growing-size ladder,
including all native/table controls, selection/resource caps, rank failures and full group gates.

## Held-out scaling terminal result
See HELDOUT-STATUS.md for the authoritative handoff.20dense curves,100single-target gates,
1500interleaved timed cells completed correctly;265559raw and4449projected identities checked.
Five-target curve median ratios against per-target fastest of12controls range0.94..3.17;
individual target maximum4.15. The g3p1009 family0 median is0.94. A/A maximum spread3.09.
These remain L0 producer assertions [pcc:unverified]. No10x or novelty established.
Record20261115T0700Z-hec-heldout-scaling closed with complete raw archive and separate native
Rust/jq/Typst report. Next step needs a mathematical cost case spanning relation generation,
sparse recombination and anchor algebra; stop equivalent arithmetic rewrites.
