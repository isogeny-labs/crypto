#set page(paper: "a4", margin: 16mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: 0.6em)
= Complete saved-cohort solver counters
Rust graph IC continuation · 23 November 2026

All 3415 saved systems and 3419 root multisets agree across the three recorded paths. This analysis performs no new solver, relation search or timing call. All raw inputs and both replay summaries are hash-bound. Producer observation: PCC-20261123-400f7d, *UNVERIFIED*. [pcc:unverified]

#image("solver-counts.svg", width: 100%)
#table(columns:(1.7fr,1fr,1fr,1.1fr),align:(left,right,right,right),[Backend],[Reported F4],[Reported linear],[Reported sum],[Unit without trace],[2179269070],[716225084],[2895494154],
[Unit with trace],[1097651810],[716225084],[1813876894],
[Six with trace],[1065026764],[686480526],[1751507290],
)

== What the comparison changes
The retained six-coordinate trace has the lower *reported subtotal* over the entire cohort. The five-coordinate trace has 3348 replay flags, 66 mismatch flags and one recording; six has 3381, 33 and one. Every captured root set matches. Coordinate elimination alone does not establish a cheaper solver workload, and these counters do not decide the complete-cost winner. [pcc:unverified]

== Missing learning work
The pinned engine's tracked RREF used to learn a trace is outside its field_ops counter. The saved rows do not contain the event trace needed to recover that cost. A separate Nov18 descent-finder study reports its own bound; it is not transferred to these binaries or the five-variable arm. Other missing work includes reduction/lifting, cold collection and graph/kernel phases, native control, RNG, hashing and serialization.

== Correctness and execution limits
The analysis checks the exact same coordinate multisets and explicit status flags on every record; checked u128 sums retain large integer counts. Existing full-equation replay is bound through its saved proofs. Fixed-cohort agreement is not a generic completeness proof for all future exceptional shapes; a focused acceptance/fallback audit remains open.

No complete-cost ratio, wall-time gain or parity is inferred. A funded Runpod comparison must retain both traced arities and account for all learning and cold setup. Provider balance and checkpoint upload remain external blockers; no new paid call occurred.

Sources: proof-bound-counter-summary.json, PLAN.md and SOURCES.md beside this report; native trace_cohort_summary.rs and trace_cohort_report.rs; existing leads-20261118/descent-finder study. #link("https://github.com/isogeny-labs/crypto/pull/4986")[PR 4986] retains the implementation and earlier controls.
