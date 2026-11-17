# Read-only verification

From the repository root, compare each source-pins.json entry against the SHA-256 of its named file. source-comparison.json records the seventeen historical/current joins; historical snapshots live under research/pencil_large_prime_20261029/validation/finite_coverage_run_binding_20261030/source_snapshot. A later source change requires a new review rather than updating these pins.

manifest.json binds every retained packet member except itself. Raw unified diffs are base64 encoded; decode with `base64 -d diffs/progress-vs-historical.diff.b64` from this packet. This preserves original diff bytes and context whitespace. TeX auxiliary logs use the same encoding. The PDF is a rendering of REPORT.md; categories.csv and review.json carry the detailed source matrix. Rendering needs rsvg-convert and pdflatex but no worker or scientific payload.

This packet deliberately uses a source-review schema, not pencil coverage admission. Executable, protocol and image fields are null; do not rename the status to satisfy analyzer admission. Root publication confirmation is recorded in the report. No numerical timing or ratio was recomputed here.
