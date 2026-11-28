# Odd-characteristic follow-up

For f(X)=X^3+AX+B, view S3(x,y,z) as a quadratic in z. Its discriminant is 16 f(x)f(y). This follows by expanding the symmetric S3 formula; it remains valid in characteristic three. For x≠y, ordinary quadratic recovery applies. For x=y, solve the resulting linear equation; if f(x)=0 on a smooth curve, the remaining constant is (3x²+A)²≠0, so no finite branch is introduced.

A proposed backend caches square classes and normalized square roots of f(x) over a finite source domain, then batches nonzero quadratic denominators. For equal nonsquare classes, normalize by a fixed nonsquare δ: if fx=δsx² and fy=δsy², the discriminant root is 4δsxsy. Mixed nonzero classes yield no roots; zero fibers and repeated sources need explicit handling. This could reduce repeated square-root work on odd prime and extension fields. It is an established algebraic identity, not a novelty claim or a measured speedup.

Next bounded experiment: independent exhaustive all-triple roots over small prime fields and GF(9)/GF(27), including two-torsion and characteristic three; then freeze complete output/timing inputs for all five NIST fields. Charge classification, square-root preparation, batching, original verification and full output. Prime interval domains need separate support logic; this is not a transfer of binary linearization.
