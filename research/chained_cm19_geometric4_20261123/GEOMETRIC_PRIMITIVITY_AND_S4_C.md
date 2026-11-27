# Actual degree-four cover: geometric primitivity and S4 monodromy

Standing: a written mathematical proof, using the exact inherited actual
Jacobian/map evidence and standard algebraic geometry below. Native A25c352
checks all new finite permutation and actual local identities. Formal
B b47cfe checks the quantified Int dependency only. No independent signed
verdict or complete geometric Lean theorem is asserted. Every theorem
in this document retains that standing. [pcc:unverified]

## Objects and explicit inherited hypotheses

Over k=F7 let

    E: y^2=x^3+2x+6,
    C: X^4+4XY^3+2Y^4+5X^2Z^2+6XYZ^2+2Y^2Z^2+Z^4=0,
    Pminus=(1:0:6), Pplus=(1:0:1), J=Jac(C).

Use actual maps f0:C->E of degree2, f1=f0 composed with R of degree2,
and f3:C->E of degree3, the R quotient. Here

    T=diag(1,1,6), R=[[1,5,2],[0,5,0],[2,1,1]].

The entire prime-field canonical-frame census gives Aut_k(C)=S3,
with three involutions conjugate to T and all corresponding quotients
genus1. It did not enumerate extension automorphisms. The actual
3-torsion conic proof constructs the k-isomorphism beta:E^3->J and
integral inverse alpha; in particular this is an actual abelian variety
statement, not just a free-module/Hermitian classification. E has trace-3,
#E(k)=11, Frobenius pi satisfying pi^2+3pi+[7]=0, and therefore
#J(k)=11^3. These are inherited hypotheses/evidence, not newly replayed
populations (941363, d9f917, 05a984). [pcc:unverified]

With omega=pi+2, ell=(omega,1,0), the already integral actual Hom
L=ell alpha has L L^dagger=[4]. Put

    f4(P)=L([P-Pminus]).

Its degree is4 by actual Jacobian push-pull, and the global Hom identity is

    [3]L = pi f0_* + (2pi+[3])f1_* - f3_* .                 (1)

This is the genuine globally divisible Hom from beta (3c0acb), not the
pointwise inverse3mod11 expression. Since 7 does not divide4, f4 is
separable. We prove that it remains primitive over kbar and that both
its geometric and arithmetic monodromy groups on its four sheets are S4.
[pcc:unverified]

## 1. Smoothness, canonical theta and parity

On Y=1 write x=X/Y,z=Z/Y, g=6x^2+3x+1 and f=x^3+2x+6. The equation is

    (z^2+g)^2=f,          B=g^2-f=x^4+4x+2.

At an affine singularity, partial_z=4z(z^2+g) vanishes. If z=0 then
B=B'=0. But B'=4(x-3)(x-5)(x-6) and B(3),B(5),B(6)=4,3,6.
If z^2+g=0 then f=f'=0. But f'=3(x-2)(x-5) and f(2),f(5)=4,1.
These split critical polynomials exclude affine singularities over kbar,
not just at k-points. At Y=0 the equation is (Z^2-X^2)^2=0; the two
points are Pminus/Pplus, with partial_Y=6 after X=1. Thus C is
geometrically smooth. A smooth plane curve cannot have distinct geometric
components, since Bezout would make their intersection singular. Hence
C is geometrically integral, genus3, nonhyperelliptic, and canonically
embedded by O_C(1), by plane genus and smooth hypersurface adjunction.
[pcc:unverified]

The line Y=0 cuts out 2Pminus+2Pplus. Consequently

    theta0=Pminus+Pplus,           2 theta0 ~ K_C.         (2)

For a nonhyperelliptic genus3 curve an effective degree2 divisor cannot
have h0>=2: a moving part would give a map to P1 of degree at most2;
degree1 contradicts positive genus and degree2 contradicts the canonical
embedding. Thus h0(theta0)=1. Since J(k) has odd order, J(k)[2]=0,
and any other k-rational theta divisor class equals theta0.
[pcc:unverified]

For an independent parity argument, T fixes on C exactly the points
with Z=0. Its other projective fixed point (0:0:1) is not on C, and
there is no point with Y=Z=0 on C. On Y=1 the fixed locus is B=0.
The values of B at x=0,...,6 are (2,0,5,4,1,3,6), so T has exactly
one k-point fixed on C. It pairs every other k-point, including Pminus
and Pplus. Therefore #C(k) is odd, without using or repeating the earlier
17-point enumeration. Native A checks these polynomial/local leaves.
[pcc:unverified]

## 2. Every geometric group endomorphism of J descends

Let F be the relative 7-Frobenius endomorphism of actual J, with its
k-model identifying the Frobenius twist with J. Because beta is over k,

    F^2+[3]F+[7]=0.

Define integers (a0,b0)=(1,0) and

    a_(n+1)=-7b_n,       b_(n+1)=a_n-3b_n.

Induction in the actual endomorphism ring gives

    F^n=[a_n]+[b_n]F.                                     (3)

For n>=1, a_n=0 mod7 and b_n belongs to {1,2,4} mod7, cycling by
multiplication by4. In particular b_n is nonzero for every positive n.
The separate Lean B module proves exactly this residue invariant and
nonvanishing for all n on standard Int/Nat, not just bounded indices.
Its End(J) interpretation in (3) is a written induction. [pcc:unverified]

Let h be any geometric GROUP endomorphism of J. Its finitely many
algebraic coefficients put it over some F_(7^n), n>=1, so it commutes
with F^n. Equation (3) implies

    [b_n](hF-Fh)=0.

Hom groups of abelian varieties are torsion-free: alternatively, if
[m]u=0 with m!=0, then u[m]=0 and the isogeny[m] is faithfully flat
and surjective, so u=0. Thus hF=Fh. Naturality of relative Frobenius says

    F h = h^(7) F.

Surjectivity of F now gives h=h^(7), hence h is defined over k. This
uses relative Frobenius and finite-field Galois descent; absolute
Frobenius over kbar must not be substituted as a kbar-linear map.
The same proof applies to End(E) and to Hom between powers of E or
between those powers and J, because they have the same quadratic
Frobenius relation. No assertion about arbitrary translations as variety
automorphisms is made. [pcc:unverified]

The map Aut(C_kbar)->Aut(J_kbar) is faithful and Galois equivariant:
the induced action on H0(J,Omega1) identifies with that on H0(C,Omega1),
and the canonical embedding makes the latter action faithful. All
induced Jacobian endomorphisms have just descended to k. Therefore every
geometric curve automorphism descends, and

    Aut(C_kbar)=Aut(C_k)=S3.                              (4)

No classification of the full ordinary geometric endomorphism order,
root-of-unity quotient of Frobenius eigenvalues or stronger Torelli
statement is required. [pcc:unverified]

## 3. f4 is etale at both points of the bitangent

T swaps Pminus/Pplus. Thus the degree2 quotient f0 is etale at both.
R fixes both points, so f1=f0 composed with R is also etale there.
The actual curve jets modulo t^3 are

    Pminus: X=1+t-t^2, Y=2t^2, Z=t-1,
    Pplus:  X=1+t-t^2, Y=2t^2, Z=1-t.

For f0 near O use u=-xE/yE=-XY/(Z^2+6X^2+3XY+Y^2).
Its leading coefficient is4t at both points; f1 has leading2t at
Pminus and1t at Pplus. Consequently the tangent eigenvalues of R are
4 and2, both nontrivial cube roots. Invariance f3 R=f3 makes the
pullback of an invariant differential of E vanish at both points.
Equivalently the tame degree3 quotient is totally ramified there.
These are actual local curve calculations, checked by native A; no
explicit f4 rational function was evaluated. [pcc:unverified]

After composing (1) with the Abel map, any target translation constants
vanish on invariant differentials. For a nonzero invariant differential
eta on E, pi^*eta=0, since pi is Frobenius. Therefore

    3 f4^*eta = 3 f1^*eta - f3^*eta.

At both Pminus/Pplus this becomes f4^*eta=f1^*eta!=0. Thus f4 is etale
at both points. The leading f4 differential coefficient here is in the
invariant differential trivialization, not a claimed explicit coordinate
formula at f4(Pplus). [pcc:unverified]

## 4. Geometric primitivity

Suppose f4 factors over kbar as C --q--> D --h--> E with both degrees>1.
Degrees must both equal2. The separable quadratic q has a deck involution
of C_kbar. By (4) it is one of the three k-involutions. Its quotient q
is over k and has genus1; D has the k-point q(Pminus). Since f4 and q
are over k and q is dominant, uniqueness of h descends h to k as well.
Choose the indicated origin on D and translate E by h(q(Pminus)). A
map of genus1 curves preserving origins is a group homomorphism, here
a degree2 k-isogeny D->E. Its dual E->D has an etale degree2 kernel
stable under Frobenius. The unique nonzero geometric element of that
kernel is then k-rational. This contradicts #E(k)=11.
Hence f4 is geometrically primitive. In particular this excludes all
geometric intermediate degree2 quotients, not only those initially
found in the prime-field census. [pcc:unverified]

## 5. Geometric and arithmetic monodromy

Let G be the geometric and A the arithmetic Galois-closure permutation
groups of f4 in S4. Geometric connectedness makes G transitive; G is
normal in A, with A/G the constant-field Galois group. Primitivity of
the cover is primitivity of G's natural action. The complete natural
subgroup certificate (start at identity, close after adjoining every
one of24 elements, then induction on generators) leaves only A4 and S4.
Because 7 does not divide24, all inertia in this closure is tame and
cyclic. We exclude G=A4. [pcc:unverified]

### 5a. The constant quadratic possibility is impossible

If G=A4 and A=S4, the constant extension is quadratic. Above an
unramified k-point of E a Frobenius element is odd, so its four-sheet
permutation has either0 or2 fixed points. Above a ramified k-value,
a nontrivial A4 inertia generator is a3-cycle or a double transposition.
In both cases inertia has exactly two orbits on the four sheets. The
distinct geometric points in that fibre correspond to inertia orbits;
Frobenius acts on these two orbits and hence fixes0 or2 of them.
This also follows from every odd normalizer case in the certificate.
Rational points mean fixed geometric points, not multiplicity-weighted
sheet degree. Every k-rational point of C lies over a k-value on E.
All such fibres would therefore have even k-point count, contradicting
Section1. Thus an A4 alternative must have A=G=A4, with constants k.
[pcc:unverified]

### 5b. Exactly two geometric branch values

A nontrivial A4 tame inertia cycle type is3+1 or2+2. Its degree4
ramification contribution is2 in either case. Riemann-Hurwitz gives
total ramification2g(C)-2=4, since E has genus1. There are therefore
exactly two geometric branch values. Quotienting the regular A4 Galois
closure by its normal V4 gives a geometrically connected cyclic3 cover
D->E over k. Inertia3 survives in A4/V4=C3 and inertia2 is killed.
There are three possibilities for its number of geometric branch values.
[pcc:unverified]

### 5c. No surviving 3-cycle branch value

Then D->E is etale of degree3, so D has genus1. The Weil bound gives
#D(k)>=8-2sqrt(7)>0, without presupposing a rational origin. Choose one
and translate the target; D->E becomes a k-isogeny of degree3. Its dual
would give a Frobenius-stable line in actual E[3](kbar). On this two-
dimensional F3 space, Frobenius has characteristic polynomial
X^2+3X+7 = X^2+1 mod3, with values1,2,2 at0,1,2. It has no eigenline.
Thus no such isogeny exists. A constant cyclic cover is not an escape:
A=G already made this quotient geometrically connected. [pcc:unverified]

### 5d. Exactly one surviving 3-cycle branch value

Over kbar, Kummer theory applies to kbar(E): characteristic7 is prime
to3 and it contains mu3. The cyclic extension has a generator z with
z^3=a, a in kbar(E)^*. At a geometric point with valuation divisible
by3 the unit part has a cubic root in a strict henselian/completed local
ring (3 is invertible, residue field algebraically closed); there is no
ramification. At a valuation not divisible by3 the extension is totally
ramified. A single geometric branch value would give exactly one
valuation of a nonzero mod3, all others0 mod3. But a principal divisor
has total degree0, and each geometric point has degree1. This is
impossible modulo3. This argument is over the actual elliptic function
field, with no assumption that the base is P1. [pcc:unverified]

### 5e. Both surviving branch values are 3-cycle values

The ramification divisor of f4 is then2R+2S for distinct geometric
points R,S of C. Since f4 is over k, D0=R+S is a k-rational effective
divisor of degree2: it is the unique half of this Galois-invariant
divisor. The pullback of eta has no poles, so by Riemann-Hurwitz

    2D0 ~ K_C ~ 2theta0.

Thus D0-theta0 belongs to J(k)[2]=0, giving D0~theta0. Section1 has
h0(theta0)=1, hence equality of effective divisors D0=theta0.
In particular the two ramification points would be Pminus/Pplus,
contradicting Section3, where f4 is etale at both. No rationality of
R and S individually was assumed before this divisor argument.
[pcc:unverified]

All three cyclic-quotient branch cases contradict established hypotheses.
Therefore G!=A4. It follows that

    G=S4, A=S4, and the Galois closure has constants F7.

This proves the stated written theorem subject to its explicit inherited
actual-object inputs and standard bridges. The finite certificate is
complete for its own natural action; the Lean theorem is complete for
its quantified arithmetic dependency. Neither is advertised as a full
formalization of this geometric theorem. [pcc:unverified]

## Consequences and precise limits

The actual degree4 retraction cover has no nontrivial geometric factor
and has full S4 monodromy. Passing to an extension field does not create
missing group endomorphisms or curve involutions for this actual J.
This constrains a low-degree quotient-chain approach to evaluating f4;
it does not exclude other correspondences or give a complexity lower
bound. The inherited nontrivial transfer tau still has the genuine global
left inverse L, so DLP scalars are preserved, but no computational speedup
follows from the current theorem. [pcc:unverified]

Explicit f4 rational functions, a full actual-object Lean geometry bridge,
independent signed verification, crypto-scale family/cost analysis and
whole original approved durable ELF/olean custody remain open. The
finite-point inverse3mod11 map's degree576 negative remains unchanged.
Next inspect actual fibre/pole divisor classes, then genuine function-
field/Riemann-Roch extraction. The theorem is for this F7 curve and is
not silently extended to every prime in a CM family. [pcc:unverified]

## Standard reference map

Milne, Abelian Varieties v2.00: I Lemma10.6 (Hom torsion-free), III
Theorem11.1 (Weil bound). Stacks0CC6 (relative Frobenius/naturality),
0C1B (differential divisor, tame Riemann-Hurwitz),09I6 Lemma9.24.1
(Kummer cyclic generator),0BYA (plane genus). Vakil, The Rising Sea,
July27 2024:19.7.3/19.7.D (canonical quartics/automorphisms),21.5.B
(smooth adjunction). Quotient descent, the group-homomorphism property of
origin-preserving elliptic maps, dual isogeny and the function-field
Galois-closure/inertia correspondence are standard structural bridges,
stated explicitly here rather than claimed as native/Lean leaves.
Full URLs, rejected search hit and exact citation correction are in the
immutable PRIMARY_SOURCES_A and attached ADDENDUM_B. [pcc:unverified]
