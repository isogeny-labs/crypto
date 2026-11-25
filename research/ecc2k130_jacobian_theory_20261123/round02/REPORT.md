# ECC2K-130: a necessary large-prime factor in principalizing isogenies

2026-11-23. Continued objective: decide whether a curve of genus130–300
and a computable subgroup-preserving correspondence can be constructed.
This component is a written mathematical deduction with unsigned native
arithmetic; independent review and full Lean bridges are absent. [pcc:unverified]

## Result and exact scope

Let A be the actual connected trace-zero subvariety of
Res_(F_(2^131)/F2)(E0), with E0:y²+xy=x³+1. If B/F2 has a principal
polarization and f:A->B is an F2-isogeny, then

    146505763881528721 divides degree(f).

The same holds for an isogeny in the opposite direction. In particular,
any genus130 Jacobian in the isogeny class requires such a prime factor
in every isogeny from the natural A. This is a written necessity result,
not a curve construction or an evaluation-time lower bound. [pcc:unverified]

The prior round's pp existence statement and this result are consistent:
the necessary prime can occur in an isogeny. Losing maximal endomorphisms
somewhere is insufficient; a principally polarized representative cannot
remain maximal at all primes above ell=146505763881528721. [pcc:unverified]

This result is not asserted for embeddings into a Jacobian of larger
dimension. Complementary factors, polarization restriction and gluing are
additional possibilities. The general genus131–300 case remains open.

## 1. Objects and arithmetic retained from round01

Write alpha²+alpha+2=0, beta=-1-alpha, zeta a primitive131st root,
pi=alpha*zeta, V=beta*zeta^-1, K=Q(alpha,zeta), O=O_K,
R=Z[pi,V]. Then K is a degree260 CM field, A is ordinary F2-simple,
and End_F2(A)=O. To see the last assertion for this particular A, alpha
acts componentwise via E0 and cyclic permutation acts as zeta on the
augmentation factor. These generate O=Z[alpha,zeta], since the field
discriminants -7 and -131^129 are coprime. There is no larger integral
order in End^0_F2(A)=K. [pcc:unverified]

With U0=0,U1=1,U_(m+2)=-U_(m+1)-2U_m, the retained exact certificate is

    U131=263*ell,
    ell=146505763881528721 (prime).

Both prime factors have exponent1. The parent check.cpp/check.txt contains
the full-factorization Lucas primality certificate for ell. Our additional
native checker confirms that131 is inert in Q(sqrt(-7)) and263 splits.
The latter also follows from263=1 modulo131 and263=4 modulo7.

## 2. The universal order is maximal away from263 andell

The recurrence gives alpha^131=U131*alpha-2*U130. Since pi^131=alpha^131,
alpha belongs to R[1/U131]; hence beta belongs as well. At every maximal
ideal of this ring, either alpha or beta is a unit, because alpha+beta=-1.
If alpha is a unit, zeta=pi/alpha belongs to the local ring. If beta is a
unit, zeta^-1=V/beta does, and zeta=(zeta^-1)^130 does too. Membership at
all maximal ideals gives zeta in R[1/U131]. Consequently

    R[1/U131] = O[1/U131].

In particular R is locally maximal at every rational prime except263
andell, including the characteristic2 prime. No division by2 has been
used. Every endomorphism order of an isogenous representative contains R,
so its possible conductor support is restricted to those two primes.
This is a written order argument, not merely a residue-field test. [pcc:unverified]

## 3. The natural polarization has a nonzero maximal-order obstruction

Let W be the Weil restriction, endowed with the restriction of E0's
principal polarization. Its restriction lambda0 to A has kernel E0[131]
embedded diagonally. Indeed, after extending the field W becomes E0^131;
A is the summation kernel, and its orthogonal complement for the product
polarization is the diagonal E0. Their intersection is the diagonal
131-torsion. Thus degree(lambda0)=131². The cyclic permutation zeta acts
trivially on this kernel. Componentwise alpha acts by elliptic Frobenius.
Since X²+X+2 is irreducible modulo131, this kernel is the simple O-module
O/P131, of cardinality131². [pcc:unverified]

Let K+ be the complex-conjugation fixed field. The extension K/K+ is
unramified at every finite prime. Only7 and131 can ramify in K/Q. At7
inertia acts only on Q(sqrt(-7)); at131 inertia acts only on Q(zeta).
Complex conjugation acts nontrivially on both factors and is therefore
in neither inertia group. Its intersection with each inertia group is
trivial, which gives relative unramifiedness. [pcc:unverified]

There is a unique prime of K above131: the quadratic field is inert there
and the cyclotomic extension is totally ramified. The prime of K+ below
it is inert in K/K+, with residue degree1 over F131; the relative residue
extension has degree2. Therefore the real ideal associated to kernel
E0[131] has nontrivial Artin symbol in Gal(K/K+). [pcc:unverified]

In Howe's notation the group B(O) is the narrow ideal class group of K+
modulo ideal norms from K. Proposition6.2 identifies it here with C2 via
the Artin map. The class of ker(lambda0) is its nonzero element.
Sources: Howe1996, Proposition6.2 and proof7.1, plus the explicit
augmentation-kernel argument above. [pcc:unverified]

## 4. Nonzero obstruction survives orders maximal atell

Suppose B has a principal polarization and is locally maximal at all
primes aboveell. Put T=End_F2(B), with K identified by an isogeny.
The Rosati involution of a principal polarization preserves T and acts
as conjugation; in particular all coefficient orders below may be taken
conjugation stable. By section2, the quotient O/T is supported only at263.
Choose k sufficiently large that263^k O is contained in T, and put

    S=R+263^k O,    R contained in S contained in T.

Then S is a conjugation-stable order and O/S is supported only at263.
All primes of K above263 split over K+:263 splits completely in K,
because it splits in the quadratic field and is1 modulo131. [pcc:unverified]

Apply Howe's Proposition6.5 to S contained in O. The generating primes
for H(O) are precisely primes inert in K/K+. None of these can be bad
for this inclusion, whose support is only at the split primes over263.
Thus the D_bad term in the pushout is zero. The induced map

    B(O) -> B(S)

is injective. In particular the nonzero class of ker(lambda0) remains
nonzero in B(S). This step uses the general proper-order pushout,
not Proposition7.2 with R incorrectly replaced by an arbitrary order.
[pcc:unverified]

Both A and B carry the S-action and any isogeny between them is S-linear,
because the rational endomorphism algebra is the commutative field K.
The polarization-class invariance argument in Step2 of Howe's proof of
Theorem1.3 applies with these S-actions: pullback of the principal
polarization differs from lambda0 by a totally positive real rational
endomorphism, while an isogeny and its conjugate-dual kernel contribute
Q+conjugate(Q), which is zero in B(S). Thus lambda0 and the principal
polarization must have the same class in B(S). The latter has zero
kernel and class zero, contradicting injectivity above. No assertion of
the full attainability theorem for arbitrary S is needed; only this
necessary invariance argument is used. [pcc:unverified]

Therefore B cannot be locally maximal atell. This also shows that A
itself has no principal polarization, although its isogeny class does.
The distinction is between the natural representative and its whole
isogeny class. [pcc:unverified]

## 5. Consequence for isogeny degree

If ell does not divide degree(f), f induces an isomorphism of ell-adic
Tate modules. After rational identification this isomorphism intertwines
K. It transports O tensor Z_ell=End(A) tensor Z_ell to integral
endomorphisms of B. Tate's finite-field integral homomorphism theorem
then gives End(B) tensor Z_ell=O tensor Z_ell. This contradicts section4.
The argument is symmetric if the isogeny is B->A. Hence every such
isogeny has degree divisible byell. [pcc:unverified]

This degree statement does not bound evaluation cost byell. Factored maps,
special representations or implicit arithmetic may evaluate a map far
more cheaply than expanding all its rational functions. We have not
constructed one and have not established a lower bound on every such
representation. In particular this theorem alone does not refute the
conditional 2^37.17 destination-solver estimate or establish the full cost.
It rules out the prime-to-ell isogeny family for genus130. [pcc:unverified]

## 6. Exact residue census and what it checks

local_check.cpp enumerates all260 labelled embeddings at each conductor
prime. Its output records every equal-value class and Frobenius orbit.
The optimized and UBSan replays agree. [pcc:unverified]

| rational prime | single-root classes | double-root classes | Frobenius organization |
|---|---:|---:|---|
|263|2|129|every residue lies in F263|
|ell|2|129|one real double;64 quadratic double orbits;one quadratic single orbit|

Atell, exactly one residue prime has pi=V in its residue field. At the
other inert primes pi still generates the quadratic residue extension.
Thus the parent witness was one special prime, not a claim that every
embedding sends pi into F_ell. This is the local mechanism by which the
universal-order obstruction can vanish. [pcc:unverified]

The counts support further local classification but are not used as a
substitute for the order and Artin arguments above. No complete list of
global principal forms, lattice ideal classes or Jacobians is supplied.
The protocol predictions are retained in PROTOCOL.md; an initial
-Werror misleading-indentation failure was repaired before census output.

## 7. Remaining question and next step

The original objective is still unresolved. A genus130 Jacobian might
exist after an ell-divisible isogeny, but needs actual Torelli realization
and a computable map. A genus131–300 carrier might use a complementary
factor and a gluing kernel to escape this equal-dimension argument.
The next separate obligation is to analyze those gluing kernels, rather
than treating the degree-divisibility result as universal nonexistence.

All mathematical conclusions here are written, unsigned deductions.
A faithful full Lean development would need the CM order/Artin obstruction,
polarization kernels, equivariant Deligne modules and Tate-module bridge;
these definitions and theorems are not available in this workspace.
The native residue check is not such a proof. No axiom or assumed
geometric conclusion is inserted to label the theorem formalized.

Literature receipt: KB unavailable (no MCP or executable); read primary
https://arxiv.org/pdf/2001.05111 §§5–7 and
https://arxiv.org/pdf/math/0002232v3 §§1–2. The latter's corrected Section3
is not used. The method is a specialization of established obstruction
machinery; no novelty claim is made. [pcc:unverified]. Full arguments
and hypotheses, rather than abstracts, informed the chosen approach.

Existing performance charts stay unchanged: no curve, map evaluator,
destination relations, solve or timing has been added. The local residue
census is a proof check, not an experiment comparing running times.
