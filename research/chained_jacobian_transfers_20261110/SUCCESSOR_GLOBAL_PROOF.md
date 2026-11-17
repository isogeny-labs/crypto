# CT24: the next global seven-transfer and composite kernel

This is a written construction from the actual elliptic maps and HLP
quotients. The next whole-Jacobian rational evaluator and its exhaustive
native controls are still to implement. Finite certificates do not replace
the geometric argument. All scientific standing is unsigned.
[pcc:unverified]

## Fixed objects and source evidence

OverF101, let Ai=E109,i × E113,i. The actual elliptic chains are

    (1,46),j12 --alpha1--> (26,58),j87 --beta1--> (59,27),j47,
    (44,21),j58 --alpha2--> (69,50),j92 --beta2--> (23,7),j89.

All four arrows have degree7. The new beta kernels are[99,94,75,1] and
[53,19,40,1]. Exact curve/global dual-x identities, positive dual[7] signs,
all109/113 point/scalar/bijection/recovery checks and actual7-kernel
Frobenius5/6 pass in preflight01/PCCb07c76. Exact previous-dual kernels
have Frobenius2/4 and are excluded by equality of their kernel polynomials,
not by a fitted j criterion. [pcc:unverified]

The accepted J0/J1 are Jacobians of monic even C0=(56,13,6), C1=(68,42,26)
in the convention y²=x6+d*x4+c*x²+b. The actual new transported2-gluing
has raw sextic[45,0,29,0,84,0,13]. Dividing the ordinate by35 gives
C2:y²=x6+22*x4+10*x²+19. It is smooth and has the product L-polynomial,
#J2(F101)=12317=109*113. Preflight05/PCC190ae8 checks both actual quotient
charts (destination,short shift,scale)=[(1,41,18),(0,37,18)], all120 curve
point images including x0/infinities, full109/113 coordinate bijections
and all three transported2-root support matches. [pcc:unverified]

## Global quotient construction over the prime field

For each i, HLP gives an F101 isogeny Pi:Ai→Ji of degree4, kernel the
Galois-equivariant graph Gammai on elliptic2-torsion and polarized norm2.
Both factor restrictions are embeddings because Gammai meets each factor
trivially. The actual Pi models/coordinate identifications are the accepted
old construction and the new C2 charts above. Use HLP math/9809210v1,
§3.2 Propositions3–4; the two elliptic j values at every stage differ,
so the graph cannot come from a geometric elliptic isomorphism.
[pcc:unverified]

Put alpha=(alpha1,alpha2), beta=(beta1,beta2). The new graph is precisely
Gamma2=beta(Gamma1): its three nonzero points are the old matched2-roots
pushed through actual alpha then beta, with Frobenius and addition checked.
Thus P2*beta kills Gamma1 as a finite etale subgroup scheme. The quotient
universal property supplies a unique regular F101 homomorphism

    Phi2:J1→J2,                 Phi2*P1=P2*beta.

This constructs a global map; it is not an inference from equality on
F101 points. Degree multiplicativity gives degPhi2=(4*49)/4=49. Since
101 is coprime to49 it is separable. [pcc:unverified]

For product principal polarizations lambdaA and Jacobian lambdaJ,
Pi^*lambdaJi=2*lambdaAi and beta^*lambdaA2=7*lambdaA1. Pulling back through
P1 the square above gives

    P1^*(Phi2^*lambdaJ2)=14*lambdaA1=P1^*(7*lambdaJ1).

Pullback through an isogeny is injective on rational polarization/Hom
classes, and Hom is torsion free (MilneAV Remark8.12). Therefore
Phi2^*lambdaJ2=7*lambdaJ1. Its positive polarized adjoint satisfies both
Phi2Hat*Phi2=[7] and Phi2*Phi2Hat=[7]. This is a (7,7)-isogeny.
The actual new kernel is P1(Ker beta),49 points with Frobenius5/6; Pi is
an isomorphism on odd-primary torsion. Native enumeration of those new
49 source-J1 classes is a remaining implementation certificate.
[pcc:unverified]

Let N1=P1Hat and N2=P2Hat. The same square yields the global formula
Phi2[2]=P2*beta*N1; the adjoint formula is Phi2Hat[2]=P1*betaHat*N2.
On rational groups the two is equivalent to known inverse2 in the existing
109/113 elliptic coordinates. No unknown DLP label is required to implement
these maps. Their degree is coprime to12317, so the induced rational-point
map is injective and, by the equal counts, bijective; both prime-order
components are preserved. Executed all-class evaluation remains a separate
native obligation. [pcc:unverified]

## Nonisomorphic Jacobian destinations

The two elliptic trace classes -7 and -11 are not F101-isogenous. Hence a
Jacobian in this product isogeny class has a unique connected F101 elliptic
subvariety of trace-7: nonzero projection to the trace-11 factor would
produce an isogeny between different trace classes. The HLP embeddings
identify that unique subvariety with the actual E109,i, not merely an
isogenous curve. An F101 Jacobian isomorphism would restrict to an elliptic
isomorphism of these subvarieties. The j values87≠47 and12≠47 rule out
J1≅J2 and J0≅J2 even as unpolarized F101 abelian varieties. No geometric
isomorphism claim over an algebraic closure is asserted. [pcc:unverified]

## Nondual composition and the exact splitting-field boundary

For an elliptic pair beta*alpha, image(alpha(E[7]))=Ker(alphaHat): inclusion
follows from alphaHat*alpha=[7], and both groups have order7. The new beta
kernel is a different line, so Ker(beta*alpha)[7]=Ker alpha has order7.
The full degree49 kernel is therefore cyclic of order49. This uses the
actual nondual check, not j alone. It is Frobenius stable because both
maps are defined overF101. [pcc:unverified]

Frobenius on each cyclic49 kernel acts by a unit modulo49, reducing to the
observed5/6 and satisfying X²+7X+101 or X²+11X+101. The characteristic
polynomial/point-count and Tate-module statements are MilneAV II Theorem1.1
and I Proposition10.20; Cayley-Hamilton applies on prime-to101 torsion.
Preflight05/PCC190ae8 and CyclicFortyNine.lean/PCC82b8fb give unique lifts
33/27, exact orders42/14 and sixth powers8/29. In each cyclic49 group,
the sixth-fixed elements are exactly the seven multiples of7.
[pcc:unverified]

The accepted Phi1 square and the new Phi2 square compose. P0 induces an
isomorphism on49-primary torsion, so

    Ker(Phi2*Phi1) ≅ Ker(beta1*alpha1) × Ker(beta2*alpha2)
                  ≅ Z/49 × Z/49.

The composite has degree2401 and polarized norm49. Its full geometric
kernel splits over F101^42; it has only49 of its2401 points over F101^6.
Composite maps retain F101 coefficients; explicitly listing every geometric
kernel point would require F101^42. This is a concrete boundary of the
current Fp6 point representation, not a barrier to evaluating the prime-
field chain. No degree42 field engine or native enumeration was introduced.
[pcc:unverified]

## Formalization and continuation

Six CyclicFortyNine finite certificates use Lean kernel decide, no
native_decide or new axioms. Existing structural isogeny/quotient helpers
are reused. Actual elliptic cyclicity, Tate-module/Cayley-Hamilton,
polarization pullback, HLP quotient and uniqueness of elliptic subvarieties
remain the written bridges above. Full geometric formalization is open.
[pcc:unverified]

Next implement Phi2/Phi2Hat on actual C1/C2 classes with the validated
coordinate scales, check both complete12317-class domains and109-source/
direct beta*alpha controls, then compute Phi2[2] and transport the actual
R1 kernel to R2. Construct the new target-chain square and both global/
rational/source certificates before accepting the next evaluated Psi.
Preserve the composite Fp42 boundary and all earlier chart/backtrack
negatives. No benchmark or computational advantage is inferred.
[pcc:unverified]
