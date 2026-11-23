# Full-relation regularity bounds: proof obligations and derivation

This is a producer-written algebraic derivation, not a Lean or signed
independent verdict. The concrete chain point ideal still needs its complete
graded certificate. Do not promote the numerical implications below before
that certificate and a review of this argument. D always means ideal-module
Castelnuovo--Mumford regularity of the stated generator homogenization.

## Upper bound, including H-torsion

Let T be a nonempty finite reduced rational point set in H!=0. Write
A=S/I(T), with a graded free K[H] basis of maximum degree r. Let F contain
homogeneous polynomials of degrees at most e, e>=1, in this same ring, and
let J=I(T)+(F). Write Z for the accepted subset and C=S/I(Z), or C=0 if Z
is empty. Then

    reg(S/J) <= r+e-1,    D(J) <= r+e.

Proof. Evaluation A_r -> K^T is onto. For each rejected t, choose a
degree-r homogeneous Lagrange representative L_t, and f in F with f(t)!=0,
degree d<=e. In A,

    L_t f = f(t) H^d L_t.

Thus J contains the degree-(r+e) representative H^e L_t for every rejected
coordinate. At every degree n>=r+e, the map (S/J)_n -> C_n is an
isomorphism: its kernel is killed by these representatives, and every
generator vanishes on Z. Multiplication by H is consequently an isomorphism
there. The kernel Q of S/J -> C is finite length, supported through degree
r+e-1. Restriction A_n -> C_n is onto, so C has quotient regularity at most
r (it is a reduced finite point algebra with H regular). Local cohomology
of 0 -> Q -> S/J -> C ->0 gives the bound. The empty-Z case is the same
argument with an Artinian quotient. A graded field extension does not
change regularity, so the proof also applies to geometrically split points.

For m independent point blocks with common H, tensor the free K[H] bases:
the quotient regularity is the sum of the block regularities, not a count
of equations or a leading-degree heuristic. No free group witnesses are
allowed in this application of the lemma.

## Dense lower bound, not a comparison of two upper bounds

Suppose A is additionally a reduced complete-intersection point algebra
with quotient regularity r. Its Artinian reduction by H is Gorenstein with
socle degree r; its Hilbert numerator h_A is symmetric of degree r with
last coefficient1. Let f in A_e be nonzero and let Z be its zero subset.
Assume at least two independent elements of A_e vanish on Z. Then

    D(I(T)+(f)) >= r.

Here is a derivation that makes the duality shift explicit. Let V_i be the
degree filtration on K^T obtained by setting H=1. The complete-intersection
socle functional lifts to a functional lambda on K^T vanishing on V_(r-1),
whose multiplication pairing is nondegenerate. Nondegeneracy follows by
taking a nonzero leading filtered part and pairing it to the socle in the
Artinian complete intersection. As K^T is a product of fields, this is a
diagonal weighted evaluation pairing with every weight nonzero. Symmetry
and multiplication of filtrations give

    V_i^perp = V_(r-i-1).

The degree-i part of Ann_A(f) therefore has dimension

    |Z| - HF_C(r-i-1),       C=S/I(Z).

Indeed it consists of vectors supported on Z orthogonal to V_(r-i-1);
restriction of that latter space has dimension HF_C(r-i-1). Consequently

    Hilb(Ann_A(f),t) = t^r h_C(t^-1)/(1-t).

The exact multiplication sequence for f gives

    Hilb(A/(f),t)
      = [h_A(t)-t^(e+r)(h_A(t^-1)-h_C(t^-1))]/(1-t).

Let delta be the first degree where Delta=h_A-h_C is nonzero. Its first
coefficient is positive, and delta<=e. If Z is empty, f is regular on A
and D=r+e. Otherwise C has regularity<=r. If delta<e, the last numerator
coefficient above is -Delta_delta in degree k=e+r-delta>r. Subtracting
Hilb(C) shows the finite H-torsion has last nonzero degree k-1, giving
exactly D=k>=r+1. If delta=e, the independent vanishing forms imply
Delta_e>=2; the numerator coefficient in degree r is 1-Delta_e<0.
Subtracting h_C makes it still negative, so H-torsion has degree r-1
and D>=r. This covers both empty/nonempty accepted sets and counts the
unsaturated generators' torsion rather than replacing them by saturation.

The filtered-duality ingredients are classical: Kreuzer, Long, Robbiano,
On the Cayley--Bacharach Property, Proposition4.2 and Theorem6.8; the paper
explicitly recalls that strict complete intersections are strict Gorenstein
rings in section6. The specialization/inequality above is derived here,
not attributed verbatim to that paper or claimed novel.
https://csdlkhoahoc.hueuni.edu.vn/data/2018/11/OnTheCBP.pdf

## Application to the frozen complete group circuit

Renes--Costello--Batina Algorithm1 has bidegree(2,2). Balanced composition
on m leaves (x_i,y_i,H) gives a common homogeneous degree e:

    m:  2   3   4   5   6   7
    e:  4  10  16  28  40  52.

The single isolating target line gives f=L(Xsum,Ysum,Zsum). For each fixed
rational target, the native discriminant certificate proves L meets the
rational curve only at that target, including the separately defined
vertical-line infinity case. The no-rational2torsion gcd certificate makes
the published complete law applicable to all input tuples and all partial
sums. There is no assertion that it is complete on arbitrary extension
points. The retained input ideal contains only rational points.

Two independent target-vanishing linear forms in the output coordinates
pull back to independent elements of A_e. For if a nonzero linear
combination vanished on every input tuple, fix m-1 inputs and vary the last
one over the 288 distinct retained signed points. Translation gives 288
distinct sums, all on one line, contradicting Bezout for a nonsingular
plane cubic (at most3 distinct intersections). Thus the dense lower-bound
lemma applies without enumerating or assuming the number of relations.
For finite targets, setting all x_i=H=0,y_i=1 gives f=1, certifying the
actual total degree e without H-padding or materializing a relation table.

On the ORIGINAL depth8 base, the dense point certificate gives a strict
CI(3,96), so r_dense=97. Its m-block common-H product is again a strict CI,
with r=97m. The chain span certificate suggests r_chain<=9; if an exact
retained degree-compatible point ideal is supplied, the upper-bound lemma
then gives the following CONDITIONAL full-system bounds:

| m | dense D lower | chain D upper | 2*upper <= lower |
|---|---:|---:|---|
| 2 | 194 | 22 | yes |
| 3 | 291 | 37 | yes |
| 4 | 388 | 52 | yes |
| 5 | 485 | 73 | yes |
| 6 | 582 | 94 | yes |
| 7 | 679 | 115 | yes |

Both arms use exactly the same unexpanded group circuit and target line;
neither arm receives solved relations as preprocessing. Extra chain
coordinates are unique, seven per point. The dense CI uses all144 retained
abscissae, not the inflated256-point auxiliary fibre. This is a claim about
standard graded ideal regularity, not affine F4 degree or solve time.

## Remaining concrete certificate

The native structured-span artifact explicitly says the retained generator
list is not materialized. Supplying a saturated point ideal only by name
must not be confused with homogenizing a concrete incomplete list. Generate
and certify the actual degree-compatible retained GB/border, including all
H-free leading terms and complete footprint, before using these bounds as
a concrete result. Review the mathematical lemmas independently; current
PCC standing is unsigned. Charge coefficient expansion and setup, keep
unplanted solving and original runtime obligations open.
