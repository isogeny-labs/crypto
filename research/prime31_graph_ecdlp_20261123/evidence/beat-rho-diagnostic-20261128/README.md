# B677 M31 cold single-target diagnostic, 2026-11-28

This is a public-only progress record for [PR #5357](https://github.com/isogeny-labs/crypto/pull/5357). It does **not** establish an index-calculus win over matched optimized Pollard rho. The measurements below are one cold invocation each on a local 5-CPU, 17 GiB host, from an isolated diagnostic Cargo build of unchanged public source modules. The repository's full native package did not build because unrelated F4 source-hash pins failed; the diagnostic build is not a release benchmark.

## Sources checked

- Public B677 M31 fixture and graph-control implementation at frozen producer commit `577d6c2c1c4a527604d965e6fb62c44df7902f36`; the current fast collector and matched-rho reference are in `research/prime31_graph_ecdlp_20261123/native/mitm_four_fast.rs` and `src/cryptanalysis/ic_boundary.rs`.
- Public benchmark history in [#5322](https://github.com/isogeny-labs/crypto/pull/5322) (four-thread graph-control improvement) and [#5202](https://github.com/isogeny-labs/crypto/pull/5202) (an earlier single-thread IC/rho panel). Those are different panels and are not combined into a claimed speedup.
- Kaarelson, *A Fast Algorithm for Elliptic-Curve Discrete Logarithms*, v2, [paper](https://github.com/kaarelkaarelson/fast-ecdlp/blob/main/v2/paper.pdf), for its asymptotic claim. Its crossover estimate does not supply a practical M31 win.

Source SHA-256: `mitm_four_fast.rs` `60241fbb59fe3e8c4ec085ace1f1d876cd9b7671c5ee5872af0d2f2230ac954f`; `b677_fixture.rs` `3f491defbc7aa1643192db4d0c7144afcec1f84a41f03489355a866459329624`; `RUN66_Y7_EXTRA168.csv` `fc484615394ee423190747d05001cea526580fb0df0279fbcf789ab6cafc0c46`; `prime31` arithmetic module `52b30e41478b0306603db7d09bcb17377cae64efcfd78d4c34db7bf91325a867`.

Curve: `p=2147483647`, `a=847514846`, `b=2087444724`, `G=(625037715,2100627251)`, subgroup order `r=2147446831`; public target `Q=(976979610,1228979761)`. The recovered scalar `987654321` passed `[d]G=Q` replay.

## What was done (commands, files, features)

An isolated release diagnostic invoked `prime31-graph-mitm-fast 40 12345678 4` once as a fresh process. External process wall was 30.34 ms, child CPU 40.24 ms, peak RSS 15.6 MB. The collector reported 26 ms internal wall, 707 relation rows, 11 queries and 10,068,344 charged probe additions, then recovered and replayed `d=987654321`. Its report includes fixture, pair-table, collection, graph and recovery stages.

The existing public `prime31-graph-rho-control` was also invoked once cold on that curve and target. External wall was 29.99 ms, child CPU 29.56 ms; it recovered and replayed the same scalar in 98,366 steps. Its policy is an independent 16-jump **unfolded, untuned** Brent walk and is not a matched optimized negation-map control.

A scratch negation-folded, fast-Mersenne rho diagnostic was checked against repository group arithmetic for 1,000 operations, then tried 32 seeds. All 32 entered short fruitless cycles (length two); none recovered a scalar. Its 13.3 ms external wall, and 12.04 ms internal wall, are failed-run costs, not solve times or a comparator.

## Decisions made (and why)

The near-equal 30 ms process walls do not support a speedup claim: the successful rho policy is weak, the attempted optimized policy failed, the source is a diagnostic package, and each successful arm has one observation. The next control must use the repo's `RhoV3::v5(true)` design (64 jumps for this order, mixed-key partition, negation folding, short-cycle escape and distinguished-point handling), or an independently checked equivalent, on the same target and host. A cold interleaved panel must charge setup, retries and every failed walk; it must verify each recovered scalar and report wall, CPU, operations and memory.

## Open threads (unfinished, pick up next time)

Build and validate the optimized rho control, then run a matched interleaved cold panel against the current public M31 IC code. The frozen diagnostic collector precedes the later 8-lane AVX2 change in #5352, so pin the final candidate separately. Publish raw per-run records and the exact build recipe before any comparison claim.

## Errors (what went wrong, how it was handled)

The complete native package stopped at F4 source-hash pin checks in `glv_invariant_base.rs` and then `prime_fast.rs`; the pins were not changed. `/usr/bin/time` was absent, so external wall/CPU/RSS came from a Python `subprocess`/`perf_counter`/`rusage` wrapper. The scratch folded rho suffered fruitless cycles and was excluded from successful comparisons. A shared-log update was blocked by automatic approval review because replacing the large public file might disclose private metadata; this public-only record follows the user's approval and contains only public source and diagnostic results. [pcc:unverified]

### Follow-up at 07:20 UTC: first optimized-control diagnostic

A separate scratch control with 64 jumps, the public mixed-key partition, negation folding, look-ahead, 16-point cycle escape and 7-bit distinguished points was built. Its **first** cold invocation ended after its 32-walk provisional cap: 3.679 ms external wall, 3.511 ms child CPU, 3,423 walk steps, 3,543 total additions, 32 distinguished points, zero collisions, no recovered scalar. This is a capped failed run and cannot be compared to a complete IC solve. The implementation uses random full-size jump scalar multiplications and fresh random starts rather than the repository v5 ladder/stride setup, so it is v5-inspired, not source-equivalent. The next check increases only the walk cap and independently replays any recovered scalar. No successful comparison has been made.
