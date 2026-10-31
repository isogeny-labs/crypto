# Checkpoint and Spot interruption review

Reviewed 2026-10-19. Original PR head: `640d54bb808672376b38034b1985519bdc158416`.
The branch incorporates main `fc5d48359644e45961cd28550eecd940e75a76cc`
by a normal merge. No rebase or force push is used.

## Corrected behavior

- [`taskq/checkpoint.py`](taskq/checkpoint.py) publishes an immutable, fsynced
  generation before its atomic digest marker. Its independent stable copy can
  be overwritten without changing the generation. A failed replacement retains
  the last committed checkpoint. Current and preceding committed generations
  are retained; a filesystem lock serializes overlapping periodic and shutdown
  saves, including retention cleanup.
- [`taskq/execute.py`](taskq/execute.py) hashes the exact bytes copied into the
  new attempt before exposing `TASKQ_RESUME_FILE`. A prior worker cannot change
  those resumed bytes after verification. Relative checkpoint roots become
  absolute, and malformed/non-UTF-8 markers fall back to older valid attempts.
- [`deploy/ephemeral/runner.sh`](deploy/ephemeral/runner.sh) exports resume paths
  before setup and records the actual command session's process group. GNU
  time stays outside that session; GNU timeout keeps its descendant timeout
  behavior. The Spot monitor signals the application group during its existing
  checkpoint window.
- [`../ecc2k130/aws/worker.py`](../ecc2k130/aws/worker.py) persists the DP stream
  and prefix required by a retained snapshot before attempting its delta
  manifest. An empty spool after a crash no longer permits publishing a
  checkpoint ahead of its points. Pending validation still permits further DP
  uploads, and malformed snapshot boundaries fail closed.
- The ephemeral CLI's literal-format lint and AWS deployment formatting are
  corrected. The Terraform tests explicitly mock the `us_east_1` provider used
  for Karpenter's registry token as well as the default AWS provider.

## Local validation

The following checks passed on the final implementation. Test durations are
diagnostics, not performance measurements.

| Check | Result |
|---|---|
| `cargo test --locked --release --lib` | 4,405 passed, 0 failed, 118 existing ignored |
| `cd taskq; python -m pytest tests deploy/ephemeral/tests -q` with a real local Redis server | 59 passed, no skips |
| `cd ecc2k130/aws; python -m unittest test_worker_spool test_worker_banner test_worker_family test_worker_cairn test_certification -v` | 101 passed |
| `make -C ecc2k130 cpu build/test-production` | Native client and certification fixture built |
| Ephemeral CLI `cargo test --locked` | 8 passed |
| Ephemeral CLI `cargo clippy --locked --all-targets -- -D warnings` and `cargo fmt --check` | Passed |
| AWS Terraform `validate`, mocked `test` | Valid; 3 plans passed |
| AWS bootstrap Terraform `validate`, mocked `test` | Valid; 1 plan passed |
| Ephemeral Terraform `validate`, mocked `test` | Valid; 3 tests passed |
| Terraform format, Python compile, Bash syntax, `git diff --check` | Passed |

Regression tests exercise interruptions before marker commit and after commit
but before stable-copy replacement, mutation by an old worker, corrupt markers,
bounded retention, overlapping saves, absolute and relative checkpoint roots,
a crash before a DP manifest, and pending DP/checkpoint validation. The runner
test sends a real process-group SIGTERM and checks the application's resulting
checkpoint, and executes the actual setup export block.

Tooling: Rust 1.98, Python 3.12, Terraform 1.9.8, GNU time 1.9, and Redis 6.2.
The existing Pydantic forward-reference warning and upstream Terraform module
deprecation warnings remain visible. Conductor was attempted before editing
and scope expansion, but its executable is unavailable in this environment.

## Scope and remaining gates

These are infrastructure correctness and validation changes. Solver algorithms,
scientific inputs, operation counts, benchmark observations, curve identities,
and admitted performance results do not change. Canonical performance graphs,
scoreboard/leaderboard panels, and benchmark mirrors therefore remain unchanged.
No new scientific or performance claim is made.

No AWS resources were launched, modified, or deployed for this review. The
original PR's historical AWS smoke runs are separate from these local checks;
the revised code has not been deployed to active workers. This review does not
claim to reproduce an actual AWS interruption notice. Exact-head GitHub CI and
conflict checks remain mandatory after the fixes are pushed and before merge.
