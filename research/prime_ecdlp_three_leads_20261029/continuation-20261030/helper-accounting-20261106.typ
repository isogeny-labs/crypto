#set page(paper: "a4", margin: (x: 15mm, y: 14mm))
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: 0.6em)
= G2: retain helper and raw relation-encoding work
2026-11-06 · source ab0df82d147f39252adde0a280a2b17f4b19f8b7
#image("helper-accounting-20261106.svg", width: 100%)
== Distinct implementations and owners
Setup parameters, setup constructor and per-oracle parameter checks now retain constructor-helper counts on success and Result-returning refusal. The selected scopes are disjoint. They capture existing synchronous current-thread helper calls, exclude calls between scopes, and emit unavailable after underlying or aggregate overflow. The numerical arithmetic is unchanged by overflow recording. Other legacy producer counters are not thereby overflow-qualified. #raw("[pcc:unverified]")

The relation encoder instead uses raw integer operations, so a coordinate-helper snapshot would misleadingly report none of its arithmetic. Its separate ledger records scalar-n row operations and coordinate-p orientation operations, plus base-entry/lookup diagnostics. It retains refusal prefixes and marks overflow unavailable. Map/lookup diagnostics do not measure complete container work. Scalar n and coordinate p keep separate units.

== New source, fresh controls and retained failure
#table(columns: (2.1fr, 1fr), inset: 5pt, stroke: 0.4pt,
 [*Phase*], [*Observed result*],
 [Oracle example suite], [32 pass, 0 fail/ignored],
 [New helper / pipeline controls], [2 / 3 included above],
 [Formatting / CLI build], [0 / 0],
 [Actual checkpoint refusal], [expected1 + message],
 [Final scoped Cargo lint / supervisor], [101 / 101: FAIL],
)
Claim PCC-20261106-59b811 is unsigned/unverified. Exact 22-object/global-clean guards, phase terminals/captures and two fresh preserved binaries are in validation/helper-accounting-20261106/receipt.json. Eight unchanged binaries/oldraw/formal dependencies are bound separately. No redundant unchanged library/process/Lean run. #raw("[pcc:unverified]")

#pagebreak()
= Conservation without double charging
== Five meaningful controls
1. Helper captures retain late Err work, merge exactly and exclude a helper call executed between scopes. Inverse 2 mod 17 retains one diagnostic plus its eight multiplication children.
2. A fresh isolated thread forces the underlying counter overflow: arithmetic still returns 6, new counts are unavailable. Sticky overflow and checked aggregate overflow also remain unavailable.
3. Setup parameter plus constructor helper counts equal the direct helper interval on the same call. Invalid primitive root retains parameter powering work; no constructor or point check starts.
4. A synthetic signed encoder row returns [1,18,12,4] with exact scalar/coordinate source classes; an invalid ordinate retains the earlier coefficient prefix. Counter overflow leaves the row unchanged and counts unavailable.
5. New producer encoding diagnostics are rejected when tampered. An older record without the optional field still replays for correctness. The replay uses the same encoder, so this is consistency checking, not independent cost proof. #raw("[pcc:unverified]")

== Two executions versus two views of one execution
Producer encoding and replay encoding are two actually executed paths. Equal numbers do not make the replay free or reusable; charge both executions. Within ONE replay execution, completed-oracle enumeration aliases attempted enumeration and must not be added twice. Within a helper capture, inverse/sqrt diagnostics overlap expanded multiplication children. The present aggregate supports the expanded-field view; choosing atomic inverse/root weights requires excluded-child attribution not yet emitted.

Producer output adds relation_encoding_work/v1. Nested replay-work stays v2 with additive helper/encoder fields; outer replay stays v2 and requests/outputs v1. Missing old diagnostics are unknown for producer pricing, not zero. Modulus, implementation and execution scope remain separate. Source calls are not machine instructions/cycles/GAE; no complete calibrated price or timing comparison is admitted.

== Material remaining coverage
Only selected synchronous current-thread helper calls are captured. Other-thread work, primality/discriminant/raw guards, JSON/binding, uninstrumented constructor/encoder logic, allocation/comparison/branch/counter/serialization overhead, complete F4/root/logical work, whole-process memory/journal and qualified interruption/resume remain open. Result retention is not crash/signal durability or execution attestation. Old affine/producer overflow limitations remain explicit.

Overall lint remains FAIL101 on inherited unused solve_one_with_trace and custody bool_comparison. Final example strict-lint is unqualified. All prior failed/capped receipts remain; the old 6168-case library result is not a pass. Formal/gate/Mathlib objects retain only their earlier 18-declaration/0-trusted scope and replay against imports, not --fresh; no new Rust/cost bridge or theorem is claimed. #raw("[pcc:unverified]")

Next extend remaining native/F4/root/logical coverage and freeze a calibrated nonoverlapping charge frontier. Frozen PCC history still requires a sanctioned reconciliation contract; no ledger side-selection/rechain or gate waiver occurred. Original G1–G3/G6 partial,G4/G5 unrun,scientific panels0,all cost/GAE/scaling metrics null,comparison_admissible=false. No funding/lease/IAM/quota/pool/access or checkpoint/protocol/runtime change.

Sources/artifacts: G2_HELPER_ACCOUNTING_20261106.md, prior G2 audits and actual source, ICMS units/windows, official Rust thread_local/Cell/overflowing_add documentation; validation/helper-accounting-20261106/. Diagram shows software ownership because no comparable performance values exist. Canonical scientific/curve/performance graphs and all older report bytes remain unchanged.
