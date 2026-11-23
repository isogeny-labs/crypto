# Actual destination-curve point and relation development fixture

Use only the explicit P-192 d7 kernel1 candidate curve descended by DESCENT_PROTOCOL. The original quotient/Jacobian twist and divisor-map requirements remain open. This fixture tests actual destination geometry and supplies a point pool for target divisor arithmetic; it does not transfer or solve the elliptic DLP.

Before data, fix the restricted factor-base condition to all affine Fp points with X/Z in {0,...,255}. Find these points by the exact linear-factor part gcd(f(a,Y),Y^p-Y), with at most64 character-split shifts per polynomial. Keep every point in the window and every split vertical line, without selecting a smaller base after seeing relations. Require at least85 points for the subsequent evaluation-space arithmetic and two fully split, simple degree4 fibers; preserve a bounded failure if these conditions do not hold.

Take the first split fiber as reference. For each other split fiber, emit the actual principal relation div((X-aZ)/(X-bZ)) on the smooth quartic, with its eight signed support entries. Check the exact product factorization of every accepted fiber polynomial. Report coefficient-matrix rank modulo the public source prime r only as coefficient rank; no r-subgroup projection, base logarithm or source-image binding follows before the twist/map checks. Changed-root and changed-row controls must reject.

This is a fixed small-coordinate factor-base fixture and vertical-line subcollector, not the full Diem relation algorithm or evidence of enough rank to solve. No paid job, timing ratio or P-192 scalar recovery is claimed.
