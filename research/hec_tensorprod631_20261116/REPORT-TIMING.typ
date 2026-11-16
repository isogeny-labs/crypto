#set page(paper:"a4",margin:20mm)
#set text(size:10pt)
= Compiled maps: stability gate failed
Genus 3 · 15 November 2026 · Unvalidated producer observations

#image("COST.svg",width:100%)

#table(columns:(1fr,1fr,1fr),[Label],[Median component],[Observed max/min],[Baseline A],[1.478167 s],[1.425993: fails1.25],[Baseline B],[1.462773 s],[1.015255: passes],[Compiled A],[1.290368 s],[1.103625: passes],[Compiled B],[1.287459 s],[1.022476: passes])

All twenty cells verify both original classes. A/A median ratios pass: baseline1.010523, compiled1.002260. The first baseline-A cell nevertheless violates the frozen spread threshold and remains included. Overall stability fails. The observed primary ratio1.145538339604 is below10and is not a qualified stable speedup.

== Diagnostic evidence changes the next action
Compiled-A median stages: two generators about272.8and267.5ms; two solvers118.3and118.7ms; two verifiers24.8and23.3ms; reconstruction448.8ms. These are disjoint process stages, but medians are not additive.

Inside reconstruction, inclusive group-add time is332.0ms and section-coordinate time4.3ms. These are nested diagnostics and must not be added to enclosing stages. Construction and dual-map work remain charged.

Post-run bounds over all cells, holding measured remainder fixed: free recorded section scopes yield at most1.0035times total improvement; even generously zeroing group-add, group-neg and section scopes yields at most1.887times. Overlapping group time is double-counted in the generous bound. These are descriptive counterfactuals, not generic theorems or preregistered gates.

== Decision and custody
Stop treating section microtuning as a plausible tenfold route on this path. The next mechanism must change repeated equation/setup construction and inversion more substantially, or justify another admitted regime.

Source0911e248f439 and checkpoint20261116T035102Z precede one cohort. A mistyped record-name launcher and a postprocessing parser error occurred outside scientific execution; both are documented. No cell rerun or deletion. Raw data and phases are archived in S3 with record20261116T0351Z-hec-tensor631.

Shared ARM host, one pair repeated, no independent validation. Common quotient/public coordinates and final descent are excluded from this component comparison. Earlier cohorts remain separate. Full cold-path, genus-two/four and the mathematical tenfold objective remain open.
