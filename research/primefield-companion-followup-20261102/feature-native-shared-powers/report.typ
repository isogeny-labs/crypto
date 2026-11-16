#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= Shared powers in compiled division schedules
T-1827 · 2026-11-07 · algebraic multiplication reduction

Each required square and cube is now prepared once per public point and reused by later recurrence steps. A compile-time scan marks power dependencies in the public index schedule. The evaluator prepares flagged powers when their value becomes available. Both full and normalized modes retain exact outputs.

#image("flow.svg", width: 100%)

*Derived reduction.* Let E/O count even/odd recurrence steps and S/C distinct slots needing squares/cubes. Previously power formation used2E+4O multiplications; sharing uses S+C. Distinct uses cannot exceed occurrences, so the saving2E+4O−S−C is nonnegative. For request7, odd nodes5 and7 share the cube of slot3: E=0,O=2,S=C=3, saving2 multiplications. This example and formula are algebraic, not timing observations.

*Updated costs.* Shared full recurrence costs S+C+4E+2O multiplications. Shared normalized recurrence costs S+C+3E+3O+A, with A even output reads. The prior constructor difference remains+1 multiplication/−1 inversion. Both modes save the same amount, so the caller-weight choice (O−E+A+1)M less than I remains valid. Geometry and resultants are unchanged and remain part of total source cost.

*Storage charge.* The evaluator adds two dense V-element field arrays and a V-byte schedule flag vector. Each field wrapper stores value and modulus as two u64s, so added array element payload is32V bytes. Headers, capacities, allocator overhead, zero initialization, array traffic, output and other source arrays are additional. Node cap is not a total-memory cap; no elapsed-time gain is inferred.

#table(columns: (2fr, 1fr, 2fr), inset: 4pt,
 [Exact check], [Count], [Coverage],
 [Value equalities], [160,660], [Shared/unshared compiled and recursive routes],
 [Static saving bound], [4,096 plans], [Single indices0–4095; strict cases exist],
 [Topology/cap and cost choices], [4 / 100], [Power flags, repeats, overflow controls],
 [Source points / windows], [1,108 / 24], [Full reference and scalar signs],
 [Additional recoveries], [617], [5,929 calls; zero unresolved],
 [Inherited public comparisons], [63], [511 calls])

*Evidence.* Both native validation commands and normal metadata build passed; warnings retained. Six production dependencies exclude old cache references. Top-u64 indices and zero/domain cases remain covered. The original4,563-case cohort is unchanged. No timing/cloud run or full Cargo-suite rerun. PCC-20261107-d71783 is unsigned producer evidence. DERIVATION.md, receipts and publication hashes provide replay and claim scope.

*Sources and next construction.* Prior compiled/normalized proofs and primary Mathlib EDS recurrence documentation; KB unavailable. Sharing is common-subexpression reuse, not a new recurrence or formal-proof claim. Next use last-use information to recycle power/value slots and reduce per-point allocation while retaining the operation savings.
