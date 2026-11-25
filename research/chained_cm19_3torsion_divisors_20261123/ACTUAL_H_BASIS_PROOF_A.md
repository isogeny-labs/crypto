# Actual integral H basis from genuine C7 3-torsion divisors

This written proof applies the prospective DERIVATION_A to the complete
first-call native data `PCC-20261123-d9f917`. It is unsigned and depends on
standard algebraic-geometry results stated below; it is not a generic
Lean theorem or an independent signed verdict. [pcc:unverified]

Let E/F7 be y2=x3+2x+6 and J=Jac(C7), with actual degree2 covers f0,
f1=f0 composed with R and degree3 cover f3 fixed in the inherited source.
Write phi_i=f_i* on Pic0 and dagger for the canonical Rosati adjoint.
The actual S3 action gives
G=(phi_i dagger phi_j)=[[2,−1,0],[−1,2,0],[0,0,3]]. In particular the
deck T fixes phi0, reverses f3 on Pic0, and TRT=R inverse. The degree3
quotient trace kills phi0; sandwiching the trace gives h+h dagger=−2
for h=phi0 dagger R*phi0. The T relation makes h self-adjoint, so h=−1
in the torsion-free Hom group. This establishes G from actual maps rather
than choosing an abstract module classification. [pcc:unverified]

For every8 nonzero geometric E[3] points the native source constructs
conics with div_C(qplus/qminus)=

    f0 inverse(P) + f3 inverse(−pi P) − f1 inverse(P) − 3Pminus.

The positive and negative conditions are imposed on the entire degree2/3
fibre algebras and the actual third-order curve jet. Coprime conics meet
in a length4 complete intersection. Their degree2 and degree6 coordinate
spaces both have dimension4 by the (2,2) Koszul resolution. C is smooth
and contains no conic component. The quartic multiplication cokernel has
length equal to the conics' common divisor on C, hence degree28−rank25=3.
Opposite fibre units and qplus(Pminus) nonzero keep that common divisor
away from both prescribed degree5 divisors. Each conic has intersection
degree8 with C; both degree3 residuals must therefore equal the complete
common divisor. This proves the displayed rational-function divisor,
including multiplicities. [pcc:unverified]

The f0/f1 fibres above O coincide, while f3 inverse(O)=3Pminus. Thus the
displayed divisor represents h(P), for the actual F7 homomorphism

    h=phi0−phi1−phi3*pi : E -> J.

The native complete division-polynomial scan supplies9 distinct actual
3-torsion points, with basis P=(F(54),F(7)),Q=(F(104),F(343)). All8
nonzero points have certificates. Since char7 is prime to3, E[3] is
reduced/etale of degree9; these points exhaust its geometric points.
Consequently h kills the finite group scheme E[3]. The quotient by this
kernel is [3]:E->E, so faithfully flat quotient descent supplies a unique
homomorphism g over F7 with g composed with [3]=h. Uniqueness makes the
descent Frobenius invariant. This is division of a genuine homomorphism,
not choosing a division point on J. [pcc:unverified]

Actual Frobenius satisfies pi dagger pi=7 and pi+pi dagger=−3. Using G,
h dagger h=27, phi1 dagger h=−3, h dagger phi3=−3*pi dagger.
Therefore g dagger g=3, phi1 dagger g=−1 and g dagger phi3=−pi dagger;
these follow by torsion-freeness of the Hom groups. The actual morphism

    beta=(phi1,−g,g−phi3): E^3 -> J

has beta dagger beta = H = [[2,1,−1],[1,3,pi],[−1,pi dagger,3]].
The (2,3) entry is (−g) dagger(g−phi3)=−3−pi dagger=pi.
Thus the full displayed H has det H=−2−(pi+pi dagger)=1. Since H inverse is integral over End(E),
alpha=H inverse beta dagger is a genuine homomorphism with alpha beta=I.
The left inverse makes the scheme kernel of beta trivial. Dimensions
agree, so beta is a degree1 isogeny, hence an isomorphism, with inverse
alpha.
[pcc:unverified]

Fix omega=pi+2, omega2−omega+5=0. With B=(phi0,phi1,phi3),

    B=beta*Tminus,
    Tminus=[[1,1,0],[−(1+omega),0,−1],[2−omega,0,−1]].

Indeed phi1=beta1, g=−beta2, phi3=−beta2−beta3 and
phi0=phi1+3g+phi3*pi. This identifies the actual orientation with the
previous frozen Tminus, rather than inferring the sign from a preferred
matrix catalogue. The prior0-of144 both-sign-compatible mixed-map result
remains valid at its original bounded scope. [pcc:unverified]

For evaluation on rational points or on n-torsion with gcd(n,3)=1,
g(P)=h([3 inverse mod n]P). In particular use4 inverse3 and11 inverse4.
For general points one may choose Q with [3]Q=P and evaluate h(Q);
independence of the choice follows from the certified kernel. This gives
an evaluation prescription, not yet a complete genus3 divisor engine.
Independent certificate verification, actual transported S/kernel/J4
arithmetic, full target models, generic Lean and durable original custody
remain separate obligations. [pcc:unverified]
