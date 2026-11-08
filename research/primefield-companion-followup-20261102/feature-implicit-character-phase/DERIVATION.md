# Observation fibers and implicit character-phase intersection

## Scope

This component has two deliverables: a bound on annihilators constructed solely
from the retained first-orbit summary, and a kernel intersecting a supplied
polynomial with an implicit phase. It does not construct the required small
polynomial from a public target. Public point coordinates, the complete
oriented transcript, additional observations or other algebraic information
are outside the summary-only bound. No recovery adapter is changed here.

## Dihedral invariance and degree requirement

Let r be an odd certified prime, h=(r-1)/2, m>=3 divide h, and L=h/m.
For a radius b and primitive m-th root zeta, observe binary signs
s_j=chi(k-b*zeta^j), excluding zero endpoints. Write
f0=sum(s_j), f+=sum(zeta^j*s_j), f-=sum(zeta^-j*s_j).
A rotation multiplies f+ and f- by reciprocal factors; reflection exchanges
them. Thus T=(f0,f+*f-) is constant on binary bracelets (dihedral orbits).

By the orbit-counting formula the number B_m of binary bracelets is

 B_m = [sum_(a=0)^(m-1) 2^gcd(m,a) + R_m] / (2m),
 R_m = m*2^((m+1)/2)                         (m odd),
 R_m = (m/2)*(2^(m/2+1)+2^(m/2))             (m even).

This follows by counting cycles of each rotation/reflection: a fixed binary
word is constant on each cycle. In particular B_3=4 and B_5=8. Reduction
modulo r can merge summaries, but cannot increase this upper bound.

Fix the initial scalar character and set V=(k/b)^m, epsilon=chi(k)*chi(b).
Because chi(zeta)=1, k -> k*zeta only rotates its signs, so T is well-defined
on the L roots V^L=epsilon. Excluding zero observations removes V=1 exactly
when epsilon=1. Put delta=[epsilon=1], N=L-delta. Some summary therefore
retains at least ceil(N/B_m) distinct V values. A nonzero polynomial in V
constructed only from that summary and required to contain every compatible
V must have at least that degree for some summary.

For W=V+V^-1, let e=[epsilon=1]+[(-1)^L=epsilon]. There are (L+e)/2
distinct W values before endpoint exclusion, and N_W=(L+e)/2-delta after.
Although two inverse V values can have different summaries, the union of
summary supports covers these N_W values. Hence some summary requires a
nonzero W-annihilator of degree at least ceil(N_W/B_m).

For fixed m these worst-case bounds grow linearly with L. They do not say
every observed fiber is large, and do not bound algorithms that also use Q,
the full oriented signs, other radii or additional public constructions.

For B fixed radii, excluding their endpoints removes at most B V values.
There are at most B_m^B retained-summary tuples, so some nonendpoint tuple
requires V-degree at least ceil(max(0,L-B)/B_m^B). This version is for a fixed
radius schedule; no adaptive-radius bound is asserted. These necessary
conditions do not construct an annihilator or establish its attainable degree.

## Implicit phase kernel

Given monic q of degree d<r and L in [1,r-1], first take sf(q). The bound d<r
ensures ordinary derivative/gcd squarefree extraction suffices here.
In F_r[W]/sf(q), evaluate the pair (D_n,D_(n+1)) by recursive doubling:

 D_(2j)=D_j^2-2,
 D_(2j+1)=D_j*D_(j+1)-W,
 D_(2j+2)=D_(j+1)^2-2.

Every product is reduced immediately. Return gcd(sf(q),D_L-2epsilon).
This equals gcd(sf(q), the full dense phase polynomial) but never allocates
the latter. The squarefree step preserves geometric support when q has
repeated factors. q=1 returns1. Invalid phase labels, nonmonic/out-of-field
coefficients, L=0 and degree/cap violations are rejected.

The recurrence uses at most3*bit_length(L) quotient products; product
coefficient lengths are at most2d-1, apart from fixed degree-one constants.
These bounds concern the recurrence vectors, not all polynomial-engine
scratch storage, squarefree/gcd work or peak process memory. Its arithmetic
cost is O(M(d)log L) plus reductions and squarefree/gcd costs, under regular
multiplication/division assumptions. Input construction is additional work.

Large-exponent fixtures use q built from24 public constant reciprocal traces,
with L up to998244352 over F_998244353. Known constant roots independently
check the result through v^L=epsilon. This qualifies the kernel only; it is
not a target-derived q or an ECDLP recovery run.

## Sources checked and next step

NIST DLMF18.5.1-2, https://dlmf.nist.gov/18.5, supplies the reciprocal identity.
Mestrovic, arXiv1804.00992v1, full-text introduction equations2-3 and the
bracelet definition, https://arxiv.org/html/1804.00992v1, supplies the
orbit-counting framework. The reflection contribution and observation bound
are derived above. Repository sources: predecessor polynomial.rs and the
native hgcd-roots/gcd.rs implementation. KB wrapper unavailable.

The next construction must retain enough independent public information:
multiple radius summaries, oriented modes or a direct public-point relation.
A modular phase kernel alone cannot manufacture that information. Investigate
multi-radius compressed constraints with an explicit construction cost and
candidate-completeness certificate, rather than assuming a uniformly small
first-summary annihilator.
