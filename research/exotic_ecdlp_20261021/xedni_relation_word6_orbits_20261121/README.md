# The coefficient-six relation word leaves seven log components

On the fixed order-43 curve \(E(\mathbb F_{47}):y^2=x^3-x-2\), the complete finite-field set of **all 42** relations \(P+6Q=\mathcal O\) gives a sign-folded 42×21 relation matrix of rank **14** over \(\mathbb F_{43}\). Its 21 columns split into seven disjoint three-column cycles under multiplication by \(-6\); each block has rank 2 and one free log scale. The resulting nullity is **7**, even when every finite-field pair is granted as a relation. A coefficient-six-only construction on this exact curve and full folded base therefore needs additional relation words or target-dependent bridges to determine logs across those components.

| Relation envelope | Rows | Folded columns | Cycle lengths | Block ranks | Rank | Nullity |
| --- | ---: | ---: | --- | --- | ---: | ---: |
| Every finite-field \(P+6Q=\mathcal O\) pair | 42 | 21 | Seven cycles of 3 | Seven blocks of 2 | **14** | **7** |

The [frozen protocol](protocol.json) fixes the source pair set, curve, exact word, sign quotient, all-pair upper envelope, and matrix boundary (`008f959653c`). The [result](result.json) retains the full column order, cycle members, block ranks, rank, nullity, and row digests; [compressed rows](rows.jsonl.gz) preserve every coefficient row. The 42 source indices match the earlier coefficient-six source-policy census. A separate [checked-Sage audit](sage_audit.py) independently enumerates the curve points, reconstructs all 42 rows, computes every cycle and block rank, and agrees exactly with the result. Its [receipt](sage-runtime-info.json) records the accepted runtime; audit stderr is empty.

## Structural proof and Lean map

Over \(\mathbb Z/43\mathbb Z\), multiplication by \(-6\) has order six, with \((-6)^3=-1\). The 42 nonzero scalars split into seven disjoint six-element orbits. Negation stays within each orbit, so each yields three sign-folded columns. A relation \(a+6b=0\) has \(a=-6b\) and stays in one orbit. Define \(v_s(x)=x\) on orbit \(s\) and zero elsewhere. Each \(v_s\) annihilates every relation, is odd under \(x\mapsto-x\), and the seven modes are linearly independent: evaluating a linear combination at each nonzero seed isolates its coefficient. Hence the folded relation system has at least seven independent null modes and rank at most 14. The exact block computation supplies the matching lower bound of 14.

The pinned [Lean module](formal/Orbit.lean) proves the multiplier order, seven-orbit partition, sign-folded three-column partition, relation-word implication for any additive abelian group, orbit preservation for scalar relations, and the masked-mode annihilation, oddness and independence theorem `XedniRelationWord6Orbits.seven_independent_relation_annihilators`. The actual [build output](formal/build-output.txt) records `#print axioms` for every reported declaration and lists only standard Lean foundations; build stderr is empty.

| Result | Lean declaration or receipt | Status |
| --- | --- | --- |
| \(P+6Q=0\Rightarrow P=-6Q\) | `relation_forces_neg_six` | Lean checked for any additive abelian group |
| Seven disjoint multiplier orbits and seven three-column folded components | `seven_orbits_cover`, `seven_orbits_disjoint`, `folded_orbit_card_three`, `folded_orbits_disjoint`, `folded_orbits_cover` | Lean checked for `ZMod 43` |
| Seven independent odd annihilators of every scalar coefficient-six relation | `seven_independent_relation_annihilators` | Lean checked for `ZMod 43` |
| Concrete curve-to-scalar identification and direct Lean matrix-rank theorem | Exact 42-row runner and checked-Sage audit | Formalization partial; these two Lean bridges remain open |

This is an upper envelope for **one relation word on one fixed subgroup**. The earlier rational-lift and local-bridge packets explain which of these rows their concrete constructions can supply; the upper envelope grants all rows regardless. The next proof step is to formalize the concrete curve-to-scalar isomorphism and sign-folded matrix, then state `Matrix.rank = 14` directly in Lean. The next construction step is to find a distinct relation word that connects the seven components and to charge its full selection cost.

## Replay

```sh
python3 research/exotic_ecdlp_20261021/xedni_relation_word6_orbits_20261121/run.py
/Volumes/SSD990/cryptanalysis/sage --runtime-info
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_relation_word6_orbits_20261121/sage_audit.py
sh research/exotic_ecdlp_20261021/xedni_relation_word6_orbits_20261121/formal/build.sh
```
