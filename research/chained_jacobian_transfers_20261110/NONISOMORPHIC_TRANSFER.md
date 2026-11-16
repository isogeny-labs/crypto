# Follow-up: explicit nonisomorphic elliptic destination

For char(k) not 2 or 3, let E:y²=x³+1 and C:y²=x⁶+1, with f(x,y)=(x²,y). The standard norm/pullback identity gives f_*f*=[2]. E has rational 2-torsion (-1,0). Shift X=x+1, giving y²=X³-3X²+3X. The degree-2 Vélu isogeny with kernel {(infinity),(0,0)} is

U=X-3+3/X,  V=y(1-3/X²),

to E2: V²=U³+6U²-3U, with the kernel point mapped to infinity. Consequently the explicit chain E→J(C)→E→E2 is φ∘[2]. For any prime-order subgroup of order r≠2, its image has order r provided φ does not kill that subgroup; the degree-2 isogeny has a kernel of order 2, so this is automatic for odd r. The target has j≠0 for p=101 whereas E has j=0, proving nonisomorphism over the algebraic closure in this instance.

`python3 verify_nonisomorphic.py` exhaustively checks E(F_101) (102 points), every ordered pair for the homomorphism identity, target-curve membership, and every scalar in an order-17 subgroup (P=(4,41)). **Local run: PASS.**

This is an explicit *nonisomorphic* destination, but it is necessarily isogenous to E and does not establish easier DLP. The genus-2 leg is proved by the general push-pull theorem, not yet checked with independently implemented genus-2 divisor arithmetic.

## Open mathematical boundary
- A nonzero map between elliptic factors of a split Jacobian is not automatic; nonisogenous simple factors have Hom=0. A genuinely useful cross-factor transport requires compatible isogeny factors and explicit nonzero composite.
- A different characteristic cannot be reached by a chain of algebraic maps over fields. An efficiently computable cross-characteristic abstract subgroup homomorphism would require a different, presently unprovided construction. No unconditional impossibility theorem for all computational algorithms is claimed.
- Next engineering task: independently implement genus-2 Jacobian arithmetic and the actual f* and f_* maps, plus negative controls.
