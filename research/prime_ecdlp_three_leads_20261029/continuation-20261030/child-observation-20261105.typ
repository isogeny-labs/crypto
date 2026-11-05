#set page(paper: "a4", margin: (x: 15mm, y: 14mm))
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: 0.55em)
= G2 prerequisite: observe the child before publication
2026-11-05 · source f543e03aa4fe6eaeccecd37c68bb4a06dae8479d

#image("child-observation-flow-20261105.svg", width: 100%)

== Implemented boundary
The std-only child_observation.rs wraps an already-spawned root child and uses existing local attempt records. A None status or deadline stop request cannot enable sidecar callbacks or terminal publication. Only an exit returned by Rust Child::try_wait or Child::wait establishes the local observed-reap state. Nonzero exit maps to Failed. A stop request followed by observed exit maps to Capped. Cached observed success is not relabeled by a late stop. This library starts no process. #raw("[pcc:unverified]")

The native recipe on committed source f543e03aa checks four source objects and all tracked worktree cleanliness before each phase. Compilation,18unit controls (7new child controls plus11existing record/driver regressions), strict standalone clippy for wrapper and fixture, and rustfmt check all exit0. The receipt binds four actual scratch binaries, phase terminals and exact encoded raw captures. No old recipe or library execution was restarted. Official observation PCC-20261105-b47218 has unverified standing; no signed verdict or execution-proof backend is claimed. #raw("[pcc:unverified]")

#table(columns: (1.2fr, 2.2fr), inset: 5pt, stroke: 0.4pt,
  [*New native control*], [*Behavior checked*],
  [Pending root], [No publish or sidecar callback; record stays unfinished],
  [Exit0], [Observed code0; one publication; repeat cannot overwrite],
  [Exit7], [Failed, never an invented zero or completion],
  [Expired deadline], [Kill request alone not a reap; Capped only after wait],
  [Premature stop/wait], [Precondition errors leave a pending child/record],
  [Publication collision], [Prior bytes retained after actual reap],
  [Late stop], [Already observed success retains original classification],
)

The dummy fixture executes only success, exit7 or blocked-stdin modes. These are software controls. There are no comparable scientific performance values, so the visual shows the state boundary rather than a speed chart. The precondition-error test is not an injected OS wait-error test. #raw("[pcc:unverified]")

#pagebreak()
= Qualification still required for a scientific driver
== Exact scope of the observations
The outcome record is currently label-only: it does not durably serialize reap, code, signal and stop observations. Caller-supplied attempt/child association and source/request labels are not attestations. Exit0 is no curve, scalar, witness or counter validation. Successful kill can race natural exit; stop-requested is a conservative cap label, not proof of causal exit reason or an exit timestamp.

Root reaping does not establish descendant cleanup or whole-process completion. The wrapper drains no pipes and supplies no source/binary/command/image binding, resource isolation, remote durability or qualified native gate/resume route. Drop does not kill/reap. Explicit handle transfer is for cleanup and loses monitor metadata; it is not resume. A record publication error may occur after terminal bytes are visible but before sync acknowledgment. These are explicit limits of the local component.

Next persist the actual observed-reap event and exact launch identities in the qualified supervisor; connect genuine gate-before-launch ordering and process-tree/deadline controls; qualify interruption/resume with retained unknown work and no uninterrupted clock. Complete native counter coverage, independent replay accounting and pricing remain separate G2 obligations. No scientific launch is authorized by this component.

== Current integration and validation dependencies
PR3452 was still open/conflicting at6968 at collection. Copilot request6657280205 had no response or visible resolver after more than an hour. Comment6658724527 defers the edit request to report-only because the append-only PCC ledgers fork after the common95-entry prefix. No frozen ledger side selection, event rechain, claim rewrite, canonical verifier patch or gate waiver was attempted. New main snapshot3a7882f5d has no relevant PCC policy/tool/ledger change since d9c. Main source changes are not qualified by old native receipts. #raw("[pcc:unverified]")

The current-head snapshot has review and migrate success; gate,size,artifacts absent. Auto-merge remains paused. Local prefix checks against d30 establish only that old base's prefix, not the current-main condition. The existing named ledger problem requires a sanctioned canonical provenance-preserving reconciliation contract. Fresh affected validation remains necessary after permitted source integration. #raw("[pcc:unverified]")

The source-bbf library run remains terminal124:6168declared,2166pass,2clock-initialization failures,45ignored,1PKM lift case unfinished,3954notreached. Cargo exit is unknown; no final summary/full pass. Raw213453bytes SHA256698fe35fe1d15a74ccce47535f75a9915d38782472fcf83379d50d4d1b6737db is NOT_UPLOADED. Inherited strictlint2 and legacyPCC13 remain failed at their recorded source scope. New component passes do not waive them. #raw("[pcc:unverified]")

== Evidence and preserved acceptance
Receipt and committed recipe: validation/child-observation-20261105/. The four inputs are child_observation.rs, child_fixture.rs, attempt_driver.rs and durable_attempt.rs. Primary source: Rust std::process::Child docs (Drop,kill,try_wait,wait), https://doc.rust-lang.org/std/process/struct.Child.html. Repository ReapObservation in ecbench/runner.rs and G2_ACCOUNTING_AUDIT.md informed the boundary.

G1–G3/G6 partial; G4/G5 unrun; zero scientific panels; comparison_admissible=false and all cost/GAE/scaling metrics null. Original PKM_GOALS acceptance and full-panel historical pin remain unchanged. No scientific panel, funding/lease/IAM/quota/pool/access change or checkpoint/protocol/runtime bypass. Existing G6 proof/gate/dependency objects are unchanged; no new Lean proof run or Rust/polynomial/cost bridge is claimed. Canonical IC/curve/performance views remain unchanged: no admitted metric, curve, phase price or reference. All earlier reports/receipts remain immutable.
