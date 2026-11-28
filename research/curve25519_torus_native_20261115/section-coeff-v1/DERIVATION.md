# A reduced support predicate and its literal degree

This is a written, partially formalized derivation for the original torus
window. Native qualification and two existing-law Lean endpoints are separate
evidence. No novel algorithm, root-discovery cost or signed theorem standing
is asserted. [pcc:unverified]

## Objects and hypotheses

Let F=Fp, p odd, and Fp2=F(sqrt(2)) with2 nonsquare. The Montgomery curve is
y²=C(x)=x³+A x²+x, A nonzero and A²!=4. Let h have exact odd order aq in the
norm-one torus, with p not dividing aq, q odd, and 1<=k with2k<a. The raw
columns are z(j,t)=h^(j+at), 1<=j<=k,0<=t<q. Work before the eightfold map;
different columns may still have the same eightfold image.

The actual fixed contract is p=2^255-19,A486662,a60824497,q213156431,k20633.
The development toy is p101,A95,a17,q3,k8. These parameter identities are
inherited from the frozen native v1 source; the present component adds no
primality, exact-order or nonsingularity certificate.

For a finite pretarget R=(r,s),s!=0, fix c4=1 and set

    U=(X-r)(c0+c1 X+c3 X²)+c5 s
    V=(X-r)(c2+X)+c5
    S=-(U²-C V²)/(X-r).

This is a monic sextic with five free coefficients c0,c1,c2,c3,c5. At R=O,
fix c5=1, take U=c0+c1X+c3X²+X³,V=c2+c4X and S=U²-CV². The original local
line-bundle/jet conventions, including the removable -R chart, are unchanged.
Finite two-torsion targets remain unsupported. Every candidate must retain
Q=[8]R and pass original indexed-row/group replay.

## Split and separable membership

Put omega=h^q and a_j=Re(omega^j). For the standard half-trace Chebyshev T_q,

    H_lambda(T)=product_(j=1..k)(T_q(T)^2-a_j²),  D=2qk.

Its roots are exactly the2qk values ±Re(z(j,t)). Indeed
T_q(Re z)=Re(z^q)=a_j, and odd q supplies the negative roots. Their
distinctness follows from the quadratic equation z²-2lambda z+1=0:
equal trace means z'=z or z^-1, and opposite trace means z'=-z or -z^-1.
The odd h-subgroup excludes the negative alternatives. Inversion would give
j+j'=0 mod a, impossible for0<j+j'<=2k<a. Equality forces j=j',t=t'.
Degree D and D distinct displayed roots establish splitting and separability.
No root is0 or±1: those traces would give torus elements of order4 or at most2.

With L=2X+A, clear the Moebius denominator:

    E_B(X)=L^D H_lambda(A/L).

Its degree is D since H_lambda(0)!=0; E_B(-A/2)!=0 since its value there is
the nonzero leading coefficient of H_lambda times A^D. Thus E_B is also
split and squarefree over F. The existing fast H_interval equals this
membership expression up to invertible denominator/scalar factors in F[X]/S.
Its original L inverse guard must stay. Dense toy checks compare vanishing,
not a claim of byte-equal polynomial normalizations. [pcc:unverified]

The same trace argument proves that the two rational x candidates for each
raw label are disjoint from those for every other label. Consequently,
distinct original raw labels imply distinct pre-[8] x values. This uses the
specific odd-order positive window and does not justify squarefree truncation
of a generic factor base with repeated signed support. [pcc:unverified]

## Rational ordinates from the norm

Take coefficients in F and suppose S divides E_B. Then S is squarefree and
all its roots x lie in F. If x!=r and C(x) were a nonsquare, the equality
U(x)²=C(x)V(x)² forces U(x)=V(x)=0. Both U,V would have the factor X-x;
their norm would have its square, and division by X-r could not cancel it.
That contradicts squarefree S. At x=r the ordinate is already s in F. The
identity-target chart has no forced factor and uses the same argument at
every root.

Moreover no membership x has C(x)=0. The root x0 would give lambda1.
For any other zero of C, x(x+A)=-1 and
lambda²-1=4/(2x+A)² is a nonzero square. But a nontrivial norm-one trace
has lambda²-1=2b², b!=0, a nonsquare. This is a contradiction.

Hence every support point has nonzero rational ordinate. Original local
lifting chooses the signed zero of the section, with its existing forced
factor convention at -R. This last line-bundle/native binding is checked by
the original extractor in controls, not by the two Lean endpoints.

The argument is over F-valued section coefficients. It does not remove the
cost of Fp root extraction or filtering from a solver. Arbitrary extension-
field coefficients can support nonsquare ordinates without the above
common-root implication; no algebraic-closure equivalence is claimed.
[pcc:unverified]

## Original nonidentity and distinct-row contract

Classical Montgomery XZ doubling is

    Nnext=(N²-D²)²,
    Dnext=4ND(N²+A ND+D²),  (N0,D0)=(X,1).

For a nonsingular Montgomery curve this pair has no common projective zero.
Three iterations give D3 of degree63 and N3 degree64. At rational nonzero-
ordinate support, D3(x)=0 precisely when [8]P=O. Thus gcd(S,D3)=1 excludes
identity images. It must be retained: the toy has two raw columns with such
images, even though they lie on the curve and satisfy torus membership.

For supported Fp charts the reduced candidate predicate is therefore

    E_B mod S=0,     D3 a unit modulo S.

It replaces explicit Frobenius/Legendre powers only for this original
distinct-raw contract; repeated raw labels are already forbidden. Repeated
post-[8] images remain allowed. Unit conditions can be encoded with inverse
auxiliaries in the quotient, or as a nonzero resultant, with their full cost
charged. No six-member original check is removed. [pcc:unverified]

## Literal degree and what it does not show

Each coefficient s_i of normalized finite S=X6+s5X5+... has total degree<=2
in the five unknowns. On the formal pencil c3=T, with the others fixed,
s5=-T²+terms of smaller T-degree. Reduce X^n using the monic recurrence.
The X5 remainder coefficient has degree at most2(n-5) for n>=6. To reach
that degree, every reduction must drop the X exponent by exactly1 while
contributing T²: this is uniquely the -s5 branch. Any branch with i<5 drops
at least2 X degrees for at most2 T degrees and has strictly smaller total
degree. Thus that unique path has nonzero leading coefficient. Terms of
E_B below degree D have strictly smaller maximal T degree. The X5
coefficient of E_B mod S consequently has exact degree

    2(D-5)=4qk-10.

This calculation concerns formal polynomials, without coefficient-function
reduction modulo T^p-T. Toy q3/k1..8 predicts2,14,26,38,50,62,74,86;
actual q/k predicts17,592,226,563,282. The native worker computes the
integers and exact toy coefficient vectors; actual E_B is never expanded.
The generic leading-path induction is not yet formalized. [pcc:unverified]

Removing p-dependent support tests gives a smaller literal algebraic system,
but these generators still grow with B=qk. This rules out only the claim
that these literal equations have degree bounded independently of B. It is
not a lower bound on minimal generators, solving degree, regularity,
arithmetic-circuit root solving or other representations. Circuit size and
algebraic degree are separate quantities. Complete coefficient discovery,
original-root lifting and cold-cost/useful-yield analysis remain required.

## Sources and prior work

- Gassert, *Chebyshev Action on Finite Fields*, arXiv1209.4396,
  https://arxiv.org/abs/1209.4396 . Its monic trace convention uses z+z^-1;
  ours is half-trace, so D_n(2lambda,1)=2T_n(lambda). Classical trace theory.
- EFD Montgomery XZ formulas, doubling dbl-1987-m, sourced to Montgomery1987,
  p261: https://hyperelliptic.org/EFD/g1p/auto-montgom-xz.html . Classical map.
- Official Sage Mumford divisor implementation documentation:
  https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/hyperelliptic_curves/jacobian_morphism.html .
  Confirms classical polynomial divisibility representation; no Sage execution.
- Internal RR9 result: research/prime31_ic_20261115/rr9/RR9_RADICAL_SUPPORT_RESULT_20261117.md
  and docs/session-logs/2026-11-17-prime31-radical-support.md. Radical support
  and squarefree reduction predate this component; generic repeated-support
  coverage objections are real and motivate the special raw-injectivity proof.
- Internal public norm and frozen section-query-v1 contracts retain original
  norm/lifting conventions. KB tools/executable unavailable; Cantor1987 AMS
  PDF403. These gaps preclude an exhaustive novelty claim.
