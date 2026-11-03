# Explicit mixed F2 sections, correct polarization and kernel limits

G22 works on the ordinary fourfold of G21's stable scalar cover. Put
Q_i=[105]P_i for i=1..4 and Q5=-(Q1+Q2+Q3+Q4). The augmentation
rotation sends the ordered five points to(Q5,Q1,Q2,Q3,Q4). Define e_k by

    product_i(1+x(Q_i)*t)=sum_(k=0)^5 e_k*t^k.

These are six explicit rational functions, including e0=1. Each monomial
uses x(Q_i) at most once. As x has pole divisor2[O], every e_k has poles
bounded by the common divisor

    D_aux=[105]^*(2 sum_i ell_i^*[O]),

where ell_1,...,ell_4 are the coordinate maps and ell_5=-sum ell_i.
Thus they give a coherent system of sections of O(D_aux). Six sections
are constructed; linear independence and a basepoint-free system are
not asserted. This is an auxiliary mixed divisor, not an identification
of the canonical3L bundle.

The divisor and functions are rotation invariant. Their coefficients are
defined over F2, so e_k(R Frob(P))=e_k(P)^2. This gives F2 descent under
the written split trace-zero identification. Pullback by[105] makes them
invariant under E[105]^4. This avoids materializing all torsion coordinates
over F2^840: it uses the scalar isogeny as an actual map.

Native arithmetic checks16 valid tuples on each elliptic sign in F2^168,
with no pole rejection. On every tuple it recomputes the ordered rotation,
the twisted Frobenius values and24 actual torsion translations, for768
translations in total. The six prime-order generators per sign are the
actual G17 bases for3,5,7; each is annihilated by105. Distinct tuples
give distinct section-value vectors. Universal Lean lemmas prove orbit
polynomial/coefficient invariance, compatibility with a coefficient-field
map, the augmentation identity and scalar-kernel translation invariance.
Field/group/divisor/descent semantics remain written or native.

## Why this system cannot be the final quotient bundle

Before scalar pullback the ordinary polarization matrix is2(I+J),
namely diagonal4 and off-diagonal2, with determinant and section dimension80.
After scalar pullback the ordinary dimension is80*105^8=1181964355031250000.
The actual G16 companion factor has degree57. The full auxiliary product
therefore has section dimension57*80*105^8=67371968236781250000,
which is not divisible by361. Lean checks both nondivisibility statements.

If the full ample auxiliary product descended through the final degree361
isogeny, Riemann--Roch/pullback degree would make this dimension361 times
an integer. Multiplication105 is prime to19, so the scalar refinement
cannot repair that obstruction. Changing a theta character does not
change this degree obstruction. The initial ordinary-only record is
preserved; the strengthened record binds the actual degree57 companion.

This is a specific obstruction to using D_aux for that quotient, not an
obstruction to the requested3L bundle. The latter has the needed19 factors.
Next: work with a multiple of the corrected determinant19 polarization
from the beginning, construct its mixed rational trivializations and
sections, and check both the odd-cover and19-kernel actions on their
ratios. The canonical243 target sections and a proper embedding remain
open. This small system demonstrates that rotation-compatible invariant
functions can be evaluated, while showing why mere symmetrization is
insufficient for the desired polarized quotient.

## G23/G24: primitive polarization outside the orbit-transfer lattice

G23 tests the two actual Hermitian matrices H from G16. For every CM row
ell whose eight integer coefficients lie in[-2,2], form the five rows
ell R^j and their Hermitian Gram sum. Each form exhausts390625 rows;
6561 are in the[-1,1] subbox. The trace filter retains20 rows for H0 and
none for H1. Neither full matrix has a match. This bounded null is retained
without an existence claim outside the box.

G24 supplies a stronger exact explanation. Modulo5, the augmentation
rotation fixes v=(1,1,1,1) and sum_j R^j=0. Therefore every matrix transfer

    Tr_R(M)=sum_(j=0)^4 (R^j)^t M R^j

annihilates v, for every M. The real coefficient matrix of any integral CM
orbit Gram is exactly such a transfer. Transfers are additive, so this
also applies to every finite sum of integral orbit Grams. But the actual
real matrices of H0,H1 send v to(3,1,4,2) and(2,4,1,3) modulo5,
respectively; their multiples3H also send v to a nonzero vector. The JSON
retains all actual products. Lean checks the actual rotation and
matrices and proves the transfer implication and additivity.

Consequently neither H nor3H can be an integral sum of five-element
elliptic-pullback orbits in this CM/augmentation basis, with no coefficient
cap. This excludes the proposed polarization-construction route, including
translated elliptic pullbacks whose polarization class is unchanged.
It does not exclude invariant mixed bundles or their sections. Fractional
homomorphisms and non-orbit mixed divisor representatives are outside this
obstruction. Nor is it an obstruction to the unrelated integral level77
factor map.

The next construction should use the corrected mixed3L divisor itself,
find the rational trivialization of its rotation pullback, and form linear
invariants inside its section space. Tensoring full divisor orbits changes
the polarization class; G24 explains why that route cannot produce3H
integrally. This leaves a concrete rational-divisor coboundary and theta
linearization problem rather than another search over larger orbit rows.


## G25: a correctly polarized orbit divisor on the scalar cover

The primitive modulo5 obstruction does not apply to the scalar105
pullback. On the selected cover for graphs0/2, the first two rows of the
actual C divide by5. Put ell=(C0/5,C1/5,C2,C3), with positive weights
n=(45,315,63,171). Native exact CM arithmetic verifies

    sum_i n_i sum_(j=0)^4 (ell_i R^j)^dagger(ell_i R^j)=33075 H
                                                        =3*105^2 H.

Thus D_orbit=sum_i n_i sum_j(ell_i R^j)^*[O] is an explicit effective,
rotation-invariant ordinary divisor of the required pulled-back
polarization. With the unchanged companion divisor57[O] it has the same
polarization as G21's full source bundle. Lean checks the actual weighted
Gram identity, all four orbit cycles, the weight bounds, and binds every
initial row to the already formalized integral matrix C.

For each group of five points ell_i R^j(P), take the five nonconstant
coefficients of product_j(1+x_j t) and product_j(1+y_j t), together
with a single1. These are41 explicit F2 rational sections of O(D_orbit),
since their poles use each component at most twice for x or three times
for y, while every n_i is greater than3. All16 retained split tuples per
sign pass rotation and twisted Frobenius, with no poles. Distinct scalar
kernel translates give24 exact noninvariance witnesses per sign,48 in
all. These ratios therefore do not immediately descend through the
scalar kernel. Full theta lifts and linear combinations remain needed.

The41 sections have the forced fixed divisor
sum_i(n_i-3) sum_j(ell_i R^j)^*[O], with weights(42,312,60,168).
They are not a basepoint-free system, and independence is not asserted.
Nor does equality of polarizations identify the canonical symmetric Pic2
bundle class. That class matching, the full weighted factor-section
spaces and actual odd/19-kernel actions are the next obligations.
The other two graphs, canonical243 sections and proper quotient
embeddings remain open. This is a positive construction on the refined
cover, compatible with the precisely limited primitive G24 obstruction.
