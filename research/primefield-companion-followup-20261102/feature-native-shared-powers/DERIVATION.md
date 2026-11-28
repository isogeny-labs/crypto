# Share recurrence powers in a public compiled schedule

T-1827, 2026-11-07. Algebraic operation-count identity and native correctness checks.

## Construction and invariant

The preceding compiled schedule evaluates full W_n or parity-normalized f_n
from public indices. Its odd steps read cubes at two operand slots and even
steps read squares at two slots. Scan the immutable schedule once: mark bit1
for a square dependency and bits1|2 for a cube dependency. A cube also needs
its square. After each field value is computed, compute its square iff bit1
is set, then its cube as square*value iff bit2 is set. Perform the same preparation
for the five base values before the recurrence. Consumers read dense power
arrays. Unneeded slots are zero-initialized but never consumed as powers.

Topological induction: every operand's value already exists; its flags include
every use recorded by the compiler, and its required powers were prepared at
creation. Thus replacing v*v and (v*v)*v by the stored values preserves every
step, including zero values, in both representations. There is no assumption
that the values are distinct or nonzero. Field arithmetic and the prior
prime/canonical/domain conditions are unchanged. Flags depend only on public
indices; values/powers are fresh per point and never shared across points.

## Exact arithmetic reduction

Let E/O count even/odd recurrence steps. Let S count distinct slots needing
a square and C distinct slots needing a cube. Without sharing, even steps
spend two multiplications on squares; odd steps spend four on cubes. With
sharing, power preparation uses exactly S+C multiplications. Therefore

    saving = 2E + 4O - S - C >= 0.

Proof of nonnegativity: there are2E+2O square-use occurrences (including the
square inside each cube) and2O cube-use occurrences. Taking distinct slots
cannot increase either count. Equality is possible. Example: request[7]
compiles odd nodes5 and7. Both need the cube at slot3; slots2,3,4 need cubes.
Thus E=0,O=2,S=C=3 and saving=2. This is a derived expression count, not a
runtime measurement. No multiplication is removed from coordinate geometry
or resultants by this change.

After dependencies and excluding common base setup, shared full mode costs

    S+C+4E+2O,

and shared normalized mode costs

    S+C+3E+3O+A,

where A counts even output reads. Constructor normalized mode still differs
by+1 multiplication and-1 inversion. The existing caller-weight decision
(O-E+A+1)M<I is therefore unchanged: both modes save the same count. Ties
still choose full. Default remains normalized with4096-node cap; explicit
cost constructor retains caller cap. The cost model is not a calibrated
wall-time predictor.

## Memory and setup accounting

Compile-time scan adds O(V) work for V retained nodes and one u8 flag per node.
Per-point evaluation adds two dense arrays of V field wrappers each, in addition
to the V values. Each wrapper stores value and modulus as two u64 fields:
the additional element payload is32V bytes, excluding vector headers/capacity,
allocator metadata, output array and geometry/polynomial data. Zero initialization,
array reads/writes and flags also cost work. Retained node cap remains a node
cap, not a total-RAM limit. These costs prevent an inferred elapsed-time win
from the algebraic multiplication reduction alone.

## Validation contract and results

This is a deterministic implementation check of a derived identity, not a
benchmark, statistical comparison or external-fastest claim. Protocol guidance
was read for this classification; its referenced local controlled-experiment
and protocol files were absent. Inputs remain the prior fixed curves/requests.

validation-01 compares both shared modes with unshared compiled and both
recursive caches:160660outputs on1108points across the four fixed request
lists. These include empty output, repeated bases,0..127 plus top2u64, and
source-shaped repeated requests. Power flags equal the independently assembled
consumer requirements, cube flags imply square flags, and topology/cap checks
pass. Static nonnegative-saving checks cover4096single-index schedules n=0..4095,
including strict-saving cases. The100cost choices and overflow controls remain.

Inherited source checks retain1108points/direct-halfproducts/covariance/signs,
24windows,114751classpairs,1792resultants and266wrong-exponent controls.
recovery-01 retains617additionaltargets/5929calls/zero unresolved and63public
comparisons/511calls. Normal metadata build has six inputs and excludes old
cache/source references. Focused native builds/assertions pass; warnings remain.
Original4563-case cohort unchanged. No timing/cloud run, full Cargo suite or
independent signed verdict. Maximum-u64 indices test recursion bounds, not
cryptographic-size field arithmetic.

Generate with generate.py then generate-recovery.py. Run validate.py and
validate-recovery.py with distinct new output directories. Receipts preserve
commands, compiler, input/output hashes; publication.json pins all deliverables.
Do not overwrite earlier evidence.

## Sources and next construction

KB tools absent; scripts/research-kb status reports no executable. Read
../feature-native-compiled-division/{DERIVATION.md,schedule.rs,division.rs}
and prior normalized proof. Standard recurrence source:
[Mathlib EDS documentation](https://leanprover-community.github.io/mathlib4_docs/Mathlib/NumberTheory/EllipticDivisibilitySequence.html),
already read in preceding components. Power sharing is common-subexpression
reuse of those identities, not a novelty or new formal-proof claim.
Next use last-use information to recycle power/value storage, preserving the
operation reduction while reducing the additional per-point allocation.
