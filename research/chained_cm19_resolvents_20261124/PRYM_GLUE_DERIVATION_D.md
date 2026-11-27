# Conditional construction/proof map for the actual genus-two target

Written derivation from qualified C and the newly read primary theorems.
No D algorithm, explicit F7 sextic or final isomorphism certificate has yet
run. Retain all distinctions between native algebra, written geometry,
full formal verification and executable divisor arithmetic. [pcc:unverified]

## Principal Prym coordinates from the actual incidence kernel

Let K=ker L~E2 have the C basis a=beta(1,-omega,0),b=beta3, with
G=[[16,-4-2omega],[-6+2omega,3]],detG=4. Actual Psi restricts to an
isogeny j:K->P=Prym(Y6/Y3), because j^dagger j=[2] on K has finite
kernel and both dimensions are2. The canonical principal Xi on P satisfies
lambda_JY6|P=2Xi. Thus j^*Xi=G: twice this equality is the already
qualified incidence Gram equality2G; cancel2 in Hom. [pcc:unverified]

The degree of the polarization G is Norm(detG)=16, hence degj=4.
kerj lies in K[2] by the Gram identity, and in kerG by pullback.
G on K[2] is diag(0,1), so kerG intersect K[2]=a(E[2]), of order4.
Consequently kerj=a(E[2]) exactly. The genuine homomorphism j*a kills
all E[2] and uniquely factors through [2], so gamma_a=(j*a)/2 exists
integrally over F7. No inverse2 modulo11 is promoted to a global map.
[pcc:unverified]

The map (gamma_a,j*b):E2->P is an isomorphism: precomposition with
diag(2,1) is j, and both j and diag(2,1) have degree4. Its principal Gram is

    Hprime=[[4,-2-omega],[-3+omega,3]], determinant1.

In basis c=gamma_a+j*b,d=j*b this becomes

    Hpp=[[2,baromega],[omega,3]], baromega=1-omega.

The two elliptic columns c and e=-baromega*c+2d have orthogonal norms2.
Their matrix T=[[1,-baromega],[0,2]] pulls Hpp back to2I and has kernel
{(baromega*Q,Q):Q in E[2]}. Therefore the actual principally polarized
Prym is the polarized quotient E2/Graph(baromega on E[2]). Check all
integral matrix/polarization identities natively before promoting this
derivation. No assumption on the hyperellipticity of Y3 is needed.
[pcc:unverified]

On E[2], omega=pi+2 acts as pi, and pi2+3pi+7=0 becomes
pi2+pi+1=0. Hence baromega=pi2. The fixed cubic x3+2x+6 has no F7
root (to be certified), so its three roots lie in F343 and pi2 cyclically
permutes them. E has j=1, distinct from0 and1728=6, so Aut over the
algebraic closure is {+1,-1}, both inducing identity on E[2]. Thus this
Galois-equivariant nonidentity graph satisfies HLP Proposition3 and
defines a smooth genus2 Jacobian over F7. [pcc:unverified]

## Explicit fixed polynomial construction without parameter search

Let alpha satisfy alpha3+2alpha+6=0, beta=alpha49. Determine the unique
Mobius transform M(x)=(A*x+B)/(C*x+D) sending each actual cubic root
to its Frobenius49 image. Solve the3-by4 homogeneous F7 coefficient
system C*alpha*beta+D*beta-A*alpha-B=0, first-nonzero normalization.
Require rank3/kernel1, nonzero determinant and C!=0; retain failures.
Check all three conjugate roots/denominators and the whole polynomial

 (A*x+B)^3+2(A*x+B)(C*x+D)^2+6(C*x+D)^3
     =lambda*(x3+2x+6), lambda!=0.

The fibre product of the two actual elliptic double covers has rational
third quotient W2=lambda*(C*x+D). Put x=(W2-lambda*D)/(lambda*C).
This predicts the explicit genus2 sextic and two degree2 maps

 H(W)=x(W)^3+2x(W)+6,     CurveD:Y2=H(W),
 p1=(x(W),Y),
 p2=(A/C-lambda*(A*D-B*C)/(C*W2),lambda2*Y/W3).

Check full degree6 identities, squarefreeness by a complete Bezout gcd,
and the W=0/infinity extension before claiming a completed morphism.
Identify the actual2-torsion gluing graph from the two pullbacks; the
polarized quotient universal property then identifies Jac(CurveD) with
the actual P, not merely an isogenous surface with the same Weil polynomial.
[pcc:unverified]

## Next exact implementation and remaining obligations

Native D should first qualify integral Hprime/Hpp/T identities and the
actual Frobenius2-torsion Mobius/sextic/two-cover equations. Keep its
original sextic even if another chart is used for arithmetic. Native
Jacobian arithmetic follows in a separate frozen unit. If a nonsquare
infinity model is needed, classify all eight actual P1(F7) fibres and use
the first nonsquare fibre with a full invertible coordinate certificate;
do not discard alternatives or reinterpret a chart change as a new curve.
[pcc:unverified]

For a finite E point (x0,y0), the p1 pullback has exact degree2 ideal
u(W)=W2-lambda*(C*x0+D),v(W)=y0, relative to p1^*(O). This can give
a concrete Mumford packet and a p1_*p1^*=[2] retraction without solving
for an unknown scalar. Generic even-degree6 group reduction, exceptional
classes and complete independent replay still require implementation.
This uses established2-gluing, with no novelty or measured solver advantage
assertion; any performance/search experiment retains original S3/EKS gates.
[pcc:unverified]
