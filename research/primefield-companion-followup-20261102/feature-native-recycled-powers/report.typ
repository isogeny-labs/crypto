#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= Recycled storage for shared division powers
T-1828 · 2026-11-07 · last-use allocation and native checks

The public division schedule now recycles storage after a value's final use. Each physical slot holds a value and its shared square/cube. Logical operands are compiled to physical slots; outputs remain pinned until extraction. Both arithmetic modes retain their multiplication savings and exact outputs.

#image("flow.svg", width: 100%)

*Derived invariant.* A logical node i lives on the inclusive interval from birth i to its final consumer; output nodes live through final extraction. Release only intervals ending before the next birth. Every live node therefore has an exclusive slot, and every operand remains intact when read. Prepare required powers immediately when each base or recurrence value is installed. No value is recomputed.

*Slot bound.* The scan uses exactly L, the peak number of overlapping intervals. Creating a new slot when none is free witnesses that many simultaneous live intervals; reuse otherwise preserves exclusivity. Thus L is optimal for these fixed whole-bundle inclusive intervals and L≤V. Separate power lifetimes, final-use overwrites, output streaming and reordered schedules are outside that optimality statement.

*Costs and limits.* The three field arrays now contain L entries each instead of V: element payload is48L bytes rather than48V. Metadata, headers, allocator overhead, output arrays, constructor temporaries and other source data remain additional. Last-use analysis adds O(V+K) setup work/storage and retains logical/physical schedules plus a slot map. Shared-power operation counts and mode choice are unchanged. Schedule.evaluate_bounded rejects a live-slot cap below L before allocating field arrays; this is not a total-RAM cap.

#table(columns: (2fr, 1fr, 2fr), inset: 4pt,
 [Exact check], [Count], [Coverage],
 [Liveness plans], [4,100], [No overlap; independent peak agrees],
 [Output equalities], [160,660], [Recycled and all prior routes, both modes],
 [Workspace boundaries], [L / L−1], [Exact cap succeeds; insufficient cap rejects],
 [Source points / windows], [1,108 / 24], [Full reference and scalar signs],
 [Additional recoveries], [617], [5,929 calls; zero unresolved],
 [Inherited public comparisons], [63], [511 calls])

*Evidence.* Native checks and normal metadata build pass; warnings retained. Seven dependencies exclude old cache references. Prior power-saving, cost-choice, duplicate and maximum-index controls remain. Original4,563-case cohort unchanged; no timing/cloud run or full Cargo-suite rerun. PCC-20261107-89903e is unsigned producer evidence. DERIVATION.md and receipts supply proof scope, replay and hashes.

*Sources and next construction.* Poletto–Sarkar1999 section4/Figure1 gives the established interval-expiration method; local shared-power source supplies the recurrence. KB unavailable. No allocation novelty or formal-proof claim. Next separate value and power lifetimes or stream outputs to shorten conservatively pinned intervals, while accounting setup metadata.
