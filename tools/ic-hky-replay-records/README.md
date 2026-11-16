# Native HKY replay records and execution

This standalone Rust crate replaces the new Python replay harness identified in
PR2655 review. The scientific computation remains the unchanged, source-bound
`ic_guess_completed_audit-restart` executable. No Python program supplies the
guards, restore handling, host records, rows, scientific aggregation, export or
process loop in this execution path. The repository's prescribed checkpoint
service remains the existing `scripts/checkpoint.sh gate` infrastructure.

The thin study Bash entrypoint invokes this native controller under the outer
watchdog. Each native child uses the original scientific binary, inputs,
one-thread settings,8GiB address-space cap and900-second watchdog. The controller
takes the same `/tmp/crypto-bench.lock` as existing source gates and heavy jobs,
and holds an excluded per-run `.batch.lock` to prevent duplicate batch owners.

## Historical handoff

The original batch stopped after four complete900-second timeouts, with raw
manifest verification passing and no fifth case started. Source e01cb6ebbf05ba9f7367eb154f888b2dd1767ae5
and `runs-hky-legacy-orchestration-20261026.tar.xz` retain the old Bash/Python
orchestrator, original protocol, complete raw tree and controlled-stop receipts.
The old Python file remains historical source; current entrypoints do not call it.

A new prospective protocol must select the remaining122 tuple identities in both
real arms:244 fresh children. The two legacy tuple identities are excluded only
because both of their arms already ran, not according to outcomes. No timeout
is retried, no legacy row becomes a new prospective row, and the original
248-process registration remains a separate historical record.

The new protocol must be frozen, pass native schema/source/binary guards, and
have an actual successful quiet S3 checkpoint/readback before a fresh case runs.
These preparations do not themselves complete an independent replay or admit a
leaderboard result. Censored ranks and independent word cost remain absent.

## Integrity and restore order

`--resume` verifies the old `SHA256SUMS` before changing runner PID/PGID files,
host records or interrupted-case status. This fixes the reviewed ordering defect
in the historical Bash runner. An incomplete restored execution is retained as
interrupted and is never silently retried. A completed immutable export is
verified without rewriting process metadata that its file index binds.

Row mapping uses the canonical export's numeric instance indices and sentinel
seed0. The frozen index map and each row's `backend_ref` retain the actual
draw/guess/branch identity. The native validator checks the Cartesian grid,
schema, source pins, outcome/flow accounting and every exported file binding.
Final JSON publication is atomic and refuses to overwrite an existing export.

Source tests include the old PID checksum regression, tampering with no new
metadata writes, canonical schema/hash compatibility, unchanged scientific
contracts, censored metrics, complete execution grids, file-bound exports and
write-once output. Test metadata fixtures are not scientific executions.

## Commands

```sh
cargo test --locked --offline --manifest-path tools/ic-hky-replay-records/Cargo.toml
cargo clippy --locked --offline --all-targets --manifest-path tools/ic-hky-replay-records/Cargo.toml -- -D warnings
cargo build --locked --offline --release --manifest-path tools/ic-hky-replay-records/Cargo.toml
```

The frozen binary offers `guard ROOT PROTOCOL`, `freeze ROOT PROTOCOL`, and
`run ROOT PROTOCOL [--resume]`. Freeze checks that the new raw directory does not
exist. Execution verifies the frozen scientific and controller pins. A registered
pause-request file is checked between cases, after the last completed case has
been checkpointed. The native scientific summary still distinguishes all
unexecuted, capped, interrupted and verified rows; no clock is promoted.

Runtime is Linux with the registered GNU time/timeout tools. Host facts are
limited to CPU/kernel/memory/pressure and the three declared thread settings;
credential values and an environment dump are never recorded. The native schema
dependency disables HTTP/file retrieval and loads the pinned local schemas.

The total extra-cloud ceiling remains USD100 including replay. This replacement
does not launch an extra VM, supply an IC price, compute factor-base logarithms
or establish a measured crossover.
