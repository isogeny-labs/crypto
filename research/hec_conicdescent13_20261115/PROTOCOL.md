# Conic residual descent correctness gate

Scope: remove translation-based smoothness search from the registered F127
quartic descent. This is an engineering implementation of classical plane-curve
divisor arithmetic, not a new complexity claim. Literature: Diem, On the
discrete logarithm problem for plane curves, JTNB 24 (2012),
https://jtnb.centre-mersenne.org/articles/10.5802/jtnb.815/ ; Tian section4.2.3,
https://arxiv.org/html/2007.03172 . KB search for symmetric Kummer inverse
returned adjacent proposals, not an implementation or novelty certificate.

Use the SAME quartic and the two closed divisors recovered in
../hec_inverse13_20261115/RECOVER.txt. Each affine graph E=(u,y-v) has degree3,
squarefree u and nonzero cubic section chart. Use the five previously public
setup points as anchors, all ten pairs in fixed lexicographic index order.
No new target, tuned pair, target scalar or group enumeration in candidate.

Solve the five homogeneous linear constraints on six conic coefficients:
C(x,v(x))=0 mod u, C(A)=C(B)=0. Require rank5/unique conic, nonzero y²
coefficient, resultant degree8, exact divisibility by u*(x-Ax)*(x-Bx),
squarefree degree3 residual, three distinct rational x-roots with exactly one
common y-root for each, disjoint support and no infinity intersection.
Record every excluded pair with its reason; do not retry or widen anchors.
Factor via Frobenius gcd and bounded deterministic equal-degree splitting.
Candidate conic/resultant/descent uses polynomial and matrix operations only.

For each accepted split residual R, verify the exact plane divisor identity
E+A+B+R~2K using the existing independent QuarticJacobian group law. K is the
known tangent section 2O+(101,43)+(34,113). Verify the resultant at ALL127x
against direct Sylvester determinants; this exhaustive check is verification,
not candidate root finding. Check corrupted v is rejected by F(x,v) mod u.
No requirement that every pair split; if no supported pair occurs, retain
that negative outcome. Acceptance requires at least one verified residual for
each registered E. No wall-time, memory or 10x claim from this gate.

Freeze source/protocol/binary hash and checkpoint/gate before one execution.
Retain any failure; no scientific rerun without separate amendment. Next step
if accepted: connect point-log kernel to the divisor identity, including
centering/orientation constants, then a separately frozen complete cost case.
The previous cold-cost bound says map-only changes cannot close the gap.
