# Resume: mathematical 10x hyperelliptic IC goal

## Goal
Establish a mathematical breakthrough giving >=10x complete IC speedup for genus2 and above.
This remains active and unachieved. Current branch research/hec-principal-relations-20261114,
worktree /Users/adamburan/pq-work/hec-followups. PRs4096 and4170 are open;4170 is currently CLEAN.

## Lead / hypothesis
Exact principal-divisor identity from interpolation through g+1 rational places yields <=2g+1
entry relation rows. This is known Riemann-Roch mathematics, not novelty; screen its end-to-end
cost and rank against best-native controls before pursuing any breakthrough claim.

## Spec
research/hec_relation_math_20261114/DERIVATION.md, committed13930db391 before implementation.
Eleven round-eight instances at seeds20260916..20260920; include both target descents, all solve
retries, catalogue construction and final verification. Four Fast/Hybrid x reference/direct
controls plus one-step walk. Candidate cap1million trials,16solves,10000descentsteps.

## State
Experimental code interpolation.rs and example principal_relations implemented. Release tests
passed two tests including18 gated solves across six instances, all generated principal identities
and target descents checked. Debug gate and build initiated; collect before experiment.
No >=10x result established. Conductor T-5922 reserved paths; progress endpoint later hung.

## Next steps
Run recorded HEC_GATE=1 pilot first (all accepted rows group-checked); then run normal pilot.
Example binary /Users/adamburan/pq-work/hec-followup-target/release/examples/principal_relations.
Use benchrec run and preserve gate output and measurement output separately. Summarize against
best-control complete time, retain capped/rank-failure rows. Extend sizes only with a cost case.
Write source-linked report/graphs/PDF and benchmark record. Keep goal active if any gate misses.

## Runs in flight
All five recorded mathematical pilots are terminal. Final release library/example tests are
being collected in the current session. Do not restart completed pilots. Current source and
latest raw results are summarized in research/hec_anchor_relations_20261114/STATUS.md.

## Stop and success conditions
Local CPU only; no cloud spend authorized. A pilot or a known-method reproduction is not goal
completion. >=10x requires held-out growing-size per-genus ladder, independent replay, baseline
ratchet and mathematical novelty review. Preserve the full objective on null/negative evidence.

## Open questions
Will short rows retain enough rank and target connectivity? Does a one-step walk give the same
gain more cheaply? Is there any mechanism beyond known interpolation that survives prior art?

## 2026-11-15 04:54 UTC: next mathematical lead
Principal pilot finished:55 gated candidate runs and330 paired solves all checked, but unrestricted
principal rows lose at every row to best-native controls. At g2p4001:123ms vs5.3ms one-step walk.
Next source ee98ca8b1d: table_factor.rs and table_oracle implement exact cubic and quartic root
lookup via projective cubing/Cardano/Ferrari, p>3, p<=1million. PROTOCOL.md at
research/hec_table_oracle_20261114/ freezes lemma and gates before code (c8a30320a6).
Two release tests passed (all monic degree1..4 atp5,7,11,13 compared to independent root division,
plus48000 randomized/constructed cases across8primes). Next is component measurement; table
construction and O(p) memory must be charged. This is NOT an end-to-end10x result.
Conductor MCP authentication failed; CLI check+claim worked for separate disjoint task T-5932.

## Table pipeline gate
The table pipeline preserves catalogue, trials, relations, rank retries, pivot hash, pivots,
row operations, modular multiplications and k on220matched configurations. Genus2 builds only
square roots (8p bytes), using square recurrence; genus3/4 builds inverse/cube/projective tables.
Three release tests passed (including exhaustive/randomized oracle checks). Ready for new
end-to-end record, eight controls and four table choices on eleveninstances x fiveseeds. Every
cold setup, catalogue and verification remains inside total time. Component cold10.4x atg3p401
is a component result only, with g2p503 regression in the initial all-degree table setup.

## Anchor screen
New protocol/source research/hec_anchor_relations_20261114/ (target-seeded H, <=2 off-anchor,
weighted forest, signed-cycle proof). Twelve gated solves passed, plus planted arbitrary-gain
forest invariants. Next: recorded55-case full gate and55-case timed pilot, compare to strongest
native AND table baselines. Tables/caches/descents/forest/verification all charged cold.
Goal remainsactive: no >=10x complete gain established. User requests skipping further repeated
coordination. Work remains in task T-5941's already-claimed paths.

## Signed-orbit pilot and packaging
Source88a969080d integrates Gray-code norm updates, one batch anchor-difference inversion,
bounded dense affine forest and incremental dense anchor elimination. Genus2 tuple family is
shuffled without replacement. Original Horner catalogue retained on both sides by explicit
pre-timing amendment. All55 group gate cases pass (15115raw,1290projected identities), and
all110 interleaved cold solves pass. Per-size five-seed median gain vs frozen table controls
1.27..2.94x, exploratory L0 macOS. Producer assertions [pcc:unverified].
Five records now closed with raw hashes and shared report/plots in
benchmarks/20261114-hec-anchor-forest. Source-linked report visually inspected, two pages.
PR4274 is open ready for review, stacked on research/hec-followups-20261114. All12 release
library tests and13 research-example tests passed;2dense tests also passed in debug. Current
Lean recheck compiled Algebra.lean and Forest.lean; outputs retained in the anchor run directory.
Checkpoint20261115T060823Z-2c7378556b09 uploaded successfully. Artifact migration commitf385e46524
was fast-forwarded locally; the report PNG now has an artifact-store manifest. Next: finish
current-head CI and S3 custody checks; preserve
independent replay, external comparison, held-out scaling and >=10x as outstanding research.
Do not mark the goal complete based on this engineering/algebraic reuse pilot.

## Compact high-coefficient quotient gate
Previous goal turn was progress: signed-orbit code,55group gates,110cold solves,PR4274 and
green CI/S3 custody on868eec5948. Current successor protocolCOMPACT-PROTOCOL.md committed
1a115ad302 before implementation. Sourceb2fddfd8b2 retains exact signed relation stream and
uses bounded coefficient arrays plus high norm coefficients only. Two kernel tests compare
22128exhaustive and8000random (h,Q) pairs to reference/full Newton norm division; all10example
tests passed, including integrated reference/compact group solves. [pcc:unverified]
Record20261115T0625Z-hec-compact-orbit is open. Next commands: benchrec run HEC_GATE=1
hec-followup-target/release/compact_bench to compact-run/gates.txt; require110 completed
reference/compact solves, identical14field fingerprints and55fully checked compact cases.
Only then run normal harness for165interleaved cold solves against frozen table control.
Do not change earlier closed raw records or their report; use a separate compact addendum.
Goal remains full mathematical>=10x, with independent replay/held-out scaling outstanding.

## Compact pilot terminal result
Gate110reference/compact runs passed (55compact group gates,15115raw/1290projected checks).
All165timed cases passed. Reference-orbit/compact median total ratios0.83..1.32, including
g2p101 loss; frozen-table/compact ratios1.22..2.98. [pcc:unverified]
Record20261115T0625Z-hec-compact-orbit is closed, with compact-run raw files, standalone
COMPACT-REPORT.pdf/compact.svg, and reproducible summarize_compact.jq/render_compact.py.
12library+10anchor tests passed release;2compact tests passed debug (57.58seconds).
Stop equivalent arithmetic rewrites. Do not call this a mathematical breakthrough.
Next: held-out dense coefficient families and growing-size ladder under a fresh frozen protocol,
all native/table controls, bounded instance selection and per-arm caps; preserve rank failures.
No experiment is running. Current PR4274 must be extended with this commit and CI followed.
