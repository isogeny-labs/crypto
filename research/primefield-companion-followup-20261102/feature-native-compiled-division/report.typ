#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= Compiled public division-index schedules
T-1826 · 2026-11-07 · native dense evaluator and explicit cost choice

The sign source now compiles its division-value dependency graph once from the subgroup order and block partition. Each public-point evaluation uses dense operand slots, replacing recursive map lookup and insertion. Full and parity-normalized representations share this schedule. No target scalar or cross-point value table is used.

#image("flow.svg", width: 100%)

*Construction.* Reserve base indices0–4; memoize each requested index while recursively compiling its children, then append its step. Every operand precedes its consumer. Store output slots in request order, including duplicates, and discard the index map. Source stores the schedule before base calibration and reuses it for raw calls and windows. A retained-node cap is checked during compilation; it is not a total-memory cap.

*Derived costs.* For V retained nodes and K requests, compilation uses O((V+K)log V) map operations; each point uses O(V+K) indexed field work and storage. Setup remains charged once per source instance. The arithmetic recurrence itself is unchanged. Default source preserves normalized mode with cap4096; an explicit constructor accepts node cap and caller weights M for multiplication and I for inversion.

*Mode selection.* Count new even/odd nodes E/O and even output requests A. Choose normalized exactly when (O−E+A+1)M is less than I; ties choose full. Comparison handles zero weights, signed differences and multiplication overflow. Geometry, resultant and final projection costs are identical across these two modes and cancel in this local decision, while remaining part of total source cost. This is an expression-cost choice, not machine calibration or a timing result.

#table(columns: (2fr, 1fr, 2fr), inset: 4pt,
 [Check], [Count], [Coverage],
 [Division outputs], [160,660], [Both compiled modes / both prior caches],
 [Schedule topologies and cap boundaries], [4 each], [Empty, repeated, dense, source-shaped],
 [Cost-model choices], [100], [Plus maximum-count overflow controls],
 [Integrated source points / windows], [1,108 / 24], [Explicit cost choice also preserves signs],
 [Additional recoveries], [617], [5,929 calls; zero unresolved],
 [Inherited public comparisons], [63], [511 calls])

*Evidence.* Maximum-u64 indices, duplicate ordering, zero and domain controls pass. Normal build has six dependency inputs and excludes recursive old caches and derivatives. Source/coprime/prime-field bounds remain; invalid torsion points can perform more eager work before rejection. Original4,563-case cohort unchanged. No native failures, timing or cloud run; full Cargo suite not rerun. PCC-20261107-ac7ce2 is unsigned producer evidence. Receipts, source pins and replay commands accompany DERIVATION.md.

*Sources and next step.* Local recurrence derivations and primary Mathlib EDS documentation; KB unavailable. Topological compilation reuses established identities, with no novelty/formal-proof claim. Next use schedule-use information to share squares and products, reducing actual field work while preserving complete source-cost accounting.
