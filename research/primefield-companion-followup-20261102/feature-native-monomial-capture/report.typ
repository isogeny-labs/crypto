#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Bounded capture and protocol binding
2026-11-07 · Synthetic validation only; native panel not launched

The instrument is bound to published commit b0b8a151ca4d and its release binary hash. The revised draft validates with zero findings using the preserved package tool/schema. The protocol remains unfrozen pending preflight and admission. [pcc:unverified]

#image("flow.svg", width: 100%)

== Custody and export
The wrapper checks protocol/binary bindings and an operator admission receipt, then exclusively creates an attempt directory and prelaunch marker. It captures stdout/stderr, kills the process group on timeout and records exit status, elapsed duration and hashes. It never retries or overwrites an attempt. The receipt is not a signature or independent authorization; budget and resource enforcement remain external.

The success adapter accepts complete65-record panels only, then emits60 target/arm rows with17 numeric event dimensions in experiment.export/v1. Full fixed cost is charged per target. Raw vectors remain separate. No clock validity or host isolation is invented. Failed/incomplete attempts remain raw custody records; failure-flow export is a remaining task.

== Tests and actual failures
Ten synthetic/subprocess methods pass, including success/exclusive-output, nonzero exit, missing executable, timeout custody, protocol hashing, export schema/binding rejection and inherited strict-parser controls. No cryptographic kernel runs in these tests. The initial timeout test incorrectly required output before the deadline; empty output is legitimate when startup has not completed. The first export used string notes where structured observation objects were required. Both failures are preserved and corrected. [pcc:unverified]

== Operational state and next work
Read-only EKS overview showed no monomial job; the existing SAT Job remains suspended. Reported pool constants do not establish live owner-policy capacity. No job or infrastructure was changed. Proposed USD1 envelope has no reserved funds. Next: failure-flow export, preflight, capture/export publication bindings, freeze and authoritative budget/capacity admission. Original4,563-case cohort unchanged.

== Provenance
The system jsonschema module lacks the required validator API; the project venv works. Earlier lookup missed the preserved tool under the research package. Its hash and schema hash are retained. The revised draft corrects flag types and execution-order enum and splits the event vector into numeric metric slots. Source/data dependencies, raw test logs and initial validation failures are saved. Claim PCC-20261107-0a8626 remains unsigned (local verdict not_covered).

Sources: matched instrument and parser; preserved experiment protocol tool/schema. No new mathematical, novelty or comparative performance claim. No native panel data exists.
