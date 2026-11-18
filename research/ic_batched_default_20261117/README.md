# ic_solve_fast: additive fast single-target prime-IC solver

All numbers [pcc:unverified] (producer-side, single host, 4 threads, one run each, seed 20261117).

## What
`prime_fast::ic_solve_fast(curve, g, q, fb_size: Option<usize>, extra, max_walk, seed) -> Option<(u64, IcFastReport)>`
runs the batched lock-step pair-table path (`ic_multi_solve_ex(.., &[q], .., table=true, batched=true)`)
on one target. `fb_size=None` auto-tunes `B = clamp(round(p^(1/3)), 32, 16384)` (`ic_fast_auto_fb`).
`ic_solve` is untouched. The log is verified `xG == q` inside the multi solver (fail-closed).

## Correctness
`cargo test --release --lib cryptanalysis::prime_fast`: 15 passed, 0 failed, 1 ignored
(`test_prime_fast.log`). New test `ic_solve_fast_recovers_same_log_as_ic_solve` checks 16/20/24-bit
planted logs, auto B and explicit B=120, equal to `ic_solve`'s log.

## Speed (example `prime_ic_collection_frontier --bits N --fb B --e2e-b B`, one run)
| bits | B | sweep ic_solve s | ic_solve_fast s | speedup | rho 1thr s |
|--:|--:|--:|--:|--:|--:|
| 28 | 645 (fast auto 645) | 4.2949 | 0.0382 | 112x | 0.0048 |
| 32 | 1626 (fast auto 1625) | 65.5484 | 0.3464 | 189x | (see json) |

Same-run batched `ic_multi_solve_ex`: 100.5x (28), 191.7x (32); the wrapper is equivalent within
timing noise. Raw data: `run28.json`, `run32.json`.

## Caveats (honest)
- Single run per size, no A/A noise floor; speedups vs the O(p) sweep are large enough that the
  direction is not in doubt, the exact ratio is not tight.
- Even fast IC remains slower than matched 1-thread rho at these sizes (0.127x at 28 bits,
  0.026x at 32 bits): this is a speedup over the old IC default, not over rho.
- Sweep B chosen equal to the fast B for the comparison; sweep's own optimum was not tuned.
- Local base lacked the frontier PR's build repair; `src/cryptanalysis/f4_fp.rs` had duplicate
  `VisitReport` fall-degree fields and a duplicate `recording_degree_falls`; the duplicates were
  removed (pure deletion) so the crate compiles. This may conflict trivially with main's repair.
