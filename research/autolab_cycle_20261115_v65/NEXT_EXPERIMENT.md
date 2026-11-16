# Next hypothesis: filter by Vieta coefficients after a subspace map

The instrumented packed scan spends about54.7%in fiber elimination and40.6%in support. Under unchanged other-stage costs,elimination-only acceleration has a2.21xmodeled ceiling. The50xtarget therefore motivates exact candidate/whole-cell pruning and lower complete-pipeline overhead,not an unsupported extrapolation from packing.

Let V be the binary source space and U a proper subspace,initially its low6,7or8basis directions. Let L_U be its monic linearized subspace polynomial,W=L_U(V),and C=X^3+e1*X^2+e2*X+e3 a candidate cubic. Reduce L_U(X)modC to q0+q1*X+q2*X^2. If all roots ofC,with repetitions,are inV,all their images are inW. Hence the second Vieta coefficient of the three images must belong to S2(W)=span_F2{w_i*w_j:i<=j}. It is exactly

T2=q0^2+q1^2*e2+q1*q2*(e1*e2+e3)+q2^2*e2^2.

Proof: expand sum_(i<j)(q0+q1*x_i+q2*x_i^2)(q0+q1*x_j+q2*x_j^2). The q0cross terms cancel because each root occurs twice;3q0^2=q0^2. The mixed cubic sum equals e1*e2+e3. No inverses,squarefree assumption or rational-lift filter is used. Repeated roots obey the same identity. This is a necessary rejection test;the final exact nilpotence support criterion is retained. Full characteristic-polynomial image support would be sufficient too: ker(L_U)=U andL_U^{-1}(L_U(V))=V even over the algebraic closure.

At d18,dimW=18-dimU anddimS2(W)<=s(s+1)/2. For dimU6,7,8 the upper bounds are78,66,55respectively in83field coordinates. These are rank upper bounds,not measured surviving constraints or independent rejection probabilities. Build the actual basis and exact quotient once,then compare the extra4field-multiplication Vieta test plus membership against saved remaining support iterations. q0,q1,q2 are already the residues after the chosen prefix of the existing linearized tower,so no factorization is needed for the guard.

Concrete counterexample to treating this guard as sufficient: inF8=F2[T]/(T^3+T+1),V=span(1,T),U=span(1),L_U=X^2+X,W=span(T^2+T),S2(W)=span(T). C=X^3+T has e1=e2=0,e3=T andq0=0,q1=q2=1,soT2=T passes. But T^5=T^2+T+1 is a root ofC outsideV,so support fails. Preserve the final oracle and every exceptional branch.

Next90s/4GiB/core0experiment: implement this guard in a fresh variant;prove subset-preserving rejection;independently exhaust small fields including even degrees,repeated roots,zero coefficients and the counterexample. Freeze actualS2ranks and seeds before measuring rejection on all ordinaryGF83candidates. Compare every accepted root/candidate with the unguarded complete scan. Charge quotient precompute,Vieta tests,remaining support,factor/lift and serialization;run4cold complete repetitions only after oracle equality. Refute if total runtime or memory worsens. No extra experiment is executed in this already-three-experiment cycle.

Further route toward50x: express this projected support guard over affine outer-parameter cells and bound/certify whole-cell rejections,retaining rank-minor-zero fibers. This is a research hypothesis,not a proven tractable solution to the main2^73quintic relaxation. Primary novelty review against subspace-polynomial composition and finite-algebra characteristic-polynomial methods is required before any novelty claim. Odd-prime interval/domain algorithms and full genuineF6matching remain active separate work.
