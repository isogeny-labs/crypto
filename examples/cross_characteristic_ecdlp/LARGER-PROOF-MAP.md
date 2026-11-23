# Larger-field extension: instance and proof map

The user's next request is to try a bigger field. This addition targets the
prime field `ZMod 65537` and the binary field `GaloisField 2 256`, with a new
prime-order-7 subgroup. The field size is much larger; the subgroup remains
a worked example and is not a cryptographic-size discrete-log instance.

## Instance statement before the Lean edit

1. **Structure above:** Mathlib `WeierstrassCurve.Affine.Point`, its additive
   group, `AddSubgroup.zmultiples`, and `AddEquiv`; finite fields, their unit
   groups and `IsPrimitiveRoot` provide the binary coefficient.
2. **Theorems used as stated:** `addEquivOfAddOrderOfEq` with the existing
   generator-image, scalar and injectivity laws. `GaloisField.card` fixes the
   target cardinality. Cauchy's `exists_prime_orderOf_dvd_card` supplies a unit
   of order 3 because 3 divides `2^256 - 1`; `IsPrimitiveRoot.geom_sum_eq_zero`
   supplies the coordinate identity rather than a copied root-of-unity law.
3. **Objects as instances:** source `⟨-1,-4,-4,0,0⟩` over `ZMod 65537`,
   target `⟨0,ω,ω,0,0⟩` over `GaloisField 2 256`, with `ω` a primitive cube
   root of unity. Both chosen generators have coordinates `(0,0)`.
4. **Assembly probe:** type-check primality, field cardinality, the existing
   root-of-unity constructor and the existing equal-order subgroup constructor
   before the concrete module. Retain exact source and outputs with the new
   versioned receipts.
5. **Missing components only:** literal discriminants, point memberships,
   doubling and one chord calculation on each model, then the order-7
   obligations. No new generic group, finite-field or root-of-unity law.
6. **Inheritance table:** below. No resource-limit override or native
   evaluation exception is planned or needed.

| Operation or obligation | Existing owner / instance |
| --- | --- |
| Prime-field arithmetic | Mathlib `ZMod`, checked primality of 65537 |
| Binary field and cardinality | Mathlib `GaloisField 2 256`, `GaloisField.card` |
| Nontrivial binary coefficient | Cauchy's theorem on the field's unit group |
| Coefficient relation `ω²+ω+1=0` | `IsPrimitiveRoot.geom_sum_eq_zero` |
| Points, addition, inverse, integer multiples | Mathlib affine-point additive group |
| Ellipticity and concrete coordinates | Instance obligations for the literal models |
| Exact generator order | `addOrderOf_eq_prime` from `[7]P=O` and `P≠O` |
| Subgroup transfer and generator image | Existing equal-order cyclic constructor |
| Scalar equivalence and congruence | `map_zsmul`, injectivity, `zsmul_eq_zsmul_iff_modEq` |

## Coordinate sketch and endpoint chain

Source generator `P=(0,0)` has `2P=(4,8)`, `3P=(2,2)`,
`2(3P)=(0,4)=-P`, hence `7P=O`. On the binary model, `R=(0,0)` has
`2R=(ω,ω)`, `3R=(1,ω+1)`, `2(3R)=(0,ω)=-R`, hence `7R=O`.
The scalar endpoint applies the inherited isomorphism to those equal orders;
for the chosen challenge it reads `Φ(3P)=3R` and scalar `k≡3 (mod 7)`.
All coordinates and equalities here are the obligations to be checked, not
assumptions smuggled into the final endpoint.

| Statement family | Classification | Quantifiers / hypotheses |
| --- | --- | --- |
| Prime, discriminants, memberships, coordinate additions, orders | Instance obligations | The two exact field/model definitions |
| Cube-root existence and field size | Endpoints | Existing finite-field/Cauchy laws |
| Transfer and scalar equality | Endpoints | Actual source cyclic subgroup; every integer scalar |
| Generic law | None added | Existing Mathlib contracts only |

Only concrete point calculations open the affine formulas via `Point.add_def`.
Existing group laws do not determine these particular coordinates; the consumer
is the generator-order obligation. Scalar endpoints use named morphism laws
without opening the isomorphism's implementation. The coefficient is defined
by classical choice; no claim of an efficient arbitrary-point evaluator or
larger prime-order subgroup follows from increasing the field cardinality.

## Sources checked and evidence scope

KB unavailable: no KB tools are loaded, and `scripts/research-kb status`
exits 127 for the absent executable. The fallback is the existing example
and pinned Mathlib v4.24.0 primary files: affine `Point.lean` and `Formula.lean`,
`GroupTheory/SpecificGroups/Cyclic.lean`, `GroupTheory/Perm/Cycle/Type.lean`,
`RingTheory/RootsOfUnity/PrimitiveRoots.lean`, `Data/Fintype/Units.lean`, and
`FieldTheory/Finite/GaloisField.lean`. These supply the actual interfaces
and hypotheses. No novelty or efficient-transfer claim is made.

Old proof receipts and claim records remain unchanged. The existing PR's
shared-journal request and inherited ledger-history review stay open; this
extension does not edit the conflicted journal while its resolver is pending.

## Checked declarations and proof status

The assembly probe passes after explicitly importing Mathlib's prime-number
extension for `norm_num`. The concrete module passes without a resource-limit
change. Early failed builds are retained: they expose unsimplified branch
conditions, characteristic-two casts and field divisions, rather than any
relaxation of the point-order goal. Prime-field divisions are discharged with
`div_eq_iff` and `div_self`; no noncomputable inverse is evaluated by `decide`.

The final scalar endpoint is a direct use of `phi.injective.eq_iff`, simplified
by `map_zsmul`, the generator-image law and `Subtype.ext_iff`. The concrete
coordinate obligations alone open addition formulas. There is no missing
generic upstream component.

| Report statement | Lean declaration(s) | Exact scope |
| --- | --- | --- |
| Field cardinalities | `source_card`, `target_card` | 65537 and `2^256` |
| Binary coefficient exists | `exists_omega`, `omega_primitive` | Unit order 3 from Cauchy; classical choice |
| Coefficient outside `F_2` | `omega_not_from_F2` | Every `x : ZMod 2` |
| Coefficient identity | `omega_relation`, `omega_sq`, `omega_inv` | The chosen primitive cube root |
| Nonsingular models | `instIsEllipticZModOfNatNatSourceCurve`, `instIsEllipticFbigTargetCurve` | The two literal models |
| Source coordinate chain | `P_double`, `P2_add_P`, `Q_double`, `P6_add_P` | The stated source points |
| Binary coordinate chain | `R_double`, `R2_add_R`, `S_double`, `R6_add_R` | The stated target points |
| Exact point orders | `P_order`, `R_order` | Both equal 7 |
| Challenge image | `phi_challenge` | `Φ(Q)=S` |
| All scalar equations | `transfer_dlog` | Every source subgroup element and integer |
| Challenge scalar equivalence | `example_dlog` | Every integer |
| Scalar residue | `source_scalar_mod`, `target_scalar_mod` | `3 ≡ k [ZMOD 7]` |

All names in the table have prefix `CrossCharacteristicLarger`. These are local
kernel-checked declarations. The declaration-bound claim `PCC-20261115-53afbe`
is unsigned; that standing is distinct from the local build and axiom audit.
Raw receipts and exact source/dependency hashes are versioned under
`receipts/larger-20261115T2025Z`. [pcc:unverified]

The unchanged repository gate passes: **94 theorems audited, zero failing,
zero trusted native exceptions** across all three modules. The explicit axiom
audit reports only `propext`, `Classical.choice` and `Quot.sound`. [pcc:unverified]

The target's parameters and chosen points descend to the four-element subfield
generated by `ω`. Field cardinality is not subgroup cardinality: no order near
`2^256`, large-subgroup security level, or point-transfer runtime is claimed.
