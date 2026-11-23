# R66 complete second descending Frobenius step, frozen before data

The accepted R65 source seal supplies the rank-32 polynomial basis of
`V1` in trace-dual coordinates and its two-sided rational field inverse.
The accepted R52 source supplies all 32 square columns of the full
degree-32 multiplication table. Reconstruct the polynomial inclusion
`J=V1^-1 O` and check its 80-dimensional quotient. Compute its complete
module `kbase`: all 80 classes of `V1/O`, with no coefficient box or
sampled subset.

For each quotient class `q`, form the actual field element `V1 q`, square
with the full source multiplication table, and express the result as
`V1^-1 S (V1 q)^[2]`. Require every scalar coordinate to be rational in
`u`. Clear the full denominators row by row; reduce every numerator
modulo its row denominator, flatten every remainder coefficient to a
binary matrix, and reconstruct all remainders from those coefficients.
Compute the full binary rank and kernel, verify all kernel products and
reject an altered nonzero kernel when applicable.

Build `V2` from `J` and all kernel representatives. Require the exact
kernel/lattice quotient identity, rank 32, both module inclusions, both
basis inverse products, polynomial inclusion of the old order, and that
the square of each of the 32 new basis columns lies in `V1`. Reject a
changed inverse. Save complete quotient, rational/binary map, kernel,
and both polynomial and field lattice bundles. If the survivor kernel is
zero, retain the full-rank terminal evidence and amend the zero-column
serialization under a fresh source revision; do not turn it into a
coefficient-box search or discard the null result.

The old R65 native run is never repeated for this calculation. One fresh
local Singular/FLINT thread, one math job, 1200 seconds, 4 GiB virtual,
seed66; no CPU affinity claim. Exit0, exact terminal marker, and zero
whitespace-question-mark error lines are jointly required. Copy frozen
source/protocol/driver into the fresh raw directory, bind each to Git,
and check every donor hash before execution. Retain failed/partial logs;
repair only under a fresh source revision and fresh attempt ID. No EKS,
Diem, freezer, Conductor claim, merge or retarget.

The mathematical claim is one complete affine descending Frobenius step,
not normalization unless the new lattice equals `V1`. Actual proper
branch divisors, level-77 maps and order-11 subgroup action are separate.
The method follows Singh--Swanson, Theorem 1.1. No statistical comparison
is made; no A/A or baseline is applicable to exact finite algebra.
