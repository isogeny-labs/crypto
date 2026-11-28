# Authentic original-cell constructor and ignored native test child

Patch-only proposal based on untouched candidate 3517ae554787690472e3ed9801623de27730e20d. No compilation, test execution or generation has occurred. The prior observer packet remains immutable.

## Real implementation

`owned_authentic_historical_child.rs` is a new cfg(test) module under solvers with two ignored tests. Its constructor creates a new root with create_dir (no reuse), freezes an n23/m2/factor-size2 binary/F4/checkpoint plan before originals, writes plan/config/attempt/run/protocol through production storage, installs the actual measurement policy, derives the actual AttemptBinding, and installs the production capture limits. The plan has twists=true because DatasetPlan validation requires paired twist coverage; this bounded software control selects cell 0 only, and makes no claim of executing its paired twist.

Preparation reads the actual clean Git HEAD and hashes the current executable. These are actual identities, not synthetic repeated hex strings. They do NOT independently prove that executable was built from that HEAD: a separately verified exact-source build receipt is a prerequisite before execution. The module refuses dirty/untracked repository state. Git checks use fixed /usr/bin/git with a cleared environment and disabled system/global config, so inherited Git directory overrides or PATH cannot redirect them. The fixture assumes a controlled local filesystem without concurrent symlink/directory replacement; static control-path aliases are resolved before separation checks. Frozen_at records actual Unix seconds before generation. The protocol is explicitly a software-control protocol with a fixed boundary; it uses the repository's actual canonical hash/freeze validator but is not claimed to satisfy a broader comparative experiment protocol.

`prepare_authentic_historical_publication_child` performs setup and execution::create, then exits. The actual native child must start immediately afterward within this already-created original 60-second launch deadline. It reloads and checks the plan/config/protocol boundary, requires the exact same executable digest, and admits that existing launch. It does not create a replacement clock. The real production calls are telemetry::measured(curves::generate), verify_and_store_worker_originals, actual CaptureEmitter construction, and owned_search_worker::run. That worker performs authentic OriginalOwner admission, map construction/publication, descent/normalization and native checkpoint search. No fake tokens, supplied Original structs, artificial basis rows, normalization cursor or custody stubs are present.

The earlier cfg(test) observer patch is included. Before owner execution the child installs the observer on the same producer thread. It emits at most one bounded acknowledgement notification through an exclusively created control file outside the canonical cell root. Unmatched boundaries allocate no notification record. At the frozen boundary it parks, checking the original deadline; if not terminated by the supervisor it panics at that deadline, never resumes producer work or renews limits. If native execution returns before the desired boundary, it fails without creating a success report. Cell files are never removed automatically. A create_new consumed marker is written and synced, including its directory, before original admission/generation; failures retain it and a second native invocation cannot regenerate original files. Loading also enforces every fixed plan dimension/seed/policy and original run limit, software native_cells=1/retries=0, exact binary/protocol/run joins and same clean source HEAD; an arbitrary self-consistent caller-selected plan cannot widen this harness.

## Exact test entry points and environment contract

Both ignored tests are under `ml_dataset::solvers::authentic_historical_child`:

- `prepare_authentic_historical_publication_child`
- `authentic_historical_publication_child`

A later authorized supervisor invokes one exact test at a time (`--ignored --exact --test-threads=1`), using the identical built libtest executable. Inputs are absolute OWNED_HISTORICAL_CHILD_ROOT (must not exist at preparation), absolute OWNED_HISTORICAL_CONTROL_PATH outside cell artifacts, OWNED_HISTORICAL_SNAPSHOT in 1..=16, OWNED_HISTORICAL_BOUNDARY complete or chunk, and OWNED_HISTORICAL_CHUNK in 0..65536 for chunk mode. Freeze these before setup; the child checks the boundary against the saved protocol. No arbitrary retry/seeding or fallback is supplied.

Index >0 excludes the guaranteed pre-Matrix initial snapshot but does not certify nonempty valid rows. A complete snapshot and tail identity are still qualified by central replay. Fixed partial chunk selection may not contain the full genuine original identity or may finish the snapshot: retain such an inconclusive run rather than claim a positive strict-tail fixture.

## Remaining supervisor and verification work

This patch implements the real constructor and real native child, not the whole supervisor. Required external wiring remains: process-group ownership and OS memory limits; bounded watchdog using execution-context.json's original monotonic deadline; readback of the actual acknowledged head; actual child termination; production terminal_outcome/requested_channel_disposition/supervisor metadata; collect_cell_artifacts and CellManifest sealing; and explicit finite historical replay with same-owner revalidation/projection. An early intentional kill should be represented as worker_failed with the actual non-success exit, not a fabricated timeout. Only actual original-deadline expiry authorizes timeout labeling. Preserve all interrupted and failed files.

The proposed native limits remain 60 seconds, work_limit16384, memory256MiB, trace128MiB, one cell and zero retries. This packet creates no execution authorization; scientific budget/deadline remain expired. Observer notification overhead is test instrumentation, not a scientific timing measurement. The intended narrow fixture checks original-source custody and mathematical replay, not corpus-scale generation or all-twist coverage.

## Validation scope

Read-only `git apply --check` against frozen candidate passed. Source implementation was inspected, including correction of initial twists=false to required twists=true before freeze. No Rust compilation, syntax-tool validation or test run was performed; type/visibility and runtime behavior remain unverified. Existing APIs and serialization tags were read directly. Actual source identifiers, constructor registration, observer hooks and full patch are retained with hashes. Exact-source KB fallback was used because the parent reports the KB tool/helper unavailable.
