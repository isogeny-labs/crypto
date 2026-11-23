# Censored cold comparison of cover IC, graph IC and rho

[Report](report.pdf) · [Full Rust summary](summary.json) · [Exact compact flow](flow.jsonl)
· [Frozen protocol](../AMENDMENT_17_COVER_COMPARISON.md) · [Custody](local-evidence.json)

The sole call on source23b4c1a70 ends at3600.002seconds, exit124. Of36planned
executions,26start and25complete:6warmups and19measured executions. Three of
five measured rounds complete correctly. Measurement3/graph_b is in flight,
with10executions unattempted. Every completed solve recovers the held-out
scalar after native and outer group replay. [pcc:unverified]

All eight completed graph arms project516rows to387, replay387witnesses and
use zero fallback solves. The analyzer's full-schedule projection flag is false
because planned arms are missing. It is not an observed projection failure.
The full frozen follow-up condition fails; no unchanged retry or retrospective
budget extension. [pcc:unverified]

Partial descriptive outer means from three complete rounds are graph/IC1.01269,
graph/rho0.05724 and IC/rho0.05652. Every confidence interval is null and the
comparison is ineligible. Observed both-clock A/A medians meet the numeric
bounds, but the required round schedule remains incomplete. These values are
not a controlled gain or parity verdict. Full-precision values and inner-clock
results remain in the original13765-byte native summary. [pcc:unverified]

Original350106-byte raw, copied analyzer/binary, source checkpoint, host facts,
all progress and pre/post hashes remain under
/workspace/ic-local-runs/local-cover-comparison-20261116-22. Raw exceeds64KiB
and is retained in the explicitly authorized local custody; the exact compact
flow and complete summary are in Git. All pre/post hashes rechecked. The last
recorded progress of the censored graph arm is descent residuals3648; exact
cut phase and partial solve wall are unknown. [pcc:unverified]

Shell CPU totals omit the unreaped descendant at censor; they are not complete
utilization. Cgroup snapshots are inclusive environment diagnostics. Legacy
primitive counters omit graph/wrapper costs, and native IC rho-related fields
are analytic/placeholder inputs; the descriptive pair values use actual rho
workers. No signed independent, canonical-schema, strongest-external, standard
lab or admitted ranking assertion. [pcc:unverified]

The original analyzer plot is retained as ratios-original.svg. Its labels were
clipped. The presentation-only Rust/Plotters renderer in presentation/ consumes
the unchanged full summary and produces the readable ratios.svg; it performs
no statistical recomputation. Its release build and warnings-denied Clippy
pass. Original template and failed render log remain local; a reserved Typst
helper name was corrected before final rendering. Final numerical summary is
byte-identical to the original copied analyzer output. [pcc:unverified]

Both final PDF pages were visually inspected. Exact PDF/SVG/150dpiPNG browsing
copies and checked hashes are under benchmarks/20261116-cover-ic-controlled-censor.
Original final PNG/SVG/PDF copies also remain in the local run directory.
[pcc:unverified]
