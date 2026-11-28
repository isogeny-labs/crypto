# R74 complete degree-five primitive factors, frozen before data

Use the accepted R73 full `F32` square matrix `S`, relative Frobenius
`M=F^5`, stable image and degree profile, plus the complete R52
528-product table specialized at `f_C`. Solve `ker(M-I)` over `F32`
and verify its dimension six and all kernel products. Apply absolute
field trace `T(v)=v+F(v)+F²(v)+F³(v)+F⁴(v)` to each of six fixed
columns and its five scalar-basis multiples. Check that the 30 trace
candidates are absolute Frobenius fixed, extract six independent
columns, and enumerate **all 63 nonzero binary combinations**.

For each combination, verify `x²=x`, build the full 32-by-32
multiplication operator from all 528 products, and check operator
idempotency and action on the source unit. A candidate is primitive
exactly when no other nonzero enumerated idempotent `w` satisfies
`x*w=w`; apply this complete Boolean order test. Verify exactly six
primitive idempotents, their pairwise orthogonality and sum to one.
For each primitive, the operator rank is its local fiber length and
the rank of `M` projected onto its factor is its relative residue
degree. Require degree multiset `1,1,1,1,2,14`, integral
`e=length/f`, total length 32, all `e` odd, weighted different
budget 12 and `e=1` for the degree-14 branch. Save all matrices and
idempotents. Any failed check rejects the candidate result; do not
coerce the profile to the R73 prediction.

Exact deterministic computation, one local Singular/FLINT thread and
one math job, seed 74, 1200 seconds, 4 GiB virtual, CPU affinity not
pinned. Copy source/protocol/driver bytes from committed Git, verify
all donor hashes, and retain raw output. Acceptance requires exit 0,
the exact marker and zero whitespace-question-mark error lines.
Preserve failed attempts and freeze repairs under a new source revision
and attempt ID. No EKS, Diem, freezer, Conductor claim, merge or PR
retarget. No statistical baseline, A/A or power claim applies.

## Version 2 integer-entry repair, frozen before attempt 02

`degree-five-local74-01` is rejected despite exit 0 and a late marker.
The complete six-primitive route reached `degreeCounts74[residue,1]++`,
which Singular rejects for an `intmat` entry; subsequent checks and
the marker are invalid. Preserve the raw attempt. Replace that increment
with explicit assignment and use Singular's tested integer `div`
operator to remove benign division warnings. The full cohort, source
data, mathematical checks and resource caps are unchanged. Run a fresh
attempt from this committed source revision.
