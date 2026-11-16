# Prime-order control: `--extra-relations` retune (local, unvalidated)

**Status:** unvalidated (pre-gate). Local unisolated host; no replay certificate, validator report or matched-rho band. Not a leaderboard row. Metric: `trials_total` (deterministic given seed), fb = 8 fixed so the per-trial cost is identical across arms.

**What this is.** A retune of an existing flag of the baseline, not a new factor-base design. The default `--extra-relations 64` makes the solver collect fb + 64 = 72 relations although only fb + 1 = 9 are needed to determine the 8 factor-base logs and the target. The measurement asks how much of the baseline cost is that surplus.

**Design (pre-registered in the session before the runs).** 48 fresh W (cofactor 1, prime-order) instances from the holdout generator seed `e222m221-holdout-v2`, rungs 10 to 17, solver seeds 1..3, arms extra = 64 (baseline), 8, 2, 0. An arm counts only if it recovers the planted log at least as often as the baseline. Data: `results/extra/e{64,8,2,0}/rows.jsonl`, `results/extra/analysis.txt`.

| extra | verified / 144 | paired n | median trials ratio vs 64 (IQR) | median trials | median relations |
|---|---|---|---|---|---|
| 64 (baseline) | 144 | - | 1 | 6900 | 72 |
| 8 | 144 | 144 | 0.225 (0.199 to 0.265) | 1557.5 | 16 |
| 2 | 144 | 144 | 0.142 (0.115 to 0.167) | 933.5 | 10 |
| 0 | 130 | 130 | 0.112 (verified runs only) | - | 8 |

**Failure.** extra = 0 fails 14 of 144 (`bounded_unknown`, spread over rungs 10 to 17): 8 relations cannot determine 9 unknowns, so the system is rank-deficient whenever the pair sums are not independent. That arm is not an improvement. extra = 2 had 0 failures in 144 (upper 95% bound about 2%), which is too few runs to set a default; extra = 1 and 3 and a larger failure-rate sample are not yet measured.

**Cost accounting.** Collection dominates and falls with the trials. Elimination and extraction run on a 10 x 9 system instead of 72 x 9, so they also shrink; they are not separately timed here (trials are the metric). Construction of the factor base, lifting (none: prime-order) and verification (`[k]G = Q` check against the planted log) are identical across arms.

**Limits.** One producer (`solve-prime-file`), fb = 8, toy sizes r <= 2^18, local host. Other producers (tournament, prime-orbit) may already use small extra values, so this says nothing about the leaderboard baselines. It does not beat rho: at these sizes the ratio to matched rho stays well above 1 (`RESULTS_rho1.md`). The goal is not met.

## Fresh-holdout failure-rate sample (v3)

280 new W instances (generator seed `e222m221-holdout-v3`, rungs 10 to 16 by the generator's p bits, r up to 2^18), solver seeds 1..3, fb 8, 840 runs per arm, extra = 1, 2, 3 and the 64 baseline. Data: `results/extra_v3/e{1,2,3,64}/rows.jsonl`, `results/extra_v3/analysis.txt`.

| extra | verified / 840 | median trials ratio vs 64 | bootstrap 95% CI (median) | IQR |
|---|---|---|---|---|
| 64 (baseline, median 4542 trials) | 840 | 1 | - | - |
| 3 | 840 | 0.148 | 0.145 to 0.152 | 0.124 to 0.180 |
| 2 | 840 | 0.136 | 0.132 to 0.140 | 0.111 to 0.166 |
| 1 | 840 | 0.123 | 0.120 to 0.126 | 0.099 to 0.149 |

Zero failures in 840 for each of extra = 1, 2, 3 (rule-of-three 95% upper bound 0.36% per arm); extra = 0 failed 14/144 (about 10%) on the first sample, so one relation of surplus is the cliff edge and extra = 2 or 3 is the safer setting. Per-rung medians are flat (0.12 to 0.13 for extra = 1), so the saving does not depend on size within this range. Both holdouts agree (first sample extra = 2: 0.142).

**Reading.** On the `solve-prime-file` producer with fb = 8, the shipped default spends about 7 to 8 times the trials it needs. The retune is reproducible on two fresh holdouts and costs no recoveries. It is still a parameter retune of the existing solver, unvalidated and local, with no matched-rho band, and says nothing about a different producer or factor-base size. Not yet measured: fb 16 and 32 (the surplus may not matter as much when fb grows), wall time including elimination, and the validation-gate evidence. The goal is not met.

## Larger factor bases (fb 16 and 32)

70-instance subset of the v3 instances (rungs 10 to 16, 10 per rung), solver seeds 1..3 (210 runs per arm), extra = 64 vs 2, paired by instance and seed. Data: `results/fb/`, `results/fb/analysis.txt`.

| fb | verified (64 / 2) | median trials 64 -> 2 | median relations 64 -> 2 | ratio (bootstrap 95% CI) | (fb+2)/(fb+64) |
|---|---|---|---|---|---|
| 8 (v3, 840 runs) | 840 / 840 | 4542 -> about 620 | 72 -> 10 | 0.136 (0.132 to 0.140) | 0.135 |
| 16 | 210 / 210 | 1294 -> 277 | 80 -> 18 | 0.223 (0.216 to 0.228) | 0.225 |
| 32 | 210 / 210 | 448 -> 150 | 96 -> 34 | 0.359 (0.353 to 0.365) | 0.354 |

The measured ratio equals the relation-count ratio. The saving is exactly the surplus relations and shrinks to 1 as fb grows past the surplus (fb >> 64).

**Does it beat the baseline's best factor-base size?** Per-trial cost is about 2 scalar multiplications plus fb square roots, so trials x fb is a first proxy for the square-root count (an estimate across different instance sets, not a measurement): baseline 36,000 / 20,700 / 14,300 at fb 8 / 16 / 32; retuned extra = 2 about 5,000 / 4,400 / 4,800. The baseline's best fb is 32 or larger, and the retune at fb 8 to 32 is still about 3 times cheaper in that proxy. Wall time (with elimination, which also shrinks) and the baseline at fb 64 and above are not yet measured.

**Reading.** The improvement over the shipped default is real and reproducible on two fresh holdouts. Against a baseline tuned over fb it is smaller than the headline 0.14, and it vanishes for very large fb. It is a retune, unvalidated, with no matched-rho band; the goal is not met.

## Wall time, fully priced, against a baseline tuned over fb (the honest number)

30 further fresh W instances (v3, indices 10 to 19 at rungs 14 to 16), 2 solver seeds, fb 8 to 512, extra 64 vs 2, serial runs in interleaved seeded order (`run_timed.py`), local unisolated host. Metric: the solver's own `total_ms` (factor-base construction + relation collection + linear algebra + verification; no lifting for prime order; extraction is part of the linear-algebra stage). 960 runs, all verified (the fb 128 cells ran in both panels and are averaged). Data: `results/timed/`, `results/timed_ext/`, `analysis.txt` in each.

| fb | baseline ms | retuned (extra 2) ms | ratio | retuned linear algebra ms |
|---|---|---|---|---|
| 8 | 677.2 | 95.0 | 0.140 | 0.01 |
| 16 | 270.4 | 57.1 | 0.211 | 0.02 |
| 32 | 122.6 | 47.8 | 0.390 | 0.04 |
| 64 | 67.4 | 35.3 | 0.524 | 0.09 |
| 128 | 53.4 | 33.5 | 0.627 | 0.41 |
| 256 | 45.4 | 32.5 | 0.715 | 1.76 |
| 512 | 76.5 | 65.6 | 0.858 | 7.28 |

The best fb is 256 for both (the optimum is bracketed: 512 is worse for both). **Best-vs-best paired ratio 0.828, bootstrap 95% CI 0.791 to 0.891 (n = 60 instance-seed pairs; seeds on one instance are correlated, so the interval is somewhat optimistic).** So against the shipped default at its own best size the retune saves about 17% of wall time, not the 86% seen at the default fb 8, and the lever disappears at large fb as linear algebra (cubic in fb) takes over.

**Caveats.** The best fb was chosen on the same 30 instances that it is then measured on (a mild optimism that affects both arms). The metric is the solver's own stage timer on a shared unisolated host, not an isolab-eks run. Toy sizes (r up to 2^18). No validation-gate evidence and no matched-rho band, so this is not a leaderboard row, and the gap to rho is unchanged in kind. This is the first number to quote for the retune; the trial ratios above are not.

**Goal status: not met.** What exists is a reproducible, modest (about 17%) wall-time gain from a parameter retune at toy size; nothing structural has been shown.
