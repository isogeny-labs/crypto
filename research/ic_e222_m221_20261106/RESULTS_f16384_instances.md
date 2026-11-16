# F = 16384 on six 44-bit instances

Extends `RESULTS_rho200.md` and `RESULTS_mid44_f16384.md`. All figures [pcc:unverified], one run per cell, producer-run. Data: `evidence/mid44/f16384/` (the E44 run of seed v2 from before, `v2b/` for v2 M and W, `v3/` for seed v3 E, M, W), analysis `evidence/analyze_rho200.py` (output `evidence/rho200/distances_f16384.{json,txt}`), peak memory samples `evidence/mid44/f16384/rss_samples_f16384.log`.

## What ran

Six 44-bit instances (seeds v2 and v3, families E, M, W) at F = 16384 (16386 relations): E and M with the H1 cofactor projection, W (cofactor 1) in its default configuration. Each is the same curve and target as an F = 8192 row of `RESULTS_rho200.md`, so each pair differs only in F. Each has the 200-run matched rho of that instance (`evidence/rho200/` for seed v2; the run's own 200 runs for seed v3). The runs went one at a time: the peak resident memory sampled over the chain was 11.8 GB, so two do not fit in 16 GB.

## Results

- All six recovered the planted logarithm; every matched-rho walk ended at it.
- **Trial law.** Observed over predicted trials: 0.999, 1.004, 1.010, 1.016, 1.003, 0.984 (mean 1.003, standard deviation 0.010, against a sampling spread of about 0.8% for 16386 relations).
- **Doubling F halves the cost.** For each instance, the F = 16384 run over its F = 8192 run: trials 0.494 to 0.506, index-calculus GAE 0.497 to 0.509, distance to the matched rho 0.497 to 0.509.
- **Distance to the matched rho (200-run mean), both arms priced across the band:**

| weight w | IC / rho, minimum | median | maximum |
|---|---|---|---|
| 0 | 9.4e3 | 1.0e4 | 1.1e4 |
| 0.1 | 8.6e3 | 9.2e3 | 9.9e3 |
| 1 | 5.1e3 | 5.5e3 | 5.9e3 |

The matched rho is cheaper than index calculus in all six rows at every weight, so the order holds across the band. Pricing the unpriced work at w = 1 adds 8.8% to the index-calculus cost and about doubles the rho walk's (which counts about one canonicalisation per step), so the distance falls to about 55% of its w = 0 value.
- The H1 runs on E and M and the default runs on W land within 16% of one another at w = 0 (9.4e3 to 1.1e4), as expected when the trial count depends on n and F only.

Limits: six 44-bit instances, one run per instance at this F; the rho reference is the repository's counted negation-map walk; the largest measured subgroup is 44 bits against 218 to 220 native; the memory figure is the maximum over the chain's 30-second samples of the largest `ic` process, not a kernel peak; independent replay, validator, pcc claims, benchmark record and EKS run are not done.
