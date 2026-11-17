# Frozen factor-base scaling test for different-j fibers

Frozen 2026-10-16 before the first run under this protocol. This is a
follow-on to the exact order-229 χ pair oracle and nine-arm held-out
panel in PR #1939. Historical inputs and receipts are immutable.

## Hypothesis and boundaries

On the explicit degree-three, different-j pair over F_(211²), enlarging
a common projected factor base from 8 to 16, 32, and 64 columns may
increase verified two-sum hit probability and useful rank per unit of
complete cold work. A χ resultant may avoid a pair-sum table, but its
root search and exact checks must be charged. The primary discriminant
is a verified unknown-scalar solve, not pair counts alone.

Use S = total equivalent source-group operations / sqrt(229). The
prespecified generic two-class collision floor is sqrt(pi/4) =
0.886226925 in this unit. The earlier eight-column signed-rho
counted-group reference was 7.979 S_group on these targets; it is a
lower-bound operation view, not a portable price. Measure new matched
rho, BSGS, and rho A/A on every target and round. A method advances
only if its complete cold S, with uncertainty and all setup, is below
the same-target control across a credible order trend. This single
order can establish a local cost comparison only.

## Immutable inputs and base selection

The source is y²=x³-21x+37, the destination is
y²=x³-81x+179 (coefficient 130 modulo 211), and the rational kernel
is {O,(3,1),(3,-1)}. The order-229 source generator, its image, and
the 64 parent targets are in
`../different_j_order_batch_20261015/PUBLIC_V1.json`,
SHA-256 `f6c3126b4d95e3c457c9e552471bc2cdff322de0e0a9de9fbba701d7c05897ba`.
Use the same four held-out targets in
`../chi_polynomial_20261015/different_j_resultant_20261016/FOUR_TARGET_PUBLIC_V1.json`,
SHA-256 `e1969305462131722556dead399593b77a919a8017f0e035e3019986b645958e`.
Their independent checker file has SHA-256
`16612e5a002fe85c62648ce62ec4cc2432868c02b83dff20174f5ff46493ebf9`.
No answer scalar or factor-base logarithm enters a solver input.

Enumerate all 44,655 rational source points exactly as the existing
`different_j_order_batch` construction does; map them with the
explicit degree-three Vélu map, deduplicate the 14,885 image points
by destination point key, and visit them in ascending key order.
Skip infinity and zero cofactor projections. Project by [195] and
retain the first 64 distinct signed subgroup orbits, storing the
first raw destination point and source preimage witness for each.
The first eight must equal the committed eight-column public base
element by element; otherwise stop and preserve the mismatch.
The four tested bases are exact prefixes of lengths 8, 16, 32, 64.
Freeze these public points and all input hashes before a solve.
The 64 discrete logs, if computed for independent checking, stay
in a checker-only file and never enter a run process.

## Exactness gate

For each base, independently enumerate the set of all unordered signed
index pairs and their sums for all 229 subgroup residues, including zero
hits and repeated summands (i=j). Compare the complete returned pair
set from the χ chord-resultant root procedure with this enumerator for
every residue. Count repeated polynomial roots separately from index
pairs. Record exceptional χ values, vertical and tangent cases, failed
splits, false candidates, table probes, polynomial calls, field calls,
and exact additions. Any mismatch stops that base's solve panel. Use a
300-second and 512 MiB hard cap per exhaustive child; time or memory
censorship is not pair-set equality.

## Complete-solve panel

Run four targets in three domain-separated rotated rounds at each
base size. The five relation arms are χ with inverse-Vélu base
construction, χ with full-image construction, direct pair table with
inverse-Vélu construction, direct pair table with full-image
construction, and destination-direct pair table. The four source
relation arms must recompute and verify the same frozen projected
columns, charging each construction and rejected candidate; source
subgroup preimages must be derived, not supplied as free witness data.
The destination-direct arm scans rational destination points in
ascending point-key order, projects by [195], and takes its first B
distinct signed orbits. It records its own public base. If that base
differs from the full-image prefix, report its yield as an unpaired
base-selection control. All relation arms use the same seed, scalar
trial stream, first verified pair rule, and cap of 8r=1,832 trials
per target. The destination arm uses the mapped subgroup. If a
source construction cannot produce the frozen base, retain that
failure and do not claim paired relation yield.

For each relation child, collect until rank B+1, solve the modular row
system, recover the target scalar, and independently check [k]G=Q and
the mapped identity. Include source BSGS, signed rho, and rho A/A
with 32 restarts of at most 20r steps. Run all eight arms at every
base, target, and round: 384 cold children. The BSGS and rho
duplicates across base sizes are intentional paired controls.
Preserve every timeout, failed relation, rank deficit, invalid
scalar, and resource-censored child. Each child has a 300-second
wall cap and 512 MiB address-space cap where the host supports it;
record any host limit that cannot be enforced.

## Cost and independent replay

Record disjoint setup, relation, row-solve, target-recovery, and final
verification phases. Within each, count group additions/doublings;
Fp² arithmetic inside and outside them; inversion and square-root
internals; polynomial gcd, split, and resultant work; table probes
and inserts; modular row operations; allocations; rejected candidates;
CPU, wall, peak RSS, startup, and I/O. The operation counters and
field counters overlap by design and are never added directly.
Calibrate native primitive costs against a source group addition with
intervals on the same host, then obtain an independent Linux
calibration. If any material primitive is unpriced or the host
calibrations disagree, publish the full count vector and leave
complete portable S unset.

A separate native checker recomputes base identities, every relation
witness, row rank and coefficients, the recovered scalar, phase sums,
and all public/checker hashes. It must not call the run's row solver.
Require exact replay of every completed child before publishing a
speed ratio. Run the primary panel on a shared host as a correctness
and counted-work result; an isolated L2 wall comparison requires
reserved physical cores, affinity and thermal receipts, five
interleaved rounds, and an admitted rho A/A noise interval. A hosted
VM is an L1 independent replay only.

## Decision and artifacts

At each B, report hit support, trials per useful rank, complete
one-target S with interval if calibrated, N=4 setup reuse separately,
and exact ratios to matched rho and BSGS in one table. A falling
trial count with a rising total is a stage improvement only. If rank
fails, inspect the independently replayed rows before changing the
base or cap. If χ and direct support agree, compare their full costs;
if not, stop and locate the first differing pair. No single-order
outcome supports a scaling exponent or deployed-curve conclusion.

Commit the source and input hashes, per-child exits and compact raw
records, independent replay, failure receipts, report, SVG and PNG
graph, and PDF beside this protocol. Mirror performance visuals under
`benchmarks/20261016-isogeny-fiber-base-grid/` and add its README
row in the same PR. This protocol remains an unexecuted plan until
the frozen base and run receipts exist.
