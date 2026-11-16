# R69 complete fifth descending Frobenius step, frozen before data

The accepted and sealed R68 source supplies the rank-32 lattice `V4`
in polynomial and field coordinates. The R52 table supplies all 32
source square columns. Reconstruct the polynomial inclusion of the old
order in `V4`, verify quotient length four, and obtain the **complete**
four-class module `kbase` of `V4/O`.

For every class, square its actual field coordinate and express it in
the `V4` basis. Clear complete denominators, reduce every numerator
modulo its row denominator, flatten all remainder coefficients to a
binary matrix and reconstruct every remainder. Solve the full kernel,
verify every kernel product, and reject an altered kernel when both
kernel and obstruction are nonzero.

Build `V5` from the polynomial old-order inclusion and all survivors.
Check the kernel/lattice length identity, rank32, both module inclusions,
both field inverse products, polynomial inclusion of the old order, and
all 32 basis squares back in `V4`. Reject a changed inverse. Save the
complete quotient, binary/rational map, kernel, polynomial basis and
field basis. A zero square rank with checked quotient length zero
certifies `V5=V4`; otherwise continue. A zero survivor kernel must be
retained and its zero-column serialization repaired under a fresh
source revision, without narrowing the cohort.

Use one fresh local Singular/FLINT thread and one math job, with
1200 seconds, 4 GiB virtual and seed69. CPU affinity is not pinned.
Exit0, exact marker and no whitespace-question-mark error lines are
required. Bind copied source/protocol/driver bytes to Git and every
donor to its hash before execution. Preserve failed/partial attempts
and freeze repairs before a new attempt ID. No EKS, Diem, freezer,
Conductor claim, merge or retarget.

Claim scope is this complete affine lattice step. Proper branches,
divisors, level-77 maps and order-11 action are separate. Exact finite
algebra has no statistical baseline or A/A.
