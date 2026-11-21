# Exact level2 input for the P-192 odd-kernel quotient

Keep the actual recorded P-192 d7 cover and kernel1. This development stage constructs input for Tian section4.1; it does not substitute another cover or claim a quotient curve.

Use the existing Fp8 coefficient field and adjoin e with e^3-3e+b=0. Verify this cubic is irreducible over Fp8 by q-Frobenius/gcd and closure. The resulting tower is Fp24. Recover all elliptic2-torsion x-coordinates e,e^p,e^(p^2), and lift the cover branch coordinates using t^2=(e+7)/(e-1). Square roots can be obtained in the Fp6 subfield using its Fp3 subfield and an explicit square root of -1 in Fp8. Check all seven finite normalized branch roots and their product against the unchanged cover polynomial.

Build a symplectic basis of source J(H)[2] by even branch-subset arithmetic and realize all64 classes in Mumford form over the declared tower. Check distinctness, order, actual basis Frobenius versus branch permutations, and the theta-characteristic shift. Reject a corrupted branch root and pairing. Fixed bounds:16 fourth-root candidates, no search over source curves or kernels. No paid job, timings or performance claim.

The working compositum is not the desired target field: the eventual quartic and quotient map must still be checked for descent/twist over Fp. Next stage computes normalized quotient level2 Weil values/theta constants, using collision-safe evaluation and regularization where necessary.
