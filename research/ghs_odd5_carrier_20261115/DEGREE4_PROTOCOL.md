# Frozen degree-four branch-pole screen

15 November 2026. This is a bounded constructive search on the exact `t=+26` witness. It is not an exhaustive degree-four cover classification. [pcc:unverified]

## Fixed curves and map shape

Take `K=F3[a]/(a^5+2a+1)`, `E:v²=u³+2u²+a`, and `Y:y²=f(x)` with

`f=x¹¹+x⁹+2x⁸+x⁷+2x⁵+x³+x²+x+1`.

Over `K`, `f=(x+1)Q1Q2Q3Q4Q5`, with five monic irreducible quadratics. Freeze the five pole denominators `B=(x+1)Qi` in the factor order returned by Sage for the exact field representation. For each `B`, the remaining four quadratic branch pairs each receive one of the three roots of the 2-torsion polynomial `U³+2U²+a`: the rational root, or either embedding of the conjugate pair. There are exactly `5·3⁴=405` assignments. [pcc:unverified]

Search maps of the form `u=A(x)/B(x)`, `v=y C(x)/B(x)²`, where `deg A=4`, `deg B=3`, `deg C=2`, and `gcd(A,B)=1`. The map equation is

`f C² = B (A³+2A²B+aB³)`.

For each assignment and each of the four non-pole factors `Qj`, impose `A ≡ B·ej (mod Qj)` with `ej` the assigned 2-torsion root expressed in `K[x]/Qj`. Chinese remainder gives a unique residue modulo the product of those four quadratics, of degree below eight. Retain only a residue of degree exactly four and test the displayed polynomial identity by exact division and square testing. An accepted map must also pass genus, source/target group-order, degree and finite-field point identity checks before any subgroup claim. [pcc:unverified]

## Scope and interpretation

The denominator shape forces all three finite poles of `u` to be branch points: the one `K`-rational branch point and one quadratic pair. Degree-four elliptic maps may have a double nonbranch pole, ramification at infinity, a different image of infinity before translation, or a different target model. **Zero accepted rows here would not rule out degree four**, nor the wider Howe window 4–20. [pcc:unverified]

The earlier cubic criterion in `CUBIC_PROTOCOL.md` and Howe's [Theorem 4.17](https://arxiv.org/pdf/2110.04221) inform the rational-function shape and degree window. The shared KB at revision `9141e386254340a9bdba3036f2bb706a` (last refreshed 14 November) returned no exact degree-four odd5 construction for two branch-interpolation queries; its stale index cannot establish novelty. Freeze this protocol and the source script in a commit before running the 405 assignments. [pcc:unverified]
