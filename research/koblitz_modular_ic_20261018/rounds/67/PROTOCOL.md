# R67 complete third descending Frobenius step, frozen before data

The accepted and sealed R66 source supplies the second rank-32 lattice
`V2` in polynomial and field coordinates. The R52 table supplies all 32
source square columns. Reconstruct the polynomial inclusion of the old
order in `V2`, verify quotient length 32, and obtain the **complete**
32-class module `kbase` of `V2/O`.

For each class, square its actual field coordinate and express the result
in the `V2` basis. Clear complete denominators, reduce every numerator
modulo its row denominator, flatten every remainder coefficient to a
binary matrix, and reconstruct each remainder from that matrix. Solve
the full kernel, verify every kernel product, and reject an altered
kernel when both kernel and obstruction are nonzero.

Build `V3` from the polynomial order inclusion and all survivors. Check
the kernel/lattice length identity, rank32, both module inclusions, both
field inverse products, polynomial inclusion of the old order, and all
32 basis squares back in `V2`. Reject a changed inverse. Save complete
quotient, binary/rational map, kernel, polynomial basis and field basis.
If the square rank is zero, the full kernel proves `V3=V2`; verify that
equality before applying the descending Frobenius normalization theorem.
If the kernel is zero, retain the null evidence and amend any zero-column
serialization under a fresh source revision. Do not narrow the cohort.

Use one fresh local Singular/FLINT thread and one math job, with
1200 seconds, 4 GiB virtual and seed67. CPU affinity is not pinned.
Exit0, exact terminal marker, and no whitespace-question-mark error
lines are required. Bind copied source/protocol/driver bytes to Git and
all donors to hashes before execution. Preserve every terminal/partial
attempt and freeze repairs before a new attempt ID. No EKS, Diem,
freezer, Conductor claim, merge or retarget.

Claim scope is this complete affine lattice step. Proper branches,
divisors, level-77 maps and the order-11 action are separate. This is
exact deterministic algebra, with no statistical baseline or A/A.
