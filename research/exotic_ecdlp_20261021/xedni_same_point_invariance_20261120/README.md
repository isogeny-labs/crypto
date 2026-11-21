# A local tangent constraint removes 21,096 CRT witness pairs

The [checked Lean theorem](formal/SamePoint.lean) proves that when two source points coalesce modulo a prime with `x₁−x₂=p·u`, `y₁−y₂=p·v`, and `u` nonzero after reduction, their incidence on an elliptic Weierstrass equation determines the reduced `a4` from the reduced `a1,a2,a3`. Higher coefficient digits that preserve those three residues cannot repair a mismatched `a4`. The frozen census applies this constraint to the **32** same-point source pairs: among **22,728** ordered local witness pairs, **1,632** satisfy it and **21,096** fail it. An independent exact-rational replay reproduced every pair count and the earlier CRT statuses.

## Mathematical proof

Write the general Weierstrass equation as

`y² + a1·x·y + a3·y = x³ + a2·x² + a4·x + a6`.

For the two integer source points, put `x₁−x₂=p·u` and `y₁−y₂=p·v`. Subtract their equations, factor the differences of squares and cubes, and cancel nonzero `p` in a domain. The resulting exact identity is

`a4·u = v(y₁+y₂) + a1(x₁v+y₂u) + a3v − u(x₁²+x₁x₂+x₂²) − a2u(x₁+x₂)`.

Let the coefficient ring be the rationals integral at `p`, with reduction to `Fₚ`. Because `u` is a unit after reduction, this identity gives a unique residue for `a4`. The `a6` term cancels, and the result does not depend on any higher digits of `a1,a2,a3`. The Lean proof states the identity over an arbitrary domain and proves uniqueness under any ring map to a field where `u` remains nonzero. Instantiating that abstract ring map with the localization of `ℤ` at `p` for these 32 concrete curves is not yet formalized; the integer valuation and exact-rational source application are checked by the scripts and written argument here. Thus the generic structural theorem is Lean checked, while the full source-specific formalization remains open at the localization bridge.

| Collision prime | Source pairs | Local witness pairs | Tangent-compatible | Tangent-incompatible |
| ---: | ---: | ---: | ---: | ---: |
| 7 | 24 | 16,128 | 1,152 | 14,976 |
| 11 | 8 | 6,600 | 480 | 6,120 |
| **Total** | **32** | **22,728** | **1,632** | **21,096** |

All 32 source pairs have `vₚ(x₁−x₂)=1`. For 28 pairs `vₚ(y₁−y₂)=1`; for four pairs at `p=11`, `y₁−y₂=0`. Hence `u=(x₁−x₂)/p` is nonzero modulo `p`, and `v=(y₁−y₂)/p` is integral. Each compatible local witness pair already produced one rational curve in the preceding one-digit CRT census; each incompatible pair was recorded there as `no_extra_digit`. The [independent audit](audit.py) computed the centered CRT coefficients, constructed the rational curve, reduced its exact `a4` fraction, and reconciled all 32 rows with that preceding receipt. Its [result](audit-result.json) is `PASS` with empty stderr.

## Proof and measurement map

| Result | Declaration or receipt | Exact conditions | Status |
| --- | --- | --- | --- |
| Secant-tangent identity | `XedniSamePoint.tangent_identity` | Domain `R`; nonzero `p`; `x₁−x₂=p·u`, `y₁−y₂=p·v`; both point-incidence equations | Lean checked |
| Unique reduced `a4` | `XedniSamePoint.a4_reduction_unique` | Same conditions for two curves; ring map `f:R→F`; `f(u)≠0`; equal images of `a1,a2,a3` | Lean checked |
| Source-pair valuation and gate census | [result](result.json) from [frozen protocol](protocol.json) and [screen](screen.py) | Exact 1,722-row source law; 32 same-point pairs; frozen 7/11 local gates | Complete, independently audited |
| Concrete localization instantiation | Written proof above and exact-rational audit | `R=ℤ` localized at `p`, rational `a4,a6` integral there, standard reduction map | Mathematical proof and computational check; Lean bridge open |

The protocol and runner were pushed before execution as `23901e60df2`. The source gzip SHA-256 is `9a594d96f369fb14f1a4cf6270b1c811ceda515e1c2a7eccab4d011ca8f3f322`; the result binds protocol, script, and gate-script hashes. The [build script](formal/build.sh) checks `leanprover/lean4:v4.35.0-rc4` and the pinned Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9`. The [build output](formal/build-output.txt) lists only `propext`, `Classical.choice`, and `Quot.sound` for both theorems, with empty stderr. No `sorry`, `admit`, custom claim axiom, or native evaluation is used. `lean4checker --fresh` is unavailable on this host.

## Replay and next construction

```sh
sh research/exotic_ecdlp_20261021/xedni_same_point_invariance_20261120/formal/build.sh
python3 research/exotic_ecdlp_20261021/xedni_same_point_invariance_20261120/screen.py > research/exotic_ecdlp_20261021/xedni_same_point_invariance_20261120/result.json
python3 research/exotic_ecdlp_20261021/xedni_same_point_invariance_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_same_point_invariance_20261120/audit-result.json
```

The 1,632 tangent-compatible witness pairs belong to 32 source pairs whose fixed reduction at 47 does not satisfy `P + 2Q = O`. Every curve constructed here has good reduction to that fixed model at 47, so a rational relation on those source points would contradict its reduced sum. Higher-digit searches for this relation should instead focus on source pairs that pass the fixed-prime relation gate. The tangent-incompatible 21,096 pairs additionally cannot be repaired by preserving their local `a1,a2,a3` residues.
