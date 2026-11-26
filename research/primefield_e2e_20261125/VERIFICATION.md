# Verification scope

This is native analysis of existing public records and conditional model inputs.
It does not run a relation solver, recover a new target, or issue an independent
performance certificate. PR #5226 owns the integration.

## Focused checks

- Rust 1.100.0; locked standalone crate imports the actual two ECbench modules.
- `cargo test --locked --release --manifest-path tools/primefield-e2e/Cargo.toml`:
  25 tests returned success. Tests include analytic optima, finite tuple counts,
  distinct point/rank inputs, unknown phases, cold/amortized totals, malformed
  records, missing timed output, single-size fit rejection, overflowing affine
  intersections, and the JSON pipeline's mismatched-arity/feasibility controls.
- Strict Clippy (`--all-targets -- -D warnings`) and touched-file rustfmt pass.
- All 15 input files are checked against their manifest SHA-256 and length.
- Recomputing the five native outputs gives byte-identical files.
- Report: nine pages rendered and individually inspected; prose lint returns
  zero findings. Python is used only for document layout and plot coordinates.
- Shared-code parity: 21 files agree with the lock. Existing prepush research
  gate passes, with no added PCC claims.

These are local producer checks. They do not confer independent PCC standing.

## Whole-repository limitation

`cargo test --release --lib` was attempted. Missing sparse-checkout compile-time
inputs were materialized without changing their bytes. The subsequent full
Rust library-test compilation terminated with signal 9 (`SIGKILL`) before the
test executable ran. Therefore the full repository test suite has not passed
in this session. The 25 focused tests above are not a substitute for that suite.

The subsequent `cargo check --release --lib` completed successfully (2m 25s),
so the new modules also type-check in the full library. That command does not
execute the full library tests.

## Data and model boundaries

The analyzer retains 768 timed rows and their producer recovery flags; 672
report full rank. All 768 lack a producer git-commit field, and this legacy
adapter establishes no complete cold-cost qualification. Failed preflights,
unpriced work, unknown phases and stage distinctions remain visible. The
replication-only file has 384 exhausted preflight rows and no timed records.

Conditional scenarios are supplied cost assumptions, not demonstrated solver
bounds. In particular the m=2 linear-trial result is architecture-specific;
it is not a universal ECDLP lower bound. No new leaderboard row or measured
speedup is produced. [pcc:unverified]

## Coordination and source access

Conductor T-1 owns the eight declared paths. The supported attached-task
conflict check returned `allow` after authenticated lease/fence verification.
The control plane is local to this session; no shared endpoint was configured.
No lease release/reclaim or policy suppression was used.

The research KB wrapper returned exit 127 because its executable/catalog was
unavailable. The fallback review covered PKM 2016 section 3.2, RFC 9380, the
pinned 2026 tensor-reduction source, archived phase records at
`aab505dd2d24cd5c98f9be1a2bd2243793a2001b`, and the signed-incidence scope in
merged PR #5187. No claim of novelty or absence follows from KB unavailability.

An auxiliary solver-source audit was stopped by an automated safety check.
It was not retried or delegated again. This deliverable uses the distinct
read-only cost-accounting and mathematical-model scope; independent solver
source qualification is not asserted.
