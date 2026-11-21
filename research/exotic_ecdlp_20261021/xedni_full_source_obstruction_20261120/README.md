# A six-pair necessary condition for the full xedni point law

A Lean-checked theorem reduces the frozen **1,722 ordered integer point pairs** to six explicit coordinate pairs for the relation `P+Q+Q=0` on an elliptic rational Weierstrass curve through both points with `a1,a2 ∈ 47ℤ`. The exhaustive certificate classifies 1,638 pairs by a nonzero tangent residue modulo 47, 42 by the same-x/distinct-y group law, and 36 by integer divisibility. The six remaining pairs have source indices **201, 243, 425, 467, 511, 553**. The earlier full-law CRT census produced eight rational relation records on indices 425 and 467 under its centered representative policy.

| Source index | `P` | `Q` | Centered CRT candidates | Rational relation records |
| ---: | --- | --- | ---: | ---: |
| 201 | `(8,19)` | `(-4,19)` | 24 | 0 |
| 243 | `(8,-19)` | `(-4,-19)` | 24 | 0 |
| 425 | `(14,7)` | `(17,10)` | 24 | 4 |
| 467 | `(14,-7)` | `(17,-10)` | 24 | 4 |
| 511 | `(15,16)` | `(23,4)` | 24 | 0 |
| 553 | `(15,-16)` | `(23,-4)` | 24 | 0 |

## Proof

For `P=(x₁,y₁)` and `Q=(x₂,y₂)`, write `D=x₁−x₂`, `Y=y₁−y₂`. The [point-group tangent theorem](../xedni_collision_tangent_obstruction_20261120/formal/Tangent.lean) proves that `P+2Q=O` implies

`D²(a2+x₁+2x₂) − a1·D·Y − Y² = 0`.

The coefficient law therefore requires `D²(x₁+2x₂)−Y² ≡ 0 (mod 47)`. With integral coordinates and coefficients it also requires `D | Y²`. When `D=0` and `Y≠0`, the [same-x theorem](../xedni_same_x_obstruction_20261120/formal/SameX.lean) excludes the relation directly in the elliptic point group. Applying these three necessary conditions to the exact source table leaves the six coordinate pairs above.

| Result | Lean declaration | Hypotheses and conclusion | Status |
| --- | --- | --- | --- |
| Full-table arithmetic certificate | `XedniFullSource.exhaustive` | Each of 1,722 explicit `Coords` table entries satisfies a same-x, tangent-residue, divisibility, or six-coordinate disjunct | Checked |
| Point relation restriction | `XedniFullSource.relation_implies_survivor` | Any index `i : Fin 1722`; elliptic `W/ℚ` containing its exact two source points; `a1=47k1`, `a2=47k2`; actual point-group relation | The pair's coordinates belong to the explicit six-coordinate list; checked |
| Index alignment and counts | [independent audit](audit.py) | Frozen gzip SHA-256 and every generated table coordinate/index; classification independent of the generator | [PASS](audit-result.json) |

The [protocol](protocol.json) froze the source, coefficient law, relation word, classification order, expected counts, six indices, and claim boundary before the certificate run. [Generation output](generation-result.json) records the Lean table SHA-256; the [independent audit](audit-result.json) confirms all 1,722 coordinate/index matches and the eight earlier positive rows. The [pinned build](formal/build.sh) uses Lean `v4.35.0-rc4` and Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9`. Its [output](formal/build-output.txt) reports only `propext`, `Classical.choice`, and `Quot.sound` under `#print axioms`; stderr is empty. The modules use no `sorry`, `admit`, custom research axiom, or native evaluation. `lean4checker --fresh` is unavailable on this host.

This necessary condition fixes the point law and relation word. The four surviving indices without a centered CRT relation remain candidates for a coefficient-shift experiment. For each of their 24 local witness pairs, the tangent identity is a linear Diophantine condition in the two shifted coefficients `k1,k2`; its solvability and the auxiliary local gates can be tested exactly before constructing rational curves.

## Replay

```sh
python3 research/exotic_ecdlp_20261021/xedni_full_source_obstruction_20261120/make_cases.py --check
python3 research/exotic_ecdlp_20261021/xedni_full_source_obstruction_20261120/audit.py
sh research/exotic_ecdlp_20261021/xedni_full_source_obstruction_20261120/formal/build.sh
```
