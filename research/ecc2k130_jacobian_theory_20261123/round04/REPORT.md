# Explicit basis and smaller-field norm tests

2026-11-23. Continuation of draft PR5079 and the genus130–300 carrier
question. No curve or evaluated subgroup correspondence has been found.
All mathematical claims here have unsigned PCC standing. [pcc:unverified]

## What this changes

The explicit maximal integral basis of K+=Q(u) checks out by two exact
discriminant calculations. PARI now initializes the field and identifies
the specified norm263 ideal. The bounded class-group successor still
times out before returning a class result. This removes the earlier
power-order initialization bottleneck, without deciding narrow
principality. [pcc:unverified]

The necessary norm conditions pass in the real subfields of degrees
2,5,10,13,26: exact generators of norm263 are retained, and exact Sturm
counts show every conjugate positive. The degree65 test times out before
returning a class result. No smaller-field exclusion was obtained.
These separate norm generators do not lift to a generator in degree130.
[pcc:unverified]

For context, the round03 candidate assumes the natural A occurs as the
actual A-subvariety of a genus131 Jacobian, with elliptic-cover degree
less than1213355897362451. Under those assumptions the remaining degree
is34453, and its polarization requires a totally positive generator of
p263=(263,u+1). Arbitrary isogenous A-factors, larger cover degrees and
genus132–300 remain outside that reduction. [pcc:unverified]

## 1. Field and integral basis

Keep alpha²+alpha+2=0, beta=-1-alpha, zeta a primitive131st root,
pi=alpha*zeta, u=pi+2/pi and s=zeta+zeta^-1. Put
F=Q(s), of degree65. The relative conjugate of u is
u'=beta*zeta+alpha*zeta^-1. Direct multiplication gives

    u+u'=-s,
    u*u'=2*s²-7,
    u²+s*u+2*s²-7=0.

The relative basis1,u has discriminant7*(4-s²). Also
Norm_F(2-s)=131 and Norm_F(2+s)=1, by evaluating the131st
cyclotomic polynomial at1 and-1. Thus its relative discriminant
has absolute norm7^65*131. [pcc:unverified]

The cyclotomic real ring is O_F=Z[s]. Indeed, using
zeta²=s*zeta-1 writes every element of Z[zeta] as a+b*zeta,
with a,b in Z[s]. Complex conjugation fixes such an element only when
b=0, by linear independence of1,zeta over F. The basis
1,s1,...,s64, sj=zeta^j+zeta^-j, differs from1,s,...,s^64
by a monic integral triangular change. Its discriminant is131^64.
[pcc:unverified]

Consequently the integral order O_F+u*O_F has absolute discriminant

    D = (131^64)^2 * (7^65*131) = 7^65*131^129.

This is the field discriminant. One justification uses the previously
specified CM field K=Q(sqrt(-7),zeta). Its discriminant is
7^130*131^258, by the coprime discriminants of the two factors.
Complex conjugation is in neither inertia group at7 or131: it acts
nontrivially on both factors, whereas each inertia group acts on only
one. Therefore K/K+ has trivial finite relative discriminant, and the
absolute discriminant formula gives disc(K+)=D. Equality of the order
and field discriminants makes the displayed130 elements a maximal
integral basis. This is written mathematics, not a formal proof.
[pcc:unverified]

## 2. Two exact arithmetic checks

The GP construction uses A0=0,A1=1,A_(j+2)=u*A_(j+1)-2*A_j,
and the source recurrence U0=0,U1=1,U_(j+2)=-U_(j+1)-2*U_j.
Since pi^131=alpha^131, the two power recurrences give

    s = (2*A131+u*(U130-A130))/U131 modulo H.

Let D0=2,D1=u,D_(j+2)=u*D_(j+1)-2*D_j. Then

    sj = (Uj*(s*Aj+A_(j-1))-U_(j-1)*Dj)/2^(j-1).

For the second identity, use alpha^j=Uj*alpha-2*U_(j-1),
and beta*pi^j+alpha*(2/pi)^j=2*s*Aj+2*A_(j-1).
This avoids iterated powers of a rational expression for s.
[pcc:unverified]

basis.gp checks the quadratic identity, all64 Chebyshev identities and
the real cyclotomic equation. In the power basis of the pinned H it
checks poldisc(H)*det(M)^2=D exactly. H is the degree130 irreducible
real-Weil polynomial retained in round03; that input is unchanged.
[pcc:unverified]

Independently, gram.cpp constructs the trace Gram matrix directly in the
cyclic Laurent ring Z[zeta]/(zeta^131-1). For a real Laurent expression
a, Tr_F(a)=(131*a0-sum(a_j))/2. Relative traces of1,u,u² are
2,-s,14-3*s². Exact fraction-free Bareiss elimination gives determinant
D, with every division checked. Controls n=3,5,11 also match
7^((n-1)/2)*n^(n-2). Optimized and UBSan builds give identical
output. These are two arithmetic implementations, not independent
human review or signed verification. [pcc:unverified]

## 3. Initialized-field successor

Official PARI documentation accepts nfinit([pol,Basis,ramified_primes]).
The native Buchall_param implementation accepts an initialized nf via
get_nfpol. The degree2 control in interface.gp confirms that interface,
including a principal norm5 ideal in Q(sqrt21). [pcc:unverified]

The degree130 successor initializes nfinit([H,Basis,[7,131]]), checks
the expected discriminant and signature[130,0], and verifies that
(263,u+1) has norm263. It then calls bnfinit on that field. The process
hits its30-second cap, exit124, before bnfinit returns. No ordinary
class vector, principal generator or narrow result is produced.
This is an inconclusive fixed-input certificate attempt; it is no
evidence of nonprincipality. No paid cloud resource was created.
[pcc:unverified]

## 4. Necessary subfield tests

K+ is the unique real degree130 subfield of Q(zeta917). The even
cyclotomic quotient is cyclic of order390: the2-primary part of
C6*C130 modulo simultaneous complex conjugation is C2, and the
remaining odd factors are3,5,13. Therefore each real field of degree
2,5,10,13,26 or65 is uniquely specified and lies in K+.
Prime263 splits completely in K+: its residue is1 modulo131 and a
quadratic residue of order3 modulo7. [pcc:unverified]

If p263=(c) with c totally positive, relative ideal norm to each such
field is a norm263 prime generated by the totally positive element
Norm(c). In a Galois field, all primes over263 are conjugate, so testing
any one is enough for this necessary obstruction. Passing separate
tests need not yield compatible elements or a lift. [pcc:unverified]

The first bounded script certifies the ordinary/narrow results in
degrees2 and5 using bnfcertify=1, then times out during certification
in degree10. A successor avoids needing a whole class-group certificate
for positive witnesses: bnrisprincipal returns a compact candidate,
which is expanded and checked by exact principal-ideal equality,
norm263 and polsturm(charpoly(generator),[-infinity,0])=0. A monic
integral characteristic polynomial also records integrality.
[pcc:unverified]

The separate replay_witnesses.gp reads only the retained explicit field
and generator expressions. Without bnfinit, it checks field
irreducibility and real signature, the monic integral characteristic
polynomial, norm263 and exact positivity by Sturm counts. Thus the
positive norm witnesses also replay without trusting a class-group
or unit computation. Their norm263 principal ideals are prime; Galois
conjugation supplies witnesses for every prime above263. [pcc:unverified]

| Subfield degree | Exact principal ideal equality | Norm | Nonpositive conjugates |
|---|---|---|---|
|2|pass|263|0|
|5|pass|263|0|
|10|pass|263|0|
|13|pass|263|0|
|26|pass|263|0|
|65|not reached before timeout|unresolved|unresolved|

For degree2 the generator is13*x+191 with x²-x-229=0, recovering
(395+13*sqrt917)/2. Every other polynomial and generator is retained in
subfield_generators.txt. The displayed provisional class numbers are
not promoted to unconditional results. The exact positive witnesses
do not depend on a complete unit-group computation: ideal equality and
Sturm counts check the supplied elements directly. [pcc:unverified]

## 5. Failure and artifact record

The first generator-script invocation used ray-class flag4 and received
only a class vector; indexing it failed. The retained initial error is
followed by corrected flag5 (generator and compact representation).
One nested degree65 read stopped on stack resizing; its empty output
and exit are retained, and the corrected runner initializes the stack
before reading. The substantive degree65 attempt exits124 with no
class result. Some readback commands used the wrong current directory;
absolute paths corrected those without changing arithmetic inputs.

basis_data.gp is472468 generated bytes, SHA256
a4b892c0909dfdb00e2a01f718cfcc6f81d082de119f69b0939d692bffb24947.
It remains local and gitignored under the artifact-size rule. Regenerate
it with basis.gp; no remote archival is claimed. The existing missing
AWS credentials also prevent archival of this object. All report source,
checkers and smaller witness outputs are committed. [pcc:unverified]

## Remaining answer

There is still no demonstrated working genus130–300 attack route.
No result here proves the general curve exists or excludes it. The
degree130 narrow-principality condition remains unresolved, followed
by compatible gluing, the Jacobian/Torelli condition and an evaluated
subgroup-preserving map. Genus alone would also be insufficient to
import the hypothetical2^37.17 destination-solver estimate unchanged.
[pcc:unverified]

For genus130 the entire Jacobian would be isogenous to A. For larger
genus the condition must be J(C) isogenous to A*B; dimension prevents
an isogeny of the entire larger Jacobian with A. No larger-genus
complement is constructed in this round. [pcc:unverified]

Sources: official PARI2.19.1 nfinit,bnfinit,bnrisprincipal,bnfcertify,
polsubcyclo,nffactorback documentation and native source:
https://pari.math.u-bordeaux.fr/dochtml/ref/General_number_fields.html
and https://pari.math.u-bordeaux.fr/dochtml/ref/Galois_theory_and_class_field_theory.html.
Cyclotomic discriminant and integral-basis background:
https://www.jmilne.org/math/CourseNotes/ANT.pdf, Chapter6, including
the cyclotomic integer-ring theorem and Lemma6.5 on coprime discriminants.
The polarization and gluing assumptions are documented with primary
references in the pinned parent and round03 reports, notably
https://arxiv.org/pdf/2110.04221.
