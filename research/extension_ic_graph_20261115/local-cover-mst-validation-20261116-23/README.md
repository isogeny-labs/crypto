# MST endpoint check: correct recovery through refused-projection fallback

[Report](report.pdf) · [Full Rust summary](summary.json) · [Exact raw](raw.jsonl)
· [Protocol](../AMENDMENT_18_MST_FILTER.md) · [Custody](local-evidence.json)

Source a457ec75b, one call, exit0 in27.517seconds. Both fresh public-input
workers recover scalar66296859173437586 after native and outer group replay.
The MST arm uses one original-solve fallback, so the frozen nontrivial
projection criterion fails. No comparative ratio or interval is produced.
[pcc:unverified]

The planner eliminates127columns through127trees and1703selected edges,
evaluating19297candidate edges; maximum column degree51, no resource refusal.
It produces a candidate with382rows and10904nonzeros from509rows/4577nonzeros.
The dimension-times-nonzeros proxy worsens, so the guard refuses it before
admission or original-input replay. Admitted and replayed rows are both zero.
The original selected system then solves correctly. [pcc:unverified]

This is a negative result for the forced75percent-retention MST+guard design
on this challenge. It does not test a different stopping or column-selection
rule, and it does not establish a wall-speed verdict. The candidate matrix
itself was not solved or group-validated. No retrospective threshold changes,
unchanged retry or full timing follow-up authorized by this validation.
[pcc:unverified]

Exact raw/full original Rust summary/progress, runtime/time/CPU snapshots and
pre/post hashes are retained here and in the original local run directory
/workspace/ic-local-runs/local-cover-mst-validation-20261116-23. All pre/post
hashes rechecked. Legacy counters omit graph arithmetic and remain incomplete;
no primary counted-cost, signed independent, canonical-schema, standard lab
or admitted ranking assertion. [pcc:unverified]

The final one-page PDF was visually inspected. Exact PDF/SVG/150dpiPNG browsing
copies and checked hashes are under benchmarks/20261116-cover-mst-validation;
original final PNG/SVG/PDF copies also remain in the local run directory.
[pcc:unverified]
