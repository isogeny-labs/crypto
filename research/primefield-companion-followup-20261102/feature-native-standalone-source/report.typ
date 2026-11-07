#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= Standalone projected character source
Native implementation and producer proof • T-1788 • 2026-11-07

The query path now evaluates projected geometry and its required division factors directly. It shares one value-only cache between torsion validation and factor evaluation. The old full-value source, derivative normalization and transported paired validator are not normal-build dependencies. Existing half-GCD geometry is reused.

#image("flow.svg", width: 100%)

== Mathematical basis
Let B(T)=ψ(r+1)(T)²ψ₂(T)/ψ(r+2)(T). Ward quasiperiodicity and the division chain rule yield B([n]T) multiplied by ψₙ(T) raised to r² equals B(T) raised to n². Therefore C(T)=π(B(T)) inverse satisfies C([n]T)=π(ψₙ(T))C(T) to power n² in the order2 or4 image. Also C(−T)=−C(T).

Multiplication by k permutes the nonzero subgroup's pairs up to signs. The product of those signs is k to power (r−1)/2 modulo r. Thus the normalized half-product of C returns the scalar quadratic character. The previous block-pairing identity equates that half-product with the implemented geometry expression. DERIVATION.md writes out the cancellation and sign proof; no derivative-normalization assumption is needed.

== Software evidence
Three selected native methods pass;91 imported methods are filtered out. All496 nonzero points of the order7,13,479 fixtures match scalar-character and old-source checks, with raw projection equality. All496 covariance checks pass. Identity, allocation cap and invalid-domain controls pass. Twenty-four dependency hashes are pinned before and after.

A separate normal metadata build emits dep-info excluding the full-value source, derivative jets and paired transport. Test and normal builds retain10 and12 unused-code/import warnings respectively. No test failed. PCC-20261107-61e379 is unsigned producer evidence; shared arithmetic precludes independent-review claims.

== Scope and next integration
The component exposes single-point raw/sign APIs. Caller-certified prime parameters, fixed-width bounds, coprime domain and projection order2/4 are retained. Geometry allocation is capped before construction. No timing gain has been measured; the square-root-scale geometry remains. Next connect this source to the public-point decoder while preserving final group verification, then address prepared-window sharing and complete counted comparison. Original4563-case cohort unchanged.

== Sources
Lauter–Stange2008, Theorem4, https://arxiv.org/pdf/0803.0728; Stange2026, division chain rule, https://link.springer.com/article/10.1007/s40993-026-00741-2; current finite-fold and projected geometry derivations. KB unavailable. All computation remains in the original prime field; no cloud experiment ran.
