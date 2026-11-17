# DEVIATIONS from PREREGISTRATION.md (written before the stage-0/1/2 production runs; appended afterwards if more arise)

D1 Output directory. The task text asks for raw outputs under `out/`; the preregistration commands write `raw/`.
   `run.sh` writes `raw/` exactly as frozen and creates the symlink `out -> raw`.
D2 Development runs. The three scripts were debugged on throw-away runs (outputs in /tmp/e1dev, not kept as results) BEFORE
   `raw/script_hashes.txt` was written. The seeds were the production seeds, so those runs are numerically identical to the
   production ones. Fixes made during development: (a) the pair table is held in globals (copying a 1e5-entry vector per lookup made
   a target cost seconds); (b) the closed-resultant formula check (`rescheck`) compares against the formal-degree-2 Sylvester
   determinant, not `polresultant` (the latter uses the actual degree and differs when a leading coefficient vanishes);
   (c) trailing semicolons / braces so that gp does not echo definitions into the TSV streams. No protocol quantity changed.
   Observed during development (disclosed because it influenced D4): the literal NC2 test (Wilson interval vs 1-2^-m) failed in
   13 of 16 cells in the dev run (deterministic, so the production run repeats it).
D3 `aggregate` reads `raw/grid_*.out` (tab-separated lines tagged CFG/TGT/END), not `raw/*.tsv`.
D4 NC2 evaluation. The preregistered criterion (Wilson 95% on pooled solution counts must contain 1-2^-m) assumes independent
   solutions and a balanced V; neither holds (solutions are clustered per target; realised nE/n differs from 1/2). The literal
   criterion is therefore still computed and reported (columns `literal_contains_1-2^-m`) AND is the formal gate: if it fails,
   the formal outcome is "inconclusive: F6 gate unmet". In addition, and clearly labelled as a post-hoc diagnostic,
   per-target counts are printed (`NC2T` lines) and a cluster (target) bootstrap interval is computed against the conditional null
   1-(nE/n)^m, plus the maximal absolute deviation of the observed mixed fraction from that null. Nothing in the
   gp stage-1 run halts on NC2 (the bootstrap needs the aggregation step), so the grid is run even if the NC2 literal gate fails;
   this violates the literal stop rule "stage 1 failure halts everything" and is done deliberately so that all other numbers exist.
   Their evidential status is then conditional on the reader accepting that the NC2 gate is mis-specified.
D5 Choices the preregistration left open (fixed in e1_census.gp before the run):
   - PC1/PC2/PC3: base V = random factor base with n = ceil(q^0.5), bits in {12,16}, all curves (PC1 only n_T>=1, one block per
     rational 2-torsion point T), target = first target of the curve's sequence, 50 trials per block.
   - NC1: V = x(kG), k=1..100 (all E-class), bits {12,16}, 20 targets, m=2,3 with Detector A, Oracle B and Oracle G;
     NC1big: V = x(kG) with n = min(ceil(q^0.65),1500), Detector A only (mixed must be 0).
   - NC2: curve 0 of bits {12,16}, alpha 0.65 (n=ceil(q^0.65)), all four FBs, 200 targets (m=2) / 60 targets (m=3); Semaev
     polynomials of b+1 (b+2 if singular), classes by the original f.
   - NC3: bits {12,16}, all four curves, FB random and twistmult, alpha 0.65, targets on the twist, 200 (m=2) / 100 (m=3).
   - Extra stage-0 block V0-TOY (not preregistered): V = all of F_q for q in {101,103}, 4 curves, 20 targets, m=2,3, with A, B, G
     and S (saturated regime where mixed zeros are plentiful). It is part of the stage-0 acceptance (any disagreement fails stage 0).
   - twistmult uses floor(n/2) multiples on each side; Farey uses u>=0, w>=1.
D6 Executed by one agent directly (not via the director/Haiku relay): the executing session wrote the scripts and ran the
   frozen commands itself.
D7 Verification session (2026-10-26). The production run (stages 0-3, 2026-10-21) had finished but RESULTS.md was missing. The
   reporting session did not edit any hashed script and did not change any number. It (a) confirmed `raw/script_hashes.txt` still
   matches `sha256sum e1_census.gp e1_aggregate.gp plot_e1.py`; (b) re-ran the complete frozen pipeline (run.sh with D changed,
   plot line dropped) in ../E1-replication; `replication_check.sh` / `replication_check.txt` show every non-timing number
   bit-identical (summary.tsv, pooled.tsv, verdict.txt, results.json, all 224 grid files' TGT lines, stage0/stage1 outputs);
   (c) added `extra_stats.sh` / `extra_stats.txt`, a descriptive re-tabulation of summary.tsv (no new data; not preregistered).
   The title of fig_mixed_vs_predictor is clipped at the right edge ("... per con"); cosmetic, left as produced.
D8 Closing session (2026-10-26, Sonnet 5.5, single agent; no director/Haiku relay available in this session). Did not recompute any protocol quantity.
   (a) Corrected RESULTS.md positive-control totals: PC2 is 400 of 400 and PC3 800 of 800 (an earlier draft said 600 and 1200); source `controls_summary.tsv`.
   (b) Added operation-count metrics (`opcounts.sh`, `opcounts_summary.sh`, `opcount_verify.gp`; not preregistered, descriptive; counts by construction, verified exactly on 3
   instrumented configs). `opcount_verify.gp` wraps `pkeys` and `tabfind` of the hashed `e1_census.gp` without editing it (run as `gp -q e1_census.gp opcount_verify.gp`).
   (c) Added `validate_results.sh` (traceability recomputation), `run_reduced.sh` (reduced replication), and stage 4 plus env knobs `E1_DIR`, `E1_BITS_LIST`, `E1_REDUCED`
   to `run.sh`; the frozen commands of stages 0 to 3 are unchanged (default behaviour identical). `results.json` is deliberately not extended, so that it stays bit-identical
   to the replication run; operation counts live in `opcounts_summary.json`.
D9 Reduced replication (2026-10-26). `run_reduced.sh` (stages 0, 1 and the 12-bit grid into `reduced/`) was executed once under heavy contention (about 20 agents on 4 cores):
   34.5 min wall but 14.9 min user CPU (`reduced_run_attempt1.log`). Stages 0 and 1 were byte-identical to the original. In that attempt 1 of 64 grid jobs
   (12_1_1_m3_interval, alpha 0.8) hit the preregistered 60 s time guard (table build 4.6 s versus 0.96 s in the original run) and stopped at 93 of 100 targets; its 93 completed TGT
   lines were identical to the original. That job was re-run with the same command and `E1_GUARD_S=1000` (`reduced/raw/grid_12_1_1_m3_interval.out`; the truncated output is kept as
   `.truncated_attempt1`): all 100 TGT lines identical. Therefore `run.sh` now takes `E1_GUARD_S_OVERRIDE` (default 60, so the full run is unchanged) and `run_reduced.sh` sets it to
   1000; `reduced_compare.sh` ignores timings and the guard column (CFG field 24) and fails on any truncated config. Its result on the final `reduced/` tree is `reduced_compare.txt`.
   Lesson recorded: the 60 s guard is a wall-time rule, so under contention a full re-run can truncate configs; the original run had 0 truncated.
D10 Diagnosis of the second replication (replicate2/, 2026-10-26 evening; Sonnet 5.5). Verdict: incomplete replication run, plus one latent nondeterminism source fixed. No scientific number changed.
   (a) The replicate2/ run was submitted while still in progress and was cut off: `replicate2/reduced/raw/` holds only a partial `stage0_oracle.out` (1783 of 47341 bytes, a byte-exact prefix of the
   original), no stage 1, no grid file, no `summary.tsv`; no gp process remained. VERIFICATION.json therefore records "replication incomplete", not a differing number. Its completed checks
   (validate_results.sh ALL_TRACEABILITY_CHECKS_PASS, byte-identical opcounts.tsv / opcounts_summary.json, instrumented check pkeys_calls_table_build=1378, pkeys_calls=5200, distinct_solutions=779)
   all matched. Together with the earlier complete run (D7, `replication_check.txt`: 224 grid files, summary/pooled/verdict/results.json identical) and the reduced run (D9, `reduced_compare.txt`),
   there is no evidence of any discrepancy in a scientific number.
   (b) Seeds: every random draw in `e1_census.gp` follows an explicit `setrand` (S0, W, W+k per stage); `vecsort`/`Set`/`Map` use are order-independent for the outputs. The only run-to-run
   nondeterminism is the wall-clock time guard (E1_GUARD_S=60, preregistered section 8/9): under CPU contention a config can stop before its last targets (seen once in D9; the original run had 0 truncated). It does not
   alter any completed TGT line, only how many targets are completed.
   Fix in `run.sh` (not a hashed script; stages and numbers unchanged): the default of `E1_GUARD_S_OVERRIDE` is now 1000 instead of 60, so reproductions are insensitive to contention (an untruncated run is
   identical to a 60 s run; the guard value is not in summary.tsv). The preregistered value 60 stays in PREREGISTRATION.md and in the original run's CFG lines (field 24). Also added knobs `E1_JOBS` (xargs
   parallelism, default 2) and `E1_PAR01` (run stages 0 and 1 concurrently, both must still pass before stage 2; default sequential), and `E1_RED_DIR` for `run_reduced.sh` / `reduced_compare.sh` so a replication can
   write to a separate directory instead of overwriting `reduced/`. `run.sh` before this edit is kept at scratchpad/run.sh.before_D10.
