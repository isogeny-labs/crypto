# L12: incremental prefix search (clause retention across N targets of one seed)

Workdir: scratchpad/L12 (copied to leads/L12). All numbers below are MEASURED from `raw/*.tsv` via `analyze.py` (full output `ANALYSIS.txt`) unless labelled derived/plan.

## Setup (measured tooling)
- Instances: the prime-PDP bit-blasted CNF (`bits-s3`, small-x factor base, lex symmetry) from the sweep emitter, patched (`patch_emitter.py`, `emitter/`) so the target x_R is variable bits; the target is supplied only by assumptions/units. One CNF per (b, l, m), seed 7, p = 3041 (b=12), p = 64433 (b=16). 256 targets per cell, strictly alternating planted / uniform.
- Solver: CaDiCaL 3.0.1 C++ API (`driver.cpp`, linked to /opt/homebrew/lib/libcadical.a). Three modes on the same CNF and same targets: `inc` (one solver, target as assumptions, all learned clauses kept), `fresh` (new solver per target, target as assumptions), `units` (new solver per target, target as unit clauses). Per-solve conflict cap.
- Correctness gate: every non-censored answer compared with the exhaustive oracle: 0 mismatches in all cells (column oracle_mismatch).
- Metric: CaDiCaL conflicts (and process CPU s) per target, per relation (= SAT target). Ratio R = inc/fresh over the first N targets (sum/sum), paired bootstrap 95% CI over targets, 2000 resamples. R < 1 means retention helps; speedup = 1/R. The first 16/64 targets of the 256-run are the 16/64 runs (inc is sequential, fresh independent), so N-scaling uses nested prefixes.
- Host load average 60-110 throughout: CPU seconds are noisy; conflicts are the primary metric.

## Deviations (stated)
- **l chosen**: m=3 at l=3 (|F|: 8 abscissae). l=2 has no planted targets and after one solve the retained solver refutes everything with 0 conflicts (degenerate); l=3 already exceeds 200,000 conflicts per target in fresh mode at b=12 (calibration, `raw/cal_b12_l3_inc.tsv`, 4/4 censored at 200k), so m=3 cells use a 20,000-conflict cap and fresh is **fully censored** (0/256 solved) at b=12 and b=16. So for m=3 the fresh conflicts/target equals the cap and R is an upper bound on the true ratio.
- **m=2 control added** (l=4, cap 100,000) because m=2 cells actually finish, giving uncensored-ish ratios at b=12, 16. This is a supplement, not the requested m=3.
- **b = 20, 24 dropped**: b=16 cells already took ~20 min each under load; not attempted.
- **N**: fresh/units runs did not reach 256 in the time cap in three cells; N_max is the shortest mode length (171, 83, 161). N=256 reached only for m3 b=12. Runs were killed at the compute cap.

## Results (measured)
Ratio inc/fresh on conflicts (95% CI); speedup = 1/R.

| cell | N=16 | N=64 | N=N_max | N_max | censored (fresh / inc) at N_max | slope dlog2R/dlog2N |
|---|---|---|---|---|---|---|
| m=2 b=12 l=4 (cap 100k) | 0.163 [0.055,0.318] | 0.055 [0.026,0.097] | 0.025 [0.014,0.040] | 171 | 6 / 0 | -0.80 |
| m=2 b=16 l=4 (cap 100k) | 0.948 [0.802,1.074] | 0.317 [0.213,0.421] | 0.262 [0.174,0.353] | 83 | 52 / 13 | -0.79 |
| m=3 b=12 l=3 (cap 20k) | 1.000 (all censored) | 0.766 [0.668,0.859] | 0.201 [0.153,0.248] | 256 | 256 / 47 | -0.58 |
| m=3 b=16 l=3 (cap 20k) | 1.000 | 1.000 | 1.000 (all censored in all modes) | 161 | 161 / 161 | 0 (uninformative) |

inc/units conflict ratios are within ~10% of inc/fresh in every cell (`ANALYSIS.txt`). CPU-time ratios are weaker than conflict ratios (assumption-mode solves cost more CPU per conflict, and load was heavy): m=3 b=12 N=256 inc/fresh cpu 0.419 [0.32,0.52]; m=2 b=12 N=171 0.041 [0.029,0.057].

Per-relation (m=3 b=12, N=256, measured): inc 209/256 solved, 106 relations found, 0.777 CPU-s per relation; fresh and units found 0 relations (all 256 censored at 20k conflicts), so their per-relation cost is unbounded here. m=2 b=12, N=171: inc 0.160 CPU-s/relation, fresh 4.19, units 3.01.

Learning curve (inc/fresh conflicts by target block, measured): m=2 b=12: 0.095 (0-32), 0.016 (32-64), 0.008 (64-128), 0.006 (128-171); m=3 b=12: 1.0, 0.53, 0.020, 0.009. Retained solver per-target cost falls to ~1e2-1e3 conflicts after the first 32-128 targets; the one-time cost is the first tens of targets.
UNSAT/SAT split: planted targets are SAT, uniform mostly UNSAT; mismatch-free. Retention helps both; per-class tables are in `raw/*.tsv` (column kind/res).

## Decision rule and verdict
Rule: ratio (speedup) >= 1.2x becomes the standard harness setting (constant_only); report whether retention changes the exponent.
- Speedup 1/R at N_max, conflicts: 40x (m2 b12), 3.8x (m2 b16), >=5x lower bound (m3 b12, fresh censored). CI lower bounds all exceed 1.2x. m=3 b=16 uncensored data does not exist (0 solved in any mode), so no m=3 ratio at b=16.
- **Verdict: constant_only** in the program's sense: adopt incremental/assumption mode as the harness default for fixed-seed relation collection (>=1.2x at N>=64). **Caveat, not a constant**: the speedup is not N-independent. log2 R falls with slope about -0.6 to -0.8 per log2 N over N=16..256, because the retained solver tabulates the fixed factor base (one-time cost ~ the first tens of targets, comparable to the per-target cost of a fresh solve times a few), after which each target costs hundreds of conflicts. This is the retained-setup/tabulation effect (cost moves from N*c to c_setup + N*c_small), not a reduction of c_setup in b. Evidence on b-dependence is thin: one-time cost grew with b (m2 b=12 speedup at N=64: 18x; b=16: 3.2x; first-32 block 0.095 vs 0.58), i.e. the amortisation window lengthens with b, consistent with setup scaling like the fresh per-target cost. Whether the exponent in b of cost per relation changes needs N growing with |F| (see next step); I do not claim it does not, nor that it does.
- Not shown: any subexponential behaviour; m=3 b>=16 not solvable in budget; only one seed per cell; CIs are over targets within one seed/curve, not over seeds.

## Skeptic re-run (one cell, ~2 minutes at no load)
```
cd scratchpad/L12 && E=emitter/target/release/prime_pdp_sat_sweep && $E emit --bits 12 --two-adicity 4 --seed 7 --l 4 --m 2 --nt 64 --out /tmp/x && for m in inc fresh; do ./driver $m /tmp/x/inst.cnf /tmp/x/targets.tsv 64 100000 | awk '{c+=$5}END{print "'$m'",c}'; done
```
Expected (measured): inc about 2.1e5 total conflicts vs fresh about 3.8e6 for N=64 (ratio about 0.055).

## Next step (plan)
Run inc vs fresh at fixed (b=12, m=3 or m=2) with N from 16 to >=8|F|... and several seeds/curves, with a larger conflict cap on fresh (>=500k) to uncensor m=3, and b=14, 16, 18 at m=2 l=3-4 to measure how the one-time cost and the saturated per-target cost scale with b; then judge against the rho_floor budget using setup/N + c_small per relation. Feed `inc` as default into the pdp-sat-synth relation-throughput objective.

## Files
driver.cpp, patch_emitter.py, emitter/ (patched Rust emitter), run_cell.sh, analyze.py, ANALYSIS.txt, raw/ (per-cell *.inc/fresh/units.tsv, inst.cnf, targets.tsv).
