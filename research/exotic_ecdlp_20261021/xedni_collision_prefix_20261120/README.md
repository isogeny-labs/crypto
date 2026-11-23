# A radius-ordered lift scan reaches its first useful collision at point 31

**On the fixed 43-point reduced group, 31 radius-ordered nonidentity rational point evaluations suffice to find a collision that determines the second generator coordinate.** The complete 42-point public input law is covered by prefix 69. In independent searches for one supplied target at a time, the exact stop count is 2–69 rational point evaluations, with median 31. Every derived scalar is checked against its supplied public point.

The [protocol](protocol.json) fixes the exact rational curve, its points `Q,R`, and the 80 nonidentity `aQ+bR` candidates from the earlier [box packet](../xedni_two_generator_lift_20261120/README.md). Candidates are ordered by increasing `max(|a|,|b|)`, then by `a,b`. Each candidate is constructed over `Q`, reduced at 47, and looked up by reduced point. The first equal-reduction pair is labels `(-1,-2)` at index 14 and `(-3,3)` at index 31. Their coefficient difference is `(-2,5)`, giving `-2G+5R̄=0` and `R̄=9G`. The previous [Lean theorem](../xedni_two_generator_relations_20261120/formal/CollisionDirection.lean) proves this exact implication in `ZMod 43`.

| Exact prefix measure | Result |
| --- | ---: |
| First generator-ratio collision | 31 rational point evaluations |
| Prefix covering all 42 nonidentity reduced targets | 69 evaluations |
| One-target stop count across frozen input law, min / median / max | 2 / 31 / 69 evaluations |
| Stops from a direct `b=0` generator multiple | 7 of 42 |
| Sum if all 42 targets are searched independently | 1,421 evaluations |
| Prefix peak coordinate size at one-target stop, median / max | 366 / 656 bits |

For a supplied target, a direct `b=0` lift yields its scalar immediately. Otherwise the search stops once both its first lift and the first collision have appeared; the scalar is then `a+9b (mod 43)` for that lift. The target point and the equality of reduced coordinates drive the stop. Its scalar is used only for the final independent point-multiplication check. [Every target row](target-rows.jsonl) retains first-hit, direct-hit if any, stop reason, exact evaluation/reduction/lookup counts, prefix coordinate peak, and checked scalar. The [80 ordered rows](prefix-rows.jsonl) and [result](result.json) fix the raw digests.

The protocol and accepted-Sage [runner](run_sage.py) were pushed before measurement in `95046614637`; `c07c51643c7` clarified a result-field name and added aggregate coordinate peaks before the successful final run. The [Sage runtime](sage-runtime-info.json) and empty stderr are retained. An [independent Fraction group-law audit](audit.py) reconstructs all 80 rational candidates and reductions, confirms the first collision and every target stop, and replays all 42 scalars over `F₄₇`; its [result](audit-result.json) passes.

These are exact point-evaluation and coordinate-size counts for one frozen construction. A point evaluation includes rational scalar arithmetic whose cost varies with coordinate size; the counts are not normalized group-operation counts or wall times. The rational curve, `Q`, and `R` are supplied setup inputs here, and their discovery cost is not in this prefix. A calibrated single-target study must charge or disclose that setup, include the full rational arithmetic and reduction work, and pair the same public target with rho under one resource envelope. A useful next experiment is an online ordering based on the rank-two height pairing, computed before target search, to see whether it lowers the 366/656-bit prefix peaks without paying to construct all 80 points first.

Replay from the repository root:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_collision_prefix_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_collision_prefix_20261120/run_sage.py > research/exotic_ecdlp_20261021/xedni_collision_prefix_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_collision_prefix_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_collision_prefix_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_collision_prefix_20261120/audit-result.json 2> research/exotic_ecdlp_20261021/xedni_collision_prefix_20261120/audit.stderr
```
