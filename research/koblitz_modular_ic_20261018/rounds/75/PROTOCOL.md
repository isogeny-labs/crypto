# R75 all finite-fiber uniformizer classes, frozen before data

Use the R72 primitive idempotents and full R52 table at both rational
omitted fibers, and the R74 primitive idempotents and full R52 table at
the degree-five fiber. Rebind each saved primitive idempotent to the
actual specialized full multiplication table, verify idempotency,
operator rank and the R72/R74 local-length/residue-degree profile.

For each **ramified** local factor, project every one of the 32 source
order basis elements `e_j` using its primitive idempotent. Let `q` be
the residue-field size, `4` for rational-fiber branches and `32` for
the ramified degree-five branches. Form `t_j=y_j^q-y_j` using the
actual absolute square map (two or five iterations). Build the full
32-by-32 multiplication operator of each `t_j` from the specialized
528-product table. Check all 32 candidates lie in the projected local
factor and satisfy `t_j^e=0` in the full fiber. Count **all** candidates
with `t_j^(e-1)!=0`, choose the first and save its complete coordinate
column. That exact nilpotency index certifies valuation one in the
normal local DVR modulo the base prime. Require a success for every
ramified branch; no coefficient-box or basis subset is allowed.

For every unramified branch, verify its saved operator rank equals its
residue degree and record the base prime as uniformizer. The two
rational fibers and the degree-five fiber are separate full native
programs and both must pass before claiming all twelve branches.
These are **uniformizer classes in the finite fibers**. Global lifts,
higher prime-power expansions, coordinate/denominator valuations,
leading coefficients, infinity and pole divisors are later tasks.

Exact deterministic computation, one local Singular/FLINT thread and
one math job at a time, seed 75, 1200 seconds per native program,
4 GiB virtual, CPU affinity not pinned. Copy source/protocol/driver
bytes from committed Git, verify all donor hashes and retain raw
outputs. Accept each program only with exit 0, its exact marker and
zero whitespace-question-mark error lines. Preserve failed attempts
and freeze repairs before a fresh ID. No EKS, Diem, freezer,
Conductor claim, merge or PR retarget. No statistical baseline, A/A
or power claim applies to the complete exact algebra.
