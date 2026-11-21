# Twenty-seven CRT representatives per eligible collision witness

The frozen height rule constructed and checked **16,200** rational curves from the 600 local witnesses on the six relation-eligible collision pairs. It found **zero** `P + 2Q = O` rational relations within this 27-representative box. All curves passed exact incidence, nonsingularity, and the prescribed reductions at 7, 11, and 47; no cap triggered. An independent Fraction audit and a complete Sage elliptic-curve group replay each checked all 16,200 rows and returned `PASS`. The maximum numerator or denominator bit length among the five coefficients was 25.

| Source index | Base local witnesses | Tested representatives | Rational relations |
| ---: | ---: | ---: | ---: |
| 273 | 168 | 4,536 | 0 |
| 315 | 168 | 4,536 | 0 |
| 1160 | 66 | 1,782 | 0 |
| 1202 | 66 | 1,782 | 0 |
| 1500 | 66 | 1,782 | 0 |
| 1542 | 66 | 1,782 | 0 |
| **Total** | **600** | **16,200** | **0** |

## Frozen construction and proof

The [protocol](protocol.json), [runner](run.py), and row packaging scripts were pushed as `b5654f87943` before execution. For each base coefficient triple `k`, collision prime `p`, and lexicographic `t∈{−1,0,1}³`, the rule uses `k' = k + 77p·t`. It includes the 600 centered representatives at `t=0` and 15,600 additional representatives. The six source pairs were selected using their already-computed relation at the fixed prime 47, so this is a conditional existence test of coefficient choices; that selection is recorded as analysis work.

The [Lean theorem](formal/Shift.lean) proves the residue-preservation mechanism: if two Weierstrass coefficient choices contain the same two points, their x difference is `p·u`, the first three coefficients differ by multiples of `p²`, and `u` remains nonzero under reduction, then `a4` has the same reduced value. Here `vₚ(x₁−x₂)=1` for each selected source pair, and `77p` contains `p²`. The other auxiliary prime and 47 divide the coefficient shifts while the relevant x differences are units, fixing their reconstructed `a4,a6` residues. The runner and independent audit check the complete coefficient tuple on every constructed curve.

| Result | Declaration or receipt | Exact conditions | Status |
| --- | --- | --- | --- |
| Difference of incidence equations | `XedniCollisionHeight.incidence_difference` | Two points on one Weierstrass equation over a commutative ring | Lean checked |
| Reduced `a4` under a square-prime shift | `XedniCollisionHeight.reduced_a4_of_square_shift` | Domain, nonzero `p`, `x₁−x₂=p·u`, ring map with `p↦0` and `u↦nonzero`, both incidence pairs, shifts of `a1,a2,a3` by `p²` multiples | Lean checked |
| Exact 27-shift census | [result](result.json), [Fraction audit](audit-result.json), [Sage replay](sage-audit-result.json) | Six source pairs, 600 base witnesses, all 27 shifts per witness | Complete, both audits PASS |
| Concrete localization bridge | Exact rational coefficient and reduction checks | Rationals integral at each auxiliary prime, with their finite-field reduction maps | Verified computationally; Lean instantiation open |

The pinned [build](formal/build.sh) uses Lean `v4.35.0-rc4` and Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9`. The [axiom output](formal/build-output.txt) lists only `propext`, `Classical.choice`, and `Quot.sound`; stderr is empty. The module has no `sorry`, `admit`, custom claim axiom, or native evaluation. `lean4checker --fresh` is unavailable on this host.

Every candidate status and exact coefficient tuple is retained in 12 ordered [candidate-row shards](candidate-rows.jsonl.gz.part-0000). The explicit zero-yield positive stream has one [shard](positive-rows.jsonl.gz.part-0000). The [package manifest](package-result.json) and [rebuild script](rebuild_rows.py) verify each shard, the compressed streams, and their uncompressed digests against the result receipt. The Python construction process recorded 1.284 seconds on the local host; this is an ordinary-host execution receipt, not an isolated performance comparison. The [Sage runtime](sage-runtime-info.json) identifies the accepted build used for the independent group-law replay.

## Replay and next proof step

```sh
python3 research/exotic_ecdlp_20261021/xedni_local_collision_crt_20261120/rebuild_artifacts.py
python3 research/exotic_ecdlp_20261021/xedni_collision_height27_20261120/rebuild_rows.py
python3 research/exotic_ecdlp_20261021/xedni_collision_height27_20261120/audit.py
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_collision_height27_20261120/sage_audit.py
sh research/exotic_ecdlp_20261021/xedni_collision_height27_20261120/formal/build.sh
```

The rational relation condition can now be expressed directly as a tangent equation in the coefficient variables. Solving that equation exactly for these six fixed point pairs is the next step; it can distinguish a height-box miss from an arithmetic obstruction across the entire `a1,a2,a3=47k` coefficient family.
