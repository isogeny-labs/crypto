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

## CI repair checks (2026-11-26 UTC)

The PR's Rust 1.98 Clippy job reported ten diagnostics in four existing library
files. The follow-up replaces equivalent integer predicates, restricts two
dispatch wrappers to their existing test callers, removes a terminal `return`,
and propagates packed-prefix pivots only into registers with unread rows.
The changed-file formatter also reformats `ecbench/isogeny.rs`; independent
comparison confirms that its only semantic edit is the equivalent bound
`p <= 2^61 - 1` to `p < 2^61`.

- All four affected files pass Rust 1.98 formatting and whitespace checks.
- The existing packed-prefix scalar-comparison regression passes over 7,680
  matrix cases. The temporary harness contains the current function and the
  unchanged existing test byte-for-byte and compiles with `-D warnings`.
- Five existing tests pass with the actual `binary_ecc/f2m.rs` compiled as a
  temporary Cargo library: hex word boundaries, 131-bit multiplication and
  squaring, and two half-trace controls. The isolated module emits warnings
  for three integration helpers whose callers are outside that harness.
- The complete maintained-target Clippy command was attempted with the pinned
  compiler, then with one compiler job, no debug metadata, and incremental
  compilation disabled. Both attempts terminated with `SIGKILL`. The second
  exposed a further unused low-register assignment, addressed by the final
  pending-register macro. No local whole-crate Clippy pass is claimed.

The six focused regression passes do not replace the repository-wide job.
Hosted CI on the published repair head remains the full strict-lint check.
No workflow, lint rule, or test was disabled or suppressed.

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

The initial integration reserved eight paths in Conductor T-1. The CI repair
follow-up reserved the four affected Rust files and the next UTC session log,
bringing the task to thirteen scopes. After the former local service stopped,
the documented stale-PID recovery restored its existing database. Conductor's
normal reconciler expired the old attempt; a supported claim created attempt 2
at fencing epoch 3. The attached-task conflict check returned `allow` after
authenticated verification of that current lease and fence. The control plane
is local to this session; no shared endpoint was configured. No coordination
policy or fence checks were suppressed.

The research KB wrapper returned exit 127 because its executable/catalog was
unavailable. The fallback review covered PKM 2016 section 3.2, RFC 9380, the
pinned 2026 tensor-reduction source, archived phase records at
`aab505dd2d24cd5c98f9be1a2bd2243793a2001b`, and the signed-incidence scope in
merged PR #5187. No claim of novelty or absence follows from KB unavailability.

An auxiliary solver-source audit was stopped by an automated safety check.
It was not retried or delegated again. This deliverable uses the distinct
read-only cost-accounting and mathematical-model scope; independent solver
source qualification is not asserted.
