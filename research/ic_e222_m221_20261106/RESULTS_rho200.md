# More instances and 200 matched-rho runs per instance

Extends `RESULTS_mid44_f16384.md`. All figures [pcc:unverified], producer-run. Rows come from `evidence/mid34`, `mid40`, `mid44` (F = 4096, `evidence/mid44/f8192`, `f16384`, `v3`); the analysis is `evidence/analyze_rho200.py`, its output is `evidence/rho200/distances.{json,txt}`.

## What was added

- **More rho runs.** `evidence/run_rho_only.py` runs the counted negation-map walk (the matched rho) 200 times on an instance without the index-calculus stage: `ic solve-prime-file` computes the matched rho after the IC stage, and here the IC stage gets a one-trial budget and ends `bounded_unknown`, so only the `matched_rho` block is kept. The walk seeds are the command's own, so the first runs equal the earlier two-run invocations'. 200 runs on 24 instances (seeds v1 and v2, subgroup orders of 34, 38, 40 and 44 bits, families E, M, W): every walk ended at the planted logarithm. Output in `evidence/rho200/`.
- **More instances.** A third seed (`e222m221-mid-v3`) at 44 bits and F = 8192: E and M with H1, W default, each with its own 200 rho runs (`evidence/mid44/v3/`). Together with seeds v1 (F = 4096) and v2 (F = 8192, plus F = 16384 for E) that is 10 runs with 4098 or more relations on 9 distinct 44-bit curves.
- A fourth seed (v4) was started and lost to a container restart; it was not rerun.

## Results

- **Rho reference.** One walk has a spread (standard deviation over mean) of about 0.5 (median 0.52 over the 26 rows), so the mean over 200 runs has a relative error of about 3.7%. The mean walk cost over sqrt(n) is 0.86 to 1.03 across the 24 instances (median 0.94). The two-run medians used before were within a factor of 8 of these means (the W44 pair, 3.1e5 and 2.5e6, against a mean of 3.8e6).
- **Trial law.** For the 10 runs with 4098 or more relations (3 at F = 4096, 6 at F = 8192, 1 at F = 16384), observed over predicted trials has mean 0.9996, standard deviation 0.0073, range 0.990 to 1.019. The sampling spread of 4098 to 16386 relations is 1.6% to 0.8%, so the scatter is inside it, and the E-curve offset seen at 38 to 40 bits (0.88 to 0.98 at 514 and 1026 relations) is absent.
- **Distance to the matched rho, both arms priced at w = 0, 0.1 and 1 GAE per unpriced operation** (cost(w) = adds + doubles + w times the arm's unpriced counters; the rho arm's per-run counters include about one canonicalisation per step, so the weight moves rho more than index calculus):

| F | runs | IC / rho at w = 0 (min, median, max) | at w = 0.1 (median) | at w = 1 (min, median, max) |
|---|---|---|---|---|
| 4096 | 3 | 4.0e4, 4.1e4, 4.1e4 | 3.8e4 | 2.1e4, 2.2e4, 2.2e4 |
| 8192 | 6 | 1.9e4, 2.0e4, 2.1e4 | 1.8e4 | 9.8e3, 1.0e4, 1.1e4 |
| 16384 | 1 | 9.6e3 | 8.8e3 | 5.2e3 |

- The matched rho is cheaper than index calculus in every one of the 26 rows at every weight of the band, so the order holds across it. Weighting both arms at w = 1 roughly halves the distance. Doubling F halves the distance (4.1e4, then 2.0e4, then 9.6e3 at w = 0), as the model's sqrt(n) / F says.
- Across the 34 to 44 bit rows the H1 configuration on E and M and the default on the cofactor-1 control W give the same distance at equal F within 8% (E44 and M44 at F = 8192: 1.9e4 to 2.1e4; W44: 2.0e4 to 2.1e4), as expected when the trial count depends on n and F only.

Limits: the 44-bit instances come from three generator seeds (some share an instance across seeds' families only by size, not by curve); 7 distinct 44-bit instances and one run per (instance, F); one run at F = 16384; the matched rho is the repository's counted negation-map walk, a reference for these instances, not a statement about rho in general; largest measured subgroup 44 bits against 218 to 220 native.
