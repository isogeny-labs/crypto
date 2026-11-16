# Capacity-selected recovery validation

[Report](report.pdf) · [Native summary](summary.json) · [Exact raw](raw.jsonl)
· [Protocol](../AMENDMENT_16_CAPACITY_VALIDATION.md) · [Custody](local-evidence.json)

Source8ab624189, one local call, exit0in40.445seconds. Both fresh public-input
workers recover scalar72041284332044320 on subgroup263807420025969577 after
native and outer elliptic replay. The Rust analyzer accepts both endpoints
with no global problems. It reports comparative eligibility false: these are
two once-only validation observations, without A/A or a rho execution.
[pcc:unverified]

Graph projection reduces488selected rows to366, replays every emitted row,
uses122pivots/1094merges, and records no refusal, fill rejection or fallback
solve. Both methods collect487verified relations at m9 and process704descent
residuals. No relative wall-speed inference follows from this design.
[pcc:unverified]

The existing heuristic capacity model chooses p1009 from the frozen list;
its prediction is not a runtime or guaranteed-yield result. Both arms explicitly
use incidence/shared tables, unlike20. The stage20censor and failed scale gate
remain unchanged. This endpoint criterion permits designing a new separately
frozen comparison. [pcc:unverified]

Exact raw, full original Rustsummary, progress logs and pre/post hashes are
retained here. Original binaries/source checkpoint/host evidence remain under
/workspace/ic-local-runs/local-cover-sieve-validation-20261116-21. Cgroup CPU
snapshots include the environment; no phase share is inferred. Native legacy
arithmetic counters omit graph/wrapper work. Native rho-related fields are
analytic/placeholder diagnostics with an empty rho array, not observed rho
costs. No signed independent, canonical-schema, standard lab or admitted
ranking assertion. [pcc:unverified]

The final one-page PDF was visually inspected. Exact PDF/SVG/150dpiPNG browsing
copies and hashes are under benchmarks/20261116-cover-graph-recovery-validation;
original PNG and SVG copies also remain in the local run directory.
[pcc:unverified]
