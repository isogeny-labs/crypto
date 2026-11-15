# Proposed nonconstant-B exact control

Proposed algebraically constructed correctness fixture, pending its own Lean
checks. This is not an empirical timing/search experiment or a complexity
claim [pcc:unverified].

Reuse E/F17: y^2=f=X^3+2X+2, the existing prime-order19 curve. Choose two
line polynomials l1=2X+8 and l2=4X+6. The intended construction is

    (y-l1)(y-l2) = A-yB,
    A=f+l1*l2 = X^3+8X^2+12X+16,
    B=l1+l2 = 6X+14 = 6H, H=X+8.

Then its norm should be

    A^2-fB^2 = (f-l1^2)(f-l2^2)
             = (X-5)(X-6)(X-10)*X*(X-3)(X-13) = G.

The six distinct x nodes are0,3,5,6,10,13. Proposed line intersection points
are(5,1),(6,3),(10,11) and(0,6),(3,1),(13,7). The target branch is Q=(0,6):
A(0)=16=6*B(0). Let F=A (already monic). This is a=3,b=1,d=6, matching
canonicalMatrixPencil rather than the earlier a=2,constant-B control
[pcc:unverified].

A proposed explicit Bezout certificate for UF+V(fH)=1 is

    V=X^2+6X+4,
    U=16X^3+11X^2+6X+12.

Before citing this as a result, prove the exact identities over F17, degrees,
monicity, distinct nodes and branch evaluation. Then instantiate the actual
canonical matrix-pencil theorem. This is a small kernel-checkable fixture
constructed from known lines; no random search, host-dependent timing, EKS
job or budget override is needed to verify polynomial identities
[pcc:unverified].

For the later actual branch/corank transfer, F(0)=16 is nonzero. The full
coordinate dimension is6 and the branch dimension should be5. The general
radical/branch theorem should imply restricted rank4 only after those missing
transfers are supplied. Do not assume the rank, use a numeric-rank oracle, or
count pencil membership as recovery [pcc:unverified].

Sources for this proposal: existing exact F17 curve fixture and the elementary
product/norm identity of two line functions. No novelty or algorithmic speed
claim. Native/Lean checks are owed before promotion.

## 2026-11-13 exact validation addendum

NormNonconstantControl now checks the proposed construction, two cubic factors,
actual norm, nonconstant B, explicit Bezout, branch/nonzero evaluation,
monicity/degrees, six distinct nodes and supplied curve points. It instantiates
the actual canonical matrix pencil with dimension6. Source audit143 and fresh
import-inclusive replay complete; all producer standing remains unsigned. See
../nonconstant-report/README.md for exact scope and evidence. The original
prediction above is preserved [pcc:unverified].

Restricted rank/corank/adjugate recovery remain open. Their proposed exact
values are preregistered in ../nonconstant-report/NEXT_RESTRICTED_CONTROL.md;
they are not promoted by the present pencil-membership check [pcc:unverified].
