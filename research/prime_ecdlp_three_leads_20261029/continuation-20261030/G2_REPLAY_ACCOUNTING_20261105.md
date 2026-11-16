# G2: retain independent replay work, including refusals

Source f866dd7a4578a5c02cb60ee7d3c2101f3b044cde. Original PKM_GOALS acceptance
is unchanged. This is accounting coverage, not a lower complete DLP cost.
All results below are producer observations, PCC-20261105-f04c69 unverified.

## Ownership and units

`rank_reference::Meter` is shared by the coefficient and augmented spaces.
It outlives a refused operation and records both constructors and prime-guard
work. `ReplayAccounting` owns that meter and an affine Curve, successful setup
counts, completed oracle-reference counts and explicit failed-oracle unknown
state. Production CLI retains this owner when replay returns an error.
Success adds `replay_work` to replay/v2; rejection prints
`pkm.toy-dlp-replay-failure/v1` with partial work. Early checkpoint/load/JSON
failures are outside this API and remain uncovered. The actual checkpoint
still refuses before input inspection. Request and producer records remainv1.
[pcc:unverified]

The scalar view is modulo subgroup order n, coordinate view modulo p. The
fixture separates F17 coordinates and scalar order19. Twenty scalar classes
count instrumented source calls, never instructions/cycles/GAE. Modular calls,
inverse calls and Euclidean iterations are diagnostic parent views; integer
products/additions/subtractions/remainders/divisions are overlapping children.
Choose a calibrated nonoverlapping charge frontier later. Do not sum both.
Fermat/reference inverses already expand multiplication children and are
separate from Euclidean scalar inverse paths. Existing uninstrumented ordinary
PrimeCurve inverse internals remain a gap. Compiler strength reduction and
instrumentation overhead preclude interpreting calls as machine operations.

Any u64 scalar counter overflow invalidates the whole exact scalar snapshot;
output becomes unavailable, not zero or exact saturation. Arithmetic continues.
Legacy affine counters have no newly qualified overflow contract. Failed setup
and oracle calls can discard internal counters; setupNone and explicit failed
oracle-work-unavailable flag preserve that unknown rather than fabricate0.
Completed calls remain counted even if later replay rejects. The shared relation
encoder, allocations/copies/comparisons/branches, serialization, counter updates,
whole-process/memory/persistence/resume and timing remain uncovered. No cost of
independent replay is silently free or folded into the producer interval.

## Meaningful controls and exact validation scope

6 standalone controls include exhaustive729 F3 matrices and old malformed/
permutation/near32bit cases; new controls check Euclidean child-call identities,
failed inverses/composite guards, independent-vs-shared ledger conservation,
late original-row/canonical-input refusal and forced overflow. Two additional
pipeline controls test serialized field separation and successful replay work,
late rank tampering with retained affine/scalar work, unknown failed oracle
work and absent failed-setup counts. The oracle suite totals23. [pcc:unverified]

Fresh guarded recipe checked22committed objects and alltrackedcleanliness before
everyphase. Standalone strictclippy/format/CLIbuild pass and actual CLI refuses
with message+exit1 before replay-envelope emission. Final Cargo scopedclippy
FAIL101 on inherited unused solve_one_with_trace and bool_comparison. Supervisor
101 remains failure; strictlint of the final Cargo example is not qualified.
All exact small captures, terminals and4freshbinary hashes are in
validation/replay-accounting-20261105/receipt.json and phase-captures.json.
[pcc:unverified]

## Next checks and retained acceptance

Qualify failed internal oracle/setup accounting and a nonoverlapping pricing
frontier, then F4/root/logical/native coverage with conservation/refusal controls.
Qualified real gate/source-image/environment/process-tree/interruption/resume
and whole-process journal are separate runtime work, not satisfied by prior
local mock-gated child primitives. No approved lab/artifact transport is present.

| Goal | Current requirement-to-evidence status |
|---|---|
| G1 | Partial: fixed toy controls, not frozen multi-curve qualification |
| G2 | Partial: new independent replay source calls; complete work/pricing open |
| G3 | Partial: equal-domain qualified external comparisons open |
| G4 | Unrun: equal-output held-out optimization panel |
| G5 | Unrun: predeclared scaling/reach and censoring panel |
| G6 | Partial: unchanged18-declaration Lean scope, Rust/polynomial/cost bridges open |

Producer pipeline prefix before Certificate is byte-identical to992 (SHA256
3eda59f0533eabc14e6bfca8f763b477e58bb0b5ee4f99efda85253c4627bf7a).
Cargo/lib/dependency/fixture/formal objects unchanged; no duplicate unchanged
full-library/proof run. Prior6168-case clock2failure/cap, strictlint2, legacyPCC13,
frozen current-main ledger fork and absent required CI passes remain. No frozen
history merge or scientific execution was attempted. Allmetricsnull,
comparison_admissible=false, full-panel historicalcard unchanged. [pcc:unverified]

Sources checked before implementation: existing G2 audits and actual scalar/
replay ownership paths; docs/ic/measurement/README.md section3; Rust i128
rem_euclid and u64 overflowing_add official documentation (linked in
SESSION_20261105T1630Z.md). Existing Euclidean recurrences are reused; no novel
ECDLP mechanism or global novelty claim follows.
