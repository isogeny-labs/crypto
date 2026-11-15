# Per-attempt envelope and the remaining algorithmic hypothesis

Let s generators in v variables have degree at most D over Fp. Put
M=binom(v+D,D). Ignore the smaller fixed engineering caps when describing the
mathematical algorithm; a capped implementation returns unknown on overflow.
All assertions below are producer-local mathematical arguments, not a checked
complexity theorem or a claim that the needed family hypothesis holds.
[pcc:unverified]

A forward echelon basis has at most M independent pivots. In a graded order,
any nonzero linear combination has the leading monomial of its greatest
participating pivot: no smaller pivot can cancel that term. Consequently its
degree is the maximum degree of a participating pivot. The span in degrees
below D is therefore spanned by the pivots of degree below D. Multiplying
each such pivot once by each variable, and absorbing new pivots, computes the
bounded-degree polynomial-calculus closure when its queue empties. This is a
standard degree-fall closure argument, not a first-fall-degree upper bound.
[pcc:unverified]

There are at most s+vM source/product rows. Each has at most M monomials and
undergoes at most M pivot cancellations. Sparse arithmetic, leading-term scans
and scalar proof references therefore use polynomially many field operations
and stored entries in s,v,M. A proof DAG has at most one raw node per attempted
row and one linear-combination node per independent pivot. It avoids expanding
the original-input multiplier polynomials, whose degrees can grow after falls.
Compaction affects only unused proof nodes, never the source equations or
acceptance obligations. [pcc:unverified]

Selected minimal-leading pivots together with original generators give at most
M+s candidate polynomials of degree<=D. Completion checks have degree<=2D.
Every monomial of degree<=2D is a product of two monomials of degree<=D, so
binom(v+2D,2D)<=M². Division strictly decreases the leading monomial and hence
uses polynomially many steps in M,s. Source replay, Buchberger checks and
original inclusion have a polynomial field-operation envelope. Bit complexity
also charges coefficient arithmetic in log p. Accepted identities establish
the complete original ideal; failed checks merely return unknown.
[pcc:unverified]

For the parent balanced norm presentation, v=O(mt), s=poly(m,t), and a proposed
D=O(m+t) would give log M=O(sqrt(n)log n) when m,t=O(sqrt(n)), n=log2 r.
The missing positive-instance hypothesis is that closure at an explicit such
D contains a complete basis on a sufficiently large effective subset of
targets, with a bounded quotient and verified relation extraction. The present
one-fixture result supplies no numeric growing-family forecast for that event.
The older flat-input rowspace hypothesis is sufficient but stronger than this
intermediate-degree formulation; its unknowns do not settle this formulation.
[pcc:unverified]

Apply the existing selective-oracle contract only if construction F(n), base
bound Bmax, fixed capped oracle work T(n), and fixed-base verified yield theta
are explicit computable bounds. With K=Bmax+1 and N=ceil(4K/theta), one block
costs at most F+N(T+poly(n))+Bmax³poly(n). Fresh base/recovery restarts charge
the inverse good-base probability and kernel nonzero-denominator probability.
The oracle never sees mixed-query labels a,b, so the parent label-hiding
argument applies. It does not require fast refutation of every target.
[pcc:unverified]

Neither the required degree/yield hypothesis nor a large-prime end-to-end
implementation is established here. Source arithmetic, canonical parsing,
actual grevlex, source/semantic binding and verifier program correctness also
have no new Lean proof in this study. The parent's unit-ideal instance and
shared retained-norm laws remain their stated limited formal support.
[pcc:unverified]
