# Universal finite cubic norm and explicit coefficient inverse

For every x=(a,b,c) in Fin223³, the exact full04 coefficient norm vanishes iff x=(0,0,0). Every nonzero tuple therefore has the derived norm unit N^221*N=1 and the explicit two-sided schoolbook coefficient inverse N^221*P. Both unchanged original Lean4.35/4.34.1 gates audit37 declarations/0 failing/0 trusted each; twenty new statements have complete original-extractor bindings121d9f. Claims remain unsigned. [pcc:unverified]

## Exact representation and primary review

The source preserves the complete Constructor01 prefixe81532c9, including no_base_cubic_root_three, and copies four full04 definitions byte-for-byte: coefficientProduct, cofactor, coefficientNorm and scale. Put N=a³+3b³+9c³−9abc and P=(a²−3bc,3c²−ab,b²−ac). The product is the schoolbook reduction by z³=3. No ambient field/domain assumption or norm inverse is supplied as a hypothesis. The final theorem requires only that the input tuple is nonzero. [pcc:unverified]

Sources checked before new codee9d4fe: complete Constructor01/full04/Word07 and the already retained official Arkworks algebra v0.5.0 cubic-extension implementation. Arkworks already uses the established cofactor inverse and calls the base inverse on the denominator; the new component discharges this mathematical finite norm/unit condition at223. The paper cited by that source was not reviewed, and no novelty claim is made. KB status/exact/negative/adjacent/theorem wrappers and conductor127, locked atlas unread; no absence inference. [pcc:unverified]

## Why zero norm forces zero coordinates

The base certificates are checked only on223 base elements: t=0 or t^221*t=1, and t³=0 implies t=0. The earlier no-cube-root certificate excludes u³=3 in Fin223. No223³ coefficient population is enumerated. [pcc:unverified]

Let p1=3c²−ab and p2=b²−ac. The source proves the ring identity. [pcc:unverified]

    p1³ − 3*p2³ = N*(3*c³ − b³).

If N=0 and p2!=0, its explicit base inverse u gives (u*p1)³=3, contradicting no_base_cubic_root_three. Thus p2=0; the same identity and cube-zero certificate give p1=0. The two cofactor relations imply b³=3c³. If c!=0, its base inverse v gives (v*b)³=3, the same contradiction. Hence c=0, then b=0 and finally a=0. The converse follows by substitution. Each ratio step is a low-degree generic commutative-ring lemma with the actual base-unit certificate instantiated afterward, so automation need not expand a variable221st power. [pcc:unverified]

## Explicit inverse without a supplied unit

For nonzero x, norm_zero_iff gives N!=0, and base_unit_certificate then supplies N^221*N=1. Independently, the exact finite schoolbook product satisfies x*P=(N,0,0), scalar multiplication transports through the product, and the product is commutative. Consequently both x*(N^221*P) and (N^221*P)*x equal(1,0,0). The norm-unit assumption in the earlier conditional formula is derived here rather than renamed or assumed. [pcc:unverified]

This proves coefficient-operation identities on every nonzero finite tuple. It does not instantiate a complete cubic CommRing/field typeclass or prove Rust norm/Fp.inv/compiler execution, actual Jacobian/group maps, counters or cost. Full04 and Word07 remain unchanged, so the exact mathematical-to-native interfaces are available for a separate refinement. [pcc:unverified]

## Controls, proof map and rejected attempts

The zero tuple has norm0 and the unit axis has norm1/product1. For reducible parameter d=1, the nonzero tuple(222,1,0) has zero cubic norm. This preserves a genuine zero-divisor case outside the specific noncube3/q223 setting, rather than asserting all cubic parameters work. [pcc:unverified]

| New declaration | Complete unsigned binding |
| --- | --- |
| `CubicFiniteNorm22308.base_unit_certificate` | `PCC-20261116-36ea9c`. [pcc:unverified] |
| `CubicFiniteNorm22308.base_cube_zero` | `PCC-20261116-906992`. [pcc:unverified] |
| `CubicFiniteNorm22308.cofactor_cubic_relation` | `PCC-20261116-de9483`. [pcc:unverified] |
| `CubicFiniteNorm22308.unit_cube_ratio` | `PCC-20261116-2dcf13`. [pcc:unverified] |
| `CubicFiniteNorm22308.zero_norm_cofactor_cubes` | `PCC-20261116-141764`. [pcc:unverified] |
| `CubicFiniteNorm22308.zero_norm_second_cofactor` | `PCC-20261116-f851b5`. [pcc:unverified] |
| `CubicFiniteNorm22308.zero_norm_first_cofactor` | `PCC-20261116-5c24db`. [pcc:unverified] |
| `CubicFiniteNorm22308.zero_cofactors_cube_relation` | `PCC-20261116-895088`. [pcc:unverified] |
| `CubicFiniteNorm22308.zero_norm_coordinate_cube_relation` | `PCC-20261116-628db1`. [pcc:unverified] |
| `CubicFiniteNorm22308.zero_norm_third_coordinate` | `PCC-20261116-904526`. [pcc:unverified] |
| `CubicFiniteNorm22308.zero_norm_all_coordinates` | `PCC-20261116-43a9b6`. [pcc:unverified] |
| `CubicFiniteNorm22308.norm_zero_iff` | `PCC-20261116-2e7c0a`. [pcc:unverified] |
| `CubicFiniteNorm22308.nonzero_input_nonzero_norm` | `PCC-20261116-e288b6`. [pcc:unverified] |
| `CubicFiniteNorm22308.derived_norm_unit` | `PCC-20261116-103800`. [pcc:unverified] |
| `CubicFiniteNorm22308.coefficient_cofactor_product` | `PCC-20261116-99e465`. [pcc:unverified] |
| `CubicFiniteNorm22308.coefficient_product_scale` | `PCC-20261116-00f1a5`. [pcc:unverified] |
| `CubicFiniteNorm22308.coefficient_product_comm` | `PCC-20261116-c67f93`. [pcc:unverified] |
| `CubicFiniteNorm22308.every_nonzero_coefficient_inverse` | `PCC-20261116-434958`. [pcc:unverified] |
| `CubicFiniteNorm22308.zero_and_unit_controls` | `PCC-20261116-b5632d`. [pcc:unverified] |
| `CubicFiniteNorm22308.reducible_parameter_control` | `PCC-20261116-8252cb`. [pcc:unverified] |

The audits include13 old constructor declarations,20 new named and4 new generated equations. Both original gates use one worker/120s/default memory and unchanged default tactic limits; only propext,Classical.choice,Quot.sound occur, no native decision/custom axiom. Source SHA2565f9ad6da74d02cc262e04a4255b31305db48f035d0976f99cfedfa76e31a873d. All complete sources/builds/audits/callers/pins/times remain in evidence/finite-cubic-norm-final-04. [pcc:unverified]

Firste5bbb1 fails because simpa leaves9*a*0*0; second7faa57 adds mul_zero but leavesa³−0. Both entire sources/logs and printed downstream sorryAx are preserved and rejected, Consumer not attempted after each Root failure. The accepted source changes only that one low-degree zero-term normalization to the original grind tactic; no theorem/hypothesis/gate/resource/limit is weakened. [pcc:unverified]

Canonical chain.dot,cubic.dot and F101 psi.dot are unchanged. Original full LP60/240/62/360/witness/seven controls/feedback/post, actual runtime/ABI/admission and matched solving-cost requirements remain open. Active Root278/Consumer12cc CI observation20:55 and authenticated store/full whole-object replay still gate normal integration; no scientific/native/paid/signed promotion follows from a publication or component proof. [pcc:unverified]
