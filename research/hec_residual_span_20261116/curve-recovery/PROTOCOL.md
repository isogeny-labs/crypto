# Quotient curve recovery from distinguished node

Prospective assertions [pcc:unverified]. Followup branch after user merged
PR4527; fixed map/imagequartic records unchanged. Primary source Bruin etal,
On(2,2)-decomposable genus4Jacobians, Proposition4.1(4),
states reconstruction via tangent conic and polar cubic branch locus.
Publisher text read at https://www.cambridge.org/core/journals/canadian-journal-of-mathematics/article/on-22decomposable-genus-4-jacobians/651E1FC3F6A593EB7B03A0642E81C7F0 .
KB query tangentconic/cubic reconstruction returned internal autolab
source1c8daaebce1785877fafc9ad73855272 (not a certificate); use primary text.

At recorded node[1,284,243,459] change variables so it becomes[0,0,0,1].
Require quartic t³/t⁴ coefficients zero; write F=A2*t²+B3*t+C4.
Require conic A2 nonsingular. Enumerate allP2(F601) points with A2=B3=0;
predict exactlyone rational branchpoint. Parameterize conic from this
point using quadratic Q(D)=A2(D)P-bilinear(P,D)D. Choose tangent direction
as parameter infinity. Require exact A2(Q)=0 and B3(Q) squarefree quintic
in affine parameter (one branch at infinity), degree5. Run same recovery
on original binomial quartic as a control.

Enumerate curve points overF601 andF601² (extension r²=7); extension
quadratic character equals base character of norm. Both twist choices
charged as algebraic qualification, not performance. Predict initial q
count523or681; q²count360423. Select scale1or7 giving qcount681, recount
both fields and reconstruct a1=79,a2=2731,Jacobian order411491. Preserve
all failures; matching counts alone is not map/lift verification or
independent proof of quotient identification. No replacement target.

Freeze source/import/native/image/control/protocol/binary hashes and push
Git before one120CPU-second invocation. No reruns/S3/cloud/speed claims.
Next exact curve/Kummer coordinate identification, originaltarget map/lift
and complete-cost qualification.
