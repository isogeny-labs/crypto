# Round 90: root-certificate replay, controlled ablation

Date: 2026-11-21 (UTC). Branch: `automation/rootfind-column-accumulator-round89-20261121`. PR: https://github.com/isogeny-labs/crypto/pull/4880

## Candidate paths and decision

1. **Retain:** In `verify` at `RootEvent::GcdStep`, the independently recomputed Euclidean remainder `verified_r` is already compared exactly to the event remainder. Move that owned vector into the GCD state instead of cloning the event's remainder. The verifier still independently recomputes and checks both the quotient and remainder.
2. **Reject after ablation:** At `RootEvent::PowerStep`, replace general `vproduct(base,base)` with symmetric `vsquare(base)`. This was mathematically equivalent and improved isolated C++ arithmetic, but the Rust square-only variant was neutral to slightly slower for complete root operations. The branch has reverted this change.
3. Frozen Round89 dense convolution, squaring, and certificate-division clone optimizations remain in the same draft PR. No merge: earlier sparse-product regressions, broad repository validation, and relations/sec remain open.

## Source-exact native Rust measurements

Four-way Rust A/B completed successfully on Ubuntu 24.04, Rust 1.100.0, CPU-pinned, 13 alternating paired trials per variant, 650 checksum-matched comparisons each. Baseline for these figures is frozen Round89 commit `fb1011afad75de2bbe473cc5ed18405fb1e1983a`. These numbers are **GCD-remainder-reuse-only**, not the combined candidate.

| Prime | Complete solve + verify | Before ns/op | After ns/op | Ratio |
|---|---|---:|---:|---:|
| 1,000,000,007 | quadratic | 159743.594 | 154966.930 | 1.0308x |
| 1,000,000,007 | quartic | 251024.906 | 233547.844 | **1.0748x** |
| 1,000,000,007 | fully split degree 8 | 1986141.125 | 1980778.625 | 1.0027x |
| 2^61-1 | quadratic | 452100.555 | 430032.891 | **1.0513x** |
| 2^61-1 | quartic | 661186.727 | 639218.672 | **1.0344x** |
| 65,521 | fully split quartic | 301775.578 | 283875.773 | **1.0631x** |
| 65,521 | fully split degree 8 | 1147193.625 | 1142547.500 | 1.0041x |

The square-only ablation had ~0.99x to 1.006x ratios on the above full workloads, confirming that the GCD ownership improvement is responsible for most of the measured gain. See [four-way successful CI run](https://github.com/isogeny-labs/crypto/actions/runs/44920200147) and its raw paired CSV artifacts. Different A/B runs show moderate CPU variation; use same-run comparisons, not absolute cross-run nanoseconds.

## Local C++ reproductions (not production Rust)

CPU-pinned Intel Xeon Platinum 8573C, GCC 14.2 and Clang 17, 15 alternating A/B trials, 1,500 polynomial examples per case, 36,000 exact arithmetic checks per compiler; UBSan passed. The isolated width-eight GCD-remainder state change measured 1.77x with GCC and 1.68x with Clang at p=1,000,000,007. Isolated symmetric square was faster, but did not help the complete Rust solver, so was rejected. The C++ figures cannot be interpreted as complete Rust throughput.

`perf` is not installed locally; direct `perf_event_open` for cycles, instructions and task-clock returns `EACCES`, so local timings use CPU-pinned `steady_clock` and **not hardware counters**.

## Outstanding gates

Keep PR in draft. Repeat source-exact Rust comparisons on the final retained candidate, investigate older sparse-product dispatch regressions, run broader repository tests, and measure representative index-calculus relations/second. No end-to-end relation-throughput claim is supported.
