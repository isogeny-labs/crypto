# Five parallel mathematical followups

This packet records component derivations, source-fidelity reviews and software checks. Reports preserve their original review-time status; later independent reviews are linked separately. No cohort timings or cost ratios are inferred.

## word-correspondence

Five UInt64 multiplication/squaring/canonical-output lemmas compiled and independently replayed under explicit canonicality and modulus bounds.

[PDF](packets/word/report.pdf) · [Editable graph](packets/word/diagram.svg) · [Independent review](packets/word-review/REVIEW.md)

Next: Complete classifier correspondence; existing classifier attempts fail at missing field-instance dependency.

## collector-invariants

Weight/decode roundtrip, Mumford divisibility preservation and negation preservation compiled and independently reviewed.

[PDF](packets/invariants/report.pdf) · [Editable graph](packets/invariants/diagram.svg) · [Independent review](packets/invariants-review/REVIEW.md)

Next: Bind remaining collector state transitions to their source-level invariants.

## exceptional-branches

Distinct-support guard excludes shared_zero_q_delta over a field; 12 branch entries mapped.

[PDF](packets/invariants/report.pdf) · [Editable graph](packets/invariants/diagram.svg) · [Independent review](packets/invariants-review/REVIEW.md)

Next: Complete the remaining branch coverage and machine classifier bridge.

## kummer-transitions

Adjacent-pair transition and branch-directed pruning proposal documented; missing r-torsion and canonical word-label hypotheses corrected additively.

[PDF](packets/kummer-v2/report.pdf) · [Editable graph](packets/kummer-v2/closure.svg) · [Independent review](packets/kummer-review/REVIEW.md)

Next: Bounded software equivalence check in progress; subgroup validation remains a caller obligation.

## target-information

Offline diagnostic implements consistency plus rank([A b])-rank(A)=1; six producer tests and independent 793-matrix comparison pass.

[PDF](rank-information/report/report.pdf) · [Editable graph](rank-information/report/information.svg) · [Independent review](packets/information-review/REVIEW.md)

Next: Bind actual retained relation streams and scalar sign convention before instrumenting a prospective policy comparison.

## Custody and reproduction

Each imported packet has a custody-map.json linking original bytes and names to retained files. Archived Lean sources use .lean.txt to keep failed candidates out of project discovery; the three reviewed modules are integrated in ../lean/same-support/. Compiled objects and render intermediates are omitted and identified by hash. Original producer manifests are retained, so their full private-directory file sets differ from the curated publication sets. Native commands and receipts retain original absolute paths.

The rank diagnostic reuses residual::Basis and changes thread-local counters, so it must run outside measured collection. Its scalar convention is Ax+bk+c=0; a future scalar exporter must explicitly map the existing collector signed-label convention. The Kummer patch is a proposal, not an applied runtime change.

Original failures, the Kummer subgroup counterexample, review blinding limitations, and missing obligations remain in the packets. The original 1,260-target requirements remain unchanged.

## Subsequent check receipts

The [Kummer software check](packets/kummer-check/REPORT.md) records 84 matching transitions across 21 models, including 21 forced fallbacks and zero observed successful geometric repairs. [PDF](packets/kummer-check/report.pdf) · [coverage graph](packets/kummer-check/coverage.svg). These are fixed software fixtures. The counter differences are recorded in summary.json; elapsed-time or cohort ratios were not measured.

[Integrated Lean validation](packets/integrated-lean-review/VALIDATION.json) records fresh compilation of all eight modules, separate readback of 16 declarations and Lake configuration elaboration using existing pinned dependencies. Failed minimal-mirror attempts remain recorded.

[Independent Kummer software-check review](packets/kummer-check-review/REVIEW.md) replayed the exact 84 rows with byte-identical output. It retains the successful-repair coverage gap and notes that the 54-case Option-selection control does not validate skipped-operation telemetry. A future runtime patch needs an explicit execution mask to distinguish not-evaluated from no-repair.
