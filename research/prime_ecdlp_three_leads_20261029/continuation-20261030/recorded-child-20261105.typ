#set page(paper: "a4", margin: (x: 15mm, y: 14mm))
#set text(font: "DejaVu Sans", size: 9.7pt)
#set par(leading: 0.55em)
= G2: persist launch descriptors and observed child exits
2026-11-05 · source fc09f0f1fdefbdb016f724528badfc9eac31abec

#image("recorded-child-flow-20261105.svg", width: 100%)

== Implemented integration
The Unix local prototype owns its Command, captures public configured program/argument bytes, and persists a unique attempt-ID descriptor before invoking the supplied gate. Refusal omits spawn. Gate/spawn errors retain distinct pre-child-handle events and preserve both primary and recording errors. After a child handle exists, actual root PID, exit code, signal, success and deadline-stop-requested fields come from std Child observations. The reap event is published before the existing outcome label. No record is overwritten. #raw("[pcc:unverified]")

Strict readback binds event bytes to the expected attempt ID and declared descriptor, checks canonical fields and outcome consistency, and rejects conflicting classes, transplanted IDs, torn/oversized/symlink records and inconsistent status. A published reap without the final outcome stays unfinished. Pending-only does not complete it. Work counts and uninterrupted time remain unavailable, never zero. This reader checks bytes and consistency, not execution truth or the writer's receipt of a sync acknowledgment. #raw("[pcc:unverified]")

== Native controls and retained failure
Final source fc09 ran a new committed five-input/global-cleanliness guarded recipe in native-final/. All27controls pass:9new integration controls plus18existing record/driver/child regressions. Strict wrapper clippy, fixture clippy and format checks exit0; supervisor0. Fresh source/binary/capture hashes and all phase terminals are in validation/recorded-child-20261105/final/receipt.json. Claim PCC-20261105-780ec8 has unverified standing. No signed verdict or execution-proof backend is claimed. #raw("[pcc:unverified]")

#table(columns: (1.2fr, 2.3fr), inset: 4pt, stroke: 0.4pt,
  [*Control*], [*Recorded behavior*],
  [Gate refusal / error / spawn error], [Descriptor exists before callback; no invented child exit],
  [Actual exit0 / exit7 / held child], [Code, signal and cap flag retained; none is scalar verification],
  [Reap before outcome crash window], [Reap readable but outcome remains unfinished],
  [Collision / transplant / corruption], [Prior bytes retained; different ID or inconsistent record rejected],
  [Bounds / duplicates / malformed hex], [Invalid input rejected; no replacement or inferred zero work],
)

Initial source b9d6a4862 passed27tests then failed strict clippy1 on chunks_exact_to_as_chunks. Its logs, source bindings and original recipe remain immutable in initial-b9d6a4862/. The decoder style was corrected without suppression and the changed parser/integration received fresh27-test validation. Earlier f543child receipts do not validate the new read-only success/signal getters. No tests/assertions/ignores/gates were weakened. #raw("[pcc:unverified]")

#pagebreak()
= Qualification and integration dependencies remain explicit
== What the descriptor does and does not bind
The configured public program/argument bytes belong to the same owned Command later passed to spawn. Request/source/claimed_binary_sha256 fields are caller labels, not execution attestations or hash checks of the executed image. Environment, image, argv0, I/O, pre-exec behavior and executable resolution are explicitly unbound. No environment values are recorded. Attempt ID uniqueness is within the selected local root, not a globally authenticated namespace.

This prototype is not wired into the production PKM entrypoint or full-panel card. A supplied Permit is not a genuine checkpoint/protocol/runtime decision. Root reaping is not descendant cleanup, witness/scalar verification or a completion-time bound. The cap flag means stop requested; successful kill can race natural exit. Drop does not stop/reap. The caller retains handles on errors and must arrange cleanup. Pipe draining, hostile-root races, power-loss, remote durability and qualified interruption/resume are untested. A sync error can occur after visible bytes. No scientific clock, price or completed full-runtime route follows.

Next qualify the source/binary/image/environment/protocol identities and genuine gate, then process-tree/deadline/interruption/resume behavior with retained unknown work. Complete F4/root/logical/native counter coverage and pricing remain separate G2 obligations. Existing G6 group/cardinality/recovery/kernel proof/gate/dependency objects are unchanged; no redundant Lean run or Rust/polynomial/complete-cost bridge is claimed.

== Exact CI evidence and frozen history
PR3452 remained open/conflicting at61f43aa9e with auto-merge paused. Required gate,size,artifacts are absent; review succeeds and auxiliary migrate fails. Actual61f jobmetadata identifies the prefetch step. --log-failed returned empty, while direct job-log retrieval returned403, so its exact error message is unavailable. A private expiring redirect diagnostic was not copied into Git. The earlier5bd missing-helper diagnosis remains only that run's evidence. Do not substitute it for61f. #raw("[pcc:unverified]")

Main snapshot389a522da has no PCCpolicy/tool/ledger delta since3a. Frozen ledgers still fork after the common95entry prefix; old-d30 prefix/local-chain checks cannot satisfy the current-main requirement. Copilot edit request6657280205 remains deferred/report-only under comment6658724527. No ordinary main merge, frozen side selection, event rechain, claim rewrite, verifier/keyring patch or gate waiver is attempted. A canonical sanctioned reconciliation contract is required; any future permitted resolution needs both-parent review and fresh affected validation. #raw("[pcc:unverified]")

The6168-case source-bbf run remains capped124 with2166passes,2clockfailures,45ignored,1PKM lift case unfinished and3954notreached. Cargo exit is unknown; no final summary/full pass. Raw213453bytes SHA256698fe35fe1d15a74ccce47535f75a9915d38782472fcf83379d50d4d1b6737db remains NOT_UPLOADED. Strictlint2 and legacyPCC13 remain failed at their recorded scopes. New local passes waive none of them. #raw("[pcc:unverified]")

== Sources, artifacts and original acceptance
Primary sources: Rust Command::get_program/get_args/get_current_dir/get_envs, File::sync_all/Drop and hard_link; links in prerequisites/README.md. Existing local record/child primitives and G2 audit informed the method. Report SVG is a state diagram: only software correctness controls ran, with no comparable scientific performance values.

Original PKM_GOALS acceptance: G1–G3/G6 partial,G4/G5 unrun,0scientific panels,comparison_admissible=false,all cost/GAE/scaling metrics null. Full-panel card remains blocked/unfunded/unimplemented at its historical pin. No scientific panel, funding/lease/IAM/quota/pool/access change or checkpoint/protocol/runtime bypass. Canonical IC scoreboard/progress-timeline, curve and performance views remain unchanged: no admitted value, curve edge, phase price or reference. All historical source/evidence/receipt/report bytes remain preserved.
