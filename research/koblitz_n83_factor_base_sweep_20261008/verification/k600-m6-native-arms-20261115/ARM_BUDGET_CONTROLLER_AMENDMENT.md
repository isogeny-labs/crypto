# K600 m6 outer arm budget controller — implementation amendment, 2026-11-15

The native `examples/n83_k600_m6_budget.rs` controller orders the nine
constructed and natural export, solve, and replay phases after a four-phase
build/stage/freeze/preflight precharge. It launches one phase at a time, gives
each an outer wall limit from the remaining proposed 3,600-second envelope,
records stdout, stderr, the child receipt and its hash, and stops on a failed
control, a natural censored outcome, cleanup failure, or budget exhaustion.
It checks each successful child receipt's schema, mode, source commit, and arm
lock hash. Docker cleanup inspection and kill commands each have a ten-second
cap. The controller writes a terminal failure record when a stage raises an
error after admission.

**Status:** implemented and unit-tested, not admitted for a K600 execution.
The precharge is an externally supplied declaration whose phase receipt
hashes are checked, but this code does not independently measure or attest
the build, staging, freeze, and preflight outer wall intervals. The result's
`charged_total_to_summary_start_ms` is a lower bound: writing the summary and
process exit occur after its measurement cutoff. A separate outer process
receipt that charges the entire controller invocation, plus a source-locked
image/build receipt and real host preflight, is required before an execution
can claim complete cold budget accounting. The proposed 3,600 seconds in
the protocol is not itself an execution grant.

`cargo test --release --example n83_k600_m6_budget` passed 3 of 3 tests;
`cargo build --release --example n83_k600_m6_budget` passed using Rust 1.99.1
on macOS arm64. The touched test covers all nine command mappings and receipt
identities, but it does not exercise Docker or a real K600 input. The root
release library gate was rerun at commit `9428ed7e275b` with
`RUSTC="$(rustup which rustc --toolchain 1.99.1)" cargo test --release --lib`.
It exited 101 after 609.57 seconds: 6,306 passed, 59 failed, 138 ignored.
The full compressed log was moved by the repository artifact automation at
`2fd1d4886a6c`; its [tracked artifact manifest](.artifacts.json) records
120,556 bytes and SHA-256
`9afa9ded60430c739369aba3b379b44c064b5ba0bfbdfcdba2fbca131dea25aa`.
Failures include `certicom_dlp`, `ecbench::band`, and `ml_dataset` tests.
The `cargo test --lib` target does not compile this new example, and this
commit changes no library source or fixture. The failures therefore cannot
be attributed to the example diff; the gate is nevertheless red and does
not permit a merge. No K600 search or timing was run here.

No performance graph or canonical leaderboard row changes: this amendment
contains no measured K600 factor-base or total IC runtime. Next, establish
an independently measured full-invocation receipt, verify its image and
input hashes on the qualified Linux host, and execute the frozen preflight
only after the host/input and execution-budget gates pass.
