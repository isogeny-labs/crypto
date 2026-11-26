# Actual-map composition beyond the fixed panel

Frozen before implementation/execution, 2026-11-26. PR #5256, stacked on
#5255; Conductor T-3. The explicit user coordinate-screen/matched-rho waiver
persists. No new PCC registration, scalar recovery or speed comparison.

Objects: the retained smooth genus-3 source H, plane quartic C, degree-two
cover pi:H->E, actual generic cubic correspondence Gamma:H->Sym^3(C), and
the retained rational degree-six morphism f:C->E. Set F=phi*pi^* for the
homomorphism induced by Gamma and B=f_* on degree-zero classes. Do not assume
the original polarized-quotient identity to derive B*F=[6]. Do not identify
every software transpose with B merely from a name or differential.

H-C: a separate exact arithmetic check gives

    phi^*(f^*omega_E) = 3*pi^*omega_E,

and an actual-equation degree bound excludes a nonzero inseparable difference
B*F-[6]. If all checks pass, the written geometric argument establishes the
identity as a homomorphism E->E, including extension-field points. This does
not by itself implement arbitrary projective inputs of the older Gamma^t
software or give an independent cryptographic signature.

Reuse the earlier separate OpenSSL field/polynomial and projective sparse
arithmetic as build-only headers with mains removed; import no original GMP
producer arithmetic. Implement the quadratic function field Fp(S)[Y]/(Y^2-h)
and its monic cubic U algebra separately. Verify source inverses, quadratic
differentiation and quotient-algebra units by exact multiplication.

Read the exact retained U,V. Require monic cubic U, V degree at most two,
source h squarefree, and U_T a unit. Evaluate the full quartic equation at
(T,V) in the generic algebra, preserving the entire rank-three algebra.
Differentiate U and V exactly, check the differentiated quartic equation,
and compute the trace of

    L(T,V) * (-U_S/U_T) / F_y(T,V).

Require equality in the source function field to 3*l(S)/(2Y), with
l(S)=16*S/1024^2+16*S^2/1024. Reconstruct a multiplication trace directly in
the basis 1,T,T^2; compare it with the cubic Newton trace formula. Record
the exact reduced trace and the field/algebra phase clocks.

Independently verify the actual f differential using the cleared bivariate
identity for X=A/B, Y=C/D:

    D*((F_y*A_x-F_x*A_y)*B-A*(F_y*B_x-F_x*B_y))=2*L*C*B^2 mod F.

Recheck its elliptic equation and degree-six Hilbert dimensions as in #5255.
Reconstruct pi's rational coordinates N/D,c^4*Y/D^2 for c=1024,
N=1+2cS+8c^2S^2 and D=1+2cS. Require the cleared elliptic equation and
N'D-ND'=c^4*l, with coprime N,D and x-function degree two.

For an intentionally conservative correspondence degree bound, clear all U
coefficient denominators with their exact polynomial lcm. Write the result
A(S,T)+B(S,T)Y. Its norm A^2-hB^2 is nonzero: its T^6 coefficient is the
square of the common denominator. Let

    M=max(2*maxdeg_S A, deg(h)+2*maxdeg_S B).

Require 0<M<=4096. Every nonconstant component above generic C satisfies
this degree-at-most-M polynomial in S and a quadratic equation in Y, so the
total projection degree to C is at most 2M. Components constant on C induce
zero Jacobian maps. Degree over H is at most three. Do not substitute the
producer's claimed projection degree nine for this independently calculated
bound.

Using canonical Jacobian polarizations, pullback along a degree-d curve map
has norm sqrt(d), and pushforward is its adjoint with the same norm. Derive
this from pull-push=[d] and the positive Rosati trace inner product on
Hom(E,J). For disconnected Gamma, apply Cauchy-Schwarz to the component
degrees. Thus deg(BF)<=6*3*(2M)*2=72M. The quadratic degree norm on End(E)
gives deg(BF-[6]) <= (ceil(sqrt(72M))+6)^2. Compute this bound in integers
and require it strictly below p. The zero differential makes any nonzero
difference inseparable, hence its degree at least p; contradiction. State
the retained smooth-model and standard push/pull/trace hypotheses explicitly.

Controls must reject: changed f differential L in the cleared identity;
changed generic V in quartic incidence; wrong trace multiplier two instead
of three; omitted nonconstant U coefficient in the clearing reconstruction;
and an integer degree envelope at least p. Controls may reuse already
computed exact values; do not count omitted computation as rejection.

One CPU, 1 GiB address space, strict build <=300 seconds, native worker
<=600 seconds. Input coefficient degrees <=128 and M<=4096 are fixed caps.
Fresh attempt directories retain source/input snapshots, all hashes, full
errors/exits and build/worker/complete-wrapper diagnostics. No widening or
new source panel after a failure. A timeout is an unresolved computation.
Metadata, literature, reporting and publication costs are outside wrappers;
original map synthesis remains separately unpriced. Prior evidence unchanged.

Sources reviewed: parent GLOBAL_CORRESPONDENCE_PROTOCOL.md,
ELLIPTIC_BACKMAP_PROTOCOL.md, POLARIZED_QUOTIENT.md, exact producer sources,
separate correspondence replay and #5255. Standard geometric ingredients:
Milne, Abelian Varieties v2.0 (2008), I.14 Theorem 14.3 (positive Rosati
trace form), III.6 (Albanese/autoduality),
https://www.jmilne.org/math/CourseNotes/AV.pdf ; Snowden, Math 679 Lecture 2
(2013), divisor push/pull, separability and quadratic degree,
https://websites.umich.edu/asnowden/teaching/2013/679/L02.html ; Galbraith's
corrected chapter 7, Lemma 7.3.6 for extension of curve maps. KB remains
unavailable as previously checked. This is verification of actual coordinate
maps using established methods; no novelty assertion.
