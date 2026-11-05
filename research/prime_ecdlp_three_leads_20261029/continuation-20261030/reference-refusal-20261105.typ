#set page(paper: "a4", margin: (x: 15mm, y: 14mm))
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: 0.6em)
= G2: retain formerly discarded affine check work
2026-11-05 · source c933e1bdef0e98e3c37282d6ec313565c432f3af
#image("reference-refusal-20261105.svg", width: 100%)
== Caller-owned stages
Setup's affine Curve now survives a Result-returning refusal. Explicit setup state distinguishes work not started, in progress, returned and rejected. Oracle checks separately retain request validation, submitted-witness verification and exhaustive enumeration. The caller merges all attempted work before propagating Err, with attempted and failed call diagnostics. This also retains witness arithmetic omitted by the old successful checker. #raw("[pcc:unverified]")

The legacy certificate reference_counts and completed_oracle_counts stay enumeration-only. They overlap attempted enumeration: do not add both. Within each stage, point/inverse diagnostics overlap expanded field operations. Scalar n and coordinate p remain distinct. Legacy affine overflow remains unqualified. No complete price or timing comparison is emitted.

== Actual checks on the new source
#table(columns: (2.1fr, 1fr), inset: 5pt, stroke: 0.4pt,
 [*Phase*], [*Observed result*],
 [Full oracle example suite], [27 pass,0 fail/ignored],
 [Formatting / CLI build], [0 / 0],
 [Actual checkpoint refusal], [expected1 + message],
 [Final scoped Cargo lint / supervisor], [101 / 101: FAIL],
)
Claim PCC-20261105-6020c1 is an unsigned producer observation. Final receipt/captures retain22guarded source objects, phase terminals and two fresh binaries. The real gate still refuses before controlled nonexistent input and before a replay error envelope. No library/legacy/Lean/process-prerequisite repeats were needed for unchanged input objects. #raw("[pcc:unverified]")

#pagebreak()
= Conservation, refusal boundaries and remaining work
== Four new controls
1. Success and lost-output refusal retain witness verification and enumeration in separate ledgers. Legacy enumeration counts remain equal to the prior enumeration view; attempted ledgers conserve work when merged.
2. A corrupt witness retains exactly the checked prefix and starts no enumeration. Duplicate detection starts no witness/enumeration arithmetic; prior request validation remains separately counted.
3. A fixed valid F257 control reaches the one-million signed-tuple guard after its scan. All257membership tests and128times257point tests remain counted; no signed tuples were visited.
4. A prime but incorrect F17 full-curve order retains the whole291point-check scan,1164field multiplications and582field additions, with no oracle/rank work started.
These are correctness controls, not scientific panels. The existing refusal control also covers retained validation before an oracle-binding error and a setup-schema error. The full suite retains old m4/all-target and scalar certificates. #raw("[pcc:unverified]")

== Preserve the actual unsuccessful attempt
Initial b013cb1e3 compiled no tests: new unwrap_err required Debug on the success Certificate. It exited101; later phases were not reached. The test now extracts Err with an explicit expectation and checks the same order-error message. Old source/recipe/bindings/logs stay in initial-b013cb1e3/. A new pinned recipe/directory validates c933; no old recipe was restarted. Final Cargo lint still fails on the unchanged library unused solve_one_with_trace and custody bool_comparison; final example strict-lint remains unqualified. No test/assertion/ignore/lint/gate waiver. #raw("[pcc:unverified]")

== Schema and cost boundaries
Nested work changesv1 tov2 for owned setup counts/state and attempted oracle stages; outer replay remainsv2 and request/producer records remainv1. The setup refactor changes producer source bytes, so f866's before-Certificate byteidentity claim does not apply here. Numerical success behavior is regression-tested; no producer timing or speedup is admitted.

Parameter guards, JSON/binding checks, setup constructor/shared relation encoding, allocation/copy/comparison/branch/serialization/counter overhead, complete F4/root/logical work, whole process/memory/persistence and qualified interruption/resume remain unpriced/uncovered. Result-error retention is not panic/crash durability, execution attestation or a production process journal. Owners are internal and fresh per attempt. Source calls are not machine instructions, cycles or GAE.

Next qualify a nonoverlapping charge frontier and remaining native coverage, followed by full process/runtime-resume accounting. The frozen PCC fork remains report-only pending a sanctioned contract; required exact-head gates/reviews remain necessary. Original G1–G3/G6 partial, G4/G5 unrun,0scientific panels, comparison_admissible=false and all metrics null. No access/funding/lease/IAM/quota/pool change or gate bypass.

Sources and artifacts: G2_REFERENCE_REFUSAL_20261105.md; prior G2 audit and actual reference/setup/replay code; ICMS accounting rules; official Rust Result ownership documentation. validation/reference-refusal-20261105/final/receipt.json and exact captures bind this source. Diagram shows software ownership because no comparable performance value is admitted. Canonical IC/curve/performance graphs are unchanged; all older reports and failure receipts remain preserved.
