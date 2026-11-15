# Cross-characteristic example: instance and proof map

The requested deliverable is a worked Lean example of scalar-preserving
transfer between prime-field elliptic-curve subgroups, published as a PR.
The exact models are `y² = x³ + 1` over `ZMod 5` and `ZMod 7`; the chosen
points are `(0,1)` and the subgroup order is 3. [pcc:unverified]

## Instance statement before the repository Lean edit

1. **Structure above:** Mathlib's `WeierstrassCurve.Affine.Point`, its
   `AddCommGroup` instance, `AddSubgroup.zmultiples`, and `AddEquiv`.
   Operations are zero, addition, integer scalar multiplication and subgroup
   inclusion; laws are inherited from these structures.
2. **Theorem used as stated:** `addEquivOfAddOrderOfEq` supplies the
   isomorphism between groups with specified equal-order generators.
   `addEquivOfAddOrderOfEq_apply_gen` supplies the generator image;
   `map_zsmul` and injectivity give the DLP endpoint.
3. **Object as an instance:** two literal Weierstrass models over the two
   prime fields, actual affine nonsingular points, and their cyclic
   subgroups. No alternative point or group syntax is introduced.
4. **Assembly probe:** the already checked standalone construction uses
   those same Mathlib objects. Before writing the repository endpoint, a
   focused probe checks the direct constructor, subtype-order lemma and
   generator-image theorem in the pinned Mathlib environment. The example
   then checks the concrete order-three obligations. The probe completed with
   exit 0 (`assembly-probe.log`, source `assembly-probe.txt`); the only adjustment was explicitly unfolding
   the two local generator abbreviations for subtype-order simplification.
5. **Missing components only:** discriminants, four point-membership
   obligations, two doubling equations and two order-three obligations.
   No missing generic group law or new upstream candidate is needed.
6. **Inheritance:** recorded below; no limit overrides are used.

| Operation or obligation | Inherited from / discharged by |
| --- | --- |
| Field arithmetic | `ZMod` with the checked prime instances for 3, 5 and 7 |
| Point representation | `WeierstrassCurve.Affine.Point` |
| Ellipticity | `WeierstrassCurve.IsElliptic`; literal nonzero discriminants |
| Point addition, zero, inverse | Mathlib affine-point `AddCommGroup` |
| Cyclic subgroup | `AddSubgroup.zmultiples` |
| Generator represents every subgroup element | `AddSubgroup.forall_zmultiples` |
| Subtype preserves point order | Mathlib's subtype element-order lemma |
| Point order is 3 | `addOrderOf_eq_prime` applied to the concrete equations |
| Transfer and inverse | `addEquivOfAddOrderOfEq` |
| Image of the generator | `addEquivOfAddOrderOfEq_apply_gen` |
| Image of a multiple | existing `map_zsmul` law |
| DLP reflection | existing `AddEquiv.injective` law |
| Scalar congruence | `zsmul_eq_zsmul_iff_modEq` |

## Statement classification and proof chain

| Statements | Kind | Owner / quantified structure | Hypotheses |
| --- | --- | --- | --- |
| Ellipticity, membership, doubling, point orders | Instance obligations | This example / Mathlib objects | Literal fields and coordinates |
| `phi` | Assembly of existing objects | Mathlib `AddEquiv` | Equal generator orders |
| Transfer table and scalar endpoint | Endpoints | This example / subgroup points and integers | `Q ∈ ⟨P⟩`, any integer scalar |
| Generic laws | None added | Mathlib | Its existing contracts |

The endpoint rewrites the source challenge as `[2]P`, applies the existing
scalar law to the assembled isomorphism, and substitutes `[2]R = (0,6)`.
For arbitrary scalars, injectivity reflects equality; the existing order
congruence lemma reads this as `2 ≡ k (mod 3)`. [pcc:unverified]

## Concrete arithmetic openings

Point-membership obligations use `nonsingular_iff'` and `equation_iff`.
Only the two literal doubling calculations open Mathlib's affine addition,
slope and coordinate formulas: their consumers are the two point-order
obligations. Group structure and the transfer constructor are not unfolded.
These are coordinate evaluations, not new group laws. The same arithmetic
pattern appears twice because the two field instances have different
characteristics; it is not a generic theorem family.

## Sources and scope

- Mathlib v4.24.0 `GroupTheory/SpecificGroups/Cyclic.lean`, generator section:
  the constructor and generator-image theorem used directly.
- Mathlib v4.24.0 `GroupTheory/OrderOfElement.lean`: prime order, subtype
  order preservation and integer-scalar congruence.
- Mathlib v4.24.0 `AlgebraicGeometry/EllipticCurve/Affine/Point.lean` and
  `Affine/Formula.lean`: actual group structure and coordinate formulas.
- Repository `problems/prime-field-chained-transfer-no-advantage.md`:
  distinguishes an abstract scalar-preserving map from an efficient
  encoded-point evaluator. This example does not close that problem.
- KB unavailable: no loaded KB tools; wrapper exits 127 for absent executable.

The primary-source dependency revision is fixed by `lake-manifest.json`.
No generic law is copied into this study, and no crypto-lean extension is
required for this instance.

## Local validation receipt

`lake build` completed successfully. The unchanged repository gate reported
`PASS 30 thm 0 trusted` and its audit reported 0 failing theorems; raw outputs
are retained under `receipts/`. This is a local kernel audit, not a signed PCC
verdict. The declaration-bound PCC record is `PCC-20261115-8b013d` and remains
unverified until the repository's independent process signs it. [pcc:unverified]

The report PDF was rendered through Pandoc/XeLaTeX and its single page was
opened for inspection. The earlier Chromium attempt was blocked by the
environment's local-file policy; no browser policy was changed.
