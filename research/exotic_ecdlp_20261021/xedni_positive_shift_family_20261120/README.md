# Relation-positive CRT classes form many rational curves

Shifting only the third CRT coefficient across the eight previously verified relation-positive classes produced **264 nonsingular rational elliptic curves with `P+2Q=O`**, at 33 shifts per class. These curves have **132 distinct rational `j`-invariants**. Each run retained the exact source points, the fixed reduction at 47, the same auxiliary coefficient residues at 7 and 11, and local curve orders 4 and 6. The accepted Sage point group verified every rational relation; an independent exact-fraction replay verified all 264 relations, discriminants, invariants, coefficient reductions, and source rows.

| Source index | Local classes | Shifts per class | Verified curves | Distinct `j` within index |
| ---: | ---: | ---: | ---: | ---: |
| 425 | 4 | 33 | 132 | 132 |
| 467 | 4 | 33 | 132 | 132 |
| **Combined** | **8** | **33** | **264** | **132** |

## Construction and algebra

The [protocol](protocol.json) fixes the eight positive centered CRT records, their source hashes, and `t∈[-16,16]`. It holds `k1,k2` fixed, sets `k3'=k3+77t`, takes `a3'=47k3'`, then solves `a4',a6'` from the two integer point-incidence equations. This preserves the selected residues at 7 and 11 and the fixed coefficient reduction at 47.

For `D=x₁−x₂≠0`, `Y=y₁−y₂` and `δ=a3'−a3`, exact incidence gives

`a4'=a4+δY/D`, `a6'=a6+δ(y₁−x₁Y/D)`.

On a centered relation-positive curve, the tangent at `Q` has slope `Y/D`. Its numerator and denominator change under this shift by `δY/D` and `δ`, respectively. Whenever the new denominator is nonzero, the tangent keeps that slope. The bounded run checks this condition as part of a complete rational point-group computation; all 264 shifted curves satisfy it. The [previous Lean theorem](../xedni_six_pair_shift_gate_20261120/formal/Shift.lean) checks the complementary necessary fact: the tangent residual is unchanged by shifting `k3`. A generic Lean converse from the incidence and tangent equations to the Mathlib point-group relation is still open. The present 264 relation assertions rest on the full Sage and independent Fraction replays, with this formalization status explicit.

| Result | Evidence | Status |
| --- | --- | --- |
| 264 elliptic rational point-group relations | [checked Sage runner](run_sage.py), [rows](rows.jsonl.gz), [result](result.json), [runtime](sage-runtime-info.json) | Complete bounded replay |
| Independent relation, discriminant, and `j` check | [Fraction audit](audit.py), [audit result](audit-result.json) | PASS, all 264 rows |
| Necessary tangent residual shift identity | `XedniSixPairShift.residual_shift` in the preceding packet | Lean checked |
| General relation preservation for arbitrary `k3` shifts | Incidence and tangent algebra above; nonvertical and nonsingularity conditions | Lean point-group converse open |

The 132 distinct `j`-invariants show the bounded family contains genuinely different elliptic curves, even though sign-mirror rows account for repeated invariants across the two source indices. This is a relation-construction result under a fixed small-field point law. It does not by itself supply relation collection, factor-base logs, or target descent on a separate discrete-log workload; those are the next algorithmic experiments under the IC measurement contract.

## Replay

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_positive_shift_family_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python /Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-recovery-20261119/research/exotic_ecdlp_20261021/xedni_positive_shift_family_20261120/run_sage.py > research/exotic_ecdlp_20261021/xedni_positive_shift_family_20261120/result.json
python3 research/exotic_ecdlp_20261021/xedni_positive_shift_family_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_positive_shift_family_20261120/audit-result.json
```
