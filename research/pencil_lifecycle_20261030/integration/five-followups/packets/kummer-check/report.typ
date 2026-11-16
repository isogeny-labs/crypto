#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
#image("coverage.svg",width:100%)

= Kummer branch-directed software equivalence

#text("Fixed-fixture validation, 2026-10-30. No scientific solve or benchmark. All 84 adjacent-transition rows across 21 retained models matched the selected pair, branch choice, updated labels, fallback flag/check count and transition-validation result. This comprises 63 ordinary transitions and 21 forced-zero-difference fallbacks. Each ordinary step invokes the unchanged source's signed expected-pair validation. Forced fallback invokes its unchanged signed reconstruction and validity checks; this harness compares its outputs between versions rather than independently reimplementing that reconstruction. The 21 forced cases traverse unsuccessful repair attempts before signed fallback. No successful geometric repair was observed; this coverage remains open.")


#text("A separate 54-case dispatcher control enumerates two branches and three possible outcomes for each of three calls, including None and repair-marker outcomes. It checks Option selection logic only; it does not establish geometric repaired-coordinate correctness.")


== Accounting observations

#text("For every ordinary transition the pruned source records 24 fewer multiplications, 36 fewer additions and 4 fewer inversions in the walk counters. For each forced fallback it records 304 fewer multiplications, 376 fewer additions and 24 fewer inversions, because the discarded double and its failed repair attempts are skipped. These are observed counter differences on fixed software fixtures, not measured time or complete collector costs. Signed reconstruction/fallback output matches. Both arms retain identical signed recovery code. Source operation counters do not charge every multiplication inside modular exponentiation as a multiplication; therefore these are native counter values, not an independent field-operation census.")


== Custody and failures

#text("Before execution PLAN.md fixed the fixture selection, assertion fields, scope and120s command bound. Source snapshots retain the unchanged historical N19 collector and a copy differing only in step pruning; wrappers run transitions, never collector solve loops. Theta data was recovered privately from the tracked Git blob. One authorized artifact GET retrieved generic-models.jsonl;483118 bytes and SHA2560e71f5493f217a1524104cd8706208355f1730ce76a584ccf69d82eda89ba469 matched before use. Fixtures and source files were hashed before execution.")


#text("The first bounded build exited101 at a wrapper Map key type mismatch, before any checks ran. AMENDMENT-1.md fixes only conversion of that key to String. The second offline locked build and all checks exited0. Both receipts and stderr are preserved. No original source, frozen result, cloud job or deployment was changed.")


== Interpretation and next checks

#text("This finite check supports the selected-pair/fallback equivalence for the specified fixtures. It does not prove all arithmetic cases, subgroup input guarantees, successful geometric repair coverage, production compilation correspondence, or a full collector cost ratio. Before scientific admission, add fixed independently witnessed successful repair cases and a new candidate method identity with explicit unexecuted repair-slot telemetry. Preserve the existing N19 cohort and keep any later cohort prospective.")

