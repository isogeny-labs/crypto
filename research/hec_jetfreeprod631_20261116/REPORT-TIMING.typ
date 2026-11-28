#set page(paper:"a4",margin:20mm)
#set text(size:10.5pt)
= Production shortcut: timing gate failed
Genus 3 · 15 November 2026 · Unvalidated producer evidence

#image("COST.svg",width:100%)

#table(columns:(1fr,1fr,1fr),[Arm],[Median component],[Observed min–max],[Previous direct],[5.887982 s],[2.295272–8.760866 s],[Production shortcut],[6.329883 s],[2.136444–9.979136 s])

All ten cells pass and reproduce both original target classes. Derived median ratio0.930188083126; registered tenfold gate fails. This noisy cohort establishes no reliable performance benefit.

== Run-order drift
#table(columns:(1fr,1fr,1fr),[Block],[Previous direct seconds],[Production seconds],[0],[2.295272],[2.136444],[1],[5.887982],[3.443370],[2],[5.718108],[6.329883],[3],[6.693868],[7.545312],[4],[8.760866],[9.979136])

Both arms slow substantially during the shared-host run. Alternating order does not eliminate this uncertainty. Do not interpret the observed ratio as a stable regression, and do not replace the cohort with a quiet rerun. Five repetitions of one pair are not five curves; extrema are not confidence intervals.

== Accounting
Only reconstruction executable changes. Both arms charge direct generation, actual solver work, exact verification, reconstruction/adjacency and final class comparison. Common quotient setup/public-coordinate preparation and conic descent remain excluded. No cold end-to-end gain is measured. The earlier1.621xcohort is separate and is not pooled or multiplied with this ratio.

== Custody and decision
Source de0bddf85eea and checkpoint20261116T033149Z precede one frozen cohort. All raw outputs/statuses are archived in S3 with bench.record20261116T0331Z-hec-jetfree631. Rust computes the statistics. Shared ARM host, no independent validation.

Retain the algebraically justified shortcut and its correctness evidence, but make no new speedup claim. Before another performance claim, use stable same-host controls and finer charged phase attribution under a new protocol. Genus-two/four and the full tenfold objective remain open.
