# Historical owned checkpoint admission audit

Source inspected: crypto-ml-checkpoint-integration-20261107 at 51546503a01be0fac926a4d4b1df047c059ff8ba. Read-only inspection; no compilation, execution, source edit or cloud action.

## Existing admission and safe interfaces

All source links below are relative to /home/ec2-user/crypto-ml-checkpoint-integration-20261107/src/ml_dataset.

- owned_search_central.rs:49–64: supported request, original-input authentication, raw manifest/config/outcome equality, capture replay, every snapshot decode/binding, report join and revalidation precede CheckedScope. Generic interrupted outcomes currently return Incomplete. They are not export authority.
- owned_search_central.rs:134–169: record already enforces original map first, canonical snapshot/chunk filenames, consecutive snapshot/chunk indices, adjacent event receipts and matching pattern. Preserve these checks before any selector invocation.
- owned_search_central.rs:186–216: wire errors stop immediately; every complete snapshot identity is authenticated; only latest snapshot receives semantic verification. Refactor selection deliberately: never classify a wire resource refusal as a truncation and never fall back after latest complete mathematics fails.
- owned_search_central.rs:289–301: check_snapshot_binding can safely factor to a root-value plus expected-index helper. Apply exactly the same checks to a strict-prefix validator's completed schema/index/identity fields. The prefix module must validate schema and the entire observed suffix, not only expose an identity object. A filename or candidate classification is insufficient authority.
- owned_search_central.rs:227–259: revalidation retains and rechecks all chunks, including a potential partial tail. Preserve entire manifest/event stream/head checks and raw outcome bytes. Do not revalidate only selected chunks.

## Original descent is a prerequisite, currently absent

owned_search_central.rs:209 calls semantic::verify, which establishes lineage/basis identities relative to captured normalized equations. Its projection at :279 explicitly says original_descent_verified=false. The authenticated map alone does not prove that captured generators descend from it.

Existing frozen component: /home/ec2-user/ml-dataset-resume-20261029/owned-search-original-descent-20261107/owned_search_original_descent.rs:34.

Exact API: verify(map: CheckedMap, root: &wire::Value, limits: Limits, deadline: Instant, owner: &mut impl wire::Admission) -> Audit. Limits contains expansion limits, work and bytes. The returned Audit retains the consumed CheckedMap and all local expansion prefixes; Audit::source() exposes that map. Audit::original_descent_verified() requires complete production expansion, exact raw lexicographic generators and exact normalized degree/lex generators, with original generator order and zero generators preserved.

The frozen owned-search-descent-integration-20261107/owned_search_central.rs proposal already retains descent: Option<Audit>, takes frame.map only after every snapshot binding, uses authenticated_map() thereafter, and recomputes semantic remaining work/bytes after descent. This is the correct ownership interface. Avoid cloning the map or granting a separate budget. Apply selected-snapshot index only once the selector is authenticated; then original descent and semantic verification must use that same root. Unknown/Cap/Owner/Deadline/Missing/Incomplete remain inconclusive; malformed/mismatched equations reject. All failures retain audits. Projection and final positive verdict both require descent success.

## Narrow failure-envelope admission

execution.rs:149–162 constructs authoritative supervisor outer forms: timeout or trace_budget_exhausted with solver_completed=false; failed exit emits worker_failed, exit_code, solver_completed=false. mod.rs:1960–1986 then adds requested_channels and supervisor diagnostics before sealing the manifest. Worker report publication can be unavailable (mod.rs:1454), so absence of a normal report is expected.

Start a new opt-in historical mode with those THREE statuses only, not generic error or budget_exhausted accepted by the existing Incomplete shortcut. Validate the manifest-bound outer object, exact false solver completion, absent-or-false scalar/pipeline/completion claims, null-or-absent degree labels, expected requested-channel disposition, and coherent supervisor fields. Preserve reported stop/exit information as observed metadata, not independently proven cause. The no-report gate should reject ANY reserved adapter_id/strategy/schema report marker and snapshot/report claim fields; otherwise a contradictory report can be laundered through a failure status. If a normal report is present, preserve validate_report unchanged rather than stripping its fields.

The current supervisor replaces a report on timeout, so an interrupted envelope normally has no report marker. Missing scalar/pipeline fields are normal and cannot be required as explicit false unless the producer envelope changes compatibly. Boolean/malformed nonfalse values must not be treated as absent.

capture.rs:211–219 defines EventLimit, StreamByteLimit, PayloadByteLimit, PayloadCountLimit, LocalByteLimit, ChunkLimit, StorageFailure. An authenticated cap stop may be permitted by the historical policy; StorageFailure should remain inadmissible initially. No capture integrity error is an allowed interruption. Outer status alone never authenticates the event chain.

## Public route and export constraints

owned_replay_caller.rs:112–180 accepts only original_owned_search_rows_v1. Add a distinct explicit historical mode rather than silently broadening it. Existing slot owner at :248–319 is sticky on any failure, bounded to eight visits, shares deadline, and never retries failed/unknown/incomplete custody. Preserve these invariants. Add a distinct positive historical verdict through solvers.rs:7613–7635; caller revalidate/project must recognize only that mode's own verdict.

owned_search_central.rs:261–281 currently authorizes only CheckedScope and projects the last snapshot with report_capture_join_verified=true. New historical projection needs selected checkpoint and terminal capture receipts separately, tail disposition and all tail descriptors, producer_attempt_complete=false, report_capture_join_verified=false in no-report route, exact outer outcome binding, descent/lineage/basis scope, all degree labels null and public features null. Projection failure must invalidate scope.

export.rs:1170–1193 currently places owned rows in restricted records with generic owned-row identifiers; :1646 emits aggregate original_descent_verified=false. New historical schema/status and record domain separation must survive export without implying current terminal success. Keep failed-original and partial-tail provenance. Aggregate claims must be derived from admitted records, not unconditional replacement.

## Recommended next substantive step

Factor the common identity checker and test exact full-root versus validated-prefix binding parity (map bytes, four original hashes, pattern, snapshot index and both map receipt coordinates/hashes), with mutation and unavailable-identity refusals. This is a real prerequisite and cannot itself activate export. Next integrate the already frozen original-descent owner into central with a same-budget complete-snapshot fixture. Then add explicit historical mode, narrow envelope parser, authenticated selector and distinct projection together; do not expose an intermediate positive path missing original descent or outcome binding.
