# Auxiliary-prime residue census isolates two coefficient-independent local obstructions

For the two reduced-event pairs selected by the preceding coefficient-box experiments (source indices **353** and **395**), exhaustive residue enumeration finds no good-reduction Weierstrass curve through their lifted points that has both the prescribed auxiliary order and `P₁+2P₂=O` at 7. The same is true at 11 under the strict order-6 gate. Thus their rational relation is excluded for **every** rational coefficient choice whose reduction satisfies either of those local gates, regardless of the centered coefficient budget. This strengthens the earlier witnesses, which checked only their selected curves.

The screen covers all 42 ordered source pairs with `P₁+2P₂=O` over `F₄₇`. For each pair and auxiliary prime, it enumerates every residue tuple `(a₁,a₂,a₃,a₄,a₆)` satisfying both lifted-point equations, then checks smoothness, two-row incidence rank, exact curve order, and the local point sum. The 7 and 11 residue choices are independent in this necessary-condition screen. They are not asserted to lift to a common rational coefficient tuple.

| Local gate | Coefficient residues through 42 pairs | Passing residues | Passing residues with local relation | Pairs with a passing local relation |
| --- | ---: | ---: | ---: | ---: |
| `#E(F₇)=4` | 14,406 | 156 | 52 | 26 |
| `#E(F₁₁)=6` | 55,902 | 392 | 36 | 18 |
| `#E(F₁₁)=7` | 55,902 | 588 | 112 | 38 |

Fourteen source pairs have relation-compatible residues at both 7 and strict-order 11; twenty-two do at both 7 and weak-order 11. This gives concrete survivors for a CRT coefficient-construction experiment. The two selected pairs have zero relation-compatible residues at 7 and strict-order 11. At weak-order 11 they each have two local relation-compatible residues, but the order-4 gate at 7 still excludes their relation.

The [protocol](protocol.json) fixes source hash, local coefficient space, gates, and row schema before enumeration. [Result metadata](result.json) stores source, code, raw-row, compressed-row, and complete candidate-stream hashes. The [42 per-pair rows](pair-rows.jsonl.gz) retain gate counts and first witnesses. The [independent audit](audit.py) enumerates the same coefficient space by four free coefficients rather than solving `a₄` first; it checks smoothness by the partial derivatives, curve order by a quadratic-character sum, and the relation by a direct point-doubling test. It matches every count, first witness, and candidate-stream digest. A separate [checked Sage replay](sage_replay.py) verifies all 206 first gate and first relation witnesses, including 82 relation witnesses; [runtime information](sage-runtime-info.json) was recorded before the Sage job. Both [audit](audit-result.json) and [Sage replay](sage-replay-result.json) passed with empty stderr.

The proof step is direct. For a rational curve with good reduction at an auxiliary prime, reduction is a group homomorphism on its rational points; a rational identity `P₁+2P₂=O` therefore gives the same local identity. The existing checked Lean declaration [`XedniObstruction.relation_reduces`](../xedni_adaptive_height_obstruction_20261120/formal/XedniObstruction.lean) formalizes this group-homomorphism implication. The exact finite residue enumeration, the concrete good-reduction map, and the local order computation are not yet formalized as a Lean elliptic-curve theorem; the missing bridge is a finite-field Weierstrass group model and kernel-checked enumeration for these particular residue gates. The current contribution is an independently replayed exhaustive arithmetic census and a conditional reduction argument, with the formal scope explicit.

Replay from this checkout:

```sh
python3 research/exotic_ecdlp_20261021/xedni_local_residue_20261120/run.py > research/exotic_ecdlp_20261021/xedni_local_residue_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_local_residue_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_local_residue_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_local_residue_20261120/audit-result.json
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_local_residue_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_local_residue_20261120/sage_replay.py > research/exotic_ecdlp_20261021/xedni_local_residue_20261120/sage-replay-result.json 2> research/exotic_ecdlp_20261021/xedni_local_residue_20261120/sage-replay.stderr
```
