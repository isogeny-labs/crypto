# Generic line norm through Velu3, before data
A new constructive route to the retained actual QR7 packet transport:
Norm_K(y+lambda*x+nu) under K={0,Q,-Q} should be a linear function on the
quotient curve,since the pushforward divisor is three image points minus3O.
This is a proposed derivation,not an accepted result. Derive and check it
using the exact point-addition/Velu formulas retained in R51/HECKE3_51.sing
and the actual packet line interpretation in R51/LINE51.sing. Reviewed
https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_curve_isogeny.html
on2026-10-31: separable isogenies,kernel points and rational maps; no Sage
execution or timing. No novelty claim for divisor norms or Velu formulas.

Generic coefficient field F2(b,c,lambda,nu),kernel coordinate t satisfies
psi3 and signed coordinate w satisfies the curve. Form the full three
translated line numerators. Predict alpha=lineCubic(t); solve beta,gamma
by coefficient extraction in x after source reduction. Require the full
source product NormNumerator=alpha*Ynum*H^3+beta*Xnum*H^4+gamma*H^6,
H=x+t. Reject changed gamma. Export all coefficients and full lifts.
Require full products for alpha unchanged,beta changing byA*alpha,gamma by
b*alpha under the packet sign(lambda,nu)->(lambda+A,nu+b). This supplies
an explicit generic image line on alpha!=0,not yet a normalized Tate target
or the actual QR-unit certificate. Those are subsequent frozen stages.

Exact computation,one fixed symbolic generic case; no sampling,power,A/A,
randomization,blinding or performance inference. Same local authorized lane:
seed52,one Singular/FLINT thread,4GiBvirtual,1200s,serialize after earlier
math terminates. Freeze input and protocol before data. Strict exit0,no?
errorlines,NORMLINE52_ALL_CHECKS_PASS;retain all attempts/partials/caps.
