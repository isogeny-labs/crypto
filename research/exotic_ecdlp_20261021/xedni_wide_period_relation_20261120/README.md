# Rational relations under the order-5/order-11 CRT policy

**The wide-period policy constructs 40 rational `P+2Q=O` records on four ordered input pairs, representing 20 distinct rational j-invariants.** The earlier order-4/order-6 policy found eight records on two ordered pairs. The new pairs, source indices 201 and 243, are sign mirrors; indices 425 and 467 are the previously witnessed sign-mirror pair. All four pairs satisfy the required reduced relation at 47. The new policy selects 13,440 coefficient-2 curves for rational checking from its full stream of 215,040 centered CRT curves.

| Stage or outcome | Exact count |
| --- | ---: |
| Full ordered point-pair law | 1,722 |
| Order-5/order-11 centered CRT curves | 215,040 |
| Locally selected coefficient-2 curves | 13,440 |
| Rational `P+2Q` checks | 13,440 |
| Rational relation records | 40 |
| Ordered input pairs with a rational relation | 4 |
| Distinct rational j-invariants among positive records | 20 |

The 40 records split by source pair as `201:8`, `243:8`, `425:12`, `467:12`. The [protocol](protocol.json) and [runner](run.py) were pushed before measurement in `21f16f13450`. Selection reads the frozen local coefficient rows and never uses the reduced relation flag. The runner reconstructs every rational curve and retains all 13,440 statuses in [compressed rational-check rows](rational-checks.jsonl.gz), with their raw digest and summary in the [result](result.json). The [independent arithmetic audit](audit.py) checks both rational point equations and every point sum by a separately written direct-doubling formula; its [result](audit-result.json) matches every status and the raw digest. A separate [checked-Sage replay](sage_replay.py) verifies all 40 positive group relations, each curve's orders 43 at 47, 5 at 7, and 11 at 11, and the corresponding local point sums. Its [result](sage-replay-result.json) passes, with the [runtime receipt](sage-runtime-info.json) retained and empty stderr.

One compact witness on the new source pair 201 is

`E: y²+94xy+188y=x³+1738x+3805`, `P=(8,19)`, `Q=(-4,19)`, with `P+2Q=O`.

The [pinned Lean theorem](formal/NewPairWitness.lean) `XedniWidePeriodWitness.rational_relation` checks this actual affine-point group relation over `ℚ`, including nonsingularity and both point equations. [Build output](formal/build-output.txt) reports only standard `propext`, `Classical.choice`, and `Quot.sound` axioms. Lean `v4.35.0-rc4` and Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9` are pinned. The 40-record census, three finite-field order checks, and distinct-j count are independently replayed arithmetic rather than Lean theorems. `lean4checker --fresh` is unavailable on this host.

The wider gates improve **rational relation construction** on this complete finite input law and centered representative policy. They also require 215,040 CRT constructions, 2,249,856 local coefficient-residue examinations, and 618,240 local relation tests before the 13,440 rational checks. For a fixed public generator, the coefficient-2 relation still applies only to target `T=-2G`; its reduced-group test is directly available. An unknown-target calculus needs a coefficient and relation-collection policy that recovers arbitrary targets, with all stages charged and paired with same-target rho. The next mathematical gate is to determine whether relation records for varied coefficients on these curves form an independent system rather than repeated certificates of the same reduced equality.

For a fresh checkout, [hydrate the sharded candidate and rational-check streams](../xedni_local_selector_20261120/REPLAY_STORAGE.md) before these frozen replay commands, then dehydrate the temporary whole-file copies afterward. The decompressed row digests are unchanged.

Replay:

```sh
python3 research/exotic_ecdlp_20261021/xedni_wide_period_relation_20261120/run.py > research/exotic_ecdlp_20261021/xedni_wide_period_relation_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_wide_period_relation_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_wide_period_relation_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_wide_period_relation_20261120/audit-result.json
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_wide_period_relation_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_wide_period_relation_20261120/sage_replay.py > research/exotic_ecdlp_20261021/xedni_wide_period_relation_20261120/sage-replay-result.json 2> research/exotic_ecdlp_20261021/xedni_wide_period_relation_20261120/sage-replay.stderr
sh research/exotic_ecdlp_20261021/xedni_wide_period_relation_20261120/formal/build.sh > research/exotic_ecdlp_20261021/xedni_wide_period_relation_20261120/formal/build-output.txt
```
