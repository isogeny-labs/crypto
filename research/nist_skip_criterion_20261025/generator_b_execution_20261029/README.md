# Verified generator/coefficient comparison — 29 October 2026

The actual 21-curve comparison is complete. Read [REPORT.md](REPORT.md),
[the PDF](REPORT.pdf), [verified counts](results/verification.json) and
[reproduction instructions](REPRODUCE.md). This additive execution record
supersedes the earlier preparation's blocked/unrun status without changing
its frozen files.

All 100,548 declared point-trait cells, 19,152 fixed envelopes, 147,875
same-field difference cells and 880 public-scalar checks passed the separate
native verifier. No exact single envelope separates all labels; no tested
scalar recipe matches. The canonical P-224 x values are 0, 1, 1 for the
omissions and 3 for the publication. That field-specific pattern retains
eight other omissions and supplies no historical explanation or attack.

All four previously unresolved factorizations remain proved complete in the
[84-record follow-up](../factor_followup_20261026/REPORT.md). Eleven historical
skipped generators remain NA. Both weakness hypotheses and the historical
skip criterion remain unresolved; no DLP was attempted.

The successful [version-2 export](export.json) passes the canonical tool with
file verification. The [version-1 partial export](export-v1.partial.json)
retains the timeout and absent verification; its four checker findings are
preserved rather than replaced by an invented execution row. See the
[admission record](evidence/export-admission.json).

Run reports: [attempt 001](reports/attempt-001/REPORT.md),
[attempt 002](reports/attempt-002/REPORT.md),
[resource diagnostics](reports/diagnostics/REPORT.md),
[synthetic planning observations](reports/preflight/REPORT.md).
The original producer/verifier binaries and prover were unchanged between
the two attempts. Only the uniform virtual-address ceiling changed.

Raw data are in `evidence/full-raw-002.tar.xz`, retrieved through
`tools/artifacts`; individual `runs/` files are ignored and hydrated for the
canonical check. Full remote GET and all archive-member checks passed:
[delivery receipt](evidence/final-native-delivery.json). Do not commit an
extracted raw tree. The source and input binding in the prepared sibling
directory remains immutable.

The [postprocess delivery](evidence/postprocess-delivery.json) verifies a
separate 33-member archive of metadata-export builds, every document-render
attempt, and all eight final page rasters/readbacks. The
[PDF inspection record](evidence/pdf-admission.json) binds the five final PDFs.
