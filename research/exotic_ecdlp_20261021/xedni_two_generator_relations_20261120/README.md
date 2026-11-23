# Rational additions plus one reduction collision determine the finite relation system

**The 81-point rational lift box on one fixed rank-two curve yields a full-rank, normalized relation system over `F₄₃`.** On its 80 nonidentity rational-point columns, 1,792 exact rational addition rows have rank 78. Thirty-eight equal-reduction collision rows raise rank to 79, and fixing `log_G(G)=1` raises it to 80. Solving this matrix from its recorded rows produces scalar values for all 42 nonidentity points of the 43-point reduced group; each scalar is independently replayed on the public point.

The exact curve is `E/Q: y²+94xy+188y=x³+1738x+3805`, with `Q=(-4,19)` and `R=(52242961/137641,-1907191742616/51064811)`. Its reduction at 47 is `y²=x³+46x+45`, of order 43, with generator `G=(43,19)`. The [prior box packet](../xedni_two_generator_lift_20261120/README.md) constructs the 81 rational points `aQ+bR`, `-4≤a,b≤4`, and verifies that they reduce onto the complete finite group.

| Exact matrix stage | Rows included | Rank over `F₄₃` | Remaining dimensions on 80 columns |
| --- | ---: | ---: | ---: |
| Rational addition | 1,792 | 78 | 2 |
| Addition plus equal-reduction collision | 1,830 | 79 | 1 |
| Plus `log_G(G)=1` | 1,831 | 80 | 0 |

The first rank-increasing collision equates the distinct rational points labeled `(-1,1)` and `(1,-4)`. Their coefficient difference is `(-2,5)`, so reduction gives `-2G+5R̄=0`; over `F₄₃` this determines `R̄=9G`. Other collision rows are retained, including dependent ones. The [frozen protocol](protocol.json) and [Sage runner](run_sage.py) were pushed before measurement in `2cb8438dc71`. The complete 1,792 [addition specs](addition-rows.jsonl.gz), 38 [collision specs](collision-rows.jsonl.gz), [rank result](result.json), accepted [Sage runtime](sage-runtime-info.json), and stderr are retained. The gzip streams are deterministic and each is below the repository artifact threshold.

A separately frozen [solve protocol](solve-protocol.json) and [solver](solve_sage.py) were pushed in `19f8b91a903`; `fdd1fc019c3` corrected a copied collision-row digest before the successful solve. The [solve result](solve-result.json) records the 1,831-by-80 rank-80 matrix and 42 verified scalar outputs in [target rows](target-log-rows.jsonl). `R̄=9G` emerges as the solved value for label `(0,1)`; the solver constructs its equations from additions, equal-reduction checks, and the `Q` normalization. A [Fraction/F₄₇ audit](audit.py) independently reconstructs all rational points and sums, matches every row digest, performs separate sparse elimination over `F₄₃`, and replays all 42 scalar outputs by finite-field point addition. Its [result](audit-result.json) passes.

## Proof map and next cost gate

The pinned [Lean theorem](formal/CollisionDirection.lean) `first_collision_fixes_ratio` proves, for arbitrary `G,R : ZMod 43`, that the exact collision hypothesis `-2G+5R=0` entails `R=9G`. Its [build](formal/build-output.txt) reports only the standard `propext`, `Classical.choice`, and `Quot.sound` axioms under Lean `v4.35.0-rc4` and Mathlib `021c46bdd042f67b538b9f9871d556a93713a9a9`. The 1,792-by-80 matrix, its ranks, the rational curve maps and point equalities, and the finite-field scalar replay are computationally checked rather than formalized as Lean matrices and elliptic-curve objects; formalization of that full bridge remains open. `lean4checker --fresh` is unavailable on this host.

This result resolves the exact finite relation system and identifies which relation supplies its missing generator direction. Construction starts with the 81 rational lifts, then performs 1,792 rational sum checks, 38 equality checks after reduction, matrix construction and solving; those are recorded work counts, not calibrated wall or operation costs. A single-target end-to-end comparison needs a frozen online interval and a same-public-point rho run under matched resources. The next experiment is to count the minimum rational point and relation work required to obtain the first rank-increasing collision, then measure the complete single-target pipeline against that paired reference.

Replay from the repository root:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_two_generator_relations_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_two_generator_relations_20261120/run_sage.py > research/exotic_ecdlp_20261021/xedni_two_generator_relations_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_two_generator_relations_20261120/run.stderr
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_two_generator_relations_20261120/solve_sage.py > research/exotic_ecdlp_20261021/xedni_two_generator_relations_20261120/solve-result.json 2> research/exotic_ecdlp_20261021/xedni_two_generator_relations_20261120/solve.stderr
python3 research/exotic_ecdlp_20261021/xedni_two_generator_relations_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_two_generator_relations_20261120/audit-result.json 2> research/exotic_ecdlp_20261021/xedni_two_generator_relations_20261120/audit.stderr
sh research/exotic_ecdlp_20261021/xedni_two_generator_relations_20261120/formal/build.sh > research/exotic_ecdlp_20261021/xedni_two_generator_relations_20261120/formal/build-output.txt 2> research/exotic_ecdlp_20261021/xedni_two_generator_relations_20261120/formal/build.stderr
```
