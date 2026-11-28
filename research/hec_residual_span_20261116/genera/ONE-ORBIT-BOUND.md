# A full one-orbit rational catalogue is a bounded-field shortcut

This is an elementary application of the classical Hasse–Weil bound, not a new lower bound on index calculus or on arbitrary endomorphism methods.

In the current domain, n=2g+1, the canonical catalogue contains exactlynnonbranch points, no zero-abscissa extraorbit, and both ordinate signs are rational. Infinity is the only remaining rational point. Hence

 #C(Fp)=2n+1=4g+3.

Hasse–Weil gives p+1−2g*sqrt(p)≤#C(Fp), so

 p−2g*sqrt(p)−4g−2≤0,
 sqrt(p)≤g+sqrt(g²+4g+2),
 p≤(g+sqrt(g²+4g+2))².

Necessary upperbounds are approximately32.967forg2,60.775forg3and96.648forg4. Under the further currentrationalautomorphismcondition p≡1mod(2g+1), the complete possible oddprime lists below thosebounds are:

- g2:11,31.
- g3:29,43.
- g4:19,37,73.

The currentfrozen164constantpanel covers allthoseg2/g3primes and the firsttwog4primes. It has not screenedp73, so no exhaustiveg4census is claimed. Eligibility also requiresirreducibility, actualoneorbit/subgroup/certificatecontracts; the bound alone does not certify anymodel. The fieldlists follow directly by checking integers1modnunder thebound, not a search forspeed. Genus2screen admitsc6/c8overF11andnoneoverF31underitsfrozencontracts; genus3screen has7eligibleconstants. This does not imply every selectedtarget gives a useful smoothdecomposition.

## Consequence for the next algorithm

For growingp, a fullrationalcatalogue necessarily has multiple signedautomorphismorbits. If phiacts aslambda on the verifiedscalarquotient and orbitjhas representativePj, then the unknown cataloguelogs have the form

 log(h*phi^a(Pj))=lambda^a*Lj.

There is one unknownLjper orbit, not one globally known source orbit. Given a rawprincipal row with signedpointcoefficientsc_{j,a}, its projected coefficient forLj is sum_a c_{j,a}lambda^a. All independent orbitrepresentatives and off-basecoordinates must be retained. Collapsing them to one normalized log would invent constraints.

For a walk relation sum_i c_i Pi − aD − bT=0 with sourceD=hP0, targetT=kD, and projectedorbitlogsLj=log_D(hPj), the exactscalar row is

 sum_{j,a}c_{j,a}sign_{j,a}lambda^a Lj − b*h*k = a*h (modell).

Equivalently useYj=h^{-1}Ljfor invertibleh: sumweighted*c*Yj−b*k=a, and Y_sourceorbit=h^{-1}. This projectionconvention must be checked against the fullunfoldedrow/operator and originalgroup identities. It is classical endomorphismfactor-base contraction, not a novel exponent, freeorbitlogoracle or proofthat a fixednfold variable reduction gives tenfoldtotalcost.

The one-orbit gate therefore supplies a correctness and small-instance preparation control. The scalable next question is useful multi-orbit relation generation/linearalgebra/target integration with complete matchedcosts. Using only one orbit as a restricted factorbase in a largerfield changes descentyield and must be charged; it is not the fullcatalogue algorithm tested here.

Primarycontext: Blache–Cherdieu–EstradaSarlabous2004Theorem1.1states Frobeniuseigenvalues have absolutevaluesqrt(p), yielding Hasse–Weil usedabove; primary link in ../frobenius/THEORY.md. Character/eigenvalue weights are established Tsakou–Ionicafactorbasequotient arithmetic, linked in ../orbit/THEORY.md. The exactbound and projectedwalkrow here are deductions with statedhypotheses, not newpriorityclaims or an unconditionalbarrier for othermethods.
