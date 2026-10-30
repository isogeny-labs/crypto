# Source-bound Lefschetz argument for the prime chain

Development argument,2026-10-30. No production result is supplied by this
document alone. Independent scientific review and formalization are pending.
The intended scope is the42 retained prime m3–9 systems with27 selectors per
block, all weights nonzero, and the exact source template checked by
prime_linear_source_donor.rs. It does not cover arbitrary factor bases or
arbitrary summation-polynomial presentations.

## 1. Graded strings

Let A_ell=k[b_1,...,b_ell]/(b_1^2,...,b_ell^2),L=sum b_i,and char(k)>ell.
Lundqvist–Nicklasson,https://arxiv.org/html/1604.06820,Theorem2.7 and Section3,
give maximal rank for every power of L. This is the external theorem used
here; the rest of the chain argument is below.

For 0<=k<=floor(ell/2),put P_k=ker(L^(ell-2k+1):A_k->A_(ell-k+1)).
Maximal rank and Hilbert symmetry give dim(P_k)=C(ell,k)-C(ell,k-1).
Moreover P_k intersects L A_(k-1) trivially: if Lw is in the intersection,
then L^(ell-2k+2)w=0,where this power is an isomorphism from degree k-1
to degree ell-k+1. Hence A_k=L A_(k-1) direct-sum P_k.
L^(ell-2k) is injective on A_k,so each nonzero primitive vector produces a
string v,Lv,...,L^(ell-2k)v with exactly ell-2k+1 nonzero elements.
The preceding direct sums,induction,and Hilbert symmetry give a graded
direct sum of these strings for all of A_ell. In particular this is a
module decomposition for multiplication by L,not a claimed ring product.

Tensor the block decompositions and adjoin the unrestricted intermediate
variables U. Every tensor primitive v has total degree K=sum k_i and its
sector is the cyclic graded module
S/(L_i^(ell-2k_i+1):all i)(-K),where S=k[L_0,...,L_(m-1),U_0,...,U_(m-3)].
All chain quartics act within each sector because they are polynomials in
the L_i and U. Multiplicities are products of the primitive dimensions.

## 2. No truncation of a low-degree relation

Write a homogeneous degree-d relation among the quartics in one sector.
Its coefficient polynomials have degree r=d-4-K;sectors with r<0 contribute
nothing. Choose the canonical truncated representatives q_j in S.
Each quartic F_j has exponent at most2 in each L_i. Therefore every monomial
of every q_j F_j has L_i exponent at most r+2=d-2-K.

For any contributing sector,k_i<=K<=d-4,and

    (ell-2k_i)-(d-2-K) >= ell-d+2-K >= ell-2d+6.

Consequently,if ell-2d+6>=0,no monomial in the relation is lost by string
truncation. A relation in the sector quotient is then literally a relation
sum q_j F_j=0 in the free polynomial ring S. For ell27,this holds for every
d<=16. Degrees below4 have no quartic coefficient and are handled by the
selector-square regular sequence alone.

This is a uniform inequality over every possible sector;checking some
selected primitive vectors would not suffice. At d17,k_last13 gives a
string with allowed last-block exponent1;the terminal L_last^2 term is then
lost. The argument deliberately does not extend across that boundary.

## 3. Free chain regular sequence and its syzygies

Use a weighted monomial order with weight m on each L_i and weight m-2-j
on U_j,ties broken by any monomial order. All weights are positive.
Expand each nonterminal quartic as

    x^2 y^2 + x^2 z^2 + y^2 z^2
       -2 x^2 y z -2 x y^2 z -2 x y z^2.

The source chain has x=L_0,y=L_1,z=U_0 initially,then
x=U_(j-1),y=L_(j+1),z=U_j;the last quartic is U_(m-3)^2 L_(m-1)^2.
Because weight(x),weight(y)>weight(z),x^2y^2 strictly leads each
nonterminal quartic. The leading monomials are thus

    L_0^2 L_1^2, U_0^2 L_2^2, ..., U_(m-3)^2 L_(m-1)^2.

Their supports are pairwise disjoint. The product criterion makes every
prefix a Groebner basis. They form a regular sequence: reduce h modulo a
prefix; if its nonzero remainder times the next F were in that prefix
ideal,one of the earlier leading monomials would divide LM(h)*LM(F).
Coprimality forces it to divide LM(h),contradicting the normal remainder.

For completeness,first-syzygy Koszul generation follows by induction.
In sum q_j F_j=0,regularity implies the last q lies in the preceding
generator ideal. Write it homogeneously as sum h_i F_i,and subtract the
corresponding pairwise Koszul syzygies. This removes the last coordinate;
repeat on the prefix. The h_i have degree d-K-8. If this degree is negative,
the relation is zero. Thus every free-sector relation is Koszul in every
degree,including the degrees8..16 where nonzero trivial relations exist.

## 4. Transport and selector-square lift

Multiply each free-sector Koszul expression by its primitive vector v and
apply the S-module map to its truncated sector. This map commutes with each
F_j,so the expression remains a Koszul expression there. No injectivity
assumption on the h_i is needed;only the no-truncation step for q_j F_j was
needed to obtain the free relation. Sum over the direct-sum sectors.

Return to R=k[b,U]. Subtract lifts of the resulting quartic/quartic Koszul
relations from a syzygy on (selector squares,quartics). Every remaining
quartic coordinate lies in the selector-square ideal. Express it as a
homogeneous combination of those squares and subtract mixed square/quartic
Koszul syzygies. The remaining relation is among the selector squares,
which form a regular sequence,so it too is Koszul. All operations preserve
the homogeneous total degree.

The field size is the83-bit prime p. In degrees at most17,the quotient by
variable p-th powers in Caminata–Gorla's FFD convention changes nothing;
the extra Frobenius trivial syzygies also have larger degree. Thus the
argument would exclude every nontrivial highest-form syzygy below17 for
the verified source template. It is not an assertion about filtered LFD.

## 5. Matching upper witness and claim boundary

In normalized last-block coordinates,the product
H=(b_0-b_1)(b_2-b_3)...(b_24-b_25) has degree13,8192 nonzero terms,and
L_last^2 H=0 modulo selector squares. Therefore H times the terminal
quartic reduces to zero,providing a degree17 syzygy after square lifting.
Project all other selector blocks and all U to zero and impose last-block
squares. H survives but every other generator vanishes,so the terminal
coordinate H cannot be a trivial-syzygy coordinate. This matches the
existing source-bound upper17 construction;it must be checked against the
same input and base hashes before combining results.

For each actual case,check prime identity,all input/source bytes,weights,
normalization inverses,leading terms,block multiplicities,Hilbert dimensions,
all degree inequalities,and the explicit upper witness. A new frozen
protocol and producer/fresh/S3-restored replay are required before recording
a conditional exactFFD17 result. Independent scientific review/formalization
remain pending. No LFD,solving-degree,regularity,security or elite-base
improvement follows automatically. Existing datasets remain immutable.
