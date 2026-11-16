# DEVIATIONS and implementation notes (E5-kummer-multiplicative-walk)

Freeze point: FROZEN_AT.txt / FROZEN_SHA256.txt are written by run.sh immediately before the first computation of the final run.
Nothing below changes a verdict rule of PREREGISTRATION section 8.

## Before the freeze (development, not data)
- Scripts were smoke-tested with tiny trial counts (3-8 trials, b = 20, 24, 28, 36) and the check scripts were run once end-to-end;
  outputs are kept in dev/ and are NOT used anywhere. Bugs fixed before the freeze: GP `for(i=m,1,...)` descending loop in chainsearch
  (no iterations), `ellrandom` -> `random(E)` (gp 2.15.4), `default(parisize...)` inside a read file aborts the read, quadratic queues
  in the orbit BFS, a rejection-sampling hang for a degenerate S_idx. The dev run of check_exhaustive showed the PC1 S_idx walk
  success of 165/274 before the freeze; the protocol was not changed in response (the walk parameters d = max(3, ...), cap, budget are as preregistered).

## Deviations / additions (all fixed before the final run, listed for transparency)
1. Supplementary proper-H instance. Preregistered b = 24 instance 2 has n - 1 = 4 q (q prime), so S_idx = {k^g mod n} with g = smallest odd prime
   divisor of n - 1 = q generates a subgroup of order <= 4 (N = 2): the pc1/nc1/kmixidx cells are degenerate there (run_cell.gp
   skips any S_idx cell with N < 64 and prints `skipped_degenerate_N`). The preregistered text only skips instances WITHOUT an odd prime
   divisor, so this case was not covered. Added by rule: the first instance index >= 3 at b = 24 (same seed rule 1234567 + 1000*b + inst)
   whose smallest odd prime divisor of n - 1 is <= 13. Instance 3 (n - 1 = 2^7 * 97967) fails the rule, instance 4 is used
   (out/SUPPLEMENTARY.txt, raw/curves.txt line SUPPLEMENTARY_INST). pc1, nc1, kmixidx are run on instances 1, 2 (2 = skipped) and 4 with 40 trials.
2. xADD counted as 4 M + 2 S exactly as preregistered (Section 3), although the implemented ladder always has x(P-Q) = x(P) with Z = 1, for which
   3 M + 2 S would suffice. The code multiplies by the explicit Z_diff = 1 so the counter equals the executed multiplications. A labelled
   sensitivity row (CR3, "xADD = 3 M") is added to costs.csv; it is not used in any verdict.
3. Time per trial is recorded in milliseconds (getwalltime), not microseconds; secondary metric only.
4. K-mix and K-cross M_arm = N (the exact orbit size from H); for the SB set with +-H = F_n^* this equals (n-1)/2 as preregistered.
5. K-pure verification is x-only (`ellmul(P, a)[1] == x(Q)`), as preregistered; it fixes a up to sign (a vs n - a), the sign is resolved by one y comparison and is not part of the count.
6. A1: the doubling that escapes a fruitless 2-cycle is counted in the operation counters (3 M + 2 S + 1 I) but not in T (T counts walk steps); the
   count of escapes is recorded in the histogram field.
7. "Ratio of normalised pooled means" (step ratios, cost ratios) is implemented as sum over common good cells of (cell mean / sqrt(n_cell)) for the
   Kummer arm divided by the same sum for the additive arm, cells with b >= 28 (or the single listed b), good = +-H = F_n^* for the SB set. Stratified
   bootstrap resamples trials within every cell independently (2000 resamples, setrand(31337)); percentile 2.5 / 97.5.
8. Extra statistics beyond the preregistered list (all labelled as extra in summary.txt): slope against log2 n, step ratio A1/A0 and K-pure/K-mix,
   per-b ratios, PC4 for (32,36) and (36,40), the CR3 sensitivity, PC1 trail/DP diagnostics.
9. default(parisizemax,...) is set at the top of every top-level script, not inside lib.gp (it aborts a `read` otherwise).
10. Cell scheduling: controls (check_exhaustive, check_identity, chainsearch, pc1 cells) run before every main cell; the cells of one b are run
    two at a time with a barrier between bit lengths (so that the 200 s skip rule sees finished cells).

## Added after the main run had finished (data existed; none of these changes a preregistered verdict rule, but they change what is reported)
11. Supplementary arm a1c (lib_a1c.gp, run_cell_a1c.gp, run_supp.sh, FROZEN_SUPP_*). Reason: the preregistered A1 (negation map, only 2-cycles
    removed) turned out NOT to be a usable matched reference: its success rate collapses with b (b28: 114/120 solved, b32: 9/80, b36: 0/48, see
    summary.txt) because longer fruitless cycles trap trails until the cap. a1c = A1 with the cycle test extended to "x equals any of the last 8
    visited points"; same table, same seeds layout (armid 14), same grid (b = 20..40, both instances, same trial counts), run without the 200 s skip rule.
    a1c is a post-hoc, non-preregistered reference. Even a1c degrades at b = 36, 40 (cycles longer than 8 remain), so no sound measured
    negation-map rho exists in this study; the sound matched reference is A0.
12. analyze.gp v2 (analyze_frozen_v1.gp is the version run by run.sh at the end of the main run; its output is kept as
    out/summary_v1_frozen_analysis.txt, results_v1.json). Changes: (a) F5 is only evaluated when every trial of both arms at b = 32, 36, 40 was a verified solve
    (the preregistered text says "using only verified solves"; v1 compared K to the budget-censored A1 values and printed a spurious "improvement condition"
    line for kmix/a1); (b) arm a1c and the pairs K/a1c added; (c) isK() fixed for a1c; (d) JSON number format. Bootstrap numbers differ in the 3rd decimal between v1 and v2 only because
    the cell set (hence the random stream) changed when a1c cells were added.
13. a1 cells at b = 40 were SKIPPED by the preregistered 200 s rule (b36 took 429 s and 512 s); they are listed in out/MISSING.txt and were not run later (A1 had 0/48 solves at b = 36, so a b = 40 run could only be censored).
14. Nothing else was changed after the freeze: sha256sum -c FROZEN_SHA256.txt reports only analyze.gp (and run.sh, which now also calls run_supp.sh) as different.

## Added during the finalisation pass (2026-10-26, after the main run; no data was recomputed except the operation-count tables, which are pure sums of raw records)
15. Operation-count tables (primary cost metric requested for shared-machine runs): ops_table.gp sums steps, M, S, I and candidate tuples directly from the raw trial records
    (out/opcounts.csv per cell, out/opcounts_pooled.txt pooled over b >= 28, instances 1, 2, arms a0/kpure/kmix/kpureT); ops_ratio.gp turns the pooled totals into trial-weighted cost ratios
    (out/opcounts_ratios.txt); build_results_json.sh writes results.json = out/results.json + these counts. These pooled ratios are trial-count weighted, not the preregistered
    sqrt(n)-normalised ratios of RESULTS.md, so they differ in the third digit (e.g. kmix/a0 at I/M = 3: 4.733 vs 4.625); the preregistered ones remain the reported verdict numbers.
    No new experiment, no new verdict rule. Polynomial evaluations do not occur in this method (x-only Montgomery arithmetic).
16. Independent full replication already present in replicate/ (finished before this pass, 2026-10-26 17:28-18:07, same scripts): out/costs.csv and out/results.json are byte-identical to the original, the
    deterministic columns of out/summary.csv are identical, raw/curves, identity, exhaustive and chains are byte-identical; only wall-clock fields (us/step, per-trial ms, the 200 s skip message seconds) differ.
17. Reduced replication (run_reduced.sh + check_reduced.sh, output reduced_check.log, wall time in reduced_time.txt: 16m54s real at load average ~15 on 4 cores, user 7m20s): controls and 28 cells re-run;
    all 4 control files byte-identical, all 28 cell files identical after dropping wall-clock fields, all 28 operation-count rows identical. A first version of the comparison in run_reduced.sh did not
    strip the per-trial ms field and reported spurious DIFFs; check_reduced.sh strips it (the raw reduced run was not repeated). ops_table.gp initially appended to an existing opcounts.csv on a re-run
    (duplicate rows from a syntax-error first attempt); it now deletes the file first and the CSV was regenerated; the pooled printout was unaffected (byte-identical before/after).

## Added during the replication-diagnosis pass (2026-10-27; no scientific number recomputed in the original dirs; PREREGISTRATION.md untouched)
18. Determinism fix of the cell driver. Diagnosis of the failed independent replication (replicate2/, VERIFICATION.json "replication"): the only scientific-content difference was that
    replicate2 skipped a1 b36 i1/i2, kpure b40 i2 and kpureT b40 i2 because the preregistered "skip if the prior cell of the same arm took > 200 s" rule (PREREGISTRATION 9) reads WALL time, which depends on machine
    contention (replicate2 ran at load ~15 beside other jobs: prior cells took 539/533/225/218 s; the original took 199/195/191/164 s for those and skipped only a1 b40). The per-cell hard limit (timeout 840 s)
    had the same wall-time dependence. Both are execution-environment nondeterminism, not randomness in the experiment (every cell is seeded by setrand from b, instance and arm id).
    Fix (rule semantics and thresholds unchanged, outcome pinned to the original run): cellrun.sh now skips exactly the cells listed in skip.list (a1 b40 i1, a1 b40 i2: the cells the original run skipped after
    a1 b36 took 429 s / 512 s) and no longer reads wall time; the timeout is CELL_TIMEOUT (default 3000 s, a safety net; no original cell hit 840 s). run.sh and run_supp.sh gained D (directory), JOBS (parallel cells,
    default 2) and REPLICATE (do not overwrite FROZEN_*) environment variables, run all cells in one pool longest-first (the per-b barrier existed only for the wall-time skip rule), and run_supp.sh runs a1c in one pool.
    Output is independent of JOBS. Old scripts: cellrun.sh.orig_e5, run.sh.orig_e5, run_supp.sh.orig_e5 in the scratchpad. compare_full.sh compares a finished replica with the original (timing fields dropped).
19. The replicate2 "reduced replication fail=1 (DIFF cell_nc1_b24_i1.txt)" was a race in the replicator, not a result difference: replicate2/check_reduced.sh compares reduced/raw/ with replicate2/raw/, and the concurrently running full run
    rewrote replicate2/raw/cell_nc1_b24_i1.txt from 19:22:31 to 19:25:40, while the check ran at 19:23:12 (reading a partial file). After the full run finished, reduced/raw/cell_nc1_b24_i1.txt and replicate2/raw/ and the original raw/
    agree on all 50 lines once the per-trial ms field is dropped. Do not run a reduced check while a full run writes the directory it compares against.
20. Timing of the fixed full reproduction (fresh directories, idle 4-core machine, JOBS=4, REPLICATE=1, D=<dir>): replicate3/ (serial controls first): 1542 s wall, replicate4/ (PARALLEL_CONTROLS=1: the three control scripts and the a1c supplement
    run beside the cell pool; order of independent seeded jobs does not change output): 1180 s wall (19.7 min). Both were compared with compare_full.sh: 89 of 89 cell files identical after dropping wall-clock fields,
    curves/identity/chains/exhaustive byte-identical, results.json, out/results.json, out/costs.csv, out/opcounts*.{csv,txt,json} byte-identical, summary.csv identical without meanT/us_per_step, MISSING set = {a1 b40 i1, a1 b40 i2}, fail=0.
