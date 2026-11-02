# R65 complete first trace-dual Frobenius step, freeze before data

Inputs are the accepted R61 polynomial trace Gram, its full rational
inverse, the full R52 multiplication table/parent list, the R53 exact
discriminant factorization, and the R64 source-field bridge. Recheck
the 32 by 32 Gram determinant as
`z^76(z+1)^24(z^5+z^3+z^2+z+1)^12` and both inverse products.
The trace dual is `V0=G^-1 A^32` for `A=F2[z]`. The quotient `V0/O`
has presentation `A^32/G A^32`; obtain its **complete** finite F2
standard-monomial basis with `std` and module `kbase`, requiring
dimension 160 and all 160 columns. Read all 32 original square columns
from the 528-product table, checking the source unit square.

For every quotient basis vector `q`, compute its trace-dual element
`G^-1 q`, square with the actual order multiplication, and express the
result back in trace-dual coordinates as `G S (G^-1 q)^[2]`. An F2
combination survives precisely when all 32 rational coordinates are
polynomials. Clear the complete row denominators, reduce modulo each
denominator, flatten **every** remainder coefficient to a binary matrix,
and reconstruct each remainder from its binary entries. Compute its
complete F2 rank/nullspace; check all kernel products and each surviving
combination again as a polynomial-valued whole-field coordinate. Reject
an altered kernel when the map and kernel are both nonzero. Save the
full quotient basis, rational numerator/denominator rows, binary matrix,
kernel, and actual survivor coordinates. Do not pick a coefficient box,
local prime subset, or sampled quotient columns.

This first step determines `V1/O`, not the final normalization. A
subsequent experiment must reconstruct the full PID lattice `V1` and
iterate until equality. The R64 generic source bridge is a pinned
external accepted certificate; actual proper poles, branch valuations
and level-77 map remain separate.

Native scope is one local Singular/FLINT thread and one math job,
1200 seconds, 4 GiB virtual, seed65. Exit0, exact terminal marker and
no whitespace-question-mark error lines are jointly required. All
source/protocol/driver copies must bind to historical Git and all donors
to their hashes before data. Preserve full logs, partial output and
rejections. Use the existing checkpoint gate. No EKS, Diem, freezer,
new Conductor claim, delegation, merge or retarget. Manual contract
checks are not canonical schema validation.

## Version 2: reconstruct the first PID lattice from accepted kernel
`first-frobenius65-01` passes the complete160-class square map:
241 binary remainder rows, rank80, kernel dimension80 and whole-field
polynomial checks for every survivor. Its three saved bundles are
content-hashed donors. A new `LATTICE65.sing` reads the saved quotient
basis and full kernel without rerunning the accepted producer. Form
`J=[G,QK]` in the trace-dual polynomial coordinates, compute its complete
univariate module standard basis `N`, and require quotient length80,
exactly32 basis columns and both module inclusions. Transport
`V1=G^-1 N` to rational O coordinates, check both inverse products and
that the inverse inclusion `O -> V1` has polynomial coefficients.
Check the square of every one of the32 V1 basis columns lies in the
trace dual using the original32 order-square columns. Reject an altered
inverse. Save both polynomial and rational basis/inclusion/square bundles.
This proves the first lattice step; it does not assert stabilization
or the final integral closure. The original local caps and strict error
gate remain unchanged. No accepted producer is rerun for this step.
