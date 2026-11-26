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
On repair head `2effde8e4cc87ae7a16818550c3af128a57e6ca8`, the hosted
[strict maintained-target Clippy command](https://github.com/isogeny-labs/crypto/actions/runs/45770045007/job/139740580536)
completed in 3m 48s. This compiles the selected test targets; it does not execute
the full library test suite. No lint rule or test was suppressed.

## Historical workflow repair

Editing the shared polynomial module also triggered twenty historical
rootfinding workflows, containing twenty-six matrix-expanded jobs. Seven
failed before compilation because their baseline branch names had been
deleted. Their baseline references now use the exact head commits recorded
by merged PRs #4999, #5002, #5003, #5008, #5010, #5011 and #5014.

All seven repaired workflows' existing native build and execution commands
returned zero locally, using the unchanged candidate source and original
arithmetic checks. These executions validate CI; their timing output is not a
new performance result. The source and helper hashes were retained.

The twenty workflows now follow the research lane in `AGENTS.md`: their
original path filters apply to main and historical-branch pushes, with manual
dispatch available. Parsed and byte-level comparisons retain all twenty-six
job definitions, matrices, timeouts and test commands except the seven
baseline-pin substitutions. Repository-wide actionlint 1.7.12 passes, as do
the four YAML-parse tests over all 876 workflows and fifteen budget-regression
tests. The seven native workflow executions used Rust 1.100.0.

The full workflow budget checker still reports 117 pre-existing violations
across 38 untouched workflow files. Each of those files was compared
byte-for-byte with `origin/main`; none was changed by this repair. The twenty
edited workflows went from 42 budget violations to zero. The full budget
checker is therefore not reported as passing.

The prior repair head had fifty-four counted checks. Moving this historical
family off PR events projects twenty-eight counted checks, below the existing
thirty-check cap. Actual check inventory and required outcomes must still be
checked on the final published head. The merge checks and check cap are
unchanged.

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
initially bringing the task to thirteen scopes. Historical workflow repair
subsequently added twenty workflow paths and `.github/CI.md`, for thirty-four
reserved scopes. After the former local service stopped,
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
