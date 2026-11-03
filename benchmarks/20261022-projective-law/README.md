# Projective genus-2 law: copied visual record

The [study](../../research/projective_law_followup_20261022/REPORT.md) is the
source of truth. Its two collector arms each solved and source-verified all
1,260 frozen targets, with same-process source-negation rho. At
`M + 0.5S + 10I` and 0.1 GAE per native logical operation, the wrapper costs
3.793783× rho and retained state costs 4.179297× rho. The direct
retained-state/wrapper ratio is 1.101617; it is below one in 12 of 36 pooled
price cells and above one in 24. The operation model is cold and complete;
isolated wall time and a fastest-external-tool runtime comparison were not
measured. In the same modeled unit, wrapper/external is 1.549223 and
retained-state/external is 1.706651 against the original wrapped affine-law
collector.

The copied [study results PDF](results-report.pdf), [run PDF](run-report.pdf),
and [cross-lead review PDF](cross-lead-report.pdf) retain their source reports.
The run PDF is the original 05:47 UTC evidence snapshot; the study results PDF
includes the later [proof audit](../../research/projective_law_followup_20261022/PROOF_AUDIT.md).
Every graph in those PDFs has an SVG copy here, including
[cost ratios](cost-ratios.svg), [direct price grid](direct-price-grid.svg),
[branch coverage](branch-coverage.svg), and the review packet's
[cross-lead](review-cost-ratios.svg), [pencil prices](review-pencil-ratios.svg),
and [pencil coverage](review-pencil-coverage.svg) views. Each SVG has a PNG rendered from that copied SVG with
`rsvg-convert -d 150 -p 150`; [COPY_MANIFEST.json](COPY_MANIFEST.json) pins
source and copy hashes. The source studies retain their own original PNGs.

The [local validation gate](VALIDATION.json) pins the study's original replay
certificate, independent validator report, and matched-rho price bands. Its
`results` symlink resolves the validator's raw-data paths back to the study;
it does not duplicate raw observations. Check with
`python3 scripts/validation_gate.py evidence benchmarks/20261022-projective-law`.
