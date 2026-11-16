#set page(paper: "a4", margin: 15mm)
#set text(font: "Helvetica", size: 9pt, fill: rgb("233746"))
#set par(leading: .55em)
#set heading(numbering: none)
#text(size: 21pt, weight: "bold")[Point sieve: complete cold comparison]

Genus 2–4 · 18 targets · 306 timed cells · 18 full gates · 15 November 2026

*Exactness passes; the registered 10× cost criterion fails.* All 18 targets recover correctly.
Dense/lazy/A/A stopping signatures match. Every fixture median favors an existing control.

#image("POINT-COLD.svg", width: 100%)

#table(columns: (1fr,1.2fr,1.3fr,1.3fr,1.3fr),align:right,
 [Genus / prime],[Best / lazy],[Dense / lazy],[A/A max],[Setup share],
 [2 / 1,009],[0.168×],[0.741×],[1.306×],[56.4%],
 [2 / 16,381],[0.297×],[1.078×],[1.414×],[32.7%],
 [3 / 127],[0.066×],[0.410×],[2.923×],[74.4%],
 [3 / 1,009],[0.430×],[0.758×],[1.916×],[39.4%],
 [4 / 31],[0.249×],[0.995×],[2.189×],[71.8%],
 [4 / 127],[0.192×],[0.760×],[1.204×],[65.7%])

*Workload.* Six reused family-0 fixtures, new target seed indices 11–13; three targets per fixture.
Primary ratio selects the fastest complete same-target result among 14 existing controls:
12 native/table choices, compact and generic rational. Point-lazy is fixed as primary;
point-dense and point-lazy-aa are secondary. Each child is independently cold.

*Accounting.* Point clocks include catalogue, allocations, 16 public jumps, walk, full last
sieve batch including unused tails, orientation/reconstruction, deduplication, forest, algebra
and final group verification. Common fixture validation and known-answer construction are
excluded equally. Extra harness group checks are outside method clocks. No pooled targets.

*Correctness and caps.* Gates check raw principal identities, public labels and dense/lazy
stopping-state equality. Timed scalar checks pass in all cells. The fixed cap is 128 batches;
no target caps in this cohort. The earlier seed-10 genus-4 cap remains preserved separately.

*Interpretation.* L0 sequential local ARM measurements; maximum A/A spread 2.923× limits
small-ratio interpretation. These are observed three-target medians, without confidence intervals.
The point sieve is established prior art. Lazy retained-storage savings do not imply runtime gains.

*Post-run diagnostic.* With every other observed cost fixed, making setup free yields at most
2.105× against the measured control across targets. Making setup and forest free yields at most
2.190×. Conditional diagnostics are not measured gains or universal impossibility results.

*Custody.* Source cd441f8d6601881ec42dfcf0a51ea3363d5e7dab; protocol 1a89594e09.
Native Rust execution / collection, native jq reduction, Rust SVG, Typst PDF.
Record 20261115T1055Z-hec-point-cold; full outputs/receipts and hashes retained.
Independent controlled replay, external comparison and mathematical novelty remain outstanding.
