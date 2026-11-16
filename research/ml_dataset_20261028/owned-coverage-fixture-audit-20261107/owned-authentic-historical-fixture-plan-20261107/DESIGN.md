# Authentic original-cell historical replay validation

## Current state and exact missing evidence

Inspected candidate 3517ae554787690472e3ed9801623de27730e20d without editing it. The historical source route is present, but there is no completed native original-cell historical replay fixture. The current genuine-durable-v2 fixture starts from a genuine supplied Original at n=2,m=2, checks map/descent and 12 rows, but supplies synthetic identity/binding, a software-only map role, a manually normalized cursor and batch_rows=128. Production profile rejects n=2; production uses batch_rows=1. It cannot be relabeled an authentic original-cell fixture or used to establish the new historical route's end-to-end admission.

Existing in-crate verified_worker_original_tests::Fixture and sat_original_owner::tests::real_factory_accepts_authentic_owner_before_failure_controls do establish a concrete construction path: frozen plan/protocol/config -> execution::create -> measured curves::generate -> verify_and_store_worker_originals -> OriginalOwner::admit using the authentic token, context, sink and capture limits. Those tests use synthetic source_commit/binary_sha256 strings and m=3 fixed in the fixture builder. A new authentic validation should retain honest exact candidate/binary identities and must not alter a plan/config after capture binding has been derived.

Central admit_original_inputs independently checks manifest descriptors, execution/capture context, measurement policy, exact config/public/factor bytes, bounded public/factor-base mathematics, original token/map identity, then descent and selected committed basis proofs. Its verification does not trust a software fixture's declared binding. All of those gates must execute on the positive fixture.

## Minimal concrete fixture

Use one frozen software-control cell, ordinary binary family n=23,m=2, factor-base size 2, seed 42, solver f4, degree_bound=3, Legacy factor base, pattern=0, channels/v5 matrix_continuation with conservative_affine_envelope_checkpoint_v1. n=23 is the smallest production-supported field. m=2 removes intermediate-pattern complexity while retaining production original mathematics, signed input, map construction, descent, native search and publication. This is a proposed minimal input, not a known passing sample; retain a failure if it produces no checked rows.

Create the plan/protocol with arities=[2] and explicit channels before computing any binding. The existing Fixture's helper hardcodes arities=[3], so add a narrowly parameterized in-crate fixture constructor in a later test source candidate rather than mutating an admitted fixture. Reuse production storage and protocol validation. Run execution::create before the measured curves::generate and verify_and_store_worker_originals calls. Use actual source revision and actual built binary hash, measured launch context, installed measurement policy, exact AttemptBinding::from_cell_files, production capture_limits, and authentic OriginalOwner::admit. Call the production worker with checkpoint policy and actual configured batch_rows=1. Do not build a second fake Original/token, assign basis/lineage rows, manually set normalization cursor or replenish native work after stop.

The initial pre-Matrix snapshot is complete but may contain no native rows. The fixture must wait for a genuinely committed native-batch snapshot with nonempty committed basis, not assume snapshot 0 or the first acknowledgement is sufficient. Validate selected mathematical rows through the central route; never choose a favorable earlier checkpoint after a later complete snapshot fails.

## Deterministic interruption is the remaining fixture mechanism

There is no existing test seam in CaptureEmitter/Publication allowing a supervisor to stop exactly after a named durable publication acknowledgement. Polling the filesystem and racing kill is not a deterministic positive fixture: it can overshoot to a later complete snapshot or fail before identity. The current publication Bridge records acknowledgement only after the actual payload/head commit; this is the correct observation boundary.

Add a cfg(test)-only, non-authorizing observer/barrier in a later source candidate after successful chunk acknowledgement, and a complete-snapshot observer after snapshot.complete is set. It may signal an external test supervisor and park the child; it must not alter bytes, reserve budgets, replace the sink/owner or invent successful state. The supervisor terminates the parked child and uses production terminal_outcome/collect_cell_artifacts/CellManifest sealing, including requested_channel_disposition. Do not renew the scientific deadline or treat the observer as a production feature. A test-only child driver must invoke the authentic producer path; its binary identity and feature/cfg choices belong in the receipt.

Arm A stops after a complete native checkpoint with rows. Arm B stops on a later snapshot only after enough acknowledged chunks contain the full immutable identity prefix and before the encoder completion sentinel. The exact cut index/byte threshold must be measured from the source snapshot schema without assuming the old 789-byte synthetic identity boundary. The genuine n=23 map can make identity larger than one chunk. Preserve zero acknowledged tail as 'none_observed'; it is not a strict tail. If no such partial boundary appears under finite limits, retain the inconclusive fixture and do not manufacture a positive transport result.

A derived, reauthenticated transport-control fixture can separately replay a genuine captured byte sequence through a real CaptureEmitter, using identical map/ack and original context, to construct an exact truncated prefix. This is useful for deterministic parser/custody tests but must be labeled a derived transport control, not an actual interrupted worker. Rebuilding its event chain and manifest is essential: unrefreshed hashes only test superficial custody rejection. Do not substitute this derived control for Arm B.

## Replay and expected controls

For each immutable fixture, construct one replay policy and absolute deadline, call verify_owned_search_historical_cell once, retain its custody, call revalidate using that owner, then project_identity. Assert HistoricalCheckedScope, never CheckedScope. Assert original_descent_verified, nonempty checked rows, selected and terminal receipts, exact raw-manifest/outcome binding, and null degree labels. Producer_attempt_complete and report_capture_join_verified must remain false; terminal success/scalar recovery must remain false. Current public historical caller must be invoked under original_owned_search_historical_rows_v1, not generation authorization.

Freeze separate adversarial clones before replay, with finite per-control policy. Each control gets one owner only; controls are distinct inputs, not retries of a refused owner:

1. Complete checkpoint with no observed tail: positive historical row scope if original/descent/lineage/basis gates pass.
2. Same source with one observed strict identity-bound tail: select immediate previous complete, retain all tail descriptors and raw bytes.
3. Later complete snapshot with invalid committed basis proof or descent while an earlier snapshot is valid: rebuild valid transport hashes/manifest, then require no positive historical scope and no backwards fallback. This is a mutation control, never a native success dataset.
4. Wrong original public/factor/map identity, omitted tail descriptor, malformed tail, incomplete identity, gaps or multiple partial tails: no positive scope.
5. Contradictory normal report or requested-channel disposition; invalid supervisor envelope: no positive scope.
6. Exhaust the existing replay owner's work/bytes/deadline during tail verification, revalidation or projection: refuse, retain charges and failed owner, never mint a fresh policy.
7. After positive admission mutate selected payload, tail payload, terminal head, original source or manifest: same-owner revalidation/projection must revoke positive status.
8. Legacy policy against historical verdict and historical policy against legacy result: no positive cross-mode admission. Historical generation must be refused.

Integrity-only malformed controls are cheap, but proof-failure control must get through authenticated capture before failing mathematics. Keep diagnostic/phase evidence showing this distinction. To observe same-owner continuation of accounting, record charges before/after every call and pointer/custody identity within the test; pointer identity alone is not proof that budgets were retained.

## Proposed finite runtime, not new authorization

No new build or execution is authorized by this document. Existing scientific deadline and total campaign budget remain expired; no generator/cloud job was run. The pending exact-candidate build approval must be resolved separately. New test source and a cfg(test) interruption seam would produce a new candidate and require an appropriately scoped build allowance; an approval for the old candidate cannot be silently reused.

For a later explicitly authorized software validation, propose concurrency 1, two native cells maximum (Arms A/B), zero retries, 60 seconds per producer including original math, original work_limit=16384, 256 MiB producer memory, 128 MiB trace, and one cumulative 60-second replay deadline per control with finite 200,000,000 checking work / 512 MiB replay owner cap. Predeclare the full control count and bound total test elapsed time (15 minutes ceiling proposed), retained file bytes and aggregate replay allowances. These are proposed safety caps, not runtime predictions or demonstrated sufficient resources. No speed/timing claim is intended. Stop and preserve failure at a limit; do not increase n, factor base, memory or work implicitly.

## Implementation decision

No executable test source is supplied yet: the current available standalone harnesses cannot exercise true original-cell admission without extensive fake parent state, and the deterministic interruption seam does not exist. An executable-looking scaffold with stubs would misstate the missing work. The next concrete implementation is an in-crate test constructor with authentic production dependencies plus the non-authorizing test-only publication barrier, in an isolated new candidate. After its build is authorized and passes, run the bounded software validation and record complete artifacts/hashes. This document and source bindings are ready to guide that implementation without touching candidate 3517ae554787.
