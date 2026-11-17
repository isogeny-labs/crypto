# Cached-relation supplied-target recovery passes

Execution checkpoint904767f2ed6; EXIT0. Fixed genus4/F19curve y²=x^9+2, ell8623, sourceD=13P0. Three independent known-answer single-target workloads rebuild relation/cache/log setup from scratch. Expected scalars are used only to construct public target inputs and for final comparison; recovery functions do not accept them. No diagnostic logtable, subgroup coordinate enumeration or BSGS is present in this solver.

For every case, the full2016candidate schedule replays126split rows,1890nonsplit rows,1881labels and9nonzero cached rows. Split-only rank1 becomes rank8. Homogeneous row elimination plus log(13P0)=1 recovers all9relative logs uniquely; all9computed point multiples reconstruct the projected public points. Gaussian log recovery uses1803counted mulmods per case. Original principal/collision identities and polynomial checks remain enabled. No setup or unused candidate tail is amortized across targets.

| case | recovered scalar | descent trials | descent additions | native scalar | native trials | native precompute ops | native group ops | native solve mulmods |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 37 | 11 | 10 | 37 | 469 | 1 | 470 | 584 |
| 1 | 2026 | 9 | 8 | 2026 | 366 | 1 | 367 | 630 |
| 2 | 7001 | 2 | 1 | 7001 | 519 | 1 | 520 | 460 |

Every descended rational decomposition equals the original R=T+aD in the Jacobian. Catalogue logs describe13P, so log(13R)=13(k+a); recover k=13^{-1}*sum(coeff*log(13P))-a. Every recovered scalar is verified by original [k]D=T. This explicit inverse13avoids a one-sided projection error.

The native control is ic::solve_dlp with fresh default setup, fixed declared seeds and max200000trials on exactly the same source/target/ell. All three controls complete. Its trials/group operations and candidate norm schedules/matrix multiplications are different work units and are not divided into a speed ratio. Validation group sums and reconstructions remain candidate work; a production benchmark cannot silently omit them or treat all norms as free.

Conclusion: the positive useful-rank case now yields an integrated relation-log/descent/scalar recovery path, with three source-verified controls and completing native comparisons. It does not establish a tenfold complete-cost gain, generic genus4behavior or genus2/3improvement. Short descent after setup is not an end-to-end comparison. The2016candidate setup and1881stored labels remain substantial costs; native scalar-elimination mulmod counts are lower in every listed control. No fastest external-tool, timing or benchmark-promotion claim.

Next technical requirement is a genuinely cheaper preparation mechanism or useful prefix, followed by a prospective complete accounting comparison. Retain this exhaustive integration as a correctness reference; do not justify a timing campaign solely from its short descent. Special-curve structure and relation dependencies need mathematical review before selecting a new mechanism. Earlier genus2zero-gain, initial span input failure, amended fixtures and generic/subgroup rank distinction remain preserved.
