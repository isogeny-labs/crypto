# Written successor: canonical rational support and a two-Jacobian chain

This is a written derivation/prospective qualification, not an executed second
Jacobian map, compiled Lean theorem or signed verdict. It does not change the
frozen current elliptic-chain A/B protocol. [pcc:unverified]

## Rational support without a growing extension at each step

Let C/k be smooth genus2 over a finite odd field, with a degree6 model and
every Weierstrass orbit of length greater than2 (the3+3/6 cases here).
Write K for its canonical degree2 class, the hyperelliptic fibre. For any
nonzero D in J(k), Riemann--Roch gives
l(K+D)=1+l(-D)=1. There is exactly one effective divisor Z in K+D. It is
Galois-stable; over a finite field the rational Picard class has no Brauer
obstruction. Hence its support has degree at most2 and lies in C(k^2).
[pcc:unverified]

The degree2 Z cannot contain a Weierstrass point whose orbit has more than2
elements. In an odd chart based at such W(r), since2W(r)~K, the reduced
Mumford representative of D has degree exactly2 and no branch support.
Repeated support is allowed and means a rational non-branch point twice.
An original infinity is also allowed; use homogeneous coordinates.
[pcc:unverified]

Consequently a rational nonzero Jacobian class can be split into its two
actual support points over a single fixed field combining k^2 with the
chosen branch-root field. For3+3/6 roots that field is contained in k^6.
Apply the already written full homogeneous Richelot point correspondence
to each support point, then add/reduce before the next step. This avoids
keeping an exponentially growing geometric support. It is not a new
asymptotic ECDLP algorithm; exact maps and all exceptions still need native
qualification and complete pricing. [pcc:unverified]

For the current odd chart t=1/(x-r),Z=y*t^3/s, a support point (t,Z) maps
to original homogeneous a=r*t+1,b=t,ordinate=s*Z. This is regular even at
t=0. In a separately frozen follow-up, require both support roots to exist
in F729 for every rational class, check the binary curve equation and all
composite map laws. Never turn a failed square root into a discarded target.
[pcc:unverified]

## Monic odd charts need not adjoin a square root

For a squarefree odd polynomial h(t)=c*t^5+... with c nonzero, substitute
t=c*T,Y=c^3*Z. Then Z^2=h(c*T)/c^6 is monic, over the same coefficient
field. For the sextic chart this is directly
T=1/(c*(x-r)), Z=y*T^3 with c=f'(r). A square-root scaling is convenient
for the current F3 fixture but is not intrinsically necessary. This is
standard coordinate algebra, not a measured optimization or a novelty claim.
[pcc:unverified]

For base-field Frobenius q, set a=c^(q-1), b=c^q*(r^q-r). Then
T'=a*T^q/(1+b*T^q). The transformed Mumford formulas use denominator
a-b*z, with weight d for u and3 for v, followed by the same moving-basepoint
class correction. This generic chart would need its own frozen positive,
negative and conjugacy tests; it must not replace current frozen profiles
retroactively. The k^6 support bound here assumes a given k-defined chain,
not merely rational endpoint equations. [pcc:unverified]

## Exact raw-dual control in characteristic3

With cyclic H_i=[G_j,G_k]=G_j*G_k'-G_j'*G_k and descending determinant delta,
the polynomial identities are
[H_j,H_k]=-2*delta*G_i and det(H)=-2*delta^2.
Factor the first codomain as (H0/delta)*H1*H2. Its raw Richelot dual model
is then4*f. The affine correspondence sends the repeated original point to
ordinate2*y, so identifying the raw back model with f in characteristic3
predicts the composite **-[2]**. The canonical y_back/2 isomorphism restores
the usual dual-isogeny [2] relation. [pcc:unverified]

This sign must be frozen and tested, not chosen after observing outputs.
A scalar dual/backtracking route is a negative control, not a fresh transfer
advantage. Classify actual rational second kernels and product cases before
claiming any non-backtracking rational chain; if a six-cycle codomain has
only its dual rational kernel, preserve that boundary. Product recoupling
and endpoint isomorphisms remain separate obligations. [pcc:unverified]
