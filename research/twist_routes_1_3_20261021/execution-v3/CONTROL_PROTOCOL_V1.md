# Ordinary group control and explicit sign propagation, version 1

Freeze source, 200-cell plan and binaries before profiling. Use the same ten
original public curves, B=8, arities 2 and 3 and target label 0. These are
observed training cases, twenty configurations and ten fixture-target pairs.
No final E28 labels are opened. Preserve every requested attempt.

In each configuration profile direct A/A, direct, eight-branch helper and
first-pair invariants with the original sign search; repeat those four with
classical F3 sign propagation. Add ordinary group meet-in-the-middle A/A and
meet-in-the-middle. All arms reconstruct the same first-B nonzero-y base from
the public fixture and target. Workers receive no expected tuples, bases,
precomputed equations, prior solutions or logarithms.

The group control finds all signed pairs using a point lookup (arity 2), or
all first-pair sums and matching target-minus-third points (arity 3). Include
identity-prefix sums. Canonicalize the first pair and retain the minimum
valid sign mask. It avoids polynomial construction and solving legitimately;
do not add that unused work to make it resemble F4.

Propagation uses the classical discriminant numerator divided by 4*target-y
for both input y-coordinates. For arity 3 propagate through the affine prefix
backwards, then add the identity-prefix chart. Verify each recovered pair on
the source curve and its sum, and take the minimum mask over roots/charts.
The square identity has a separate Lean proof; the group-coordinate bridge
is checked by finite native and independent PARI checks, not claimed as a
new formal group-law theorem.

Each worker has a 15,000 ms shared cooperative solver budget, 18-second hard
timeout, one-second kill grace, 4 GiB address space, 32 MiB/file and one thread.
Cap the batch at 3,800 seconds and 1 GiB output, checkpoint every attempt.
Charge startup, parsing, base construction, actual algorithm, roots/signs,
exception handling, deduplication, source verification and serialization in
Callgrind simulated guest instructions Ir. No wall time is admitted on L0;
parent, OS/kernel, independent replay, matrix algebra and full IC are excluded.

Require exact equality of the entire ordered [x,y,sign_mask] witness list to
the frozen independently constructed E26 census, as well as the complete
tuple set, base and target. Parent and independent native auditor reopen all
hashes and raw single-event Ir totals. Every accepted row gets PARI point,
sum and wrong-sign replay. Require <=5 percent spread for each of the three
A/A pairs in a configuration before interpreting its paired cost ratios.
The frozen plan names all comparisons, including optimized F4 versus group
MITM. Report all losses, missing pairs and the ten distinct target pairs.

This is a cold finite-decomposition diagnostic. It has no full relation
pipeline, matched rho or newly measured external full-root solver. Neither
a known lifting formula nor an ordinary group control is a novel attack.
Keep prior measurements intact and do not promote a validation-gated row.
