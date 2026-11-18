# Explicit polarized quotient modules, prior to geometric reconstruction

[pcc:unverified] Uses the ordinary cyclotomic/Frobenius identification and canonical pairing premises from prior stages. Arithmetic exactness is checked; geometric interpretation and finite-field reconstruction remain independently unreviewed.

## Source lattice and CM type

In O=Z[zeta7], xi_a=(zeta^a-zeta^-a)/7 generates the inverse different and is totally imaginary. For embeddings indexed by e in1..6, its imaginary sign is the sign of sin(2*pi*a*e/7). The p-adic CM type for pi=P(zeta) is the3 embeddings with P(zp^e)=0 modulo421, using the chosen prime zp and complex embedding consistently. Since421 splits completely and pi*barpi=421, these are precisely the positive-valuation embeddings. Choose a such that every a*e mod7 is in{1,2,3}; this gives positive xi_a on that type with no floating-point sign test.

The alternating form E(x,y)=Tr(xi_a*x*bar(y)) is integral and unimodular on O; the gate computes it rather than assumes a standard symplectic matrix. The sign convention is E(x,i*x)>0 on the associated complex vector space. Consistency of the trace/polarization convention with the chosen CM type is required; merely finding any unimodular alternating matrix would not suffice.

Classnumberone makes the source rank-one O-lattice principal. The totally-positive-unit square property from the prior theorem implies uniqueness of the principal polarization for a fixed CM type up to O-linear isomorphism. With pi retained, this supplies the proposed identification with the source Jacobian's polarized ordinary module. It does not supply a coordinate isomorphism or a geometric point converter. Canonical lift/reduction choices must remain explicit in a later reconstruction.

## CRT kernel to integral over-lattice

Lagrange projectors in O/659 give sigma eigenvectors in the actual O powerbasis. Their dual vectors are normalized with the computed E, not a guessed coordinate pairing. Take the mixed isotropic subspace H for gamma55/slope1. The second representative gamma12/slope555 is its predicted isometric partner.

Let M be the preimage of H under O->O/659, with integer basis columns B. Then [O:M]=659³ and the quotient lattice is L=M/659. The quotient polarization is659E on L, so its integral Gram is B^T*E*B/659. Isotropy gives integrality and the index gives determinant1.

The integral matrices F=B^-1*pi*B and V=B^-1*barpi*B preserve L; FV=421 and F^T*G*F=421G. Matrix Q=659*B^-1 describes the source inclusion O->L in these bases. It has determinant659³, intertwines F, and satisfies Q^T*G*Q=659E. This is an explicit isogeny in the ordinary module category; it is NOT an evaluable map on Mumford divisors.

## Exact partner isometry

Use u=w/659 from kernel659-order. The candidate matrix U=B2^-1*w*B1/659 must be integral/unimodular and satisfy U^T*G2*U=G1 and U*F1=F2*U. These are exact checks of the proposed lattice identification, stronger than checking a permutation of abstract slopes alone. Sigma must fail to preserve each new lattice, consistent with the smaller multiplier order.

## Sources and remaining boundary

Primary: Marseglia, Computing square-free polarized abelian varieties over finite fields, https://arxiv.org/pdf/1805.10223 , (5.1), Theorem5.4 and section8. It describes the ideal/polarization/period framework used here. Our formula, normalization and representative computation are a specialization, not a theorem quoted from that paper. Prior Milne/Deligne background and index/classnumber premises are in the preceding stage notes.

The gate does not compute a symplectic normal form or period matrix, approximate theta constants, prove finite-field non-hyperellipticity, construct a curve or transport a target. Integral module data alone cannot establish a10x solve. Those are the next constructive and cost obligations, with all genus2/4 and independentvalidation obligations retained.
