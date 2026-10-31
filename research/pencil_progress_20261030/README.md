# Isolated pencil partial-progress diagnostic implementation

T-927. Software implementation and fixture evidence only; no new scientific run, performance result, prospective protocol freeze, or execution admission. Full four-lead and 1,260-target scope remains unchanged. Current planning envelope stays 2026-10-30 15:08:40–22:08:40 UTC; this work does not reset it.

## Source and rationale

`evidence/native_solver.baseline.rs` preserves the copied native solver, SHA256 e71e9cf9b356db1f16781f394994b0ca13abbb05b4a7db82bfd2a6925a0ef4df. `src/native_solver.rs` is an isolated fork. The original research file and root runner/adapter remain untouched. A generated unified diff and file hashes accompany the software receipts. The baseline remains scientific known-data context, not freshly revalidated mathematics.

The terminal gate's two pencil watchdog timeouts lack completed child phase reports. Completed boundary snapshots can preserve prefixes without changing timeout acceptance. They cannot reconstruct missing historical data, determine a measured bottleneck, or establish that the timeout is corrected. The source-bound design consulted native `measured`, setup counter reset, root adapter `observer=None`, and runner `assemble` timeout exclusion. Repository AGENTS.md and session log were read. Relevant primary IO references: [Rust rename](https://doc.rust-lang.org/std/fs/fn.rename.html), [File sync_all](https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_all). The implementation uses same-directory `hard_link` publication rather than replacement rename to avoid clobbering an existing committed snapshot; it synchronizes the file and parent directory. Linux/local-filesystem semantics are assumed; cloud durability is not established.

## Implemented contract

- Existing `solve_one` delegates to `solve_one_with_progress` with disabled `Noop`. Disabled callbacks do not construct snapshot JSON. Algorithm RNG, ordering and math operations are retained in the isolated fork; no-op scientific equivalence has not been established by a solver execution.
- Snapshots follow rational table completion, cold setup before reset, candidate setup/validation completion, each walk generation and sieve, fixed completed-candidate milestones, and solver return. Candidate milestones are 1, 4, 16, 64, 256, 1024, 4096, 16384 per walk, after relation processing or rejection. This yields at most 165 emission opportunities for 16 walks, below the 256 cap. It does not sample within a long unfinished stage.
- `cold_setup` and `candidate` epochs remain separate. Cumulative prefixes and existing completed phase aggregates are exported without invoking final phase reconciliation. Do not sum successive snapshots. A final returning snapshot is still diagnostic-only, not an accepted final result.
- Identity binds session, execution, workload, seed, attempt, source, method/config, binary and protocol digests. This module validates supplied identity consistency; it does not independently measure or attest those digests. The future parent must supply and verify them.
- Every snapshot explicitly forbids an accepted scalar and candidate/rho ratio. The sidecar is separate from final ChildOutput/SolveReport. No root timeout code changed.
- Maximum 256 committed snapshots and 65,536 encoded bytes each. One bounded serialization buffer and exclusive temporary sibling. Unix directory creation explicitly requests mode 0700 and temporary file creation mode 0600 (umask may further restrict). Successful hard link publishes a complete file without overwriting an existing destination; then temporary unlink and directory sync. IO error disables future snapshots and is retained in in-memory totals. Published count/bytes advance immediately after successful hard_link; committed count/bytes advance only after temporary unlink and parent-directory sync succeed. Failures in that window leave published distinct from durability-confirmed totals; they disable later writes and are never silently retried.
- IO totals count attempts, successful writes/bytes and completed writer intervals. A snapshot contains prior IO totals; its own completed write time cannot appear within itself. Interruption can leave missing final IO totals and an incomplete temporary file. Neither is zero work.

## Accounting boundaries and limitations

The callbacks sit outside individual measured math closures, preserving symbolic/field ledger intent; they still cost wall time and allocation. Early setup callback overhead enters cold setup elapsed time. The cold_setup_complete callback occurs after cold setup elapsed capture and before the candidate clock, so it belongs to neither epoch clock. Candidate callbacks enter candidate envelope overhead. The final returning callback occurs after candidate elapsed capture; its cost remains in the enclosing adapter/process/worker interval and must be separately included when integrated. No final elapsed cost may be treated as complete without that adapter accounting. Writer timing excludes caller JSON construction and Snapshot cloning and is not the total diagnostic overhead. The root adapter currently has no allocation observer. Global allocator peaks, when available elsewhere, are not additive or safely subtractable.

Payload encoding is bounded, but constructing caller-supplied JSON before entry is not bounded by the sink. This fork supplies only fixed-size phase/counter maps. Filesystem latency is not bounded; watchdog timeout remains authoritative. A stalled diagnostic write can contribute to timeout. The directory parent must be runner-owned/trusted; this is not an adversarial path sandbox. Rejecting symlink temporaries does not make a maliciously replaced parent safe.

The fork currently exports grouped candidate setup (not separate solver_setup/target_validation milestones), no per-walk-end snapshot, no graph rank/retained-coefficient fields, and no initial identity-only emission before setup. These are explicit deviations from the broader design, not promises of full diagnostic coverage. The fixed schema has no completed/accepted-cost path.

## Software verification only

Named command, serial fixtures only:

```
cargo test --offline --manifest-path research/pencil_progress_20261030/Cargo.toml --lib progress::diagnostic_fixture_tests -- --test-threads=1
cargo clippy --offline --manifest-path research/pencil_progress_20261030/Cargo.toml --lib -- -D warnings
cargo fmt --manifest-path research/pencil_progress_20261030/Cargo.toml
```

`evidence/tests-review-release.*` and `evidence/clippy-review.*` are the final-source logs. Earlier iteration logs are retained separately. Nine fixtures cover distinct epochs/identity/sequence checks, byte cap, write disabling, existing partial temporary, existing destination preservation, forbidden accepted result/ratio, malformed JSON, symlink temporary, milestone selection, snapshot cap, disabled no-op, Unix private permissions and injected post-publication unlink/directory-sync failures. Original source tests compile but are filtered out: no solve_one/rho execution or target generation occurred. Build dependencies are existing local/offline Rust dependencies. There are no scientific samples or timing claims.

## Required production work before admission

1. Independently review the fork diff, diagnostic observer effects and accounting boundaries; more fault injection is required for write_all/file sync and actual publication crash windows. Current tests inject post-link unlink/directory-sync failures and simulate existing temporary/destination failures; they do not simulate every syscall or actual process kill.
2. Add optional explicit child diagnostic context and scoped production adapter plumbing. Create per-execution directory under the session output tree; verify checkpoint/output inclusion and hash-bound parent association. Do not enable through uncontrolled ambient environment.
3. Parent must bound reads, validate the identities and sequence, retain failed/truncated sidecars separately, and keep null results on timeout. A synthetic Reaped/late-ChildOutput timeout fixture in the real runner remains unimplemented; the isolated module cannot prove runner integration.
4. Freeze diagnostics-enabled instrumentation and control/overhead accounting prospectively. Preserve known failures and all warmup/control/replay cost. Required mathematical replay, full cohorts and independent source/count review remain outstanding. This module does not authorize repeating a failed gate.

The local validator checks schema, identity, epoch, sequence cap and nonacceptance fields; it intentionally does not attest arbitrary boundary names or JSON counter contents. Until a parent enforces the exact counter and boundary schema, sidecars are untrusted diagnostic material and cannot support independent phase conclusions.
