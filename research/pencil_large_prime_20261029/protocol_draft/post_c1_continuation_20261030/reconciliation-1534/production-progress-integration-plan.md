# Prospective production progress integration — patch plan only

2026-10-30; read-only delegated follow-on to T-927. No repository edits, builds, tests, trials, solver/rho execution or cloud actions in this planning task. The seven-hour planning envelope and independent review requirement remain unchanged. Neither the isolated module nor this plan is production complete or execution admitted.

## Reconciled source

Current diagnostic module SHA256 5415467accaf0e8b6426a8834838ac6fc7e7a463520b02dfcbc8eb7d1338cd89 includes explicit Unix 0700/0600 modes and separate published/durability-confirmed totals. Its saved receipt establishes nine release diagnostic fixtures only, not production integration.

Current production SHA256s:

- src/cryptanalysis/ecbench/record.rs: 5ebb2ecd907d4e388004e5b3db05614bbdd6c0c0e080bba9ccc7d302e621f407
- src/cryptanalysis/ecbench/runner.rs: 838ca5bc8b12243b4c72adf2a09a122b39ee1a752cd4c762c800a34b7d92698d
- src/cryptanalysis/ecbench/pencil.rs: 48424cb11a2b308f385d4628dc92024ea7c09589da340247f85a5c2ac5f31b8e
- src/cryptanalysis/ecbench/methods.rs: 681d85a05dee7a5cce54c1d250375c97d338bf0b18d8203609447f003918d3a0

Relevant primary IO references and source review remain the T-927 design/README references; this plan proposes no new math mechanism. Existing runner sources, not assumptions about an external checkpoint wrapper, establish the integration surfaces below.

## Minimal patch boundaries

1. **record.rs109 ChildInput**: add optional `diagnostics: Option<DiagnosticContext>` with serde default and skip-if-none. Context carries schema version, runner-created absolute leaf location/parent contract, exact execution identity, caps and diagnostic profile hash. Hash source/config/workload/binary/protocol externally before execution, not by each inner callback. Context is explicit stdin data; no ambient environment activation. Add `None` at existing Rust struct literals throughout root code/tests; retain historical serialized bytes with absent field. Manifest binds the instrumentation choice independently of math-method parameter identity.

2. **runner.rs739 input construction**: allocate a unique diagnostics parent per execution under the same session output tree, including attempt identity. Runner owns trusted parent; child creates the private leaf. Do not silently share an existing directory on retries. Any resumed checkpoint must preserve the same process/leaf or explicitly identify a new attempt; do not infer restore count from snapshot sequence. Reject unsafe path components or symlink ancestors within the controlled output tree. Bind parent ingestion to the exact context sent to child.

3. **record.rs382 child_main**: validate context after rebuilding workload/resolving method and before invoking the algorithm. Validate that identity matches expected workload/method/seed/attempt. Initialize diagnostic sink within a new enclosing instrumented-method interval; produce common entry/exit observations for every arm. Keep existing math solve and report semantics separate. Preserve failures as errors/diagnostic gaps according to prospective contract, never accept partial data in place of final ChildOutput.

4. **methods.rs2482 solve_measured → solve_inner2585 → three_leads.rs119 → pencil.rs66**: use explicit optional diagnostic sink/context through new `*_with_diagnostics` internal wrappers; retain old public wrappers passing None. Avoid ad hoc bypass dispatch in child_main, which would skip current applicability guards, measurement::Session, online-phase checks and complete(). Only the selected new instrumented pencil adapter selects the isolated fork; historical v2 source must remain reconstructable. Prefer a separately named instrument/profile revision rather than silently rebinding frozen v2 evidence. Root dependency wiring must avoid duplicating independent copies of the thread-local meter/kernel or measurement module: inspect Cargo features and re-export one shared kernel/measurement implementation before patch acceptance. Current isolated crate cannot simply be imported as a drop-in production artifact without that check.

5. **runner.rs after run_child return, before records seal at~805**: bounded sidecar ingestion creates a separate diagnostic manifest, referenced by an optional default-skipped Record diagnostic attachment, or a separately sealed session manifest linked by exact record identity. Prefer explicit optional Record field so custody is direct; do not stuff partial math fields into Record.detail or counters. Parent validates raw sidecars even for timeout; accepted final report selection remains runner.rs899–910 unchanged. Parent IO failure is recorded separately and cannot turn timeout into success or conceal a failure.

## Bounded ingestion contract

After reaping the child, open only the expected private leaf. At most 256 expected numeric JSON filenames, and independently cap directory entries to prevent unbounded traversal. Reject symlinks/nonregular files. Read each with a byte limit of 65,537 (one byte beyond cap signals rejection); no unbounded read_to_end. Hash exact bytes before schema parsing. Enforce exact schema, identity and sequence 0..N-1 <256, boundary whitelist and epoch/order transitions, phase-map allowlist and numeric types/ranges. Enforce monotonic counters within each epoch, not across the intentional reset. Snapshots are cumulative prefixes: never sum them. Reject duplicate JSON keys rather than relying on serde_json Value's last-key behavior. Unknown/extra counter fields are retained raw but cannot silently gain accepted diagnostic status.

Temporary files are separately bounded and classified incomplete; never parse them as committed snapshots. A visible JSON does not itself prove parent-directory fsync completed. Published versus durability-confirmed status remains distinct; final sink totals, when available, must reconcile with observed files. When the child died before reporting them, durability confirmation is unknown. Preserve these uncertainties and hash-bound raw data. Exact checkpoint/output custody still requires an independent wrapper-level inclusion check; placing files under output is only a prerequisite.

## Full cost interval and comparable instrumentation

Observed existing boundaries:

- runner.rs318 starts process_wall_ns after ChildInput serialization; its elapsed value is captured on reap, before output-reader joining, JSON parse, record assembly, parent audit, sidecar ingestion and publication.
- record.rs463 method_call_start encloses solve_measured and complete() through method_call_wall_ns~495, but excludes earlier workload construction, diagnostic context construction placed before it, later ChildOutput serialization and parent work.
- pencil.rs168 adapter elapsed is captured before accounting attachment. Native cold_setup_complete diagnostic lies between epoch clocks, and final native diagnostic occurs after candidate elapsed. Writer time excludes callback JSON construction and snapshot cloning.

Therefore neither native phase sum, adapter elapsed nor existing process_wall_ns is the complete cost of this instrumented execution. Add a parent `execution_envelope_ns` starting before directory/context creation and input serialization and ending after sidecar ingestion, required parent audit, record construction and publication work within a precisely declared boundary. Publishing the envelope's own final record creates a self-reporting tail: keep that tail separately in session/resource receipts, do not pretend it is included. Retain existing clocks for compatibility and label them nested; never sum overlapping wall intervals. Actual cumulative worker admission remains authoritative for all active work, failures, warmups, replay and recovery, including intervals with no final child output. A timeout has no accepted final cost; observed parent intervals can still be retained as censored resource evidence.

Use identical diagnostic configuration, binary/profile identity, sink limits and common entry/exit lifecycle for candidate, rho and rho-AA. Preserve method-specific stage callbacks without fabricating rho stages. Exact IO volume will differ because stage counts differ; calling this equal overhead would be false. Charge observed overhead fully to its arm, expose common versus method-specific diagnostic work, and keep operation-count and elapsed-time conclusions separate. A diagnostic pilot cannot establish timing-qualified candidate/rho rankings. Any timing qualification needs the frozen matched A/A and order controls and a prospective instrumentation comparison; do not subtract estimated diagnostic cost or retroactively combine historical uninstrumented clocks. If the scientific question requires precisely matched observer frequency, freeze a common external sampling scheme and validate its overhead separately instead of claiming this event-driven scheme provides it.

## Software-only production fixture plan

Existing watchdog_record_tests at runner.rs1177 calls plan(), Workload construction and child_main with BSGS at1206. Do not execute or reuse it for this no-solver task. Add a distinct synthetic fixture with literal serialized Workload/Instance/ResolvedMethod/HostCapsule/Conditions/SolveReport values, no workload generation or child_main call. If constructing all values is brittle, first extract the pure accepted-report selection and diagnostic attachment into a helper used by assemble, then test that helper and static assembly wiring; label this narrower coverage honestly until a full assembly fixture exists.

Synthetic Reaped matrix: timed_out/interrupted × output absent/late complete/late exhausted × valid/missing/malformed/wrong-identity/truncated diagnostics. Assert timeout or interruption status unchanged; no recovered scalar, total_gae, s, accepted phases/counters, source verification or online clock; diagnostic attachment remains explicitly partial and separately hashed. Set timed_out=true for direct assemble test so no source multiplication can execute. A separate pure helper test can cover normal noncensored forwarding without executing verification. Test child input absent-context roundtrip and context mismatch rejection before algorithm invocation using a fake injected dispatch callback that panics if reached.

Parent IO fixtures: too many files, overlong file, symlink, duplicate keys, wrong sequence/epoch, same-epoch counter regression, stale attempt/seed, post-link undurable file, truncated temporary, source/binary/config mismatch, and common-arm configuration equality. These are native software fixtures with fake numeric counters; no solve_one/rho/target generation, CRIU test, benchmark, cloud payload or scientific replay. Preserve all named test logs and source hashes.

## Admission remains separate

Before any experiment: scoped reservations; code review of actual diff; root production compile/fixtures; rebuilt binary/image hashes; changed instrument and known-data amendment independently reviewed; complete diagnostic custody and resource enforcement; matched controls and full 1,260-target/replay obligations retained. None of these gates is supplied merely by this plan or the nine isolated fixtures.

## Reservation check and next implementable slice

Read-only `conductor check` returned **Clear. Nothing else in flight touches this** for exact paths src/cryptanalysis/ecbench/{record,runner,pencil,methods,three_leads}.rs and Cargo.toml at this inspection. This is not a claim or durable reservation. Recheck and reserve before any edits; shared session log and module registration paths need separate checks if included.

Smallest useful production slice: add a separate ecbench diagnostic schema/ingestion module with strictly bounded reader and synthetic fixtures, optional default-skipped ChildInput diagnostic context, and a pure helper selecting accepted final output under timeout/interruption that assemble calls unchanged. Bind diagnostic manifest to the existing per-execution identity in runner, but keep diagnostic production disabled pending sink plumbing. This slice can establish fail-closed sidecar custody and real runner acceptance semantics without selecting an experimental solver or enabling any run. Minimal expected reservation is record.rs, runner.rs, new progress.rs (or diagnostics.rs), and ecbench/mod.rs for registration; first inspect/check the latter two. Do not change Cargo.toml or math dispatch until the following sink-plumbing slice. Preserve the pending pieces explicitly; this first slice is not full instrumentation, a performance result, or trial admission. A fullcost envelope can be added alongside ingestion, but cannot be called complete until its publication tail and resource receipt are bound.
