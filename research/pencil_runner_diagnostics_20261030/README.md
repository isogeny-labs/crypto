# Production diagnostic custody slice — disabled producer

T-933 adds a bounded typed sidecar reader and explicit child context to ecbench. This is software work, not a scientific run or production-complete instrument. All arms still construct ChildInput with `diagnostics: None`. `child_main` rejects any supplied context before workload construction. The mathematical producer and frozen pencil implementation are unchanged.

## Implemented surfaces

- `src/cryptanalysis/ecbench/diagnostics.rs`: explicit context/identity, bounded structural sidecar ingestion, exact-byte hashes, record association, separate diagnostic manifest, and shared pure timeout/interruption acceptance helper.
- `record.rs`: optional context defaults absent and is omitted when None; any present context is rejected before Workload::build or method resolution. An invalid synthetic curve fixture proves that early rejection without generating a target or running a solver.
- `runner.rs`: accepted report selection uses the shared stop helper. The existing timeout/interruption semantics are retained. A separate optional custody branch, inactive while contexts are None, follows record sealing and synchronization; it writes a hash-bound diagnostic attachment under exec without populating accepted math result or cost.
- `audit.rs`, root CLI and child-overhead example: explicit None additions maintain Rust struct literal compatibility. Historical serialized child inputs remain unchanged.

## Reader boundaries

Identity includes session, execution, source, binary, method/config, workload, protocol, seed and attempt. Context validates a single safe leaf name under a runner-trusted output parent. Reader checks exact identity against snapshots; it also checks session/sequence/seed/binary/workload/method against exact sealed record bytes. Workload and method hashes use serde_json serialization of the parsed record Value; this convention must be used by a future context creator and frozen before activation. Source/protocol/attempt are supplied context identities, not independently attested by this disabled slice.

At most256 committed files plus one temporary; at most65,536bytes each, and bounded read of cap+1 to detect overflow. Names must be numeric six-digit slots with .json or .tmp; symlinks/nonregular entries are rejected, Linux uses O_NOFOLLOW. Parent is assumed trusted and no live child may mutate the leaf during ingestion. This is not a hostile filesystem sandbox.

Typed Snapshot, PhaseTotals, Allocation and IO totals reject unknown outer fields. Counter key inventories match field/native/symbolic source types; boundary transition rules enforce the observed producer schedule, walk IDs and fixed milestones; temporary sequences belong to the last/current write. Counter maps, phase elapsed/counter/allocation fields and prior IO totals cannot regress, and prior IO bytes reconcile with exact previous snapshot sizes. Field/native numeric ranges match u64; symbolic counters allow u128. Recognized phase maps are typed; this is structural validation, not proof of arithmetic correctness, complete phase partition or true execution. `counter_semantics_verified=false` remains explicit. Sequential snapshots are cumulative prefixes and must never be summed. Temporary bytes are separately classified and hashed, never accepted as complete JSON. Durability confirmation remains unknown even when a JSON file is visible. No diagnostic entry becomes an accepted scalar or final ratio.

## Custody and cost limits

The attachment binds exact sealed record bytes and manifest bytes by SHA256; successful record publication precedes sidecar publication. If attachment publication fails, runner errors rather than claiming complete custody. Ingestion failure is retained as a diagnostic error attachment with null manifest. No failed scientific outcome is rewritten.

`ingestion_wall_ns` measures the reader call only, including its validation/hashing; it excludes setup, solver, record publication, manifest serialization and publication. Existing process/method clocks retain their original boundaries. A complete outer resource envelope and publication tail have NOT been implemented in this slice. No complete end-to-end cost or equal observer overhead is claimed. Before activation, every arm needs the same common context/sink profile and observed arm-specific overhead must be charged; candidate stage-dependent IO cannot be assumed equal to rho.

## Named software tests and initial blocker

Initial command `cargo test --offline --release --lib diagnostic_ingestion_tests -- --test-threads=1` failed before compilation because sparse path dependency `crates/ecdl-tensor-ic` was absent. Native stderr is retained under evidence/root-test.stderr. This is not a passing root build. After both sparse dependencies were materialized, the real root release build succeeded and all14 named software fixtures passed (11 ingestion,1 context,2 acceptance). The exact tested hashes and raw logs are in evidence/release-tests-receipt.json. Two subsequent equivalent lint corrections are recorded separately; final merged-tree release revalidation remains required.

The ingestion/context tests use literal records, fake counters and temporary files. Root explicitly authorized a tiny known-answer plan/Workload fixture for the actual assemble regression test only; it injects a complete synthetic late SolveReport and does not call any solver. No tests call solve_one, rho, existing BSGS watchdog fixtures, benchmarks or cohort generation. The explicit-context fixture invokes child_main only along its new early rejection path, before Workload::build. Existing unrelated scientific fixtures must remain filtered out.

Required named suites: diagnostic_ingestion_tests, diagnostic_context_tests, diagnostic_acceptance_tests. The source-bound independent review closure is retained in evidence/t933-ingestion-followup-closure.md. That review and the fourteen passing software fixtures apply to their exact recorded source hashes; they are not scientific validation or final merged-tree checks. The actual assemble fixture covers output absence/presence crossed with timeout/interruption, retaining null accepted scalar/cost/verification and empty phases/counters despite a complete late successful report. No cloud jobs or scientific source-data executions were authorized by this change.

## Remaining work

Full source-bound independent review; counter numeric-range/phase partition strengthening where appropriate; exact binary/profile/source identity creation; checkpoint inclusion verification; all-arm sink lifecycle and bounded native callbacks; complete outer cost/resource receipts; prospective amendment and independent admission. Full 1,260-target/replay scope and all prior failures remain unchanged. This slice cannot admit a retry or full cohort.

A second root attempt after the first materialization was also blocked before compilation by missing sparse `crates/hashed-ecdl-tensor`; evidence/root-test-materialized.stderr preserves that error.

Initial Clippy failed on int_plus_one and cmp_owned. The correction replaces >=MAX_FILES+1 with >MAX_FILES and preserves decimal-string seed association using an explicit expected string and as_str(); it does not accept a numeric JSON seed. Lint rerun and final post-merge fixtures are separately recorded rather than rewriting the earlier pass/failure logs.
