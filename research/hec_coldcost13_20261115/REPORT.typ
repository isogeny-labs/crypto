#set page(paper: "a4", margin: 16mm)
#set text(font: "Helvetica", size: 10pt)
#text(size: 21pt, weight: "bold")[Complete cold-cost gate: failed]
#v(6pt)
Genus 3 · same public source pair · 20 valid cells · 15 November 2026

All 20 cells recovered and verified 183,049. The cold quotient-plus-mapped-IC
candidate has median core time 1.780975 s. The fastest preregistered native control,
Compact, has median 0.797166 ms. The derived best-native/cold ratio is
0.00044760096615589, below the threshold 10.

#image("COST.svg", width: 100%)

#table(columns: (1.6fr,1fr,1.4fr,1fr), inset: 5pt,
 [Arm],[Core median ms],[Observed range ms],[Process median ms],
 [Cold candidate],[1780.974708],[1706.934–1938.463],[1784.938625],
 [Compact],[0.797166],[0.435–1.196],[5.186084],
 [Fast],[1.452792],[1.267–2.769],[5.570083],
 [Hybrid],[1.188166],[1.148–2.023],[5.494542])

*Method.* Five sequential alternating blocks; fresh processes and public-input-only
child directories; one worker binary. Core starts before input validation and ends
after scalar verification, including all construction, calibration, scans and solve.
Final certificate/measurement output and startup are excluded from core but included
in the secondary process-wall metric. Algorithm log output within core is charged.

*Interpretation.* Local macOS ARM, L0/unisolated, producer unvalidated. Five timings
of one pair are not five independent curves. Extrema are not confidence intervals.
The observed ratio corresponds to a 2,234.13-fold cold/core cost disadvantage here;
it supplies no universal scaling verdict or admitted tenfold result.

#text(size: 8pt)[Source edf84d571f5b; pre-run checkpoint 20261115T185949Z; record
20261115T1859Z-hec-coldcost13. Raw worker stdout/stderr and cells.tsv retained.
All algorithms verify the same source target; no arm dropped or scientific rerun.]
#pagebreak()
#text(size: 19pt, weight: "bold")[The bottleneck is broader than mapping]
#v(5pt)
#image("PHASES.svg", width: 100%)

*Post-run bound.* Subtracting both maps in each cell before taking the median leaves
240.507208 ms. Keeping only relations, descent and verification leaves 52.211917 ms.
These measured remainders still exceed the native total. Optimizing maps alone
cannot meet the tenfold criterion if the remainder is unchanged. This descriptive
counterfactual was not a preregistered success criterion; analyze.rs records it.

*Decision.* Reject the current cold implementation at this instance. Do not promote
relation-count reductions into a total gain. A successor needs a cost case that
changes construction, transfer and/or destination arithmetic together, or a justified
different admitted regime. The genus 2/3/4 mathematical tenfold objective remains open.

#text(size: 8pt)[Individual phase medians are not additive. Raw per-cell phases and Rust reducer
source retained. No independent VALIDATION.json, leaderboard admission or old-record mutation.]
