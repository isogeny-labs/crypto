# Interpolation degree budget and direct-target collision

Status: algebraic screen using standard Riemann-Roch; no performance result,
new algorithm or novelty claim. Original genus2/3/4 targets remain unchanged.
No scientific producer execution is needed for the derivation.

## Direct interpolation is already the rational-point addition path

Let R=(u,v), deg u=g, and P=(a,b) with u(a)!=0. Any polynomial w of degree
at most g satisfying w=v mod u and w(a)=b is uniquely

    w=v+t*u,  t=(b-v(a))/u(a).

Set U=u*(x-a), H=(f-w²)/U. Its degree is g: f has degree2g+1,
w² degree at most2g, and U degree g+1. Its signed residual divisor E
satisfies [E]=-R-[P-infinity]. The reduced representative of R+P is
(H.monic, -w mod H.monic), exactly Cantor composition/reduction.

This remains valid when u is irreducible: the congruence records vanishing
at the entire closed divisor, with degree-weighted multiplicity, and does
not require splitting it first. When u(a)=0 the coprime formula fails;
general Cantor cancellation handles the case, not an uncharged omission.

Current counted/native/src/jac.rs add_place computes exactly t, U and w,
then calls reduce. Thus proposing this construction as a new bootstrap
would relabel the existing implementation. The scalar/FB row information
and all source normalization costs still have to be tracked.

## Why polynomial degree alone does not shorten the residual

For n distinct prescribed abscissas, let v be the unique degree<=n-1
interpolant and U their degree-n product. For y²=f, deg f=2g+1,

    deg((f-v²)/U)=max(2g+1,2deg v)-n.

There is no leading-term cancellation because the two numerator degrees
have opposite parity. If deg v=n-1 (the generic full-degree case), this is

    r(n)=max(2g+1-n,n-2).

The minimum over n is g, attained at n=g+1 and n=g+2.

|genus|two optimal point counts|generic residual degree|
|---|---|---|
|2|3,4|2|
|3|4,5|3|
|4|5,6|4|

This explains why switching three to four points kept a quadratic residual
in the recent genus2gate. Larger sign families do not lower the degree.
Degree drops of v can reduce r in overdetermined larger-n systems; they
are exceptional interpolation dependencies, not forbidden events. They must
be retained, and a generator exploiting them needs its own cost case.

## Full function-space version, with explicit hypothesis

Let h have actual pole divisor B of degree m>2g-2, and let E be a
prescribed effective zero divisor, disjoint from B, of degree n. Assume
its vanishing conditions have rank n in L(B). Riemann-Roch gives
ell(B)=m+1-g. Existence of a nonzero h in L(B-E) then requires

    1<=ell(B)-n=m+1-g-n, hence m-n>=g.

The unprescribed zero divisor has degree m-n. This applies equally to
rational points, jets and closed points counted by residue degree, provided
the stated full-rank hypothesis holds. Denominators do not remove the
budget: finite poles enter B. With m<=2g-2 this argument does not apply;
small hyperelliptic pole spaces can instead contain x-only functions whose
paired zero divisors yield trivial/cancelled Jacobian relations.

This is NOT a general lower bound on relation generation or IC. A clever
algorithm may choose dependent E cheaply, exploit special divisors, use a
moving pole divisor, improve useful rank, or reduce arithmetic costs. The
point is to identify which hypothesis it escapes before calling an enlarged
interpolation space a breakthrough. Finding E with n=m-g+1 and
ell(B-E)>0 already asks for dependent evaluation conditions; a random-search
probability or a cheap finder has not been established here.

## Next admissible mechanism

A shorter-residual proposal must specify how its dependency is found and
charged, without knowing target logs. A rank proposal must prove or measure
new target-identifying information rather than extra split outputs. An
arithmetic proposal needs a strong prepared reference and full cold cost.
Do not run direct-target interpolation as a new method, or launch another
higher-degree sign family solely for its larger candidate count.

## Primary sources and derivation boundary

Handbook of Elliptic and Hyperelliptic Curve Cryptography, chapter4,
Theorem4.106 (Riemann-Roch equality above2g-2), and Jacobian background:
https://www.hyperelliptic.org/HEHCC/chapters/chap04.pdf
Michael Stoll, Arithmetic of Hyperelliptic Curves, sections2/4, odd-degree
function basis and divisor arithmetic:
https://www.mathe2.uni-bayreuth.de/stoll/teaching/ArithHypKurven-SS2019/Skript-ArithHypCurves-pub-screen.pdf
The interpolation formulas, conditional dimension deduction and collision
with current code are the explicit screen here, not a speed theorem
attributed to either source. Existing section-sieve/SCREEN.md already covers
the higher-dimensional target pencil and its incidence/cost obligations.
