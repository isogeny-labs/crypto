# Compile public division-index dependencies once per source

T-1826, 2026-11-07. Native construction and deterministic software checks.

## Scope and mechanism

The existing value caches perform recursive BTreeMap lookup/insertion for each
public-point evaluation. Their dependency indices depend only on requested n,
not on field values or target scalar. Compile that DAG once, then evaluate a
flat topological list using integer operand slots and a dense vector of values.
The runtime accepts only curve/point coordinates and a mode; it performs no
map lookups, recursion or hidden-scalar enumeration.

Schedule::new(requests,node_cap) reserves base indices0..4. For nonbase n,
recursively compile exactly the standard recurrence dependencies, memoizing
n -> slot in a temporary BTreeMap. Append each step after its children.
Store outputs as (slot,even) in request order, preserving duplicate requests.
Drop the map when compilation finishes. A node cap below5 rejects; before
appending every nonbase node, check the cap. The cap covers retained nodes,
not output-request length, stack or total allocator memory.

Each dependency is smaller than n: the first nonbase odd n=5 and even n=6
are safe, and n/2+2 fits even at u64::MAX. Induction proves the schedule has
all operands earlier than their consuming slot. Induction on this topological
order then proves equality to the original recursive recurrence, including
zero values at torsion indices. Both full W_n and parity-normalized f_n
representations use the same schedule. The latter's proof is preserved in
../feature-native-normalized-division/DERIVATION.md.

For V distinct retained nodes and K requests, compilation uses
O((V+K) log V) map operations, O(V+K) retained schedule/output storage,
and logarithmic index-depth recursion. Each point evaluation uses O(V+K)
field work and O(V+K) temporary storage, with constant-time indexed operand
reads. This removes the per-point logarithmic map factor from value lookup;
it does not change the field recurrence's index complexity. Setup is charged
once per Source instance, not amortized away in a single-target cost.

## Source integration

For public block partition [w,k,s,e], compile requests

    [r, r+1, 2, r+2, (r+1)/2, 2w] and, if s!=0, [s+e].

These cover the torsion check, beta seed, initial half-index value, step value
and tail. Source stores the immutable schedule and mode before calibrating the
base point. Every raw point and every nonidentity window entry reuses it.
Field values are never retained across points. Default Source::new preserves
the normalized mode and uses a4096-node cap. new_with_costs accepts an explicit
node cap and CostModel; cap exhaustion returns Domain. This is an additional
explicit resource bound, not a guarantee that every permitted u64 order fits.

The field domain remains caller-certified prime5<=p<=u64::MAX/2, canonical
nonsingular curve and nonzero-y point; source also retains odd prime r>=7,
r!=p, gcd(r,p-1)=1, r<=u64::MAX/2 and geometry-lane bounds. The constructor
validates these before forming r+2. Evaluating all requests eagerly means an
invalid torsion point may incur extra work before rejection compared with the
old cache; accepted-point outputs are unchanged.

## Cost-based mode choice

Schedule compilation computes E/O, the exact counts of nonbase even/odd steps,
and A, the count of even output requests including duplicates. Full mode costs
6E+6O multiplications and one constructor inversion, apart from shared setup.
Normalized mode costs5E+7O+A+1 multiplications relative to the same shared
constructor baseline and no inversion. The +1 is the net constructor change:
two squares for (2y)^4, minus one multiplication in the fourth base value.

Given caller-declared nonnegative multiplication/inversion weights M,I,
choose normalized iff (O-E+A+1)M < I; ties choose full. Compare the signed
difference rather than overflowing large total scores. For nonnegative delta,
checked u128 multiplication overflow implies extra>I since I is u64; full
is then correct. For negative delta, normalized wins unless both weights vanish.
No machine calibration or elapsed-time measurement is supplied by this API.

Geometry, class recurrence, resultants and final projection are identical
between these two modes, so their costs cancel in this local route choice.
They still contribute to the complete source and recovery cost. Compile/setup,
allocation, branches, reductions and actual machine inversion cost require
separate accounting before any elapsed-time conclusion. This is an exact
choice under a supplied expression-cost model, not a claimed timing optimum.

## Executed checks

validation-01: four request lists (empty, bases with repeats,0..127 plus top2u64,
and a source-shaped list with repeats). All four schedule topologies and exact
node-cap/one-below boundaries pass. On all1108nonzero-y points of the eight
previous fixture curves,160660outputvalues agree with both recursive caches
and both compiled modes.100weight-pair choices agree with direct score
comparison; synthetic maximum-count/weight controls cover overflow and ties.
Canonical/domain/point controls remain. Max-u64 tests cover index bounds,
not cryptographic-size field arithmetic.

Inherited source tests retain1108points, direct half-products/covariance/signs,
24windows,114751classpairs,1792resultants and266wrong-exponent controls.
A second source with explicit M=1,I=0 also returns the expected1108signs.
recovery-01 retains617additionaltargets/5929calls/zero unresolved and the
63publiccomparisons/511calls. Normal metadata build has six dependency inputs
and excludes recursive old caches and derivative sources. All native checks
pass; warnings retained. Original4563-case cohort unchanged. No timing/cloud
run; full repository Cargo suite not rerun. Unsigned producer evidence only.

Run generate.py and generate-recovery.py, then validate.py NEW_DIRECTORY and
validate-recovery.py ANOTHER_DIRECTORY. Saved receipts pin commands/compiler,
inputs and stdout. Do not overwrite saved outputs. publication.json pins the
proof, source, receipts and inspected report.

## Sources and next work

KB unavailable: no loaded MCP tools and scripts/research-kb status reports no
executable. Read local normalized/value recurrence derivations and source;
[Mathlib EDS documentation](https://leanprover-community.github.io/mathlib4_docs/Mathlib/NumberTheory/EllipticDivisibilitySequence.html)
supplies standard recurrence identities, already read for the preceding step.
Topological compilation is an implementation of the existing DAG, not a new
EDS identity or a formal Lean result. No novelty claim. Next reduce recurrence
arithmetic through shared squares/products, using schedule-use information;
keep geometry and resultant costs in the complete accounting.
