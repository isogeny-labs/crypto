#set page(paper: "a4", margin: (x: 15mm, y: 14mm))
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: 0.6em)
#set heading(numbering: none)
= PKM continuation: attempt records and statement fidelity
2026-11-05 · G2/G6 software prerequisites · unsigned local observations

#image("attempt-record-flow-20261105.svg", width: 100%)

== G2: a local attempt-record component
The new std-only Rust primitive creates a unique attempt directory and canonical binding record, requests file and directory synchronization, and publishes a synced outcome through a nonoverwriting hard link. A missing terminal remains unfinished even if a pending file contains a complete payload. Work counts stay explicitly unavailable. Duplicate records, corrupted or transplanted bindings and I/O failures are retained as errors. The native test log records5 fault-control passes with strict clippy exit 0 on component source 3210f04b3. #raw("[pcc:unverified]")

This library launches nothing and is outside the active Cargo/Lean source inputs. It is not wired into the driver or full-panel card. Callers still need checkpoint/protocol/runtime gates. Source/request labels are supplied data, not attestations; completed status is not witness, scalar or cost verification. An error can occur after a terminal link is visible; inspection reports bytes, not receipt of the writer's durability acknowledgment. Filesystem semantics, power-loss behavior, hostile-directory races, remote persistence and qualified supervisor/resume remain separate obligations.

The next technical step is to create the attempt before a gated child starts, capture every terminal result, preserve unknown work after interruption, and reconcile resume without an uninterrupted clock. That integration remains open; this component supplies only local record mechanics.

== Integration review and affected native checks
After Copilot request 6652134122 had no response for more than an hour, fallback explanation 6653243863 preceded ordinary main merge bbf241b7e501be105c1878a4e05fc9fc89c3c877. Its parents are published 382552944 and main d30b9e0db. Only Oct30/Nov5 logs and problem index conflicted. Local prefix/paragraph-occurrence checks retain 1845/2092 and 64/95 parent paragraphs, plus 97/107 parent links as 109. All 407 earlier study blobs are unchanged; prepared request-session entry was appended verbatim once. These are producer-side checks. #raw("[pcc:unverified]")

The new 17-path native recipe is pinned and running in /workspace/pkm-pr3452-main-20261105/native/, supervisor 6743. The first phase logs 26 release-example passes,0 failures/ignored; further phases are not qualified by that receipt. Two additional local Cargo dependency trees (isogeny_algos and Pencil) have supplemental identities captured during compilation and rechecked at collection. The original 17-path guard was not retroactively changed. #raw("[pcc:unverified]")

No bound source was edited or newer main merged while active. The new record component is independently compiled/tested outside those inputs. Library execution still needs its actual declared count, generated binary, terminal reconciliation and 2400-second execution cap after build. Old 5848-case failure/cap evidence remains immutable and does not validate new dependencies.

#pagebreak()
= G6: bind the exact supporting statements
The changed Lean gate locally reports 18 declarations, 0 trusted exceptions, and module kernel replay exits 0 against imports (not --fresh). Lean4.34.1; Mathlib d13f23b723b8a846827a245b89c10fc7d3f11612. The five existing contracts now have normalized statement hashes and separate PCC claim records. Each local verifier exits 0, but every standing remains unverified without an independent signed verdict. No new theorem source or stronger mathematical result is supplied. #raw("[pcc:unverified]")

#table(columns: (1.2fr, 1fr), inset: 5pt, stroke: 0.5pt,
  [*Contract*], [*Claim (all unsigned)*],
  [cyclic_point_relation], [PCC-20261105-48d13a],
  [prime_card_generates], [PCC-20261105-f5ef23],
  [prime_order_point_relation], [PCC-20261105-1d3b07],
  [recovery_combination], [PCC-20261105-b52ab0],
  [target_ambiguity], [PCC-20261105-d6264e],
)

The group contracts assume exact cardinality and generation, or derive generation from exact prime cardinality and a nonzero point. The scalar contracts assume valid original rows and a directly checked weighted-unit-row or target-normalized kernel witness. Kernel ambiguity concerns retained scalar equations. Rust representation, polynomial correspondence and complete solving/extraction-cost bridges remain open. See proof-map.md and the exact normalized declarations in formal-claim-bindings.json.

== Policy dependency found and retained
Main introduces proof-carrying claim records. Its 95-entry ledger remains an exact prefix; new observations/contracts are appended using the official tool. An initial sparse-checkout initialization error is preserved separately; no committed history or claim body was rewritten. The local ledger/canary/tests return 0. After 404 exact tracked archive inputs were restored, benchrec reports 37 records/current progress. #raw("[pcc:unverified]")

Global PCC verification still exits 1 on 13 unchanged main claims: the legacy validator raw_data path ../../../research/ic_rho_reference_20260923/reprice/ic-boundary-ledger-round5-2026-09-22.json is interpreted as escaping the repository. The same reports also expose missing digest bindings. The shared verifier, frozen claims/report and trust configuration are unchanged here. The named problem pcc-legacy-validator-relative-path-binding.md requires proper shared-verifier/evidence-format review with escape/digest canaries, preserving frozen bytes. No local trust exception, signed verdict or verification waiver is introduced. #raw("[pcc:unverified]")

Auto-merge remains paused. New commits are local/unpushed pending remaining native outcomes, resolution review and exact-head required CI/reviews/thread resolution. Pending/absent checks are not passes. The initial 95349-byte policy error log stays outside Git, NOT_UPLOADED, with final digest/path/bounded excerpt in policy-captures/receipt.json; all historical native failures/caps remain unchanged.

== Evidence, primary sources and scope
Validation directory: continuation-20261030/validation/main-d30b9e0db-20261105/. It holds resolution-review, formal-receipt, formal-claim-bindings, formal-pcc-checks, supplemental dependency identities, 26-example and 5-test component receipts, plus policy captures. Primary implementation sources: Rust File::sync_all, OpenOptions::create_new and hard_link docs; Linux fsync(2), including directory synchronization; prior G2 accounting audit; PKM2016 Algorithm1/§3.2 for separate query/relation/LA owners. No new identity or ECDLP mechanism is claimed.

Original PKM_GOALS.md acceptance remains unchanged: G1–G3/G6 partial, G4/G5 unrun, zero scientific panels; all cost/GAE/scaling metrics null and comparison admission false. No scientific launch, funding, lease, IAM, quota, pool or gate bypass. Canonical IC/curve/performance views stay unchanged: this PR adds software prerequisites and statement bindings, not a frozen scientific value, phase cost, reference or curve. Existing reports/PDFs are preserved. The diagram shows record ownership rather than a performance comparison.
