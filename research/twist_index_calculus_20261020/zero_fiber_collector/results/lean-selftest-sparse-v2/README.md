# Exact sparse Lean selftest input closure

The full existing seven-fixture selftest passes, terminal exit 0. It runs
from a separate repository containing exactly the tracked `tools/lean/`
subtree exported from source head `59e732cff1c`. The proposed workflow
changes only the selftest checkout to that subtree, in non-cone mode.
The fixtures, expectation matching, pinned Lean 4.34.1, audit implementation
and trust manifest are unchanged. This verifies the checkout input closure;
new-head CI remains an independent required observation.

Both positive fixtures pass. All five negative fixtures fail with their
specified reason, including the nonstandard axiom, empty module,
unapproved native decision and sorry checks. The existing explicitly
trusted fixture remains visibly reported as trusted. No theorem or
fixture is skipped or granted additional trust.

The shared-host test uses the common reservation and an unchanged
8 GiB address-space / 1,200-second aggregate phase. A local `bin/lean`
wrapper supplies the documented `-j1` option to every actual Lean call,
including the axiom audit. It changes execution concurrency, not logical
acceptance. The initial interrupted local attempt and its retained
thread-creation failure are preserved in `../lean-selftest-sparse-v1/`.
The first attempt's environment variable did not establish the required
thread bound. No complete pass or exact cause for every individual
interrupted fixture is inferred from those partial logs.

The original CI failure is separately preserved in that sibling's
`failed-ci-sanitized.log`: job 121105883613, run 40172421560, old head
`59e732cff1c`, runner disk at 39 MB free and toolchain extraction failing
with `No space left on device`. It is an infrastructure failure before
proof verification. No compiler downgrade or proof-policy change addresses
it. The sparse checkout reduces that selftest's materialized inputs;
whether the current runner has sufficient disk is verified by CI.

`command.txt`, queue/admission and envelope receipts, actual stdout/stderr,
terminal `exit`, exact workflow copy, pin/version and source hashes are
retained. These are infrastructure validation records, not a new
cryptanalytic experiment, proof of the failed concrete certificate or a
performance measurement.
