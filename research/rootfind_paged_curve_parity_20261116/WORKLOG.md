# Task-local worklog for the actual-curve paged-index gate

The repository's `docs/session-logs/2026-11-16.md` is reserved by Conductor
T-6720 while its ready PR #4416 is under verification. Conductor rejected a
new exclusive claim on that file. This task therefore records current facts
here, within T-6726's disjoint write scope, and leaves the canonical log
untouched until its reservation clears. [pcc:unverified]

## 2026-11-16T10:40Z — Gate freeze

### What was done (commands, files, features)

- Read the current production builder, public curve/base APIs, and prior
  source-pinned paged prototype. Froze the K13/K19/K23 actual-stream parity
  protocol before writing its test. The shared KB source
  `source:e9db8cc2be204429e8fb1daa12a3324c` and current local program
  paper bound the prior-work claim; the KB refresh predates the new code.

### Decisions made (and why)

- Use a separate Cargo integration test path and compare the exact collision
  stream to the existing production builder. Avoid the two source files held
  by T-6708 and `.resume` held by T-6342.

### Open threads (unfinished, pick up next time)

- Pin this protocol commit, implement and run the fixed parity test, retain
  receipts, and publish a visual report in a ready stacked PR.

### Errors (what went wrong, how it was handled)

- A Conductor conflict blocked the canonical session log; this worklog
  preserves the task facts without taking that reserved path.

## 2026-11-16T10:45Z — Actual-curve parity test implemented

### What was done (commands, files, features)

- Wrote `tests/rootfind_paged_curve_parity.rs` against the public Koblitz
  constructor and production torsion builder. It reconstructs the ordered
  pair-sum stream with the group law, computes the census-compatible base
  digest, compares each insertion to a standard hash map, checks every
  collision's full group equation, and compares the full collision vector
  and counters with production. Rustfmt adjusted three assertion layouts.
  No parity run has been made yet. [pcc:unverified]

### Decisions made (and why)

- Commit the test source before the fixed release run, so any observed output
  binds to a source commit. Keep timed measurements out of this gate.

### Open threads (unfinished, pick up next time)

- Run the source-pinned test, retain exact stdout/stderr, derive the per-cell
  page occupancy and report the result at its checked scope.

### Errors (what went wrong, how it was handled)

- The first rustfmt check requested formatting only; it was applied before
  source pinning.

## 2026-11-16T10:47Z — Default toolchain refusal

### What was done (commands, files, features)

- Ran the frozen Cargo integration test from source commit `1103716b6d`.
  Cargo exited 101 before compiling the test because default rustc 1.93.1
  is below several pinned AWS dependency requirements of 1.94.1. The
  complete stderr is retained as `test.stderr`; stdout is empty.

### Decisions made (and why)

- Rerun the unchanged test with already installed Rust 1.99.1, as earlier
  project receipts did. This is a compiler-admission correction, not a
  changed test or curve outcome.

### Open threads (unfinished, pick up next time)

- Capture the 1.99.1 run separately and inspect its actual parity results.

### Errors (what went wrong, how it was handled)

- The default compiler refused locked dependencies before test execution;
  no algorithm evidence follows from that exit.

## 2026-11-16T10:48Z — Cargo wrapper correction

### What was done (commands, files, features)

- `cargo +1.99.1 ...` exited 101 immediately because the shell resolves
  Homebrew Cargo rather than rustup's shim. Its stderr is retained in
  `test-1.99.stderr`. `rustup run 1.99.1 cargo --version` confirms Cargo
  1.99.1 is installed.

### Decisions made (and why)

- Use `rustup run 1.99.1 cargo test ...` on the unchanged source and preserve
  this second wrapper failure separately.

### Open threads (unfinished, pick up next time)

- Execute the actual test with the pinned toolchain.

### Errors (what went wrong, how it was handled)

- The Homebrew Cargo wrapper did not interpret the `+toolchain` syntax;
  no test was run.

## 2026-11-16T10:49Z — Compiler path correction

### What was done (commands, files, features)

- `rustup run 1.99.1 cargo test ...` also exited 101 before compiling the
  test. Its stdout and stderr are retained as `test-rustup.stdout` and
  `test-rustup.stderr`. Cargo was 1.99.1, but Cargo found Homebrew's rustc
  1.93.1 on `PATH`; the dependency admission message confirms this.

### Decisions made (and why)

- Set `RUSTC` to the absolute installed 1.99.1 compiler for the next
  unchanged run. Preserve the third admission failure separately.

### Open threads (unfinished, pick up next time)

- Execute and inspect the pinned parity test.

### Errors (what went wrong, how it was handled)

- The mismatched compiler path prevented test execution; it is not an
  algorithm observation.

## 2026-11-16T10:55Z — Platform build boundary

### What was done (commands, files, features)

- With `RUSTC` pinned to 1.99.1, the top-level Cargo integration-test
  command compiled dependencies but exited 101 while building the unrelated
  `icprog` binary. `icprog` calls Linux-only `libc` affinity and limit APIs
  unavailable on macOS. The complete output is retained in
  `test-rustup-rustc.stdout` and `test-rustup-rustc.stderr`.

### Decisions made (and why)

- Add a study-local Cargo harness that depends on this checkout's `crypto`
  library and points its test target at the unchanged source-pinned test.
  This avoids the root package's binary target while retaining the exact
  library, curve fixtures, index prototype, and assertions.

### Open threads (unfinished, pick up next time)

- Commit the adapter, then run the unchanged test and retain its output.

### Errors (what went wrong, how it was handled)

- The failure is a macOS build incompatibility in an unrelated binary, not
  a parity result.
