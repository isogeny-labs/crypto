# Mid-size panel above one machine word (subgroup order 34 to 35 bits)

Extends the 26 to 31 bit panel of `RESULTS_native_band.md`. The fields here are 36 to 37 bits (E, M) and 34 bits (W), so the point arithmetic is past 2^32; `ic solve-prime-file` ran unchanged (p up to 2^50 was already accepted). One instance per family, F = 128, 2 matched-rho runs, `--extra-relations 2`, table forced (`IC_SUM_TABLE=1`). Data: `evidence/mid34/` (5 outputs, `MANIFEST.sha256`; binary blake3 `f619a240...fd2f`). Everything below is [pcc:unverified], one run per cell, producer-run.

| instance | config | n bits | relations | trials observed | trials predicted | obs / pred | IC (GAE, w=0) | matched rho (GAE, median of 2) | IC / rho |
|---|---|---|---|---|---|---|---|---|---|
| E34-0 | default | 34 | 130 | 230.1M | 272.6M | 0.84 | 1.34e10 | 1.12e5 | 1.2e5 |
| E34-0 | H1 | 34 | 130 | 72.1M | 68.2M | 1.06 | 4.2e9 | 1.12e5 | 3.8e4 |
| M34-0 | default | 35 | 130 | 599.6M | 545.3M | 1.10 | 3.49e10 | 1.39e5 | 2.5e5 |
| M34-0 | H1 | 35 | 130 | 67.5M | 68.2M | 0.99 | 3.9e9 | 1.39e5 | 2.8e4 |
| W34-0 | default | 34 | 130 | 69.3M | n/a | n/a | 4.0e9 | 1.23e5 | 3.3e4 |

- All five runs recovered the planted logarithm; every matched-rho walk ended at it (eligibility: negation only, A = 2).
- Observed over predicted trials sits within the roughly 9% sampling spread of 130 relations plus one-run noise (0.84 to 1.10). The model's trial law holds past one machine word; no new term appears.
- The matched rho is cheaper than index calculus in every cell, at w = 0 and w = 1 (IC rises 5 to 6% at w = 1). The distance is 2.8e4 to 2.5e5 times.
- H1 cuts the cost 3.2x (E) and 8.9x (M) against the same curve's default, as the cofactor-projection model says.
- E34 default and H1 re-ran with identical trial counts (230097563 and 72132515) on a second invocation, so the runs are deterministic.

Limits: one instance per family, one F, 2 rho runs; the rho column is unweighted GAE, so the section 5a band is covered for the IC arm only and rho is not repriced. The largest measured subgroup is now 35 bits; native is 218 to 220.
