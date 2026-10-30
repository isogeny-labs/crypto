# RESULTS G1-sound-negation-map-rho-baseline

Directory D = /tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/G1-sound-negation-map-rho-baseline

Scope: scratchpad study in PARI/GP 2.15.4 only. No repository file touched, no benchmarks/ row, leaderboard change or PR. The separation lemma is not used. No claim about the
repository's index-calculus baseline is made. The "gap" below is distance remaining to the matched rho reference N1, not a pass/fail gate; nothing here is an improvement over N1.

Every number below is copied from `out/summary.txt`, `out/solve.csv`, `out/ratios.csv`, `out/opcounts.txt` / `out/opcounts.csv` or `results.json` (assembled from those). Preregistration is
`PREREGISTRATION.md` (frozen 2026-10-26 17:47 UTC, not rewritten). Deviations: `DEVIATIONS.md` (5 items; two cells hit the preregistered 840 s cap).

## 1. What ran

66 cells: 24 N1 cells (stage B), 12 control / sensitivity cells (stage A: n1sab, n1c2, n1r64), 30 K-mix / K-pure / A0 cells on the extension instances (stage C), plus the frozen E5
raw cells for instances 1 and 2 (read-only copies, hashed). 64 cells exited rc = 0 with all planned trials; `n1sab b36 i1` (12 of 20 trials) and `n1c2 b36 i1` (10 of 20) hit the cap (rc = 124); missing trials are missing, not imputed.
Trials: N1 240 / 240 / 180 / 120 / 104 at b = 24 / 28 / 32 / 36 / 40 (as planned). Wall time of the whole run: about 67 minutes (18:38 to 19:45 UTC) with 2 cells at a time on a
machine shared with other agents (secondary and contention-affected; see section 6).

## 2. Primary metric: counted operations (`out/opcounts.txt`)

Per walk step, pooled over b >= 28 and all instances, escapes and conversions included:

| arm | M / step | S / step | I / step | total steps (b >= 28) | native per step beyond SEL and DPT |
|---|---|---|---|---|---|
| N1 (negation, r = 32) | 2.047 | 1.031 | 1.016 | 131,915,962 | CAN 1, CYC 2 |
| A0 (no negation, r = 16) | 2.000 | 1.000 | 1.000 | 96,454,351 | none |
| K-mix | 21.499 | 11.999 | 1.000 | 76,188,339 | LAD 2.500 |
| K-pure | 21.504 | 12.002 | 1.000 | 103,007,317 | LAD 2.501 |

N1 escapes: about 0.0154 per step by the 2-cycle test and about 0.00024 per step by the Brent test (escape = 3 M + 2 S + 1 I). DP lookups and trail starts are about 1.3e-4 per N1 step.
Per-(arm, b) totals of steps, M, S, I, escapes, DPs, trails and DP collisions are in `out/opcounts.csv` (also in `results.json`).

Mean steps per trial and R = mean steps / sqrt(n/2) (mean of per-instance R), failures censored at the budget:

| b | N1 steps/trial | N1 R | A0 R | K-mix R | K-pure R |
|---|---|---|---|---|---|
| 24 | 3513.4 | 1.409 | 1.364 | 1.624 | 1.877 |
| 28 | 14323.8 | 1.446 | 1.423 | 1.468 | 2.013 |
| 32 | 55563.0 | 1.437 | 1.341 | 1.469 | 2.135 |
| 36 | 224219.0 | 1.506 | 1.335 | 1.433 | 2.050 |
| 40 | 880487.0 | 1.378 | 1.347 | 1.538 | 2.007 |

## 3. Controls

| control | result (from `out/summary.txt`) | verdict |
|---|---|---|
| PC1 N1 solve rate | 240/240, 240/240, 180/180, 120/120, 104/104 at b = 24..40; worst instance 1.000 everywhere; Wilson lower limits 0.984, 0.984, 0.979, 0.969, 0.964 | PASS |
| PC2 steps N1/A0, pooled b 28-40 | 0.748 [0.686, 0.814] (all four instances); 0.756 [0.693, 0.826] (E5 instances); per b: 0.731, 0.719, 0.759, 0.797, 0.719 | PASS (point in [0.71, 0.80]) |
| NC1 n1sab (no cycle test) | 2/30, 1/30, 0/30, 0/12 at b = 24, 28, 32, 36 against N1 1.000; Wilson upper 0.213, 0.167, 0.114, 0.243 | detector fires at every b, including b >= 32 as required |
| NC1 n1c2 (2-cycle test only) | 30/30, 30/30, 29/30, 1/10 at b = 24..36 | fires at b36 only (b32: 0.967, no drop of 0.20) |
| NC2 split-half null (odd / even trials, geometric mean over 4 instances) | 0.965, 0.972, 0.968, 0.989, 0.872 at b = 24..40 | within [0.9, 1.1] at b = 24..36; b40 is 0.872, OUTSIDE the preregistered band (descriptive check, 20 trials per half-instance at b40 on instances 1, 2 and 6 on instances 3, 4, so noisy; not a gate for H1, but reported as a miss) |
| independent stats check K-mix/A0 steps | 0.767 [0.702, 0.842] (E5 value 0.787) | consistent |
| N1 r = 64 sensitivity | 40/40 (b36), 24/24 (b40) solved; steps r64/r32 = 0.827 (b36), 1.042 (b40), 2 instances each, no CI | solve criterion met; "steps within +-10%" met at b40, MISSED at b36 |

## 4. Gap: K arms against the matched N1 (pooled b 28-40; headline = 4 instances per b, co-primary = E5 instances 1, 2; 95% CI, instance-level then trial-level bootstrap, B = 2000, setrand(31337))

Steps: K-mix/N1 = 1.025 [0.944, 1.118]; K-pure/N1 = 1.422 [1.286, 1.568]; K-mix/A0 = 0.767 [0.702, 0.842]; K-pure/A0 = 1.063 [0.973, 1.167].

Cost ratio K-mix / N1 (M + 0.8 S + (I/M) I, native work priced as stated; ratio > 1 means N1 is cheaper). Format: point [95% CI] for the headline, then the E5-only lower limit.

| I/M | w = 0 | w = 0.1 GAE (native set B) | w = 1 GAE (native set B) | w = 1 GAE (set A: LOOK + START only) |
|---|---|---|---|---|
| 3 | 5.907 [5.438, 6.438]; e5 lo 5.392 | 4.268 [3.929, 4.652] | 1.768 [1.627, 1.927] | 5.902 [5.433, 6.432] |
| 10 | 3.235 [2.977, 3.525]; e5 lo 2.953 | 2.473 [2.276, 2.695] | 1.314 [1.209, 1.432] | 3.232 [2.975, 3.523] |
| 30 | 1.879 [1.730, 2.048]; e5 lo 1.715 | 1.564 [1.439, 1.704] | 1.084 [0.998, 1.182]; e5 [0.990, 1.211] | 1.878 [1.729, 2.047] |
| 100 | 1.287 [1.185, 1.403]; e5 lo 1.175 | 1.167 [1.074, 1.272] | 0.984 [0.906, 1.073]; e5 [0.899, 1.100] | 1.287 [1.185, 1.403] |

K-pure / N1 at w = 0: 8.193 [7.410, 9.035], 4.486 [4.058, 4.947], 2.606 [2.357, 2.873], 1.785 [1.615, 1.968] at I/M = 3, 10, 30, 100; at w = 1 (set B): 2.451, 1.822, 1.504, 1.365 (all lower limits above 1).
S = 1.0 M instead of 0.8 M changes K-mix/N1 at w = 0 to 6.110, 3.370, 1.941, 1.308 (rows "w0,S=1.0" in `out/summary.txt`). Per b, K-mix/N1 at I/M = 100, w = 0: 1.440, 1.275, 1.285, 1.195, 1.402 at b = 24..40 (lower limits 1.272, 1.120, 1.126, 1.012, 1.117).

Ranking by AGENTS.md 5a (order must hold across w = 0, 0.1, 1 and sets A and B): N1 is cheaper than K-mix at I/M = 3 and 10 across the whole band (CI above 1 in every column).
At I/M = 30 and 100 the w = 1 set B column has a CI straddling 1 (point 1.084 and 0.984; the lower limit is below 1 for both instance sets) while the other columns put N1 ahead, so those two cells are UNRESOLVED, not a K-mix win.
There is no cell where the upper CI limit is below 1: no verified improvement of K-mix (or K-pure) over N1 anywhere in the band. Distance remaining at the largest I/M with native work at 0: K-mix costs 1.29x the matched rho (N1), K-pure 1.79x.

## 5. Preregistered predictions assessed

| prediction | observed | assessment |
|---|---|---|
| G1-H1: N1 solves >= 95% at every b, no instance < 90%, steps ~0.8 of A0, sabotage visibly worse | PC1, PC2 (0.748), NC1 as above | CONFIRMED (NC2 b40 and r64 b36 are minor misses of side checks) |
| N1 R = 1.55 (1.3-1.8) | 1.378 to 1.506 by b | in band |
| steps N1/A0 0.80 (0.72-0.92) | 0.748 [0.686, 0.814] | in band (pooled point; per-b 0.719 at b28 and b40 is just below 0.72) |
| steps K-mix/N1 0.98 (0.85-1.12); K-pure/N1 1.35 (1.15-1.6) | 1.025; 1.422 | both in band |
| counted M, S, I per step N1: 2.00-2.08, 1.00-1.05, 1.00-1.05 | 2.047, 1.031, 1.016 | in band |
| K per step 21.5 M, 12.0 S, 1.0 I within 3% | 21.499, 11.999, 1.000 (K-mix); 21.504, 12.002, 1.000 (K-pure) | in band |
| n1sab solve < 0.50 at b32 and b36 | 0/30 and 0/12 | confirmed (b24 was also low, 2/30, not "may stay high") |
| n1c2 solve < 0.95 at b36 | 1/10 (10 of 20 trials completed) | confirmed, on a reduced sample |
| N1 r = 64 solves >= 0.95, steps within +-10% of r = 32 | solves 64/64; ratios 0.827 and 1.042 | solve part confirmed, step part half missed (b36) |
| G1-H2 cost K-mix/N1, w = 0: 5.6, 3.1, 1.8, 1.25; CI lower > 1 at all four | 5.907, 3.235, 1.879, 1.287; lower limits 5.438, 2.977, 1.730, 1.185 (E5-only 5.392, 2.953, 1.715, 1.175) | CONFIRMED (falsifier not triggered; observed ratios are 3 to 6% above the point predictions, matching the observed step ratio 1.025 vs predicted 0.98) |
| cost K-mix/N1, w = 0.1 (B): 4.0, 2.4, 1.5, 1.1 | 4.268, 2.473, 1.564, 1.167 | same direction, 3 to 7% higher |
| cost K-mix/N1, w = 1 (B): 1.7, 1.25, 1.04, 0.94 | 1.768, 1.314, 1.084, 0.984 | same direction, 4 to 5% higher |
| K-pure/N1, w = 0: ~7.9, 4.2, 2.5, 1.7 | 8.193, 4.486, 2.606, 1.785 | same direction, 4 to 7% higher |
| G1-H3: ratio falls with weight; K-mix lower limit below 1 only at w = 1 (I/M = 100, possibly 30), reported unresolved | w = 1 (B): lower limits 0.998 / 0.990 at I/M = 30 and 0.906 / 0.899 at I/M = 100; upper limits 1.182 / 1.211 and 1.073 / 1.100 | CONFIRMED, cells reported unresolved by 5a |
| no verified improvement (upper CI < 1) at any w | none | CONFIRMED |

Overall: prediction CONFIRMED. The sound negation-map rho (N1) is a sound baseline here (all trials verified), saves about 25% of A0's steps, and the x-only Kummer arms do not beat it in counted work at w = 0 for any I/M in {3, 10, 30, 100}; the ratio shrinks monotonically with I/M (5.9, 3.2, 1.9, 1.3) toward the step ratio 1.025 without reaching 1.
The E5 comparison against A0 (K-mix/A0 steps 0.767) overstated K-mix because it did not include the +-fold for the reference; against N1 K-mix is 2.5% above in steps and about 1.29x above in counted cost at I/M = 100.

## 6. Secondary: wall time (contention-affected, not a cost metric)

Summed cell wall time from `out/opcounts.txt` (ms): n1 604,275; n1r64 532,545; n1sab 207,841; n1c2 94,588 (G1 cells). The K and A0 sums (kmix 2,460,210; kpure 2,724,509; a0 514,379) mix E5-era cells with the extension cells run here and are not comparable across arms.
These timings come from a machine shared with up to about 20 agents and 4 cores with two cells at a time; use operation counts.

## 7. Limitations

- NC2 at b40 outside the preregistered band (0.872) and r64/r32 steps at b36 (0.827) outside +-10%: reported, not explained away; both are small-sample side checks with no CI.
- NC1 b36 rows rest on 12 (n1sab) and 10 (n1c2) completed trials.
- Only 4 instances per b (2 E5 + 2 extension), so instance-level CIs rest on 4 clusters per b.
- Unpriced native work (selector, DP test, canonicalisation, cycle compares, ladder branches, DP lookups, trail starts) is a lower bound: a trail start is really one scalar multiplication, counted as 1.
- Group-operation counts of K arms are x-only ladder operations (xDBL 3 M + 2 S, xADD 4 M + 2 S, conversion 1 I + 1 M) from E5 code; N1 and A0 are affine adds (2 M + 1 S + 1 I).

## 8. Reproduce

Full (about 67 min contended): `bash D/run.sh` (checks hashes of frozen files first, then stages B, A, C, the frozen analysis, `opcounts_g1.gp`, `make_results_g1.gp`).
Reduced (about 5 minutes): `bash D/repro_quick.sh` re-runs 7 cells and checks they equal `raw/` (apart from timer fields), re-runs the frozen analysis and both addenda on the existing raw data in a scratch copy, and checks `out/summary.txt`, `out/solve.csv`, `out/ratios.csv`, `out/opcounts.*` and `results.json` byte for byte.
Output hashes: `out/FINAL_SHA256.txt`. Machine-readable results: `results.json`.
