#set page(paper: "a4", margin: 15mm)
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.55em)
= Exact unit descent with checked trace reuse
Rust extension-field graph IC · recorded mathematics · 23 November 2026

#image("flow.svg", width: 100%)

The graph collector and sparse kernel stay fixed. Opt-in target descent uses a constant-unit substitution to reduce six coordinates to five, then the existing F4 learning/replay mechanism. Every shape mismatch takes the checked full fallback. Dedicated worker calls clear the cache and separate arities before work.

== Full captured-cohort correctness
#table(columns:(2fr,1fr,1fr,1fr),align:(left,right,right,right),
 [Backend],[Systems / roots],[Replayed],[Fallbacks],
 [Five coordinates],[3415 / 3419],[3348],[66],
 [Six-coordinate control],[3415 / 3419],[3381],[33],)
Both match every captured root set and every original equation, with zero failures. No new relation search or elapsed improvement follows. Producer evidence is *UNVERIFIED*, PCC-20261123-ecef76. [pcc:unverified]

== Fixed first 32 systems: partial solver counters
#image("coordinate-counts.svg", width: 100%)
#table(columns:(2fr,1fr,1fr),align:(left,right,right),[Backend],[F4 multiplies],[Linear multiplies],[Six coordinates],[22809017],[6375427],
[Original unit],[20422112],[6673604],
[Normalized unit],[20422112],[6673604],
[Unit with trace],[10379039],[6673604],
)
All four controls match the same 34 roots and coordinate fingerprint. These counters exclude reduction, collection, native control and serialization; they are not complete pipeline costs.

== Profile and execution limits
Full-record Callgrind totals are 29290711138 / 29289230219 simulated instructions (old / normalized unit), dominated by unrelated JSON/hash startup. The compact profile schedule remains unexecuted. Instruction profiles are not timing measurements.

Current-root library tests fail to compile unchanged base syntax in binary_hyperelliptic_index_calculus.rs. Scoped binaries use the explicitly pinned Root41 library. Runpod balance and unavailable checkpoint upload block a new paid comparison; no successor pod exists.

Sources: #link("https://github.com/isogeny-labs/crypto/pull/4986")[PR 4986], retained replay JSON and compact controls beside this report; native source trace_integration_report.rs. Prior art: Joux–Vitse §§3.1–3.3 and the existing exact unit/F4 trace implementations. Original Run51 qualifier137 and separate replay0 remain unchanged. No complete-cost ratio or parity claim.
