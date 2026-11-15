#set page(paper: "a4", margin: (x: 15mm, y: 14mm))
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: 0.55em)
= PKM G2: record before gate, retain interrupted work
2026-11-05 11:42 UTC · local component and terminal library collection

#image("attempt-driver-flow-20261105.svg", width: 100%)

== Component delivered
The std-only attempt_driver.rs composes durable_attempt.rs with caller-supplied gate and work callbacks. The attempt record is created before evaluating the gate; Refuse omits work. Gate/work errors try to publish Failed while preserving the primary error and a separate publication error. A panic is not mistaken for completion. Read-only inventory retains every explicitly named attempt, including unfinished attempts beside a later retry, with counts and uninterrupted time unavailable. It is not an exhaustive history scan. #raw("[pcc:unverified]")

The native log records 11 passes: six new driver controls and the five existing record controls. Compilation, strict standalone clippy and rustfmt check exit 0. Tests exercise refusal before work, failed creation before either callback, gate/work errors, collision preserving both errors and pending bytes, all terminal labels, mock unwind plus retry, and duplicate/path/mismatched inventory rejection. Source/binary hashes and exact encoded captures are in attempt-driver-checks/receipt.json. Official claim PCC-20261105-8d2c82 has unverified standing; no independent signed verdict exists. #raw("[pcc:unverified]")

== Qualification boundary and next integration
Permit is supplied data, not an actual checkpoint/protocol/runtime decision. This library starts no process, enforces no deadline, and has not been wired into the PKM CLI or full-panel card. Source/request labels are not attestations. Completed is a callback label, not scalar or count validation. Trusted local directory assumptions remain; no hostile-root, power-loss, remote durability, production reaping or real resume guarantee follows.

Next: connect record-before-gate ordering to a qualified native supervisor, preserve actual child-reaping/deadline observations and every failure, then qualify interruption/resume without one continuous clock. Complete phase/counter coverage and pricing remain separate. No scientific launch was made.

== Existing G6 statements
Five existing cyclic/cardinality/recovery/kernel contracts have exact statement PCC bindings (48d13a, f5ef23, 1d3b07, b52ab0, d6264e). All standings remain unsigned/unverified. The unchanged project/gate/dependency objects retain the recorded audit of 18 declarations, zero trusted exceptions and official module replay against imports, not --fresh. No redundant proof run or new theorem is claimed. Rust/group representation, polynomial and complete-cost correspondence remain open. #raw("[pcc:unverified]")

#pagebreak()
= Current pinned library run: terminal and incomplete
#image("library-status-20261105T1142Z.svg", width: 100%)
The existing detached supervisor16391 was collected, never restarted. Library build exits0; execution declared6168 cases and ran 10:14:25–10:54:25 UTC with a 2400-second cap after build. Observed2166pass +2fail +45ignored +1unfinished +3954notreached =6168. Wrapper/supervisor124; direct Cargo exit unknown and final suite summary absent. This chart is a recorded test-status census, not a scientific performance comparison. #raw("[pcc:unverified]")

The unfinished case is PKM shared_lift_system_matches_lifts_one_and_the_brute_force_fiber, not the historical JV case. Its runtime cause is unknown. Both failures are diagnostic_reap_publication_regression_tests clock initialization. Same-binary focused diagnosis records2passes/2failures: ENOENT at DiagnosticSessionClock::begin, before sidecar assertions. This runtime lacks /proc/self/ns/time and /proc/self/timens_offsets; boot_id is readable. Runner source exactly matches main d30b9e0db. No proc substitute, assertion change, ignore or gate waiver was introduced. The old chain-SAT degree-bound test logs ok in this execution; full-library qualification remains incomplete. #raw("[pcc:unverified]")

All21source objects and actual232521608-byte test binary match the build receipt. Raw library log213453bytes, SHA256698fe35fe1d15a74ccce47535f75a9915d38782472fcf83379d50d4d1b6737db, stays outside Git, NOT_UPLOADED: no approved transport. Exact path, bounded excerpts, terminals and focused capture are in library-final-20261105T1134Z/receipt.json. Claim PCC-20261105-1fa454 is unverified. Older caps/failures/receipts remain immutable. #raw("[pcc:unverified]")

== Publication blockers and preserved acceptance
Aggregate validation remains failed: two inherited strict-clippy errors,13unchanged legacy PCC report-path failures, the two clock controls and capped suite. Auto-merge remains paused. A current-head gate/size/artifacts pass, required reviews/resolved threads and expected-SHA guard remain mandatory. No pending or absent check is a pass.

Original PKM_GOALS acceptance remains G1–G3/G6 partial, G4/G5 unrun, zero scientific panels, comparison_admissible=false and all cost/GAE/scaling metrics null. No panel/funding/lease/IAM/quota/pool change, and no checkpoint/protocol/runtime bypass. Full-panel card remains blocked/unfunded/unimplemented at its historical pin. Canonical IC/curve/performance graphs remain unchanged: no admitted value, curve, phase price or reference was added.

== Evidence and primary sources
Study-relative validation/main-d30b9e0db-20261105/: attempt-driver-checks/receipt.json; library-final-20261105T1134Z/receipt.json; formal-receipt.json; formal-claim-bindings.json. Editable graph data and SVGs are adjacent to this report. Rust catch_unwind documentation distinguishes unwind from abort; Child::wait/try_wait document reaping. Linux time_namespaces(7) documents CONFIG_TIME_NS and proc interfaces. Existing durable_attempt contract and G2 accounting audit informed the implementation. Primary URLs are in prerequisites/README.md and problems/ecbench-time-namespace-runtime.md.
