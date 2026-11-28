# Cross-characteristic example: instance and proof map

## Characteristic-two extension: instance statement before the Lean edit

The additional request is an odd-characteristic source and an even-characteristic
target. A field of even characteristic has characteristic 2. The worked target
is the actual field `GaloisField 2 2`, of cardinality 4, rather than `ZMod 4`.
Its model is `y² + y = x³`; the earlier short model is singular in
characteristic 2. The source and target subgroup orders must match.

1. **Structure above:** the same Mathlib affine-point groups and cyclic
   subgroups, now over a field `K` with `CharP K 2`. `GaloisField` provides
   the concrete finite-field specialization; `CharTwo` provides its arithmetic.
2. **Theorem used as stated:** `addEquivOfAddOrderOfEq` and its generator
   image lemma, followed by `map_zsmul` and injectivity. `GaloisField.card`
   supplies the field cardinality without a new field construction.
3. **Object as an instance:** `⟨0,0,1,0,0⟩ : WeierstrassCurve K`, with
   `R = (0,0)` and `S = (0,1)`. These points are already rational over the
   prime subfield, and also define points over the genuine field of four elements.
4. **Assembly probe:** check the existing constructor with the target field
   and equal-order obligations as parameters, then specialize it to these
   coordinates. Retain the source and output under the new binary receipt.
5. **Missing components only:** the binary discriminant, two memberships,
   `R + R = S`, `S + R = O` and the order-three obligation. The source
   obligations are imported unchanged. No generic law is added.
6. **Inheritance:** field and characteristic from Mathlib `GaloisField`;
   arithmetic from `CharTwo`; ellipticity from the literal discriminant;
   all point operations, subgroup operations and scalar laws from the existing
   inheritance table below. No limit override is needed.

The proof chain is `Q = [2]P`, then `Φ(Q) = [2]Φ(P)` by `map_zsmul`,
then `[2]R = S` by the binary coordinate obligation. For an arbitrary scalar,
`Φ(T) = [k]R` becomes `Φ(T) = Φ([k]P)` by the generator-image law;
injectivity gives `T = [k]P`. The existing source scalar congruence gives
`2 ≡ k (mod 3)`. No new generic cyclic-group argument is required.

| Binary statements | Classification | Existing owner / hypotheses |
| --- | --- | --- |
| Binary ellipticity, point memberships, doubling and order | Instance obligations | Mathlib Weierstrass model, `Field K`, `CharP K 2` |
| Binary transfer | Assembly | Mathlib equal-order constructor, order 3 on both sides |
| Scalar endpoints and table | Endpoints | Existing morphism laws and concrete coordinates |
| Four-element field cardinality | Endpoint | `GaloisField.card`, exponent 2 is nonzero |

Only the concrete coordinate obligations open the affine formulas; they are
not supplied by generic morphism laws. No generic replacement is possible for
these literal coordinates across arbitrary curves. As in the original example,
`Point.add_def` is the named opening for a doubling calculation. The construction
uses classical choice and provides no general efficient encoded-point evaluator.
The primary sources additionally include Mathlib v4.24.0
`Algebra/CharP/Two.lean` and `FieldTheory/Finite/GaloisField.lean`.

The assembly probe and final module both type-check. The first concrete build
left characteristic-dependent numeral goals; the next exposed Lean's placement
rule for a documentation comment after `omit`. The retained third build and
axiom audit pass. No limit was raised. The versioned receipts retain those
attempts rather than replacing the earlier F5-to-F7 verification record.
The characteristic-two claim record is `PCC-20261115-6a7830` and remains
unsigned; local kernel validation is recorded separately. [pcc:unverified]

The unchanged repository proof gate reports `PASS 50 thm 0 trusted`, auditing
both modules with zero failing theorems. Its raw output is in the versioned
binary receipt. All generic binary-point declarations additionally carry
`DecidableEq K`; the concrete four-element field uses a classical instance.
The no-field-map theorem needs only `Field K` and `CharP K 2`.

| Report statement | Lean declaration | Hypotheses and status |
| --- | --- | --- |
| Binary model nonsingular | `CrossCharacteristicTwo.instIsEllipticBinaryCurve` | `Field K`, `DecidableEq K`, `CharP K 2`; checked instance |
| Binary generator order 3 | `CrossCharacteristicTwo.R_order` | Same field assumptions; local kernel check |
| Target challenge image | `CrossCharacteristicTwo.phi_challenge` | Source challenge in the order-three subgroup; local kernel check |
| All scalar equations preserved and reflected | `CrossCharacteristicTwo.transfer_dlog` | Any source subgroup point, any integer; local kernel check |
| Scalar congruence | `CrossCharacteristicTwo.target_scalar_mod` | Any integer; local kernel check |
| Actual four-element target field | `CrossCharacteristicTwo.F4_card` | `GaloisField 2 2`; local kernel check |
| F5-to-F4 endpoint | `CrossCharacteristicTwo.F5_to_F4_dlog` | Any integer; local kernel check, PCC unsigned |
| No unital source-to-binary field map | `CrossCharacteristicTwo.no_field_map` | `Field K`, `CharP K 2`; local kernel check |

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
