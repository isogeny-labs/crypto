# Character-phase canonical recovery

Let r be the caller-certified odd prime subgroup order, h=(r-1)/2,
m>=3 divide h, and L=h/m. The public character source and its base-field
assumptions are unchanged. After its nonzero initial output s=chi(k), put
x=k/b0, V=x^m and epsilon=s*chi(b0). Then V^L=epsilon.
This character constraint is sufficient for the canonical lift; no fourth root
of -1 or divisibility of r-1 by 4m is required.

## Domain and orientation

In W=V+V^-1 the domain is squarefree(D_L(W)-2epsilon), where
D_j(V+V^-1)=V^j+V^-j. The character constraint is invariant under V inversion
because epsilon=epsilon^-1. Both admissible orientations can therefore share
one canonical branch per W factor. CanonicalFiber still separates ramified
W=+/-2 and any one-orientation fibers created by subsequent observations.
The lift uses exponent L and phase epsilon; it does not call the old
quarter-exponent constructor. The lift routine's fourth-power phase check
accepts epsilon=+/-1 and its polynomial filter enforces V^L-epsilon.

Write S_j=U_j(W/2), S_-1=0. For odd L=2t+1 the squarefree domains are
(W-2)(S_t+S_(t-1)) for epsilon=1 and
(W+2)(S_t-S_(t-1)) for epsilon=-1. This includes L=1.
For even L=2n, positive character retains the two disjoint squarefree
D_n-2 and D_n+2 factors built by the predecessor second-kind formulas.
Negative character has the squarefree domain D_n. When n is even and the
bounded public trace search returns checked s2 with s2^2=2, it partitions
D_n=(D_(n/2)-s2)(D_(n/2)+s2). Unlike its predecessor, each factor occurs once,
with character label -1, rather than once for each quartic phase.
All identities are polynomial identities in characteristic r>L; squarefree
and disjointness controls cover both endpoints and both character signs.

## Updates and transport

At radius b, let t=b^-m, t0=b0^-m and c=t/t0. The current variable is cV,
with character epsilon*c^L. This transported character is used in the
streaming signed equation; squaring epsilon here would lose its sign.
The signed relation retains sigma=(-1)^h, including odd h.
The powered evaluator directly uses A^2*(cV)*F1(cV)*F_(m-1)(cV),
A=m*b^h. Both paths impose mode0 before the signed product.
The initializer calls absorb with first=false because it has not already
imposed mode0. Scalar recovery retains history replay and final point checks.

## Costs and bounds

The slot cap bounds per-branch section output, not total memory. Insufficient
section capacity selects streaming on the same character domain. Domain
construction, canonical squarefree checks, phase powering, all field
inversions, root-search trials, copies, allocations, lifting, source calls,
replay and point verification remain charged. Phase polynomials still grow
with L. No timing ranking follows from fewer orientation branches.

## Sources checked

The preserved predecessor files reciprocal.rs, canonical.rs, signed.rs,
transport.rs, seeded-image.rs, phase-domain.rs and split-negative-initial.rs
provide the implementation contracts. NIST DLMF 18.5.1-2 provides the
first/second-kind identities: https://dlmf.nist.gov/18.5 . The prior literature
review is retained in JOURNAL.md. Research KB was unavailable in this runtime;
no novelty claim is made. Scalar enumeration appears only in qualification
oracles, never in the public recovery adapter.
