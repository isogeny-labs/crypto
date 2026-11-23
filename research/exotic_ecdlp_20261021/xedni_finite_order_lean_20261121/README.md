# The reduced generator has order 43 and reaches the frozen target

On `E₀/F₄₇: y²=x³−x−2`, let `Q̄=(17,10)` and `T=(10,1)`. Lean proves with Mathlib's actual elliptic-curve point group that **`addOrderOf Q̄=43`** and **`22 • Q̄=T`**. Consequently, for every integer `n`, `n • Q̄=T` implies **`n mod 43=22`**. This supplies the exact finite-group fact used in the fixed-curve rational-lift height proofs.

The [protocol](protocol.json) froze the field, curve, points, theorem goals, and source hashes before the formalization. The pinned [Lean module](formal/FiniteOrder.lean) constructs nonsingular points on the exact curve and checks a short addition chain:

| Multiple or sum | Point |
| --- | --- |
| `2Q̄` | `(14,40)` |
| `4Q̄` | `(44,16)` |
| `8Q̄` | `(23,4)` |
| `16Q̄` | `(15,31)` |
| `32Q̄` | `(29,23)` |
| `16Q̄+4Q̄+2Q̄` | `T=(10,1)` |
| `32Q̄+8Q̄+2Q̄+Q̄` | identity |

Each nontrivial addition is proved using Mathlib's `WeierstrassCurve.Affine.Point` group-law lemmas and explicit `ZMod 47` slope equalities. The last identity gives `43Q̄=0`; `Q̄` is nonidentity, so primality of 43 yields its exact order. Subtracting `22Q̄=T` from `nQ̄=T` and applying the order criterion gives the residue-class theorem.

## Proof map

| Result | Lean declaration | Status |
| --- | --- | --- |
| `Q̄` is nonidentity | `XedniFiniteOrder.Q_ne_zero` | Checked |
| `43Q̄=0` | `Q_mul43_zero` | Checked from explicit group-law chain |
| `22Q̄=T` | `Q_mul22_target` | Checked from explicit group-law chain |
| `addOrderOf Q̄=43` | `Q_order` | Checked |
| `nQ̄=T ⇒ n mod 43=22` | `target_residue_class` | Checked |

The retained [build output](formal/build-output.txt) reports only `propext`, `Classical.choice`, and `Quot.sound` for these declarations. Stderr is empty; the module has no admitted claim, custom research axiom, or unverified native evaluation. The proof uses the pinned Lean toolchain and Mathlib manifest.

This closes the **finite point order and target-class** obligation in the [minimum-height lift](../xedni_minimum_lift_height_20261121/README.md) and [height-exclusion](../xedni_explicit_height_exclusion_20261121/README.md) proof maps. The rational Mordell–Weil generator certificate, the group-homomorphism bridge from rational points with good reduction, and the canonical-height comparison remain to be formalized for a fully curve-specific Lean theorem.

Replay from the repository root:

```sh
sh research/exotic_ecdlp_20261021/xedni_finite_order_lean_20261121/formal/build.sh
```
