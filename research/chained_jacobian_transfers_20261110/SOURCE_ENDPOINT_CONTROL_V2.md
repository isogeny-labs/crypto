# Direct elliptic-source endpoint control, corrected recovery scalar

Scientific standing unsigned. This version corrects the recovery scalar in
SOURCE_ENDPOINT_CONTROL.md and its frozen PCC-20261112-d55462 claim. The
original file and failed native21 receipt remain unchanged. In particular,
39 is inverse14 modulo109, while inverse28 modulo109 is74. The isolated
native21 test reached the reverse28 equality and failed at the subsequent
recovery assertion; PCC-20261112-122fda preserves that failure. [pcc:unverified]

Use the global factorization and norm/pullback identities established in
PRIME_FIELD_PSI_PROOF.md/78176c and QUARTIC_PULLBACK_PROOF.md/4934b4. Write
K0,j=R0 P_(0,j), H0,j=N_(0,j)R0hat and K1,j=R1 P_(1,j). Then

    H0,i K0,j = [4] if i=j, and zero otherwise;
    [4]Psi = K1 alpha H0;
    Psi K0,j = K1,j alpha_j.

For the last equality, cancel multiplication by4 in the torsion-free
homomorphism group of abelian varieties. This does not assert that geometric
point groups lack4-torsion. On the implemented rational groups, the equality
also follows from invertibility of4 on their odd order12317. The old global
factorization remains an explicit dependency. [pcc:unverified]

When the public source is an elliptic point P and the source Jacobian class
is K0,j(P), one actual alpha_j followed by the direct T1 pullback yields the
same concrete destination as constructing K0,j(P) and applying Psi. This is
the required direct comparison route for these inputs; the statement does
not evaluate arbitrary Jacobian classes from one elliptic coordinate.
[pcc:unverified]

The target coordinates satisfy H1,i Psi K0,j=[4]alpha_j in the matching
component and zero in the other. Since PsiHat Psi=[7], reversing Psi and
then applying H0 gives[28]P. The public recovery multipliers are

    28*74  = 2072 = 1 + 19*109;
    28*109 = 3052 = 1 + 27*113.

Thus74 modulo109 and109 modulo113 recover P. Neither multiplier depends on
a discrete logarithm. The incorrect historical39 gives28*39=2 modulo109,
so it recovers[2]P and fails on every nonzero point of that group.
[pcc:unverified]

The native22 control retains every109/113 source point, exact destination
equality, injectivity, both norm coordinates, reverse28, every source scalar,
and wrong-quarter rejection. It adds integer inverse assertions before the
point loop. Native acceptance is recorded separately; no timing result is
inferred. Sources checked: the existing seven_chain.rs source-recovery test
already records reverse28/inverse74; the accepted norm/Picard proofs above
and their primary references apply. KB unavailable; no novelty claim.
[pcc:unverified]
