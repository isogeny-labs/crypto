# Wall-clock A/B protocol: integrated F4/Groebner leads (frozen before data, 2026-11-16 23:06 UTC)

Host: 4-vCPU Intel Xeon @2.10GHz (AVX-512), cloud VM, no PMU, 15 GB RAM. Load avg 0.35 at start; ps showed no
cargo/rustc/bench processes. Only this agent runs. Integrated commit 929c01c601.

Binaries (sha256 recorded in measure/hashes.txt):
- N = scratchpad/integrated/bin (f4_fp_bench, koblitz_f4_kernel_bench, crossbred_bench)
- B = scratchpad/baseline/bin (HEAD before the leads)

## Pairs (arm A = "new", arm B = "old"; ratio = median(old)/median(new), >1 means new faster)
f4_fp_bench 3 large (one run = one trial; each case's f4_ms/solve_ms is already a median of 3):
- AA    : N defaults vs N defaults (noise floor)
- CROSS : N with F4_FP_PAR_OLD=1 F4_FP_MEM_OLD=1 KIC_M4RI_OLD=1 vs B (cross-binary check; expected ratio 1)
- PAR   : N defaults vs N F4_FP_PAR_OLD=1   (f4-parallel marginal, on top of f4-memory)
- MEM   : N defaults vs N F4_FP_MEM_OLD=1   (f4-memory marginal, on top of f4-parallel)
- ALL   : N defaults vs N all toggles        (integrated effect, same binary)
- TOTAL : N defaults vs B                    (integrated effect, cross binary)
Each at RAYON_NUM_THREADS = 1, 2, 4.

GF(2) (gf2-m4ri): koblitz_f4_kernel_bench 5 x and 5 sym (metric median_ms, a median of 5), crossbred_bench 9 11 13
(metrics: per-row "xb wall" and "F4 wall" for n=9 and n=13, plus process wall time). Pairs AA, M4RI (N defaults vs
N KIC_M4RI_OLD=1), CROSS (N KIC_M4RI_OLD=1 vs B), TOTAL (N vs B).
koblitz at RAYON_NUM_THREADS=1,2,4. crossbred (13 s/run, single-threaded brute force dominates) at 1 and 4 only,
CROSS/TOTAL at 4 only. This cut (no crossbred at 2 threads, no crossbred cross at 1) is made up front for the
60-minute budget.

## Order and trials
Blocks b = 1..8. In each block, for each thread count, for each pair, run A B B A (odd b) or B A A B (even b).
Pairs and thread counts are interleaved inside each block so drift hits all arms alike. 16 trials per arm for f4
and koblitz; crossbred gets 8 trials per arm (4 blocks) for budget. Trials are paired by position within the block
(first A with first B, second A with second B).

## Statistics
Per case, pair, thread count: median new, median old, ratio = med(old)/med(new), paired bootstrap 95% percentile CI
of the ratio of medians (resample trial pairs with replacement, 2000 resamples, seed 20261116).
Noise threshold for a case/thread: tau = max(|ln lo_AA|, |ln hi_AA|, |ln r_AA|) from the AA pair at the same case
and thread count. An effect is significant iff its CI excludes 1 AND |ln ratio| > tau.
CROSS is expected to be within noise; if it is not, the toggles do not restore baseline speed and that is reported.

## Fingerprints
Every f4 run: basis_fp, solve_fp, field_ops, solve_field_ops, steps, max_rows, max_cols, basis_len, solutions must
match the baseline-binary reference for the case. koblitz: rows, cols, rank, word_ops. crossbred: tables with wall
columns stripped must equal the baseline's. Any mismatch is reported and the run flagged.

## Outputs
measure/raw/*.txt (every run), measure/runs.jsonl (one line per run with timings), measure/rows.json (every
statistic row), measure/RESULTS.md (tables per lead, prediction vs outcome).
Predictions are the ones in leads/*/NOTES.md (pre-registered there).
