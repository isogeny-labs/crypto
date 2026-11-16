# Independent exporter successor source review

Status: blocking findings; no runtime executed. Assigned candidate changed during review. The review below is pinned to observed-after-drift manifest b63943f0d46ebb5a5bcd806bd97028308240137e8b5900bc6dff3cd1d702ddec.

## R1 — Retry resource snapshots cannot resolve their registered plan

retained_attempt_join_v2.rs:1024-1031

validate_resources searches cohort.plans by attempt.session_id, but that inventory has primary session IDs only. checked_attempts already resolves retry_of to a primary slot and checks retry custody plan digest, then passes the retry session to validate_resources. A distinct-session retry with resources fails session snapshot lacks prepared plan.

Required correction: Pass the already validated Slot or plan digest into resource validation; compare snapshot session to retry and snapshot plan to parent slot. Add a distinct-session retry control reaching resource validation.

## R2 — Evidence fields are displayed but not enforced by comparison eligibility

retained_attempt_join_v2.rs:1247-1260,1359-1363,1511-1515

custody_closed and diagnostics_ok are stored but omitted from evidence_complete, reasons, candidate_reference_eligible and eligible_for_measured_pair. Missing diagnostics attachment returns false without reason. A checked priced record/resource/relation can remain pair-eligible with absent diagnostic attachment or structurally unclosed ownership.

Required correction: Keep derived finite arithmetic costs if useful, but separate raw priced data from comparison eligibility; derive required evidence blockers under the frozen contract and require those blockers absent for eligible fields. Do not redefine that policy during review.

## R3 — Incomplete sessions and partial retries abort export instead of being retained

retained_attempt_join_v2.rs:550-557,1100-1133

session_custody requires complete status,3240 records,last_seq3239 for every session. validate_session_attempt_coverage requires all3240 sequences for every retry session. A killed session or partial recovery cannot export its retained evidence even as ineligible rows, despite the advertised retained-failure boundary.

Required correction: Separate expected primary enrollment from observed attempts and session closure. Preserve partial sessions with explicit missing slots/custody status; retries must not replace primaries. If complete-session-only scope is intended, amend and independently review that narrower contract prospectively.

## R4 — Candidate changed during independent review without preserved original snapshot

manifest.json / review handoff

Assigned manifest f57de9b056e286d43028b17f4387f8496a7e12edb3ddf37d776b01898656c0d9 and patch c99b64d73c7fbf338033f95f942623792ed2e6768dbd30fe41fe15b1e00a58f9 changed to b63943f0d46ebb5a5bcd806bd97028308240137e8b5900bc6dff3cd1d702ddec / c851c6a35ce2667f046f6b778268b42878806e07ef1e80841145f242308dac42. Producer confirmed no immutable original copy. Current bytes frozen independently in observed-after-drift; original review messages remain historical.

Required correction: Preserve current observed snapshot and explicitly label unrecovered originals; reconstruct originals only if every original digest matches. Make any further changes a separately sealed successor.

## Source checks

- All seven actual hydrated prepared-plan files and validation receipt match registered digests; each plan has180workloads/3240executions, total1260/22680 with18900measured and3780warmup.
- Slot building derives arm/role/method/seed/round/warmup/workload from checked plans; primary bijection and explicit retry parent checks replace caller aggregate counts.
- FilePins open actual files, verify bytes and SHA256, reject duplicate JSON keys and deserialize native types.
- DiagnosticChild and diagnostic_attachment/v2 use native serializers/bindings; resource PublicationTail::observed revalidates prefix semantics; ordinary records require native line seals.
- Nine-category analyzer is called directly; null/partial price cells or mapper blockers keep finite cost null.
- Cargo direction root->native_price->paired_export has no reverse root dependency in pinned manifests.
- Six source-written controls cover smaller enrollment, retry parent, triplet identity, injected fields, null price cells and changed file bytes; they were not executed and do not cover R1/R2/R3.

No compiler, tests, exporter or payload was run. Nine schema/mapper pins matched actual donor/overlay bytes; donor runner.rs differs from the claimed Git-head bytes, so integration compatibility is still unverified.
