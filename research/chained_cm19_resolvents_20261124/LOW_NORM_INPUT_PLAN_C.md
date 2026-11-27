# Lower-norm actual Prym input, before first C call

Native B7dafac and written actual-object bridge d76527 give L=ell*alpha,
ell=(omega,1,0), beta:E3~J(C), and the real Hermitian H. These fixed
actual sources were read. The exact kernel has coordinate basis
beta(1,-omega,0),beta(0,0,1), since its defining equation is
omega*x0+x1=0. No lattice/parameter/point search is needed.
[pcc:unverified]

Retain the already qualified tau section L*tau=I. Build F with columns
tau,(1,-omega,0),(0,0,1). Its inverse has rows ell,
e0^t-tau0*ell,e2^t-tau2*ell. Check both full integral matrix products,
then compute F^dagger H F and the restricted2-by2 Hermitian matrix G.
This proves a global actual kernel decomposition, rather than a scalar
inverse valid only modulo11. [pcc:unverified]

For input kappa3=beta3 and rho3=e2^t*alpha, require L*kappa3=0 and
rho3*kappa3=I. From the actual B incidence identity, the pullback Gram
matrix of Psi is 2H+ell^dagger*ell. Thus kappa3 norm3 predicts the
new chain Gamma3=Psi*kappa3 ambient norm6 and
(rho3*Psi^dagger)*Gamma3=[2], with image in the same two-dimensional
Prym. The written actual geometry remains d76527, generic law upstream
crypto-lean38. Certify the complete integral matrices first; no global
minimum, measured runtime advantage or complete divisor evaluator is
asserted. Preserve the original norm124/248 kappa chain unaltered.
[pcc:unverified]
