# G3 derivation to implement and independently check

2026-11-13. This is a written derivation for the proposed adapter; no new
native execution or completed Lean proof is asserted. See the frozen
[protocol](G3_DESCENT_PROTOCOL.md) and [source catalog](evidence/genus3-primary-01/sources.json).
All mathematical statements here have unsigned standing. [pcc:unverified]

Let q>3 be prime, K=Fq³ and sigma the q-Frobenius. Put u0=t²+bt+c,
u1=u0^sigma, u2=u1^sigma. Assume the existing core accepts distinct roots,
pairwise coprimality, coefficient independence and Ciani smoothness. In its
Ciani coordinates, w=(Z²,X²,Y²) and (s0,s1,s2)=R w satisfy s1²=s0s2.
The three square coordinates are the homogenized u_i(t). The original
Moore coordinate matrix descends C to Fq; retain its exact convention.

## The elliptic model and all inverse branches

Write f=u0u2=t⁴+b3t³+b2t²+b1t+b0. Let
B=b3/2, s=(b2-B²)/2, alpha=b1-2Bs, beta=b0-s². Then
f=(t²+Bt+s)²+alpha*t+beta. The core rejects alpha=0. For A=t²+Bt+s set
x=2(y+A), Y=2xt+Bx+alpha. Direct elimination gives

Y²=x³+(B²-4s)x²+(2Balpha-4beta)x+alpha².

The core's short coordinate is X=x+(B²-4s)/3. If x is nonzero,
t=(Y-Bx-alpha)/(2x), y=x/2-t²-Bt-s. If x=0, the finite point Y=alpha
has t=-beta/alpha and y=-(t²+Bt+s); the point Y=-alpha is the second
quartic infinity. The first infinity is the elliptic origin. In the
local parameter v=1/t the two expansions y/t²→+1,-1 give these two
different limits. A formula returning unsupported at x=0 is incomplete.
[pcc:unverified]

## Completed fibre ideals

The cover pi0 has y²=u0u2 and deck involution changing X's sign.
For finite (t,y) with u0(t) nonzero, its fibre in the plane is

L=Y-(y/u0)Z, Q=X²-(u1/u0)Z².

If u0(t)=0, coprimality makes u1,u2 nonzero and the fibre is
L=Z, Q=X²-(u1/u2)Y². At the two quartic infinities the fibres are
L=Y-Z or Y+Z, Q=X²-Z². The line/conic complete intersection has
length2 and lies on C. When the conic restricts to a square its length
is still2, with its full infinitesimal structure. These ideals require
no choice or extraction of a support root. Apply the existing inverse
Moore coordinate forms to express them in W coordinates. [pcc:unverified]

Let I_i be the conjugate fibre ideal sheaves on C. Since C is smooth,
each fibre is Cartier, so their product is the ideal of the **sum** of
the effective divisors. Their intersection would instead retain only
the maximum local multiplicity. Products of the three choices L_i or
Q_i give eight generators of degrees3 through6. Multiply each by all
monomials of the complementary degree to6. On C these still generate
the same ideal sheaf: outside its support, one product and one suitable
coordinate monomial are nonzero. No separate collision-root search is
needed by this construction. [pcc:unverified]

This span is stable under Frobenius because it comes from a product of
all three conjugates. For each K-generator h, the coefficient traces
Tr(c*h), c in {1,g,g²}, span its Fq descent: the trace pairing of K/Fq
is nondegenerate. In particular these traces generate after extension
back to K. Reduce their degree6 forms with the existing plane-quartic
ring multiplication tables. The existing reducer must establish the
required complete generating space; the sheaf argument alone does not
replace that implementation check or prove a degree bound for every
possible input model. On the fixed fixtures every input is required to
pass its unchanged completeness and rank conditions. [pcc:unverified]

## Actual origin and collision control

The first infinity has Ciani fibres (X,Y,Z)=(+1,+1,1),(-1,+1,1).
Across the three conjugate covers, one sign point is shared by all
three fibres and the other three each occur once. Thus D6(O) has
multiplicities3,1,1,1. A reduced union has degree4, not6. The product
ideal preserves the triple jet automatically; independent point and
local-parameter constraints must confirm it. The canonical origin
shift is h(O), where h(P)=[D6(P)-2Dinf]. Define phi(P)=h(P)-h(O).
This computes the shift from the actual cover and never calibrates it
from identities between sampled scalar multiples. [pcc:unverified]

## Homomorphism and odd-subgroup injectivity

Over an algebraic closure, C has the sign-change V4 action. Its full
quotient is a genus0 conic and its three order2 quotients are the
elliptic curves E0,E1,E2. For quotient pullbacks p_i and norms n_i,
p_i n_i=1+tau_i and n_i p_i=[2]. The sum 1+tau0+tau1+tau2 is zero
on Pic^0 because the full quotient has trivial Pic^0. Consequently
sum p_i n_i=[2]. For i≠j, tau_i acts as -1 on the j-th elliptic
character and +1 on the i-th quotient; hence [2]n_i p_j=0. Hom groups
of abelian varieties have no integer torsion, so n_i p_j=0. This
does not assume the conjugate elliptic curves have coprime orders.
[pcc:unverified]

Let P=sum p_i on the product of the three conjugate elliptic factors
and N=(n0,n1,n2). Both NP and PN are [2]. With the product principal
polarization and the canonical Jacobian polarization the maps are
adjoint, so deg P=deg N=8. The Galois action cyclically permutes the
factors, giving the descended map Res(K/Fq)E0→Jac(C). On rational
points this is exactly P(P0,sigma(P0),sigma²(P0))=phi(P0). In
particular n0(phi(P0))=[2]P0 over K. Thus phi is a homomorphism and
is injective on every subgroup of odd order. An unknown scalar relation
Q=kG maps to phi(Q)=k phi(G), with the same k modulo the odd subgroup
order. No injectivity on the entire even-order group or global inverse
isogeny follows. [pcc:unverified]

The native API need not yet evaluate n0 on an arbitrary canonical
Jacobian class to preserve a DLP scalar; such a norm is a separate
implementation obligation for algorithms that need it. A faster
destination solve and all charged computational comparisons remain
open. Full curve/Picard/ideal-sheaf/descent formalization is not supplied
by the proposed cleared polynomial and conditional group lemmas.
[pcc:unverified]
