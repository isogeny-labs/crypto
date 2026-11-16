#set page(paper: "a4", margin: (x: 15mm, y: 14mm))
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: 0.6em)
= G2: retain work spent checking independent replay
2026-11-05 · source f866dd7a4578a5c02cb60ee7d3c2101f3b044cde

#image("replay-accounting-20261105.svg", width: 100%)

== Implemented ownership
The independent rightmost-pivot scalar checker now has a caller-owned shared meter. Coefficient and augmented spaces contribute to the same scalar ledger, including their separate construction/prime guards, reductions, provenance sums, original-row recovery and target-changing kernel checks. Affine setup, equation checks and completed oracle-reference counts remain a separate coordinate view. Scalar modulus n differs from coordinate field p; the fixed fixture uses F17 coordinates and order19. #raw("[pcc:unverified]")

Success adds replay_work to replay/v2. Rejection retains partial work in a failure envelope. The real CLI checkpoint gate still runs before input/replay; early gate, input and JSON errors are outside this accounting API. Failed internal setup/oracle counts remain explicitly unavailable. A refused call cannot be reported as zero work merely because it returned no counts. Any scalar counter overflow invalidates the exact scalar snapshot without changing arithmetic results. #raw("[pcc:unverified]")

== Fresh guarded controls and actual failure
#table(columns: (2.1fr, 1fr), inset: 5pt, stroke: 0.4pt,
 [*Phase*], [*Observed outcome*],
 [Standalone rank controls], [6 pass; 0 fail/ignored],
 [Cargo oracle example controls], [23 pass; 0 fail/ignored],
 [Standalone strict rank lint / format], [0 / 0],
 [CLI build / actual checkpoint refusal], [0 / expected1 + message],
 [Final scoped Cargo lint / supervisor], [101 / 101: FAIL],
)
All results are producer observations: PCC-20261105-f04c69 standing unverified. No independent signed verdict or execution-proof backend is claimed. Fresh recipe guards22committed objects and alltrackedcleanliness before everyphase. Four fresh binaries, exact small logs and phase terminals are pinned in validation/replay-accounting-20261105/receipt.json and phase-captures.json. #raw("[pcc:unverified]")

#pagebreak()
= Source-call units, refusal work and next coverage
== Coverage controls
Three new standalone controls check Euclidean inverse child-call identities (2mod19), inverse failure and composite-prime guards; conservation across separately owned versus shared row-space ledgers; retained work after late sparse-certificate/noncanonical refusals; and overflow becoming unavailable. Existing controls exhaust all729 two-by-three F3 matrices plus membership/recovery/kernel checks, row order, near32bit values and malformed records. Two new pipeline controls check serialization and p/n separation, retained work after rank tampering, failed-oracle unknown work and absent failed-setup counts. The oracle suite includes the6rank controls and totals23; these are not29distinct controls. #raw("[pcc:unverified]")

== Alternative views are not additive prices
Twenty scalar source-call classes include modular arithmetic, integer children, inverse/Euclidean iterations, prime guards and input checks. Modular/inverse/iteration diagnostics overlap integer children. Do not charge both. Euclidean scalar inverses and coordinate/reference inverse implementations use different primitive paths; keep their units separate. These calls are not machine instructions, cycles, wall time or GAE. Compiler strength reduction and instrumentation overhead remain outside such an interpretation.

Successful setup and completed oracle counts are retained; failed internal calls may lose counters and are marked unknown. Legacy affine overflow is not newly qualified. Allocation, copying, comparison, branches, serialization, counter overhead, shared encoder, full F4/root/logical work, whole-process memory/journal/resume costs and calibration remain uncovered. Replay audit work is separate from producer timing: it is neither silently free nor folded into that earlier interval. coverage_complete=false and priced_total=null remain explicit.

== Failures remain failures; unchanged scopes remain unchanged
Final Cargo lint stops in the unchanged library on unused solve_one_with_trace (native_solver.rs:1351) and bool_comparison (custody.rs:713). The final Cargo example strict-lint scope is therefore not qualified. No allow/ignore/skip, assertion change or gate waiver was added. Earlier passing phases do not make supervisor101 pass. #raw("[pcc:unverified]")

Library/Cargo/dependency/fixture objects remain unchanged. The old6168-case suite still has2clock failures and a40minute cap with unfinished PKM lift work; no full-suite pass is asserted. Formal/gate/Mathlib objects remain identical to their earlier18declaration/0trusted scope and kernel replay against imports, not --fresh. No redundant unchanged library/proof run occurred. The producer pipeline prefix before Certificate is byte-identical to992. #raw("[pcc:unverified]")

Next: retain failed internal oracle/setup work, define a nonoverlapping charge frontier, extend native F4/root/logical coverage with conservation/refusal controls, and qualify whole-process accounting/runtime-resume. Frozen PCC append forks still require a sanctioned provenance-preserving contract; no frozen main merge or ledger rechain was attempted. Required exact-head CI is not claimed passed. No approved lab/artifact transport or scientific panel was introduced.

Original G1–G3/G6 remain partial; G4/G5 unrun; scientific panels0, comparison_admissible=false and all cost/GAE/scaling metrics null. Canonical IC scoreboard/progress-timeline, curve and performance views remain unchanged because this accounting change admits no metric, curve edge, phase price or reference. Diagram shows ownership because there are no comparable performance values.

Sources: existing G2 audits and actual replay/reference paths; docs/ic/measurement/README.md section3; official Rust i128::rem_euclid and u64::overflowing_add. Detailed links and coverage contract: G2_REPLAY_ACCOUNTING_20261105.md and SESSION_20261105T1630Z.md. Old reports and raw evidence are preserved.
