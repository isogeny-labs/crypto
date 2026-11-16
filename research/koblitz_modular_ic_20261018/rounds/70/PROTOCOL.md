# R70 complete sixth descending Frobenius step, frozen before data

The accepted and sealed R69 source supplies the rank-32 lattice `V5`
in polynomial and field coordinates. The R52 table supplies all 32
source square columns. Reconstruct the polynomial inclusion of the old
order in `V5`, verify quotient length two, and obtain the **complete**
two-class module `kbase` of `V5/O`.

For both classes, square the actual field coordinate and express it in
the `V5` basis. Clear complete denominators, reduce every numerator
modulo its row denominator, flatten all remainder coefficients to a
binary matrix and reconstruct every remainder. Solve its full kernel,
verify every kernel product, and reject an altered kernel when both
kernel and obstruction are nonzero.

Build `V6` from the polynomial old-order inclusion and all survivors.
Check the kernel/lattice length identity, rank32, both module inclusions,
both field inverse products, polynomial inclusion of the old order, and
all 32 basis squares back in `V5`. Reject a changed inverse. Save the
complete quotient, binary/rational map, kernel, polynomial basis and
field basis. A zero square rank with checked quotient length zero
certifies `V6=V5`; otherwise continue. A zero survivor kernel must be
retained and its zero-column serialization repaired under a fresh
source revision, without narrowing the cohort.

Use one fresh local Singular/FLINT thread and one math job, with
1200 seconds, 4 GiB virtual and seed70. CPU affinity is not pinned.
Exit0, exact marker and no whitespace-question-mark error lines are
required. Bind copied source/protocol/driver bytes to Git and every
donor to its hash before execution. Preserve failed/partial attempts
and freeze repairs before a new attempt ID. No EKS, Diem, freezer,
Conductor claim, merge or retarget.

Claim scope is this complete affine lattice step. If equality holds,
apply the Singh--Swanson fixed-point theorem only after source field,
trace-dual and conductor hypotheses are audited. Proper branches,
divisors, level-77 maps and order-11 action are separate. Exact finite
algebra has no statistical baseline or A/A.

## Version 2 zero-kernel repair, frozen before the second attempt

`sixth-frobenius70-01` is rejected despite a late marker: Singular
reported a zero-column matrix conversion error after the full two-class
binary map had rank two. Preserve that raw attempt. The amended source
branches on kernel dimension zero before constructing an empty matrix.
It requires an explicit nonzero 2-by-2 binary minor, checks that the old
order basis in `V5` has quotient length two, verifies its field-coordinate
matrix is the identity, checks all 32 source basis squares in the old
order and `V5`, and rejects a changed inverse. It saves the full binary
map and explicit zero-kernel/old-order basis bundles. No previous
accepted producer is rerun; this is a fresh full two-class attempt with
the same resource caps and strict error gate.
