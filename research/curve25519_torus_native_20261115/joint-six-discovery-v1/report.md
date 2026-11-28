# Joint-six discovery: interpolation obstruction and a product-jet target

24 November2026. Written analysis; all standing producer unsigned/unverified.
No new scientific/native call, Lean endpoint, timing or solver run.

In characteristic different from2, the regular elliptic section space has
pole divisor H=5O+R and basis
1,x,y,x^2,xy,phi_R=(y+s)/(x-r). Normalizing xy leaves the unknown message
in L(4O+R), with received function -xy; normalizing phi_R leaves L(5O),
with received function -phi_R. Both are D=H-T,deg(D)=5 and a received
function rho in L(H) outside L(D). [pcc:unverified]

## An all-multiplicity route rejection

Let N>=8 distinct rational evaluation points avoid the poles, T and the
support of an arbitrary divisor A. Set d=deg(A). For s>=1, suppose an
interpolation polynomial Q(Z)=sum a_i Z^i has a_i in L(A-iD) and total
local multiplicity>=s at all received graph points. Then d<6s forces
Q=0; at d=6s, every nonzero Q is b(P)*(Z-rho)^s. Its only function-field
root is the known rho outside the message space. For A=sH this universal
boundary polynomial exists. No optimality claim about the threshold8.
[pcc:unverified]

The proof repeats graph division with a decreasing pole budget. Weighted
Gauss valuations give W_D(FG)=W_D(F)+W_D(G) and W_D(Z-rho)=H. After t
divisions, the quotient has budget A-tH of degree e=d-6t and remaining
multiplicity u=s-t. A nonzero leading coefficient bounds its Z-degree by
5l<=e. Evaluation at rho lies in L(A-tH+lT). If nonzero, its Nu zeros
would give Nu<=e+l. But e<=6u implies e+l<=36u/5<8u<=Nu, forcing
another graph factor. After s divisions the budget is negative, or zero
and the quotient is scalar in Z. DERIVATION.md supplies the valuation
cancellation and local-order steps. [pcc:unverified]

The conclusion concerns the standard six-agreement GS certificate in
these charts, uniformly at every multiplicity/list degree. Frobenius-sparse
powers remain polynomials with total multiplicity p^e*s, so increasing
inseparability does not change this obstruction. It does not exclude
other decoders, nonuniform multiplicities, exceptional charts or joint
coefficient solving. This is a written algebraic rejection criterion;
a capped/incomplete solver run remains inconclusive. [pcc:unverified]

## A different joint-six condition

For a finite SET X of distinct regular physical raw base points, put

    f_c(P)=xy+c0+c1*x+c2*y+c3*x^2+c5*phi_R,
    F_R(c)=product_{P in X} f_c(P).

Each factor is affine linear with c0 coefficient1. At a coefficient
vector c*, its zero factors contribute local order1 and all other
factors are units. Thus mult(F_R,c*) equals the number of section zeros
in X. Since a nonzero section has at most six zeros, six distinct base
members are equivalent to vanishing of all252 Hasse derivatives of
orders0 through5 in the five coefficients. In this regular distinct
stratum the elliptic divisor law gives their raw sum R. [pcc:unverified]

If an EXACT division-free or unit-guarded product circuit has length L,
evaluation in k[epsilon0,...,epsilon4]/(epsilon)^6 computes all252 jets
with O(252^2*L) scalar work. This preserves a circuit evaluation exponent;
it supplies no coefficient selection or rootfinding bound. F_R still has
degree |X|, and derivative degrees can grow with |X|. Duplicate physical
points repeat factors and can manufacture false sixfold multiplicity.
[pcc:unverified]

## Next acceptance object

First prove that a norm/resultant of the existing quotient-query
recurrence is exactly this physical-base product up to declared nonzero
factors, including rationality, aliases, branch and unit guards. That
identity is proposed, not established here. Then develop a separately
versioned Rust jet compiler and a NON-ENUMERATING solver on public target
inputs. A candidate-coefficient evaluation interface is not discovery.
[pcc:unverified]

The conditional common-base model's gamma<3 optimized-family target
(gamma<2 at fixed B~ell^(1/6)) is unchanged. All failed selection, setup,
memory, original witness/index/group replay, useful yield, rank/private
tags/kernel and scalar recovery stay charged. No job is scheduled.
[pcc:unverified]

Primary sources: Beelen--Montanucci2025, Definition3/Theorems4--5
([PDF](https://backend.orbit.dtu.dk/ws/portalfiles/portal/383901026/List-decoding_of_AG_codes_without_genus_penalty.pdf));
Snowden's [divisor/Riemann--Roch lecture](https://websites.umich.edu/asnowden/teaching/2013/679/L02.html);
Kopparty--Saraf--Yekhanin2011, Definitions5--6
([PDF](https://people.csail.mit.edu/dmoshkov/courses/codes/Multiplicity.pdf));
Wakefield--Yoshinaga2007, Sections1--2
([PDF](https://arxiv.org/pdf/0707.2672)). SOURCES_CHECKED.md records the
power-decoding boundary and full review. All mechanisms above use classical
algebra; KB unavailable and no novelty clearance or independent proof.
[pcc:unverified]
