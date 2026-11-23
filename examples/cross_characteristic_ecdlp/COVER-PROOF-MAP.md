# Concrete cover, specialization and weaker target

The user's clarified objective is a concrete cover/quotient route ending at
a weaker target, with a smaller field. This replaces the earlier direction of
enlarging the binary ambient field. Preserve the earlier sources and receipts
as historical evidence; do not interpret them as the requested construction.

The construction uses the Tate order-seven family with parameter t:
`b=t³-t²`, `c=t²-t`,
`E_t: y²+(1-c)xy-by=x³-bx²`.
The source specializes t=2 in F65537. The target specializes t=ω in F4,
where `ω²+ω+1=0`. The cubic cover is
`C_t: v²+(1-c)u³v-bv=u⁹-bu⁶`, with `π_t(u,v)=(u³,v)`.

The target is deliberately tiny: the transported subgroup has order 7.
This is an existential toy construction, not a reduction of a secure or
arbitrary large-prime-order ECDLP to an easier instance. [pcc:unverified]

## Instance statement before the Lean edit

1. **Structure above:** existing Mathlib affine-point groups, `Subfield`,
   scalar maps, subgroup homomorphisms and equal-order cyclic isomorphisms.
2. **Theorems used as stated:** `Point.baseChange` and `Point.map_injective`
   transport the already checked binary point identities into the explicit
   four-element subfield; `addOrderOf_injective` preserves the generator
   order. `addEquivOfAddOrderOfEq` identifies the two actual order-seven
   subgroups. `Nat.Coprime.nsmul_right_bijective` certifies multiplication
   by the cover degree 3. Existing homomorphism laws give scalar reflection.
   `AdjoinRoot.lift` also constructs two actual specializations from
   `A=ℤ[z]/(z²+z+1)` with `t₀=2+65537z`. Its defining root relation is the
   only input contract; existing injectivity of the constant embedding
   supplies `CharZero A`. The source receiving field is `F_(65537²)`, with
   its reduced coefficients descending to `F_65537`.
3. **Objects as instances:** the literal subfield `{0,1,ω,ω+1}` inside the
   previously checked characteristic-two field. Its field operations are
   inherited from Mathlib `Subfield`; its carrier closure and cardinality
   are concrete finite obligations. The source is imported unchanged.
4. **Assembly probe:** check the finite-subfield constructor, its cardinality,
   base-change point homomorphism and order preservation before assembling the
   final transfer. Retain attempts and final output in a new receipt.
5. **Missing components only:** closure of the four-element carrier, its
   cardinality, the mapped-model equality, and concrete cover equations.
   Source and old binary point addition are imported unchanged.
6. **Inheritance:** below. No new generic law, trust exception or resource-limit
   change is required.

| Operation | Existing owner / concrete obligation |
| --- | --- |
| Four-element field | Mathlib `Subfield`; finite carrier closure |
| Characteristic 2 | Inherited subfield instance |
| Ellipticity | Existing nonzero discriminant reflected through inclusion |
| Rank-three finite cover algebra | `AdjoinRoot.powerBasisAux'` for the monic polynomial `U³-x` |
| Point map and injectivity | Mathlib `Point.baseChange`, `Point.map_injective` |
| Generator order 7 | Existing larger example, `addOrderOf_injective` |
| Cross-fiber subgroup identification | Existing equal-order cyclic constructor, checked seven-section table |
| Cover-degree multiplier 3 | Existing scalar homomorphism; coprime to 7 |
| Scalar equivalence | Existing map laws and injectivity |

## Geometric proof and precise formalization boundary

The common integral family supplies the seven sections
`O`, `(0,0)`, `(b,bc)`, `(c,b-c)`, `(c,c²)`, `(b,0)`, `(0,b)`.
Their specialization identifies the chosen cyclic subgroups across
characteristics. It is not a field homomorphism or a morphism between fibers.
On these seven-element groups it can be evaluated by a finite table; nothing
here gives an efficient analogous lift for arbitrary large subgroup order.

The smooth projective cubic cover has genus 4 in these two good fibers.
The points above `(0,0)` and the origin have ramification index 3. Hence
`π*([P]-[O])=3[(0,0)-∞]`, while pushforward sends `[(0,0)-∞]` to `[P]-[O]`.
After specialization, the composite sends the source generator to `[3]R`.
The scalar is unchanged when the target generator is `[3]R`.

Lean checks the actual finite fields, elliptic point groups, affine cover
equations and the resulting concrete scalar-preserving homomorphism. Mathlib
does not supply a usable hyperelliptic Jacobian/pullback specialization model
in this pinned environment. The genus, divisor pull/push and common-family
interpretation are a written geometric proof, not claimed as a fully formal
Jacobian construction. Do not replace those missing geometric objects with
an arbitrary product group or label this a GHS implementation.

The target is weaker by explicit seven-element enumeration; no GHS
vulnerability or runtime advantage over the already small source subgroup
is claimed. A larger prime-order example remains a separate construction.

## Sources checked

KB tools are absent; `scripts/research-kb status` exits 127. Fallback sources:
the pinned Mathlib subfield, affine-point and cyclic-group modules; repository
`research/chained_transfer_proof_20261112/PROOF.md` (degree identity and
complementary-quotient zero composite); `docs/GHS_TRANSPORT.md`; and
`src/cryptanalysis/ghs_quadratic.rs`. The last file contains a bounded quadratic
GHS constructor despite older documentation calling higher-magic construction
unavailable. No run or theorem from its comments is inferred without checks.

## Checked status and theorem map

The complete five-module project passes the unchanged repository gate:
156 audited theorems, zero failing, zero trusted native exceptions. The first
137-theorem receipt, before the integral-lift companion, is retained separately.
Explicit audits report only `propext`, `Classical.choice`, `Quot.sound`.
All earlier source modules and verification receipts remain unchanged.
[pcc:unverified]

| Claim | Declaration(s), prefix `CoverDecodeExample` | Formalization |
| --- | --- | --- |
| Four-element field and smaller target | `small_card`, `target_smaller` | Kernel checked |
| Concrete family fibers | `source_family`, `target_family` | Kernel checked |
| Rank-three cover algebras | `sourceCoverBasis`, `targetCoverBasis` | Typed basis witnesses |
| Cubic root relation | `source_cover_root`, `target_cover_root` | Kernel checked |
| Affine cover maps | `source_cover_equation`, `target_cover_equation` | Kernel checked |
| Exact target point orders | `R_order`, `S_order` | Kernel checked |
| Composite multiplier and injection | `phi_generator_multiple`, `phi_injective` | Kernel checked |
| Source challenge image | `phi_challenge` | Kernel checked |
| Scalar equivalence | `transfer_dlog`, `example_dlog`, `scalar_mod` | Kernel checked |
| Complete residue table | `seven_row_table` | Kernel checked for `Fin 7` |
| Universal family identities | `universal_seven_identity`, `universal_tangent_identity` | Kernel checked over `ℤ[X]` |
| Universal cover identities | `universal_source_substitution`, `universal_target_substitution` | Kernel checked over integer multivariate polynomials |
| Fixed integral model and two reductions | `CoverIntegralLift.concrete_good_specializations` | Kernel checked ring maps, exact models and nonzero discriminants |
| Genus four, divisor norms and specialization | Written in `COVER-CONSTRUCTION.md` | Not a formal Jacobian construction |

The companion's instance obligations are `exists_source_root`,
`source_root_relation`, `target_root_relation`, `zPolynomial_degree`,
`integral_root_relation`, `parameter_source`, `parameter_target`, `source_model`,
`target_model`, `source_discriminant`, and `target_discriminant`; their owner is
`CoverIntegralLift`. Its endpoint is `concrete_good_specializations`.
They instantiate Mathlib finite-field Cauchy, `AdjoinRoot`, polynomial degree
and `WeierstrassCurve.map` laws. `CharZero` is inherited from the injective
constant embedding. No generic specialization theorem is asserted here.

The field/cardinality/base-change assembly probe passes after its recorded
corrections. No generic group law is introduced. Four native identity receipts
are emitted by `identity_receipts.cpp`, with exact Lean proof pointers. Rust is
absent locally, so repository Rust replay is not claimed; the C++ issuer checks
32 derived points per identity and refuses four perturbed negative controls.
Statement ids and proof hashes are checked independently as metadata.

Raw logs are versioned under `receipts/cover-20261115T2113Z`; declaration-bound
publication claims `PCC-20261115-1148a8` and `PCC-20261115-97a477` remain unsigned.
The unpublished claim and ledger bytes survive in their own archival receipts.
[pcc:unverified]
