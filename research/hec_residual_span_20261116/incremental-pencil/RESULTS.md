# Incremental pencil baseline: same relations and base-field work

The once-only run at pushed checkpoint `f5d84e08de0` exited 0. All 64 counted calls completed on the 32 retained single-target workloads across eight models of genera 2, 3 and 4. The old and new solvers agreed at all 998 accepted prefixes. Every scalar, determining outcome and repeated counter vector matched. The separate Rust checker compared all 128 modulus records with the retained original controls and found every base-field counter unchanged, not only REDC. These are local observations, not signed PCC verdicts. [pcc:unverified]

Ratios are old repeated solver / new incremental solver on subgroup arithmetic. Every source/target case is included. Base-field ratios are exactly one throughout.

| Genus | p | Subgroup REDC ratio | Subgroup inversion ratio |
|---:|---:|---:|---:|
| 2 | 101 | 0.871–1.156 | 3.077 |
| 2 | 251 | 0.925–6.033 | 10.483–27.182 |
| 2 | 601 | 1.034–1.581 | 27.222–31.694 |
| 3 | 113 | 1.010–1.176 | 2.800–2.867 |
| 3 | 421 | 1.364–1.982 | 12.757–13.718 |
| 4 | 101 | 1.751–2.350 | 12.000–12.040 |
| 4 | 149 | 2.056–2.741 | 18.027–19.351 |
| 4 | 197 | 2.245–3.042 | 23.021–25.188 |

Incremental elimination removes many repeated normalizations, but natural-order incremental pivots can use more REDCs than the old Markowitz pivots on small genus-2 cases. Neither implementation uniformly dominates every counter. Retain both controls; do not silently choose a favorable arm after measurement or turn fewer inversions into a weighted total without a measured model. The change does not lower base-field work and does not establish a tenfold whole-method improvement. [pcc:unverified]

The generator, setup, source normalization, source validation where present, residual checks and final scalar check are unchanged and charged. In counted calls the repeated reference solve is absent. A counter-disabled correctness gate runs both solvers on every prefix. Exact order certification and known-answer construction remain outside under the original public-metadata contract. For C4, the selected-model Gaussian qualification and general determinant proof limitation remain unchanged.

This is baseline engineering, not a new mathematical mechanism. It supplies a stronger available comparator for the next relation-generation or base-field arithmetic candidate. Because some REDC counters regress, it is an option rather than an unconditional replacement for every case. No scientific retry, input replacement, budget extension, arbitrary cross-modulus total, wall-time, external-fastest, matched-rho or leaderboard claim is made.

Sources, imports, original raw records, native dependencies and binary were frozen and pushed before execution. Raw outputs, prefix gates, exit status and hashes are retained in Git during the S3 outage. All returned scalars passed original-group checks; independent signed scientific admission remains separate.
