# Round09 validation

Receipts, including failed attempts, are retained in runs.tar.xz.

| Check | Receipt | Outcome |
|---|---|---|
| Root library | root-lib-tests-01.log | 4405 passed,0 failed,118 ignored |
| Native study | native-tests-01.log | 106 passed,0 failed,1 ignored |
| New output replay | replay-01.log | All five JSON outputs byte-identical |
| Prior round08 | prior-round08-replay-01.log | All eight JSON outputs and separate checks unchanged |
| Frobenius circuits | q7-Frobenius-coordinate-circuits.json | 48 univariate identities,40 sigma identities,80 exact slices,2880 steps,400 transition controls |
| Parsed descent | q7b-parsed-descent-chart-check.json | 48 identities,576 chart squares,2880 additional steps |
| Trace descent | q7c-invariant-rational-generators.json | 129600 pairing entries,360 idempotents,64 traces,5760 values,16 recoveries,128 equivariance checks |
| Bounded low-genus control | l7-diagonal-twist-reflection-controls.json | All50 signed cases,zero stable-and-retaining cases in the known component |
| Coupled restrictions | m12-coupled-function-restrictions.json | 40320 evaluations;all eight functions nonzero on720 known columns;no new relation |
| Lean build | lean-build-04.log | Full pinned project passes;17 new named proofs |
| Transitive audit | lean-axioms-02.log | 17 audited declarations,0 failing,0 native trust |
| Dashboard/site | dashboard-check-01.log,site-tests-01.log | Current evidence pins;49 site tests pass |
| Workflow parse | workflow-tests-01.log | Four tests pass;focused25-minute cap unchanged |

The initial Rust build had an indexed-array json parsing error. Its following
stale-binary invocation rejected the output directory; neither produced an
accepted result. Lean build03 exposed a missing DecidableEq parameter in
the generic recovery lemma; build04 and the final audit check the corrected
statement. A preceding wrong relative append path left build02 checking the
earlier15-theorem source; it is not the final proof receipt. Build and
attempt logs are preserved. All theorem claims refer to build04 or later.

The global Miller divisor/normalization proof, connected Kummer-cover
identification, constant-deck argument and Galois descent application are
written in PROOF_MAP.md. They are not fully formalized in Lean. Native
finite-field semantics are an explicit boundary. No lean4checker binary is
available. Same-agent fresh replay and separate parser share field/group
primitives; no external audit is claimed. The low-genus control covers one
known component, not every Jacobian factor. Coupled restrictions are not
complete divisors. F2 polynomial relations, saturation, a closed embedding,
Jacobian identification, general reduction and individual descent remain
open. The user assesses IC efficiency; no cost is measured.

Canonical status narratives and dashboard metadata are updated. Cost and
rho-comparison graphs are unchanged because no cost was measured. The new
editable study diagram records constructions, counts and bounded negatives.
Earlier archives are unchanged. Remote checks are inspected on the pushed
commit separately; local passes do not imply pending CI has passed.

The full proof-gate run passes123 project theorems with0 trusted native
proofs (lean-full-gate-01.log). Its complete build, cache and transitive
audit receipts are retained as lean-full-project-*-01.log, covering earlier
modules and the new one together.

Native assembly issues two Lean-bound IDC certificates, each with64 issuer
and128 extra checks (record-01.log). All65 source bindings and54 raw bindings
pass their retained hash receipts. ARCHIVE_VALIDATION.txt confirms the
archive digest, all57 member hashes, all five fresh outputs and a
byte-identical deterministic repack. No ordinary raw file changed after
sealing. The editable SVG, PNG, Typst and four-page PDF are generated from
the ledger and report text; every page was inspected for clipping,
overlap, symbols, links and legibility. The figure's root formula was
reduced slightly in size to fit its box before the final inspection.
