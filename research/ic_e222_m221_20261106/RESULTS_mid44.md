# Mid-size panel at 44 to 45 bit subgroups, F = 4096

Extends `RESULTS_mid40.md`. One instance per family, F = 4096 (4098 relations), two matched-rho runs each, `--extra-relations 2`, table forced. Needed one solver change: `ic solve-prime-file` capped the factor base at 1024; the cap (`MAX_FB_SIZE` in `src/bin/icx/prime_external.rs`) is now 8192. The three runs used one binary (blake3 `c0918c74...de98`, recorded in `evidence/mid44/binary.json`). E and M ran H1 only; W is the cofactor-1 control. Data: `evidence/mid44/` (3 outputs, `MANIFEST.sha256`). Everything below is [pcc:unverified], one run per cell, producer-run.

| instance | n bits | relations | trials observed | trials predicted | obs / pred | IC (GAE, w=0) | IC (GAE, w=1) | matched rho (GAE, median of 2) | IC / rho (w=0) | wall time |
|---|---|---|---|---|---|---|---|---|---|---|
| E44 H1 | 45 | 4098 | 2.142B | 2.149B | 0.997 | 1.62e11 | 1.69e11 | 4.5e6 | 3.6e4 | 3.1 h |
| M44 H1 | 45 | 4098 | 2.156B | 2.149B | 1.003 | 1.63e11 | 1.70e11 | 5.6e6 | 2.9e4 | 3.1 h |
| W44 default | 44 | 4098 | 2.146B | 2.149B | 0.999 | 1.62e11 | 1.69e11 | 3.9e6 | 4.2e4 | 3.0 h |

- All three recovered the planted logarithm and every matched-rho walk ended at it.
- Observed over predicted trials is within 0.3% in all three, against a sampling spread of about 1.6% for 4098 relations. The E-curve H1 offset seen at 38 to 40 bits (mean 0.92 over four cells) is absent here (0.997), so it was sampling noise or an F-dependent effect, not shown to be a bias.
- The elimination at F = 4096 took 0.6 s (E: 647 ms, M: 588 ms), far below a dense F^3 count: the model's 4.7 F^2 elimination term stands, and elimination stays negligible against the trials.
- The matched rho is cheaper than index calculus in every cell, at w = 0 and w = 1 (IC rises 4 to 5% at w = 1): the distance is 2.9e4 to 4.2e4 times. The rho median comes from two runs that differ by up to 2.2x within an instance (W44: 2.4e6 and 5.4e6), so the last column carries that spread.
- Across the 34, 38, 40 and 44 bit panels, quadrupling F at a fixed n cuts the distance by about 4 and raising n raises it as sqrt(n) / F, as the model says.

Limits: one instance per family; two rho runs, unweighted GAE (rho is not repriced across the section 5a band); each F = 4096 run holds about 5.2 GB, so only two fit in 16 GB at once (the first W44 launch was killed for memory and rerun alone); wall time 3 hours each. The largest measured subgroup is now 44 to 45 bits against 218 to 220 native.
