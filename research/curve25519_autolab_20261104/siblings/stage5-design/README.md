# After Stage 4: the band's table pricing, and why a distinguished-point walk would not help (design note)

Status: desk note with a check against committed records. No new run. Every number is `[pcc:unverified]`.

## 1. Is charging every stored point in the band intended?

Yes. AGENTS.md §5a, owner ruling of 2026-11-08: "The band prices the table oracles' counted per-step inserts". The index-calculus table oracles, `wwalk` included, record one `table_insert_attempts` per walk step behind the marker `counted:table_insert_attempts`. From that date the band adds this counter to `U` on every arm. Prime-field rho lists its per-step canonicalisation in `cost.unpriced` and is charged for it the same way. The sibling arm's `sibling_second_points` and `sibling_lift_products` are counted natives under the same rule.

What this means for the sibling line (Stage 4, j0b7-40, one `sib8-p512-aut3` solve):

| work | count | priced at w = 0 / 0.1 / 1 |
|---|---|---|
| group additions, one per sibling pair | 513,560 | 1 GAE each, at every w |
| second points | 512,506 | 0 / 0.1 / 1 GAE each |
| table inserts, one per stored point | 1,026,052 | 0 / 0.1 / 1 GAE each |
| lift products, one per parent | 64,195 | 0 / 0.1 / 1 GAE each |

- At w = 1 a sibling pair costs about 4 GAE for its 2 stored points: 1 addition, 1 second point and 2 inserts. The plain walk costs the same 2 GAE per point, so at w = 1 the sibling mechanism's advantage is gone by construction.
- The second point is already priced at its field work (1 squaring + 2 additions in bitop/v1). A cheaper second point would not change the band, which charges per operation, not per field operation.
- Repricing it at a fraction of a GAE would be an accounting change, which the success criterion does not count as progress.
- Even a free second point would leave the arm at about 1.45 times rho-cubic at w = 1: 2.12 M becomes 1.60 M against rho-cubic's 1.11 M at w = 1. The rule stands as the owner set it. Whether a table insert should cost a full group addition at w = 1 is the owner's call, not this note's.

## 2. Would a point-dependent walk with distinguished points help?

**The model.** The target walk's rows involve k unknowns: s1, s2 and log Q with tw = 2, so k = 3. It needs k independent collisions; rho needs one.
- A walk that stores every point and never merges needs about √(2k·r/A) stored points.
- rho needs about √(π·r/(2A)) steps.

**The model fits the Stage 4 records** (j0b7, p = 34/37/40, 540 measured runs per arm):

| arm | measured, stored points or steps / √r | model |
|---|---|---|
| tw2cj-lazy (A = 2) | 1.719 | √(2·3/2) = 1.732 |
| sib8-p512 (A = 2) | 1.659 | 1.732 |
| tw2cj-lazy-aut3 (A = 6) | 0.957 | √(2·3/6) = 1.000 |
| sib8-p512-aut3 (A = 6) | 0.963 | 1.000 |
| rho-cubic (A = 6) | 0.535 | √(π/12) = 0.512 |
| rho-neg (A = 2) | 0.988 | √(π/4) = 0.886 |

**A distinguished-point variant.** A point-dependent jump (r-adding on the orbit class) with distinguished-point storage would cut the inserts to about one per 2^d steps. But walks then merge, and getting k = 3 independent collisions costs the Kuhn–Struik multi-collision sum: about (1 + 1/2 + 3/8) = 1.875 times rho's single collision, on the same class graph.

- **Prediction:** about 1.9 × rho-cubic at every weight. That is worse than sib8-p512-aut3 at w = 0 (0.89 to 0.93) and about the same at w = 1 (1.8 to 1.9). So rho-cubic would be lower at every weight: a resolved loss instead of today's unresolved.
- **Siblings do not survive the change.** An x-only sibling is a dead end that is not stored unless it is distinguished, so it can collide only by chance.
- **Decision:** not built. The model and the records already place it, and a run would spend a protocol on an arm predicted to lose at every weight.

## 3. What the model says would move the band

The gap to rho is the collision count, not the cost per point:
- rho needs one collision.
- The C = 2 walk needs three, one for each of s1, s2 and log Q.
- With three collisions, the walk must store √3 × √2 / √(π/2) ≈ 1.95 times rho's steps before the cost per point is even considered.
- At w = 0 the sibling trick halves the cost per point, which is how the walk reached 0.9 times rho-cubic. At w = 1 no trick that stores a point per operation can.

What would change the picture: rows that pin log Q with fewer collisions than unknowns. Examples are seed logs known in advance (a precomputation, which a cost comparison must then charge) or relations that do not add unknowns. That is a change to the index-calculus construction itself, not to the walk, and it is the open question to put to the lead atlas, not something this note can settle.

## Files and evidence

- Counts: `../stage4-aut3/sessions/j0b7-40/records.jsonl`, one `sib8-p512-aut3` record's `work_native` (oracle_setup phase) and `native` (relations phase).
- Model check: the measured means above are recomputed from `../stage4-aut3/sessions/j0b7-{34,37,40}/records.jsonl`. Steps are `tw_steps` (IC arms) and `counters.walk_steps` (rho arms), each over √r.
