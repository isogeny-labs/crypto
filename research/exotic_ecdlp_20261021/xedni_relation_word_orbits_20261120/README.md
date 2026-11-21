# The `P+2Q` relation word has three disconnected log components

On the fixed order-43 curve `E(F₄₇): y²=x³−x−2`, even an **optimistic upper envelope containing every finite-field `P+2Q=O` pair** gives a sign-folded relation matrix of rank **18 across 21 columns**. The 42 available rows split into three seven-column doubling cycles; each block has rank 6 and leaves one free log scale. A single known generator log fixes the scale of its component, while the other two components require another relation word or an explicit target-dependent bridge. Python exact group arithmetic and an independent accepted-Sage point-group and matrix replay agree on all rows, cycles, and ranks.

| Relation source | Rows | Sign-folded columns | Cycle lengths | Block ranks | Total rank | Nullity |
| --- | ---: | ---: | --- | --- | ---: | ---: |
| Verified rational `k3`-shift family | 264 | 21 | One supported row in one cycle | 1 | 1 | 20 |
| Every finite-field `P+2Q=O` pair | 42 | 21 | 7, 7, 7 | 6, 6, 6 | **18** | **3** |

The second row is a ceiling for this exact finite-field curve, full sign-folded base, and relation word: it grants all 42 finite-field relations regardless of whether a rational lift, local gate, or coefficient choice realizes them. The first row is the [measured construction](../xedni_relation_rank_20261120/README.md). This distinction makes the remaining rank deficit visible without extrapolating the eight rational-lift classes into unmeasured rows.

## Mathematical proof and formalization

In any additive abelian group, `P+Q+Q=0` implies `P=−(Q+Q)`. Under a scalar identification with `Z/43Z`, this is `a=−2b`. The multiplier `−2` has order 7 modulo 43. Its 42 nonzero scalars therefore partition into six seven-element multiplier orbits, paired by negation into three sign-folded components. Every row is supported entirely inside one component. Around each seven-cycle, six equations determine the remaining coordinates from one starting value, and the seventh is consistent because `(−2)^7=1`. Thus each block has rank 6, giving total rank 18. The exact block rank is also independently calculated from the complete 42×21 matrices.

| Result | Lean declaration or receipt | Exact assumptions | Status |
| --- | --- | --- | --- |
| Relation-word step | `XedniRelationWordOrbits.relation_forces_neg_double` | Any additive abelian group, `P+Q+Q=0` | Lean checked |
| Multiplier order and partition | `multiplier_seven`, `six_orbits_cover`, `six_orbits_disjoint`, `orbit_card_seven`, `sign_pairs` | `ZMod 43`; six explicit seeds `{1,2,3,6,7,9}` | Lean checked |
| No cross-orbit relation | `orbit_closed`, `scalar_relation_same_orbit` | Scalar relation `a+b+b=0` in `ZMod 43`, with `b` in a declared orbit | Lean checked |
| Concrete curve and matrix bridge | [runner](run.py), [accepted Sage audit](sage_audit.py) | All 42 nonidentity points of the stated `E(F₄₇)`, exact source-pair alignment, sign quotient, and full 42×21 matrix | Independent replay PASS |
| Direct Lean `Matrix.rank = 18` theorem on the concrete curve | Requires a formal generator/isomorphism from `E(F₄₇)` to `ZMod 43`, the sign-folded column map, and block-rank composition | Same concrete curve | Formalization partial; those bridge definitions and the numeric matrix theorem remain open |

The [protocol](protocol.json) was committed before the census. It fixes the source and arithmetic code digests, curve, full-base policy, row word, orbit rule, resource limit, and claim boundary. The [result](result.json) retains all 42 ordered source indices, the full column order, cycle members, block ranks, total rank and row checksum; [compressed rows](rows.jsonl.gz) preserve every matrix row. The [Sage audit result](sage-audit-result.json) matches the exact matrix, all source flags, cycle partition, and ranks with empty stderr. Its [runtime receipt](sage-runtime-info.json) records the accepted local build.

The [pinned Lean build](formal/build.sh) uses `v4.35.0-rc4` and Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9`. Its [output](formal/build-output.txt) lists only standard `propext`, `Classical.choice`, and `Quot.sound` foundations under `#print axioms`; no reported theorem uses `sorry`, `admit`, a custom claim axiom, or native evaluation. `lean4checker --fresh` is unavailable on this host.

The next construction experiment must address the existing coefficient screens. The [coefficient-10 packet](../xedni_residue10_rational_20261120/README.md) checked 1,488 locally selected centered CRT curves and found zero rational `P+10Q` relations; the [coefficient-14 packet](../xedni_periodic_coefficient_20261120/README.md) checked 48 centered curves that passed both auxiliary relation tests and also found zero rational relations. A new bridge-word proposal should first derive its rational lift condition and cost, then vary coefficient representatives or the auxiliary-prime policy under a frozen protocol. Measure novel row rank and target coverage alongside relation yield before attempting a full IC workload. Any complete candidate must retain the one-target online and cold costs and a same-point rho reference under the repository measurement contract.

## Replay

```sh
python3 research/exotic_ecdlp_20261021/xedni_relation_word_orbits_20261120/run.py > research/exotic_ecdlp_20261021/xedni_relation_word_orbits_20261120/result.json
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_relation_word_orbits_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python /Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-recovery-20261119/research/exotic_ecdlp_20261021/xedni_relation_word_orbits_20261120/sage_audit.py > research/exotic_ecdlp_20261021/xedni_relation_word_orbits_20261120/sage-audit-result.json
sh research/exotic_ecdlp_20261021/xedni_relation_word_orbits_20261120/formal/build.sh
```
