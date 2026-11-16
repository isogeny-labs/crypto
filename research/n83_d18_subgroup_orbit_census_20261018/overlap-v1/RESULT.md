# Exact U/D18 overlap and mixed-support result

The independently audited census found **zero shared signed-Frobenius orbits** between the two exact primary N83 K=16,627 bases. U and D each contain 2,760,082 point rows; the union contains 33,254 signed-orbit columns and 5,520,164 distinct point rows. The producer identified each complete orbit by its least affine point and least x coordinate; the independent pass identified it by the SHA-256 digest of all 166 sorted affine points. Both complete passes agreed on zero overlap. The producer took 18.915 seconds wall (16.740 seconds process CPU, 59,785,216 bytes peak RSS on macOS); the independent pass took 18.486 seconds wall. The exact compressed archive SHA-256 values matched their retained S3 download receipts. The [machine receipt](overlap.json), [independent audit](audit.json), source, tests and hashes are retained here.

The exact distinct unordered point-multiset counts for mixed assignments use the disjoint U/D point sets and allow repeated points. Dividing these counts by the subgroup order gives the following **uniform-target support ratios**:

| Summands from U+D | Support / subgroup order | Same-base support / order at that arity |
|---|---:|---:|
| 2+2 | 6.00065438668 | 1.0001 (m=4) |
| 3+2 or 2+3 | 5,520,770.05407 | 552,078.2055 (m=5) |
| 3+3 | 5.07926303130e12 | 2.53963979682e11 (m=6) |
| 4+2 or 2+4 | 3.80944865367e12 | 2.53963979682e11 (m=6) |

The ratios count available point multisets, not distinct group sums or relations a solver can find. Even the 2+2 union bound on a uniform target saturates at one, so the table is not a measured hit probability or a sixfold runtime prediction. A mixed 2+2 relation uses a 33,254-column log system, twice the individual-base unknown count. That rank cost must be charged against any access benefit. The existing per-summand `constrain_frobenius_orbit_domains` circuit can assign two U and two D domains without increasing each slot's size; a checked mixed group-lift, positive control, natural-search and useful-rank panel have not yet been executed for this exact pair.

The archives are the exact S3-custodied U and D objects pinned in [the frozen protocol](PROTOCOL.md), verified again by compressed SHA-256. This census uses the already downloaded immutable archives and makes no S3 write. The earlier generic archive replays check curve arithmetic; this stage checks structure, coefficients, exact orbit identity and intersection. Synthetic tests reject duplicate labels and points, changed archive bytes and forged overlap counts, and explicitly enumerate small mixed multisets to check the support formula. `candidate_id: null` and `selected_best_total_runtime: null` remain unchanged. Natural relation yield, graph/linear algebra, individual-log verification and matched same-point rho remain required to select a total-runtime winner.
