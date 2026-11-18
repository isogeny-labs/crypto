# Ramified genus2 l5 polarized admission

Prospective assertions [pcc:unverified]. Original y²=x⁵+7 over F601,
existing conditional Frobenius pi=P(sigma), no replacement curve/target.
In O/5=F5[t]/t⁴ choose t=sigma-sigma^-1, conjugate(t)=-t.
Prior rank2 Frobenius algebra implies pi+1 has order>=2 in t.
Norm pi*bar(pi)=601=1mod5 forces pi+1 anti-invariant because its
square and conjugate product vanish. Thus pi+1 is nonzero multiple t³,
rank1 and squarezero. Prediction: exactly6 stable maximal isotropic planes.

Let R=im(pi+1). Symplectic rank1 nilpotence implies ker(pi+1)=Rperp.
A stable Lagrangian either maps onto R or lies in Rperp; either way
contains R. Conversely every Lagrangian containing R is stable.
Such planes correspond to lines in symplectic Rperp/R of dimension2,
so number=5+1. This predicts a count, not quotient self-isomorphism.

Checker reconstructs unchanged pi using frozen fast::prepare; reproduces
exact charpoly, rank2 algebra, rank1(pi+1), squarezero. Enumerates all806
dimension2 subspaces in F5⁴ by unique RREF and all5^6 alternating forms.
For each nondegenerate sigma-invariant form, checks Frobenius invariance,
156 total Lagrangians and6 stable Lagrangians all containing R. The actual
canonical Weil form belongs to this set by curve automorphism invariance;
no arbitrary choice of one form is substituted for that premise.

Standalone producer self-check, not independent Frobenius reconstruction
or geometric pairing computation. Norm-witness/KERNEL-NEXT-GATES sources
Milne10.23, Weil pairing/Rosati background apply; no novelty claim.
Reuse copied integer routines from frozen frobenius-primes/main.rs;
freeze native/import/source/protocol/binary hashes and Git push before
one120CPU-second invocation. Retain failed outputs, no rerun/S3/cloud.
No speed measurement. After counts, classify polarized quotients and only
price transport if a concrete complete-cost advantage exists.
