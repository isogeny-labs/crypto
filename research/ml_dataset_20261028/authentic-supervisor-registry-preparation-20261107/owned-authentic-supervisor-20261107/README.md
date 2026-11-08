# Authentic bounded supervisor and real historical replay acceptance proposal

Patch-only extension of the authentic child/observer proposal. Original candidate 3517ae554787690472e3ed9801623de27730e20d and prior frozen packets are unchanged. `supervisor.patch` includes the complete observer, constructor, child and supervisor changes and applies directly to that candidate. The shared production original-artifact checkpoint patch is a separate REQUIRED dependency; do not execute without both.

## Implemented real path

The Linux-only ignored `ml_dataset::solvers::authentic_historical_child::authentic_historical_supervised_acceptance` test runs current_exe twice with exact ignored libtest names: first preparation, then the native child. It reuses actual dataset::SupervisedChild for process-group kill/wait, poll descendant cleanup and Drop cleanup on failures. Both subprocesses receive production-equivalent setpgid, PDEATHSIG plus parent identity check and RLIMIT_AS256MiB. Environment is cleared and only fixed required test inputs/trusted PATH are passed.

Preparation has a separate30-second bound and creates the actual original60-second launch before native spawn. The supervisor consumes that existing monotonic deadline; it never renews it. Native total output is watched at128MiB, external child logs at1MiB. A1MiB total external directory allowance bounds each serialized control receipt before write, including final replay output. Child log limits are polling thresholds: an overshoot is retained and can leave no room for a receipt; it is not represented as hard byte enforcement.

The observer notification must match the frozen boundary and freshly read real committed head. The supervisor then actually kills/waits the worker. An early intentional kill has no invented stop reason and production terminal_outcome records worker_failed with actual exit. Genuine deadline crossing, including crossing during readback/kill, records timeout. Output overflow records trace_budget_exhausted. Production requested_channel_disposition, the exact11-key supervisor envelope, collect_cell_artifacts and CellManifest storage seal the actual artifacts. Notification alone is never acceptance.

After sealing, source calls actual verify_owned_search_historical_cell with one finite replay policy (200M checking work,512MiB owned bytes,128MiB stream/payload,60-second absolute replay deadline). The same custody owner is revalidated and projected. Charges and failures are retained in the receipt; there is no replacement policy on failure. The acceptance additionally requires nonempty genuinely verified rows and exact requested boundary semantics. Complete mode must select the requested snapshot with no observed tail and matching receipt. Chunk mode must yield a bound_truncated_prefix at the requested index, exact final chunk/receipt, and selected immediate predecessor. A chunk that happened to contain complete wire does not qualify the partial-tail test.

This tests the real central API, not public historical caller/CLI/export integration. No generic verify_cell error is ignored or converted into success; that public route has its separate retained-caller policy requirement and is not claimed tested here.

## Required real registry dependency

Inspection found that verify_and_store_worker_originals returns authentic original descriptors only in memory, while ordinary finalization writes worker-artifacts.json after the solver. Killing before finalization makes production collect_cell_artifacts classify originals as partial files, but central original admission requires public-instance/factor-base roles. Writing worker-artifacts.json early in only the fixture would mask this production gap and conflict with immutable normal finalization.

Therefore this packet writes NO fixture-only early registry. It requires the separate shared production fix: immutable original-artifact checkpoint written from actual original receipts before native dispatch, and collector fallback using that real checkpoint. Parent is preparing/reviewing this dependency. No descriptors are invented after killing a child, no fake token is created, and capture hashes are not repaired to force acceptance.

## Frozen scope and invocation prerequisites

The inherited constructor freezes n23,m2,binary/F4, factor size2, seed42, degree3, work_limit16384, explicit v5 checkpoint policy and paired-twist metadata; selects only cell0 with one native child, zero retries. It records actual clean Git source identity/current executable hash, rejects scope-broadened loaded plans and original limits, and uses a durable create_new producer-consumption marker. Control output is outside canonical cell artifacts. All failed files remain.

Inputs: absolute OWNED_HISTORICAL_CHILD_ROOT (must not exist), absolute OWNED_HISTORICAL_CONTROL_PATH outside the cell; OWNED_HISTORICAL_SNAPSHOT1..16; OWNED_HISTORICAL_BOUNDARY complete or chunk; OWNED_HISTORICAL_CHUNK0..65535 for chunk mode. Add absolute OWNED_HISTORICAL_BUILD_RECEIPT and expected OWNED_HISTORICAL_BUILD_RECEIPT_SHA256. Receipt reference is read nofollow/nonblock as a regular file, capped at1MiB and hash checked. The authorized parent must independently verify its exact-source build provenance beforehand; hash/reference preservation does not establish binary-to-source linkage by itself.

The supervisor has a180-second cooperative overall deadline after invocation setup, bounding preparation/native/replay scheduling. Synchronous filesystem/source operations and sealing are not interruptible by that loop: an external process timeout remains required before claiming a hard total wall bound. Original native clock checks and process cleanup still apply. Controlled local filesystem/no concurrent directory replacement is assumed. Notification and fixture bookkeeping are software instrumentation, not scientific instruction/timing measurements.

This source creates no execution authorization. Fullbuild and native validation remain pending; scientific campaign deadline/budget remain expired. No preparation, worker, replay or cloud process was run.

## Verification performed and still missing

Read-only inspection and git apply --check only; no compile or runtime tests. Exact primary sources checked include production supervisor mod.rs1266/1880-1990, execution::terminal_outcome, artifact collection, central historical projection field shapes and original admission. Independent source review identified and fixed requested-boundary qualification, post-kill deadline labeling, bounded receipt writes, regular-file build-reference reads and non-Unix test cfg guarding. Type/visibility checking, real runtime qualification and the shared registry dependency remain outstanding.

The full solvers.rs snapshot is S3 evidence only if Git duplication limits apply. Git can preserve supervisor.patch, the new test module, small observer/publication sources, README and hashes. Base source is retrievable at candidate3517ae554787690472e3ed9801623de27730e20d; apply this combined patch once, plus the separately reviewed registry dependency. KB helper was unavailable; exact local-source inspection is the recorded fallback.
