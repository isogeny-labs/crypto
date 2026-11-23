#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= Separate lifetimes in one field arena
T-1830 · 2026-11-07 · typed value / square / cube allocation

The division evaluator now tracks the value, square and cube separately. Required powers are still prepared once, but their slots become reusable after their own final consumers. A single arena replaces the three bundled arrays. Exact recurrence arithmetic and source behavior are preserved.

#image("flow.svg", width: 100%)

*Typed uses.* Odd recurrence steps consume two values and two cubes; even steps consume three values and two squares. Final extraction pins only values. Each present entity is born with its logical node and remains live through its final typed use. Absent powers receive no slot. Same-birth entities remain exclusive during power preparation; output order and duplicates are retained.

*Proof and bound.* Birth-order interval allocation never overwrites a live entity, and its slot count equals the typed interval peak P. Each typed interval lies inside its old bundle interval, with at most three entities per bundle. Therefore P≤3L for prior bundle peak L. This is optimal for the fixed inclusive typed intervals, not for arbitrary schedule reorderings, recomputation, output streaming or in-place final-use reuse.

*Accounting.* Arena element payload is16P bytes versus the prior48L. One vector allocation replaces three. The slot map now holds three entries per logical node; setup metadata and logical/physical schedules remain charged. Headers, output/constructor temporaries and other source arrays are additional. Shared-power savings and mode-choice scores are unchanged. Schedule.field_slots reports field-element slots; evaluate_bounded rejects a smaller cap before arena allocation. The old module's bundle cap is not silently reinterpreted.

#table(columns: (2fr, 1fr, 2fr), inset: 4pt,
 [Exact check], [Count], [Coverage],
 [Typed lifetime plans], [4,100], [No live alias; independent peak; P≤3L],
 [Output equalities], [160,660], [Both modes and all prior routes],
 [Workspace boundary], [P / P−1], [Exact cap succeeds; smaller rejects],
 [Source points / windows], [1,108 / 24], [Direct products, covariance, signs],
 [Additional recoveries], [617], [5,929 calls; zero unresolved],
 [Inherited public comparisons], [63], [511 calls])

*Evidence.* Strict nonbase storage reductions exist in the exact plan checks. Power-saving and cost-choice controls remain. Native checks and seven-input metadata build pass; warnings retained. Original4,563-case cohort unchanged; no timing/cloud run or full Cargo-suite rerun. PCC-20261107-afba18 is unsigned producer evidence. See DERIVATION.md and receipts for replay, hashes and assumptions.

*Sources and next construction.* Prior bundled proof plus established Poletto-Sarkar1999 interval expiration; KB unavailable. No allocation novelty/formal-proof claim. Next account metadata and remaining source phases together before further specialization; emitting outputs at production could shorten final pinning but needs a separate ordering/cost proof.
