# DEVIATIONS and disclosures (E2-twin-target-amortization)

Written before the first data run; appended after the run if anything changes.

## Pre-freeze development (no cell of the frozen grid was run)
* Code was debugged on throw-away runs at 16-20 bits (curve 1 or 2, seeds 1-2, mu in {0.75, 1.5, 3}) and one S0 run at 26 bits;
  outputs are in `../../e2dev/` (scratchpad) and are NOT used anywhere.  One rho dev run at 22 bits, curve 1 (identical command to the
  frozen one, later re-run) was looked at before the freeze.  No 22/26/30-bit grid cell was run before FREEZE.sha256.
* Pre-freeze bugs fixed: GP copies by value on every function call (O(n) per lookup for a 3 MB array), so `e2_find` and `e2_verify` take
  their arrays by reference (`~`); `default(parisizemax)` moved out of the library; multi-line top-level blocks wrapped in braces.

## Implementation choices not spelled out in PREREGISTRATION.md (all frozen in the code before data)
1. Joint sorted array (A3/A4) is `vecsort` of the concatenation, priced n_J ceil(log2 n_J) comparisons with n_J = 2N^2 (frozen formula
   "n ceil(log2 n)" applied to the array actually sorted).  Probes in the joint array are the real probes of the second binary search.
   Consequently the frozen joint arm is slightly MORE expensive than two stand-alone arms in native work.  To show that this accounting
   choice does not decide the verdict, `e2_analyze.gp` also reports a best-case joint accounting ("epsBest", "savBest": two separate
   sorted arrays, stand-alone probe counts, shared scan only).  This is an extra, clearly secondary column; the gates use the frozen arm.
2. Target budget: T_max = ceil(6 W / yhat) with yhat = distinct keys / ((l-1)/2).  A target with several matching keys yields several relations;
   each verified match counts as one relation.  Verification (ellisoncurve, elladd, and R == ellmul(P, s0+k)) is excluded from all costs.
3. Start-up cost of the stream (double-and-add for s0 P, `#binary(s0)-1+hammingweight(s0)-1` GAE) is charged to each collection that starts a stream.
4. PC1 "two separate runs": the second separate run has the same deterministic scan, table and sort as the first, so its op counts are taken
   from the first run's phase counts and only its own collection (stream s0+100000) is measured.  The table is not rebuilt twice.
5. NC1 is run for the curve-1 runs only (table of curve 1, targets from curve 2, same number of targets as the E stream of that run).  Verification
   requires the stream point to lie on the table curve.
6. J0: the N^2 mixed pair steps and their Legendre tests are executed; the N(N-1) Legendre tests that the unpartitioned arm also applies to
   same-type pairs are added analytically (the same-type pairs are already executed in the partitioned build).
7. Hit-rate check NC2 uses distinct E-half abscissae / ((#E-1)/2); the stopping rule (stop at W relations) makes the |z| statistic mildly anti-conservative.
8. Cost accounting of rho: steps + init (3*bits+1 per attempt) + 16 branch-table set-up (16 (3 bits+1)) in GAE, comparisons (one per step) at w; scalar
   bookkeeping mod l is unpriced for rho and for the relation arms alike.
9. The mu column key in the analysis is rounded to multiples of 1/4 (0.75 -> 3/4); no effect on data.
10. Run order: L2/L1 (22 bits) and the two PC1-critical 22-bit cells (mu 3 and 0.75, curve 1, seed 1) are executed first, then the rest of the grid
    (order only; each cell is an independent process).  The grid queue skips cells whose raw file already holds a RESULT line.

## After-run deviations

* (2026-10-26, finalisation pass) `e2_analyze.gp` prints GP reals with a space before the exponent ("8.84716 e-5"), so the frozen `results.json`
  it wrote was invalid JSON.  The original is kept as `raw/results_gp_original.json`; `results.json` is the same file passed through
  `sed -E 's/([0-9]) e(-?[0-9]+)/\1e\2/g'` (formatting only, no value changed).  `results.csv`/`summary.csv` still contain the same artefact in a few cells.
* The task asked for raw outputs under `out/`; the frozen driver writes `raw/`.  `out` is a symlink to `raw` (no frozen file edited).
* Reproducibility check (not part of the grid): three grid cells (22b mu3 c1 s1; 26b mu1.5 c2 s2; 30b mu0.75 c1 s1), the 26-bit rho (c1) and the
  22-bit lemma control were re-run on 2026-10-26 into scratchpad/e2_repro; all non-timing lines and all RESULT columns except the nine wall-clock (`ms_*`)
  columns were identical to the 2026-10-21 files; rho and L1/L2 files were byte-identical.
* No code or protocol change after the freeze.  Cells dropped by the S0 rule: none (raw/dropped_cells.txt absent).  The 32-bit stretch cell ran (S0 projection 150 s).
* (2026-10-26, closing pass) A full end-to-end replication by the unmodified `run.sh` (copy of the directory in `replicate/`, started 16:37, finished 16:57 UTC, 2 parallel gp jobs, contended box) reproduced
  `results.csv`, `results.json`, `summary.csv` and `raw/analysis.out` byte-for-byte; all 63 `run_*.out`, 9 `rho_*.out`, `L1_L2_*.out` agree on every non-wall-clock line (checked with diff; only `*_ms` fields differ).
* Operation-count table added after the run: `opcounts.sh` (not part of the frozen set; reads only the RESULT lines of `raw/run_*.out`, writes `opcounts.csv`) and `raw_rho_opcounts.txt` (median rho steps from `raw/rho_*.out`).
  No value was recomputed; `run.sh` is unchanged because it is hash-frozen, so `opcounts.sh` is run after it.
* results.json additionally carries keys `opcounts` and `rho_opcounts` merged by `opcounts.sh` (the GP-written content is unchanged; byte-identity with the GP output holds for the file before this merge).
* (2026-10-26, replicate2 diagnosis) The independent replication in `replicate2/` was reported as "not reproduced" only because the check was made while its full `run.sh` copy
  (`run_replicate2.sh`, identical except `D=`) was still running.  It has since finished (`driver_full.log`: "run.sh finished").  Comparison after completion: `results.csv`,
  `summary.csv`, `raw/analysis.out` byte-identical; `L1_L2_b22.out`, `L1_L2_b26.out`, all 9 `rho_*.out` byte-identical; all 63 `run_*.out` agree on every field except the nine wall-clock
  columns (`ms_scan, ms_buildE, ms_buildEp, ms_sort, ms_collE, ms_collEp, ms_pc1, ms_nc1, ms_j0`) and the `*_ms`/`us_per_*`/`PROJ` timings in PHASE and S0 lines; `opcounts.sh` run on `replicate2/`
  gives byte-identical `opcounts.csv` and `raw_rho_opcounts.txt`.  `replicate2/results.json` as written by GP is invalid JSON for the known "x e-5" formatting artefact; after the same
  `sed` fix and `opcounts.sh` merge it is byte-identical to `results.json`.  Not compared by design: `raw/run_sh.log`, `raw/results_gp_original.json` (the latter equals the GP-written replicate2 file).
  All randomness in the scripts is seeded (`setrand` in e2_lib, e2_lemma, e2_rho, e2_analyze; the bootstrap and sampled-lemma streams too); no nondeterminism found, no script changed.
