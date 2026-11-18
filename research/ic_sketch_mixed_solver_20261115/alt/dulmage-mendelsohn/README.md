# LANE dulmage-mendelsohn: Dulmage-Mendelsohn / block-triangular decomposition of IC relation matrices

All figures are exact counts from `raw_output.jsonl` (128 streams) or the summaries `summary.txt`; label **[pcc:unverified]**
(no pcc claim id, no replay certificate, no matched-rho band, not a benchmark-record row, planted synthetic rows only).

## Verdict
* Lever class: **native work only (w>0)**. It does **not** reach counted GAE at w=0. The DM arm stops at exactly the same row
  as incremental-gauss (slack 0 in 128/128 streams), so trials, relations and the |F|^2 oracle table are unchanged. [pcc:unverified]
* Q1: **no slack**. The target is pinned at the first prefix where the factor-base part B of the rows (target column deleted;
  the target is in every row) is linearly dependent, i.e. the first failed augmentation of an incremental maximum matching. That prefix
  equals the incremental-gauss pin prefix, and equals incremental-gauss-echelon's, in 128/128 streams. The rows actually needed are the
  unique circuit (= the horizontal DM block of B reached by the failed alternating search); it is minimal (every S-i matchable; S-i never pins
  in the library solver) and equals the coarse DM block recomputed from scratch in 128/128. [pcc:unverified]
* Q2: solving only that block cuts row_ops versus incremental-gauss everywhere (per stream 128/128, matching bookkeeping included), vs echelon in
  most cells, but the absolute saving is tiny against the p24 cost of ~56.6k GAE. Mixed-peel reads 2-4x more rows (its own full-rank rule), so it is
  not a fair arm at equal trials. Distance to matched rho (rho.negation ~5.0k GAE vs incumbent ~56.6k) is **unchanged**. [pcc:unverified]

## Method (`examples/ic_alt_dulmage_mendelsohn.rs`)
Generator copied from `examples/ic_mixed_solver_stream.rs` (identical row prefix; stream length 12*cols instead of 40*cols; no run exhausted it,
all arms solved and correct in 128/128). |F| in {64,128,256,512}, m in {2,3}, seeds 0..15; modulus 2^61-1.
* Library baselines run exactly as in the stream example (`matrix_by_name`; target-pinned stop for incremental-gauss and -echelon,
  full-rank-and-pinned stop for mixed-peel); row_ops from `work()`.
* DM arm: Kuhn augmenting-path maximum matching of the B rows (|F| columns), one augmentation per arriving row, every adjacency probe counted
  (`matching_probes`; a probe is an integer compare, not a multiply-subtract, but is charged as one native op, conservatively). First failed
  augmentation: reached rows S = circuit, reached columns N(S), asserted |N(S)| = |S|-1. Then the library solver runs on the block only (rows S restricted to
  N(S) plus the target column): `block_row_ops_ig` / `block_row_ops_echelon`. Total = block row_ops + probes. No SCC is needed by the solver:
  the failed search already yields the minimal block, so the SCC/fine decomposition is verification only (below, charged separately: `fineA_bookkeeping`).
* Verification per stream: slack vs library pin; circuit == coarse DM block (from-scratch matching, rows reachable from unmatched row);
  minimality (S-i independent for all i); S-i does not pin (every i when |S|<48, else about 24 evenly spaced i); block solve equals planted target.
* Fine DM (Tarjan SCC over the matched digraph) of the full matrix A including the target column: the SCC holding the target column has exactly
  as many rows and as many columns (target included) as the circuit has rows, in 128/128 streams. So the fine block of A containing the target is the circuit.

Commands:
```
cargo build --release --example ic_alt_dulmage_mendelsohn
./target/release/examples/ic_alt_dulmage_mendelsohn research/ic_sketch_mixed_solver_20261115/alt/dulmage-mendelsohn/raw_output.jsonl 16
python3 research/ic_sketch_mixed_solver_20261115/alt/dulmage-mendelsohn/summarize.py .../raw_output.jsonl > summary.txt
```
Runtime 4.9 s of CPU total.

## Results (means over 16 seeds; `summary.txt` has all columns)
| |F| | m | rows at pin (IG=ech=oracle) | circuit rows (min..max) | IG row_ops | ech row_ops | DM probes | DM block ops (IG solver) | DM block ops (ech solver) | DM total vs IG | DM total vs ech | mixed-peel rows / row_ops |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 64 | 2 | 41.9 | 10.2 (3..16) | 535 | 297 | 132 | 124 | 81 | 256 | 213 | 150.8 / 2335 |
| 64 | 3 | 60.1 | 45.4 (34..51) | 4955 | 2088 | 799 | 3341 | 1538 | 4140 | 2337 | 102.4 / 3848 |
| 128 | 2 | 82.8 | 12.8 (6..22) | 1040 | 583 | 271 | 170 | 106 | 441 | 377 | 330.9 / 5458 |
| 128 | 3 | 119.6 | 84.9 (69..93) | 24173 | 7824 | 2367 | 14697 | 5247 | 17064 | 7614 | 233.4 / 11099 |
| 256 | 2 | 152.2 | 17.4 (7..31) | 1967 | 1037 | 452 | 243 | 144 | 695 | 596 | 792.9 / 13277 |
| 256 | 3 | 237.5 | 165.1 (144..184) | 138168 | 32731 | 6848 | 76083 | 20627 | 82931 | 27475 | 493.6 / 33395 |
| 512 | 2 | 285.7 | 25.1 (11..65) | 3791 | 1910 | 809 | 373 | 207 | 1182 | 1016 | 1839.9 / 32746 |
| 512 | 3 | 472.1 | 331.1 (307..355) | 844833 | 157378 | 19847 | 461184 | 99084 | 481031 | 118931 | 1160.1 / 113956 |

(IG = incremental-gauss, ech = incremental-gauss-echelon.) Per-stream wins of DM total (probes included) over the baseline, out of 16: vs IG 16/16 in all 8 cells;
vs ech 14,3,15,7,16,14,16,16 for (64,2),(64,3),(128,2),(128,3),(256,2),(256,3),(512,2),(512,3); vs mixed-peel 16,16,16,16,16,15,16,6
(but mixed-peel reads far more rows, so that is not an equal-trial comparison). m=3 block of echelon-solver at |F|=64 and 128 loses to echelon in many streams
once probes are charged (3/16 and 7/16 wins).

## Cost in the study's units [pcc:unverified]
p24 incumbent ~16.5k + ~40.1k = ~56.6k GAE; the matching planted cell (|F|=128, m=2, 82.8 rows vs 87 real relations) has:
| w (GAE/op) | vs incremental-gauss | vs echelon |
|---|---|---|
| 0 | 0 GAE saved | 0 GAE saved |
| 0.1 | 599 ops * 0.1 = 59.9 GAE (0.11% of 56.6k) | 206 ops * 0.1 = 20.6 GAE (0.04%) |
| 1 | 599 GAE (1.06%) | 206 GAE (0.36%) |
Native work of the incumbent itself at this cell (IG 1040 ops) is already <=1.8% of 56.6k even at w=1, so no lever on native work can move the S gap of
56.6k vs 5.0k. For |F|=512, m=3 the saving is larger in ops (844,833 -> 481,031 vs IG; 157,378 -> 118,931 vs ech) but that cell is not a p24 configuration and
the |F|^2 table (262k group adds) dominates anyway. [pcc:unverified]

## Negative results and caveats
* Pinning needs rows only while B stays independent: the relations you must collect are fixed by B's first dependency, not by the solver; the DM view
  proves the incremental solver stops at that point (slack 0), so no solver change can reduce trials.
* An oracle with hindsight can only discard the rows outside the circuit (rows read minus circuit rows), which saves arithmetic, never trials.
  Not-in-circuit fraction at pin: rows 41.9 -> 10.2 in circuit (|F|=64,m=2) ... 472.1 -> 331.1 (|F|=512,m=3). m=2 circuits are 8-25 rows, m=3 circuits are ~70% of rows.
* Real p24 relations were **not** used: `sessions/partB-p24/records.jsonl` holds counts only (`counters/relations_found`, `matrix_rows`), not row supports;
  regenerating them needs ecbench, which this lane may not run. Planted rows have uniform columns; real prime-abscissa rows have skewed column frequencies
  (small primes common), which would shorten circuits, untested.
* Genericity: pin equivalence (e_t in row space iff B rows dependent) holds for generic values; 128/128 agreement, no exception seen (modulus 2^61-1).
* Probes are charged as full ops and the fine-DM SCC bookkeeping (`fineA_bookkeeping` in raw output) is outside the DM totals as it is verification only.
* The block-solve uses the library solvers (dense-ish, including the target column), not a dedicated sparse null-vector solve; a specialised solve of the
  (|S| x |S|-1) system could be cheaper, unmeasured.
