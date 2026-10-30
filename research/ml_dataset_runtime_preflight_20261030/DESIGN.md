# Native runtime-preflight runner

This tool times a real local `crypto dataset generate` or `crypto dataset worker`
command for one declared arm. It is a bootstrap measurement instrument, not a
dataset generator or a substitute for a frozen experiment protocol. The final
protocol must identify the exact command, plan, source, binary, host, limits,
and arm represented by every observation. The target population remains the
full requested curve, twist, size, arity, method and key matrix; a pilot
observation is not evidence of coverage beyond its own cell.

The runner accepts only a dataset `generate` or `worker` command, rejects
publication/remote/budget options, fixes an explicit timeout, creates a new
output directory, and records the binary SHA-256 before and after the command. It
captures stdout and stderr, records a monotonic `Instant` duration around the
supervised process, and emits an immutable receipt even when the child fails.
Each call has one arm label and one supplied source commit. The runner hashes
the exact plan/config/protocol input file before and after the command, and
requires output at `OUTPUT/attempt`. `generate` is limited to `--max-cases 1`.
The source commit string is caller-supplied and must be verified against the
build receipt by the admission workflow. A failed or
censored call remains a measured attempt, never an extrapolated solve.

The duration includes `timeout` process startup, dataset command startup,
and child wait. It excludes runner setup, input hashing, output hashing and
receipt writing. It is a local throughput-planning observation, not an
isolated comparative timing. `generate` is preferred when its frozen bootstrap
protocol is available because it includes parent-cell setup. Direct `worker`
observations are explicitly marked `worker_only` and cannot stand in for
end-to-end `generate` timing or a V2 policy/capture path unless the retained
sidecars prove that exact path.

Before timing an actual solver, file the arm manifest and draft
`experiment.protocol/v1` per the pinned official checker. One measurement per
declared arm informs fixed and per-sample estimates; the final frozen protocol
records the observation receipt hashes and the extrapolation rule. A later
accepted run uses its own frozen plan/protocol and writes its own artifacts.
The runner never uploads, publishes, or makes a scientific claim by itself.

The implementation is `runner.rs`, compiled with Rust 1.98 using `-D warnings`.
`test-runner.sh` checks success, failure, timeout and publication rejection
using local stub binaries. Those tests execute no cryptographic solver and do
not establish a runtime estimate. No actual arm is admitted until its draft
protocol and source/binary/input identity contract have been filed.
