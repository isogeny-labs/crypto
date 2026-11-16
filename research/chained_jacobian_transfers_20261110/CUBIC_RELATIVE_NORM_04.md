# Full cubic relative norm, trace and conditional inverse at223

`lean/CubicRelativeNorm22304.lean` identifies the full three-conjugate product and additive trace under CommRing R, IsCharP R223 and z³=3. Both unchanged original Lean4.35/4.34.1 gates pass55 audited/0 failing/0 trusted, with one worker/120s/original static proof memory defaults. Fourteen new named statements have complete bindings through the unchanged original PCC extractor, data60bdae. Claims remain unsigned. [pcc:unverified]

## Full expressions and the original cofactor

Let e be the canonical Fin223 embedding and E_z(a,b,c)=e(a)+e(b)z+e(c)z². The exact frozen03 byte prefix supplies the full q-power action and derived encoding injectivity. With q=223, the new statements identify the following expressions. [pcc:unverified]

    P(a,b,c) = (a²−3bc, 3c²−ab, b²−ac),
    n(a,b,c) = a³+3b³+9c³−9abc,
    E^q · (E^q)^q = E_z(P(a,b,c)),
    E · E_z(P(a,b,c)) = e(n(a,b,c)),
    E · E^q · (E^q)^q = e(n(a,b,c)),
    E + E^q + (E^q)^q = e(3a).

The proof transports the two diagonal conjugates through the canonical coefficient embedding, reduces products with z³=3 and computes the stated cofactor. Multiplying by E gives the determinant norm already used in the frozen cubic non-square obstruction. The full norm theorem then explicitly reassociates multiplication and rewrites by these two established identities, without expanding powers of degree49,952. No field or domain hypothesis is assumed. [pcc:unverified]

The coefficientProduct definition states the complete schoolbook product reduced by z³=3, in the exact coefficient order of original Kf::mul. Its encoding theorem proves that the encoded product equals the product of the encodings. A separate finite-coefficient polynomial theorem proves n(x·y)=n(x)n(y). Scalar encoding and negation/subtraction transport are proved explicitly. These are algebraic coefficient models; raw UInt64 operation trees and compiler/counter correspondence are not supplied by them. [pcc:unverified]

## A supplied base-norm inverse gives a two-sided inverse

For s in Fin223 with s·n(x)=1, certified_norm_inverse proves both E_z(x)·E_z(sP(x))=1 and E_z(sP(x))·E_z(x)=1. The premise is an explicit base-ring inverse certificate. The proof uses scalar encoding, the cofactor identity and canonical embedding of1; it does not assume the desired ambient inverse. [pcc:unverified]

This is the algebraic structure of original common.rs832fd288 Kf::inv: obtain the two conjugates, multiply them to p, compute the constant coordinate of a·p, invert that base value and scale p. Constructing the actual extension field, proving every nonzero input has nonzero norm, identifying the native returned base inverse and proving full Rust UInt64/powmod/compiler/counter refinement remain open. No native invocation or measured runtime result was produced. [pcc:unverified]

## Controls, complete bindings and rejected first source

The kernel-checked nonadditivity control gives n(2,0,0)=8, whereas n(1,0,0)+n(1,0,0)=2 modulo223. The zero control gives n(0,0,0)=0 and excludes every base inverse certificate there. No223³ coefficient population is enumerated. A multiplicative algebra norm does not by itself provide an additive elliptic/Jacobian group map or an ECDLP transfer. [pcc:unverified]

| New declaration | Complete unsigned binding |
| --- | --- |
| `CubicRelativeNorm22304.coefficient_embedding_zero_one` | `PCC-20261116-a15596`. [pcc:unverified] |
| `CubicRelativeNorm22304.coefficient_embedding_neg` | `PCC-20261116-555da3`. [pcc:unverified] |
| `CubicRelativeNorm22304.coefficient_embedding_sub` | `PCC-20261116-103c6d`. [pcc:unverified] |
| `CubicRelativeNorm22304.coefficient_embedding_three_nine` | `PCC-20261116-7caeae`. [pcc:unverified] |
| `CubicRelativeNorm22304.coefficient_product_encoding` | `PCC-20261116-214163`. [pcc:unverified] |
| `CubicRelativeNorm22304.scalar_encoding` | `PCC-20261116-4c0ddf`. [pcc:unverified] |
| `CubicRelativeNorm22304.conjugate_cofactor` | `PCC-20261116-c9dacc`. [pcc:unverified] |
| `CubicRelativeNorm22304.cofactor_identity` | `PCC-20261116-e757cf`. [pcc:unverified] |
| `CubicRelativeNorm22304.full_relative_norm` | `PCC-20261116-18eae5`. [pcc:unverified] |
| `CubicRelativeNorm22304.full_relative_trace` | `PCC-20261116-8f29fb`. [pcc:unverified] |
| `CubicRelativeNorm22304.coefficient_norm_multiplicative` | `PCC-20261116-a9c34c`. [pcc:unverified] |
| `CubicRelativeNorm22304.certified_norm_inverse` | `PCC-20261116-efe59c`. [pcc:unverified] |
| `CubicRelativeNorm22304.norm_nonadditivity_control` | `PCC-20261116-7a0413`. [pcc:unverified] |
| `CubicRelativeNorm22304.zero_norm_no_inverse_control` | `PCC-20261116-ba9d4d`. [pcc:unverified] |

There are49 named statements and six generated equations in the full source:35 earlier named,14 new named; earlier basisFrob/encode equations plus coefficientProduct/cofactor/coefficientNorm/scale. Only propext, Classical.choice and Quot.sound occur in the unchanged original audits. Source SHA256`e804f278901b824feaa29361636ba5c6f20dc41015a72a0196816d6df13556e7`, exact03 prefix9bd383b7e9d3ad241fa92e24d3b853cdba7011bf36d0cbe66fec89a7999e5ead. [pcc:unverified]

First original Root attempt8a55c6 failed1 in full_relative_norm: grind expanded the nested powers beyond its polynomial-degree threshold49,952 and the failed theorem printed sorryAx. The complete rejected source/9,108-byte build log remain immutable; Consumer was not reached. The accepted source changes only that derivation to explicit multiplication reassociation and exact cofactor rewrites, preserving the original theorem and both original gates. [pcc:unverified]

Sources checked: frozen03/02 coefficient embedding/Frobenius/injectivity, CubicNormDescent determinant/square obstruction, CubicNormWordBridge01 raw-operation boundary and whole original common.rs832fd288 Kf::mul/frob/inv; actual Lean4.34/4.35 Semiring.mul_assoc and ring/characteristic interfaces. Primary reviewa4c32b records the competing hypothesis/controls before editing. KB MCP absent, status/exact/negative/theorem wrappers127, conductor127 and locked lead-atlas access; no novelty/absence inference. [pcc:unverified]

Canonical chain.dot, cubic.dot and reports/prime-field-actual-04/psi.dot were read and retain their existing scopes: this adds coefficient-ring algebra at223 and no new curve, Jacobian edge, F101 subgroup or computational price. Their exact bytes are unchanged. The new cofactor diagram and source-linked PDF explain this separate obligation. Original scientific LP60/240/62, original360/witness/seven controls/feedback/post, full runtime/admission and matched construction/transfer/solve/recovery costs remain separate. Paid STOPe42949 persists. [pcc:unverified]
