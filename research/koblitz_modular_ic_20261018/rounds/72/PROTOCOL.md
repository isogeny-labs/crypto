# R72 primitive local lengths, frozen before data

Reconstruct the complete 32-dimensional fiber algebra of the R70-normal
affine order at both `z=0` and `z=1` from the accepted R52 528-product
table. Reproduce R71 stable Frobenius image dimensions 4 and 8 and
branch counts 2 and 4. Verify Frobenius squared is the identity on each
stable image, as R71 found only degree-two residue fields.

Compute the entire Frobenius-fixed subspace as the image of `F+I` on
the stable image. Enumerate all `2^r-1` nonzero fixed vectors, `r=2`
and `r=4`, and verify each is an idempotent in the full specialized
algebra. For every idempotent, build its full 32-by-32 multiplication
operator using all 528 products. Define primitive by the **complete**
Boolean order test: no distinct nonzero idempotent `w` satisfies
`e*w=w`. Verify the primitive idempotents are pairwise orthogonal and
sum to the source unit. Record each multiplication-operator rank.

Because the affine order is normal at both finite primes, the local
factor for a primitive idempotent is a finite free local algebra over
the base DVR, and its fiber dimension equals `e*f`. R71 gives each
residue degree `f=2`; require every local rank is even, divide by two
to obtain the ramification index, and require the local ranks sum to
32. Keep valuations, uniformizers, and pole divisors as later tasks.

Exact deterministic computation. One local Singular/FLINT thread, one
math job, seed 72, 1200 seconds, 4 GiB virtual, CPU affinity not
pinned. Copy and bind source/protocol/driver bytes to the frozen Git
revision and donor hashes before execution. Accept only exit 0, exact
marker and zero whitespace-question-mark error lines. Preserve any
failed raw attempts; repairs require a fresh committed revision and
attempt ID. No EKS, Diem, freezer, Conductor claim, merge, or retarget.

Controls are both complete fibers, all square slots and product slots,
the source unit, Frobenius-stable images, complete idempotent enumeration,
all primitive orthogonality products, rank/degree sums, and the R71
residue profiles. There is no statistical baseline, A/A or power claim.
