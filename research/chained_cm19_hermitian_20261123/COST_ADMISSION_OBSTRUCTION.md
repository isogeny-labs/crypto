# Computational admission: a genus-three source does not certify an ECDLP gain

This is a written bound comparison, not a timed experiment or a lower bound.
It accompanies the positive mixed-kernel classification without promoting
an algorithmic advantage. All statements unsigned. [pcc:unverified]

Primary source checked: Claus Diem, *On the discrete logarithm problem for
plane curves*, JTNB24(2012),639-667, DOI
[10.5802/jtnb.815](https://doi.org/10.5802/jtnb.815), Theorem1.
It gives expected soft-O(q) for nonhyperelliptic genus3 class-group DLP
via a degree4 plane model. The expectation is over internal algorithmic
randomness, not a random input family. This is the published2012 theorem,
not merely the earlier2005 heuristic abstract. [pcc:unverified]

Our original input remains a prime-order subgroup of E(Fp), with
ell<=#E(Fp)<=p+1+2sqrt(p). Generic rho therefore already has expected
O(sqrt(ell))=O(sqrt(p)) group cost, before optimized constants and the
strongest reproducible ECDLP alternatives. Moving that same subgroup to
a genus3 Jacobian over the same Fp does not make its order about p^3.
The factor11 in J(C7)(F7) of order11^3 illustrates this distinction.
[pcc:unverified]

Thus the published genus3 soft-O(p) bound, by itself, does not establish a
better ECDLP exponent than the existing soft-O(sqrt(p)) reference. This is
a comparison of available upper bounds, not a proof that every specialized
quartic implementation must be slower or that constants cannot change.
The positive144 mixed4-maps leave p unchanged. A prime field has no proper
subfield, so these Fp self-maps do not themselves supply smaller-field
descent. [pcc:unverified]

Any new benchmark proposal needs a separate mechanism and quantitative
admission showing how it may overcome that reference after charging actual
source conversion, polarized identification/correspondence, map setup,
relations, linear algebra, individual descent, scalar recovery, inverse
replay and memory. Citing a larger ambient Jacobian or fixed map degree
is insufficient. The strongest known direct algorithm remains part of the
baseline. Original EKS-only task ownership and paid STOP are unchanged;
no benchmark is launched on the basis of this bound comparison.
[pcc:unverified]

The next actual-map work is useful as a correctness/construction question,
with the computational-advantage gate held until a specific new mechanism
has its own complete-cost argument. Independent replay, generic Mathlib,
approved original custody and normal CI/merge requirements persist.
Sources checked (KB): unavailable; primary theorem and exact source-bound
results reviewed, no novelty or absence claim. [pcc:unverified]
