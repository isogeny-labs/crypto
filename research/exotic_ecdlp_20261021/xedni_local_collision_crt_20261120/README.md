# Higher-precision CRT census on auxiliary x collisions

The frozen one-extra-digit CRT rule completed all **336** ordered pairs excluded by auxiliary x-coordinate collisions in the preceding full-law census. It processed **39,672** pairs of local gate witnesses, constructed **18,576** rational curves with the prescribed reductions, and found **zero** rational relations `P1 + 2P2 = O` among them. An independent exact-arithmetic audit reproduced every row and checked every constructed curve's point incidence, local reductions, and rational relation status.

## Frozen input and results

The [protocol](protocol.json) and [runner](run.py) were committed and pushed as `ce25e64b668` before execution. The input is the full 1,722-row ordered-pair source law from the preceding coefficient-choice study, with SHA-256 `9a594d96f369fb14f1a4cf6270b1c811ceda515e1c2a7eccab4d011ca8f3f322`. Selection uses only exact rational and auxiliary-prime x coordinates, never the recorded reduced-relation flag. It retains the 336 pairs with distinct rational x coordinates and x collision at 7 or 11; 42 same-rational-x pairs remain outside this construction.

For each selected pair, the collision-prime gate enumerates all compatible local Weierstrass coefficients, including the one-equation case when the two reductions are the same point. The other auxiliary prime uses the earlier distinct-x rule. Each local witness pair gets the centered CRT representative for `a1,a2,a3`, followed by one deterministic multiple-of-77 correction to the first effective coefficient to realize the collision-prime `a4` residue. The correction is derived from exact rational coefficients. All constructed curves also reduce to the fixed source model at 47.

| Collision at | Local point relation | Source pairs | Local witness pairs | Constructed curves | No extra-digit realization | Rational relations |
| ---: | --- | ---: | ---: | ---: | ---: | ---: |
| 7 | Same x, different y | 200 | 10,080 | 10,080 | 0 | 0 |
| 7 | Same point | 24 | 16,128 | 1,152 | 14,976 | 0 |
| 11 | Same x, different y | 104 | 6,864 | 6,864 | 0 | 0 |
| 11 | Same point | 8 | 6,600 | 480 | 6,120 | 0 |
| **Total** | | **336** | **39,672** | **18,576** | **21,096** | **0** |

Of the 336 input pairs, 140 have no local gate at one auxiliary prime; the other 196 are fully processed. Eight input pairs carry the source's reduced relation flag, which was consulted only after construction. The per-pair cap of 10,000, total cap of 300,000, and 300-second wall cap did not trigger. The Python process completed in 3.681 seconds on the local host; this is a construction receipt, not an isolated performance comparison. Every input status and local gate count is in [pair rows](pair-rows.jsonl.gz); every constructed curve and its exact coefficients is in the [candidate-row shards](candidate-rows.jsonl.gz.part-0000). The [result](result.json) binds their raw and compressed digests. The [positive-row file](positive-rows.jsonl.gz) is empty and retained as an explicit zero-yield artifact.

## Independent check and scope

The [independent audit](audit.py) enumerates local coefficients separately, counts local curve points by direct `(x,y)` enumeration, reconstructs the rational CRT curve from each coefficient triple, and evaluates `P1 + 2P2 = O` using an independent tangent formula. It matches the 39,672 local witness pairs, all 18,576 candidate rows, all 336 source statuses, and the zero relation count. Its [receipt](audit-result.json) is `PASS` with empty stderr.

The candidate-row gzip is retained as 17 ordered byte shards of at most 32 KiB each. This delivery amendment follows the frozen run: the artifact migration job received an S3 `403 Forbidden` while probing the new candidate-row object, and the trusted policy prevents changes to `.artifacts/config.json` in this PR. [Reconstruction](rebuild_artifacts.py) verifies both compressed and raw SHA-256 digests against the frozen result before writing the original gzip. The protocol, runner, measured rows, and their digests remain the original run artifacts.

The prior locally distinct-x rule found 8 relations among 32,256 constructed curves on a different source stratum. These exact rates describe their respective frozen rules; the strata have different local geometry, and this census does not establish a universal relation probability. The one-extra-digit rule also leaves the 42 same-rational-x pairs and alternative multi-coefficient or wider-height CRT policies open. This packet measures target-independent curve selection; a one-target ECDLP comparison would require target acquisition, relation extraction and linear algebra, full charged setup, and matched same-point rho.

## Replay

From this checkout:

```sh
python3 research/exotic_ecdlp_20261021/xedni_local_collision_crt_20261120/run.py > research/exotic_ecdlp_20261021/xedni_local_collision_crt_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_local_collision_crt_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_local_collision_crt_20261120/rebuild_artifacts.py
python3 research/exotic_ecdlp_20261021/xedni_local_collision_crt_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_local_collision_crt_20261120/audit-result.json 2> research/exotic_ecdlp_20261021/xedni_local_collision_crt_20261120/audit.stderr
```
