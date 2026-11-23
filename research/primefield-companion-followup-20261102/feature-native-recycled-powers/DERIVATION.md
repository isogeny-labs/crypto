# Recycle value and power storage after the last use

T-1828, 2026-11-07. Derived lifetime bound and deterministic native correctness checks.

## Allocation model

Use the preceding shared-power schedule with V logical nodes. Logical nodes0..4
are born while initializing bases; every subsequent node i is born at its
corresponding recurrence step. Compute last[i], initialized to i, as the maximum
consumer birth among all operands. If node i supplies an output, set last[i]=V,
pinning it through final output extraction. Each node's value, square and cube
are bundled into one physical slot with inclusive lifetime [i,last[i]]. An
unused base has the singleton lifetime [i,i].

Scan births in increasing order. Before birth i, release slots whose last use
was i-1. Reuse a free slot, or create one if none is free. Compile each recurrence
operand to a physical slot; retain the logical-to-physical map for births and
outputs. Flags still refer to logical nodes. Runtime initializes each base and
its required powers immediately, then evaluates physical steps. Each destination
gets its value and flagged powers before future uses. No value is recomputed
and no extra field arithmetic is introduced.

## Correctness and optimality at the stated scope

Invariant: each live logical node has an exclusive physical slot containing its
correct value and any required powers. New allocation uses only expired slots;
all operands are earlier logical nodes and their last uses include the current
step. Thus their slots remain valid. The destination is distinct from every
operand live at the same step. After evaluating the unchanged recurrence, storing
the result and preparing flagged powers restores the invariant. Output pinning
makes final extraction valid, including duplicate requests and even-value scaling.
Zero values and stale unneeded power cells are harmless: only flagged powers
can be consumed, and they are prepared before use.

Let L be the maximum number of overlapping inclusive intervals. The scan uses
exactly L slots. At every new allocation with no free slot, all existing slots
are live, so that many overlapping intervals force the extra slot. Conversely,
released slots are reusable without overlap. Hence L<=V and this allocation is
optimal for these fixed, whole-bundle, inclusive intervals. It is not a claim
of globally minimal memory: separate power lifetimes, overwriting an operand
on its final instruction, output streaming, schedule reordering or recomputation
can change the model.

## Costs and limits

Last-use analysis, expiration buckets and allocation add O(V+K) setup time and
memory for K outputs. The retained schedule includes both logical and physical
steps, a V-entry slot map, flags and outputs. This extra setup/metadata is charged
once per Source instance. Evaluation allocates three arrays of L field wrappers
instead of three arrays of V wrappers. With each wrapper storing two u64s,
array element payload is48L bytes rather than48V bytes. Headers/capacity,
allocator overhead, constructor temporaries, output array and other source
geometry/polynomial data are additional. No measured total-memory or timing
claim follows from this payload bound.

Both mode arithmetic counts and shared-power saving2E+4O-S-C are unchanged.
The optional caller cost choice remains (O-E+A+1)M<I, with full-mode ties.
Existing public-point, prime, coprime-subgroup and word-size conditions remain.
Source automatically uses the recycled evaluator, with its existing logical
node cap (default4096). Schedule::evaluate_bounded additionally accepts a physical
live-slot cap; if L exceeds it, Workspace is returned before field-array allocation.
This cap is on the direct Schedule API, not a total-RAM bound or a new source
constructor argument. Normal evaluate uses no separate live cap; L<=V still holds.

## Exact checks

validation-01 checks4100schedules: singleton indices0..4095 plus empty, repeated
bases, dense0..127/top2u64, and source-shaped repeated requests. An independent
consumer scan reconstructs last use; slot-owner simulation rejects overlap;
a difference-array sweep independently computes the interval peak and matchesL.
All pass, including strict reuse cases. The previous4096nonnegative arithmetic
saving checks and100cost choices remain.

All160660outputvalues agree in both modes against unrecycled shared-power,
unshared compiled and both recursive references. Exact-L evaluation succeeds;
L-1 returns Workspace. Prior topology/logical-node-cap/maximum-index/domain
controls remain. Source checks retain1108points/direct-halfproducts/covariance/
signs,24windows,114751classpairs,1792resultants and266wrong-exponent controls.
recovery-01 retains617additionaltargets/5929calls/zero unresolved and63public
comparisons/511calls. The normal build has seven dependency inputs excluding
old cache/source references. All native builds/assertions pass; warnings remain.
Original4563-case cohort unchanged; no timing/cloud run or full Cargo-suite rerun.
Producer evidence is unsigned and not an independent verdict.

Generate with generate.py then generate-recovery.py. Run validate.py and
validate-recovery.py with distinct new directories. Receipts pin compiler,
commands and inputs/stdout; publication.json pins deliverables. Preserve old
outputs. Maximum-u64 tests exercise index bounds, not cryptographic-size fields.

## Sources and next construction

KB unavailable (no loaded tools; wrapper no executable). Read local shared-power
proof and source. [Poletto–Sarkar1999, section4 and Figure1](https://c9x.me/compile/bib/linearscan.pdf)
provides established expiration-based interval allocation. Here the straight-line
schedule permits an unbounded pool and no spills; the exact whole-bundle peak
proof is given above. No allocation novelty or formal Lean proof claimed.
Next distinguish value, square and cube last uses to reduce conservatively bundled
lifetimes, or stream outputs on final production while retaining request order;
account metadata/setup before choosing further specialization.
