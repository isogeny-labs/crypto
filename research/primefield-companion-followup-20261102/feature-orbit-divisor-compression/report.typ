#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Implicit orbit products: a translation construction
Producer derivation • 2026-11-07 • T-1783

The complete subgroup product has a sparse translation ratio. For an odd prime-order rational subgroup H of a nonsingular short Weierstrass curve over a prime field, let K be its monic kernel polynomial and F(Q)=K(x(Q)). With S nonzero in H and the Miller function normalized at infinity,

#align(center)[F(Q+S)/F(Q) = 2 y(S) K′(x(S)) / f(r,−S)(Q).]

The divisor proof and local-parameter normalization are in DERIVATION.md. Taking the ratio of this identity at two points cancels the constant. This yields a four-product cross ratio using two logarithmic-length Miller evaluations, without constructing K. This is an arithmetic derivation, not a timing result.

#image("flow.svg", width: 100%)

== The interface that remains
The ordinary field-value identity requires points outside H. On subgroup targets F is zero; when H is the full rational point group, there are no rational outside points. A local-leading-term evaluator and a divisor bridge to the current partial geometry residual are required before integration.

== A restricted compression gate
If F equals a constant times a rational q-th power times t primitives, each with divisor support at most s, then s t ≥ r−1. Each simple subgroup zero must occur in some primitive modulo q. Translated Miller primitives have support at most three. This is a bound on this multiplicative representation only, not on finite-set projected identities or general arithmetic circuits.

== Checks and standing
The derivation includes an exact hand-substitution example on y²=x³+1 over F₅ with subgroup order three. No experiment or software test ran. Claim PCC-20261107-528a4d is unsigned producer evidence, not independently verified. The original cohort is unchanged. No statistical graph is applicable.

== Sources
Victor S. Miller, Short Programs for functions on Curves (1986), Algorithm 1: https://crypto.stanford.edu/miller/miller.pdf. Ben Lynn, Computing the Tate Pairing, Miller recurrence: https://crypto.stanford.edu/pbc/notes/elliptic/tate.html. Existing projected-residual and linear-constructor derivations reviewed. KB unavailable in this runtime.
