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
