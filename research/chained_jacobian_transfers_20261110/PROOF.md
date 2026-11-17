# Constructive chained Jacobian DLP transport: proof and limitations

**Theorem.** Over any field k of characteristic other than 2 or 3, take E: v²=u³+1, C: y²=x⁶+1, and E': w²=(z−5)³+1. The smooth genus-2 curve C admits the degree-two morphism f:C→E defined by (x,y)↦(x²,y). The map τ:E→E', (u,v)↦(u+5,v), is an isomorphism. On Jacobians, the chain

E = J(E) --f*--> J(C) --f_*--> E --τ--> E'

equals τ∘[2]. Consequently it transports the discrete logarithm of any point in a prime-order subgroup of odd order r, with no loss of order.

**Proof.** For a finite degree-d morphism of smooth projective curves, the divisor pullback and norm/pushforward satisfy f_*f*(D)=dD, since the sum of ramification indices times residue degrees over each closed point is d. Passing to degree-zero divisor classes gives f_*f*=[d] on J(E)=E. Here d=2. The coordinate translation τ is an isomorphism with inverse (z,w)↦(z−5,w). Hence T=τ∘[2]. If P has prime order r≠2, then ord(T(P))=r, and T([a]P)=[a]T(P). QED.

**Scope:** This proves a concrete, efficiently evaluable, three-stage *same-characteristic* transport. E' is isomorphic to E, so no easier DLP or nonisogenous destination is claimed. The pullback is not an isogeny from a dimension-one elliptic curve onto a dimension-two Jacobian. A second elliptic quotient of C need not yield a nonzero cross-factor map. A prime-to-binary algebraic transfer is not established.

**Test:** `python3 verify.py` enumerates E(F_101) with 102 points, selects P=(4,41) of order 17, and verifies all 17 subgroup scalars under τ∘[2]. The Jacobian identity is proved algebraically, not independently implemented using genus-2 divisor arithmetic.

**Next milestone:** Implement actual genus-2 divisor classes and induced maps; search for a nonzero composite to a nonisomorphic target; benchmark complete cost. This is not yet a practical cryptanalytic improvement.
