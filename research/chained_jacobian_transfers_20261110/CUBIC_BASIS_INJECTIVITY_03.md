# Derived cubic basis injectivity at223

`lean/CubicBasisInjectivity22303.lean` derives encoding injectivity and the fixed-expression equivalence under only CommRing R, IsCharP R223 and z³=3. It preserves the complete frozen02 source as an exact byte prefix. Both unchanged original Lean4.35/4.34.1 gates pass37 audited declarations/0 trusted. There are12 new named statements,23 earlier named statements and two generated equation theorems; the new use of encode in simp materializes encode.eq_1. Whole proof evidence and twelve complete original-extractor bindings are in `evidence/cubic-basis-injectivity-final-03`, data97a994. Claims remain unsigned. [pcc:unverified]

## Three equations separate coefficients

Write e for the canonical Fin223 embedding and E_z(a,b,c)=e(a)+e(b)z+e(c)z². The frozen full-expression theorem identifies the three Frobenius images with the following coefficient combinations, using A=e(a), B=e(b)z and C=e(c)z². [pcc:unverified]

    y0 = A+B+C,
    y1 = A+183B+39C,
    y2 = A+39B+183C.

In characteristic223,1+183+39=0,183²=39,39²=183,183*39=1 and3*149=1. The inverse combinations are A=149(y0+y1+y2), B=149(y0+39y1+183y2), C=149(y0+183y1+39y2). Therefore equality of the three images implies equality of A,B,C. The formal three_coefficient_separation lemma proves exactly this structural implication in every stated commutative ring. [pcc:unverified]

## Cancel the generator and recover the coefficients

The relation z³=3 implies z has inverse149z². The formal cancellation lemmas prove uz=vz implies u=v and uz²=vz² implies u=v. Given two equal encoded expressions, apply q-power twice and the frozen full-expression formula to obtain the three equations. Separation recovers e(a)=e(d), e(b)z=e(e)z and e(c)z²=e(f)z². Cancel z or z², then use canonical embedding injectivity to recover all three coefficients. This proves encode_injective without a domain, a field structure or an assumed basis-injectivity hypothesis. [pcc:unverified]

The resulting fixed_expression_without_injectivity states E_z(a,b,c)^223=E_z(a,b,c) iff b=c=0 under exactly the ring, characteristic and cubic-relation hypotheses. It applies the frozen conditional fixed-expression result with the newly derived injectivity theorem. No223³ coefficient enumeration is performed. [pcc:unverified]

## Complete statement bindings and negative control

Six transparent type-annotated corollaries restate the canonical embedding results without named arguments inside declaration types. The unchanged original PCC extractor now reaches each actual declaration assignment and binds the full statement. The six older truncated records and additive correctiona1a45d remain immutable and incomplete; no verifier, keyring or policy is changed. [pcc:unverified]

| New declaration | Complete unsigned PCC binding |
| --- | --- |
| `CubicBasisInjectivity22303.coefficient_embedding_injective` | `PCC-20261116-dbd721`. [pcc:unverified] |
| `CubicBasisInjectivity22303.coefficient_embedding_add` | `PCC-20261116-ac06d3`. [pcc:unverified] |
| `CubicBasisInjectivity22303.coefficient_embedding_mul` | `PCC-20261116-da7ef4`. [pcc:unverified] |
| `CubicBasisInjectivity22303.coefficient_embedding_pow` | `PCC-20261116-a30e60`. [pcc:unverified] |
| `CubicBasisInjectivity22303.coefficient_embedding_fixed` | `PCC-20261116-02d8f1`. [pcc:unverified] |
| `CubicBasisInjectivity22303.coefficient_embedding_constants` | `PCC-20261116-7d9c90`. [pcc:unverified] |
| `CubicBasisInjectivity22303.three_coefficient_separation` | `PCC-20261116-74d5d0`. [pcc:unverified] |
| `CubicBasisInjectivity22303.cubic_generator_cancel` | `PCC-20261116-1991d0`. [pcc:unverified] |
| `CubicBasisInjectivity22303.cubic_squared_generator_cancel` | `PCC-20261116-7ec033`. [pcc:unverified] |
| `CubicBasisInjectivity22303.encode_injective` | `PCC-20261116-b73b20`. [pcc:unverified] |
| `CubicBasisInjectivity22303.fixed_expression_without_injectivity` | `PCC-20261116-1b1e8b`. [pcc:unverified] |
| `CubicBasisInjectivity22303.zero_generator_not_injective` | `PCC-20261116-ec3f56`. [pcc:unverified] |

The zero_generator_not_injective control proves that over Fin223, encode at z=0 maps distinct tuples(0,1,0) and(0,0,0) to the same element. It documents a concrete failure when the cubic relation is removed. It is a kernel-checked finite control, not a native run. [pcc:unverified]

Source SHA256`9bd383b7e9d3ad241fa92e24d3b853cdba7011bf36d0cbe66fec89a7999e5ead`. The full frozen02 prefix remains5198ffb8ef0945dd4754255ef2f37d420ca9ad7f4064d03f799c8ca593011edf. Both original gates use one worker,120s and original static proof memory defaults. Only propext,Classical.choice,Quot.sound occur; no new axiom/hole/native decision is accepted. The isolated three structural lemmas also compile on both original toolchains under20s, custody2fd3a0. [pcc:unverified]

Sources checked: exact frozen01/02 declarations and original common.rs832fd288 diagonal Frobenius, official actual Lean characteristic/power/canonical-cast interfaces. Primary review332d4d records the alternative hypothesis, acceptance criteria and zero-generator control. KB MCP unavailable, status/two queries/theorem wrappers127 and conductor127; no novelty or absence inference. [pcc:unverified]

All earlier02 tactic/toolchain/induction/render failures and limited binding records stay preserved. Constructing an actual extension field, relating it to native UInt64/powmod/compiler/counters and actual Jacobian maps, and admitted matched computational costs remain separate. Dedicated exact-source CI retains the actual source/head/workflow/original tools and every partial/full proof outcome. No native population, timing result or paid launch was performed. [pcc:unverified]
