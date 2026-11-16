# Actual first-gate observations

Snapshot report, not a performance verdict. This reports the completed 60-target
fresh epoch and separate computation replay. It preserves the failed measurement
acceptance and initial missing independent finite-coverage input.

Reproduce the PDF with `pdflatex -interaction=nonstopmode -halt-on-error report.tex`.
`coverage.svg` is editable vector source; `report.tex` contains the same count plot
in TikZ. `observations.csv` is extracted from the native-ingestion count receipt,
not a new trial. The PDF was visually inspected as one page with no clipping.

Evidence: `../../../actual_gate_results_20261030/` contains ingestion, initial
analysis, computation audit, controller terminal, per-arm counts and versioned
raw locators. `evidence.sha256` binds the exact companion receipts. These paths
are relative to this directory. Raw versions remain in their original S3 bucket;
no experiment is repeated to obtain them.

The initial 36 null prices are not a claim that counting is impossible. They record
the coverage-review admission state at this snapshot. Any subsequent price review
must be a separately retained output, with the metadata erratum disclosed.
