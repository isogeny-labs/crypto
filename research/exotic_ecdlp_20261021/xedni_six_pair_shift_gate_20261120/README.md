# CRT shift gate for the six surviving xedni source pairs

The full-source tangent theorem leaves six ordered source pairs. For their **144 local witness classes** (24 per pair), a Lean-checked shift theorem and a 144-row arithmetic certificate show that **136 classes cannot satisfy `P+2Q=O` under any integer 77-shift** of their centered CRT coefficients. The other eight have zero tangent residual at the centered representative and are precisely the eight rational relation records already verified by the full-law CRT audit. Thus increasing coefficient height within these fixed local residue classes creates no new relation-eligible class for this point law and relation word.

| Source indices | Local witness classes | Shift-excluded modulo 3,619 | Centered zero residual |
| --- | ---: | ---: | ---: |
| 201, 243 | 48 | 48 | 0 |
| 425, 467 | 48 | 40 | 8 |
| 511, 553 | 48 | 48 | 0 |
| **Total** | **144** | **136** | **8** |

## Arithmetic and proof

Write `D=x₁−x₂`, `Y=y₁−y₂` and, for centered integer CRT coefficients `k1,k2`, set

`R(k1,k2)=D²(47k2+x₁+2x₂)−47k1·D·Y−Y²`.

The [point-group tangent theorem](../xedni_collision_tangent_obstruction_20261120/formal/Tangent.lean) gives `R=0` as a necessary condition for the rational relation on an elliptic curve through the exact point pair. Every integer shift preserving the selected coefficient residues at 7 and 11 has `k1'=k1+77u`, `k2'=k2+77v`. Direct expansion gives

`R(k1',k2')=R(k1,k2)+47·77·(D²v−D·Y·u)`.

Hence a nonzero `R(k1,k2) mod 3619` excludes the relation for **all** integers `u,v`. Shifting `k3` cannot change this necessary condition. The [Lean theorem](formal/Shift.lean) derives the exclusion from the actual Mathlib elliptic point-group relation and proves the shift identity. Its [case module](formal/Cases.lean) checks all 144 residuals and proves exactly eight centered zeros.

| Result | Lean declaration | Exact scope | Status |
| --- | --- | --- | --- |
| Shift identity | `XedniSixPairShift.residual_shift` | All integer coordinates, centered coefficients, shifts `u,v` | Checked |
| Point relation exclusion | `XedniSixPairShift.no_relation_of_shift_mod3619` | Elliptic `W/ℚ` through the two integer points; `a1=47(k1+77u)`, `a2=47(k2+77v)`; nonzero centered residual modulo 3619 | Checked |
| Complete local-class gate | `XedniSixPairShift.exhaustive`, `centered_zero_count`, `noncentered_shift_excluded` | Explicit 144-row source/local table generated from the frozen witness policy | Checked |
| Independent gate replay | [audit](audit.py) | Alternate local-gate enumerator, independent centered CRT search, all source and row hashes | [PASS](audit-result.json) |

The [protocol](protocol.json) fixed the source digest, six input indices, prior runner and result hashes, shift law, observable, resource limit, and claim boundary before the census. The [result](result.json) retains all 144 classifications in [compressed rows](rows.jsonl.gz) with raw and packed hashes. The independent audit reproduced all six 3×8 local gate lists and every residual; no row is unresolved. The eight centered classes have complete rational point-group and local-order replay in the [prior full-law packet](../xedni_full_law_crt_20261120/README.md).

The [pinned build](formal/build.sh) uses Lean `v4.35.0-rc4` and Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9`. Its [output](formal/build-output.txt) lists only `propext`, `Classical.choice`, and `Quot.sound` for point-level theorems and only `propext` for the arithmetic case certificates; stderr is empty. There is no `sorry`, `admit`, custom claim axiom, or native evaluation. `lean4checker --fresh` is unavailable on this host.

This is a necessary relation gate over the frozen point law and fixed auxiliary residue classes. The eight relation-positive classes permit further coefficient shifts; the exact count of distinct nonsingular rational curves along those shifts is a separate question. A complete discrete-log method would additionally require relation generation and target recovery on a specified subgroup, with all stages charged under the measurement contract.

## Replay

```sh
python3 research/exotic_ecdlp_20261021/xedni_six_pair_shift_gate_20261120/run.py > research/exotic_ecdlp_20261021/xedni_six_pair_shift_gate_20261120/result.json
python3 research/exotic_ecdlp_20261021/xedni_six_pair_shift_gate_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_six_pair_shift_gate_20261120/audit-result.json
python3 research/exotic_ecdlp_20261021/xedni_six_pair_shift_gate_20261120/make_cases.py --check
sh research/exotic_ecdlp_20261021/xedni_six_pair_shift_gate_20261120/formal/build.sh
```
