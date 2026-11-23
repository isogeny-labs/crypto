# Iteration 6 report rendering

Date: 22 November 2026, America/Los_Angeles. This is a seven-page report
of the oriented quadratic S3/Dickson representation and the iteration-seven
growing-family and rational-fibre availability refinements. It does not revise
the earlier eight-page report or its figures.

## Deliverables

- `REPORT.md`: human-facing account with source map and exact evidence scope.
- `REPORT.pdf`: seven US Letter pages; embedded Liberation Sans subsets,
  native text, vector equations and a vector topology diagram.
- `render_report.py`: editable ReportLab layout source. It reads the current
  `report-validation.json` before rendering evidence status.
- `figures/rational-chain-topology.svg` and `.png`: the same multiset in
  a finite left chain and a balanced tree with a cancelling child.
- `render-receipt.json`: exact artifact sizes/hashes, software versions,
  render commands, validation status and visual-review record.
- `report-validation.json`: evidence-status input. It retains the first-run
  inherited root syntax failure and records the source-bound restored-build
  status without treating prepared controls as executed observations.

Each produced artifact is below 64 KiB. Page-by-page raster QA is retained
outside the published research directory under `tmp/pdfs/prime31-iteration6/`.

## Reproduce

From the repository root:

```sh
python3 research/prime31_summation_compression_20261122/iteration6/render_report.py
```

Render the resulting PDF with Poppler for visual inspection:

```sh
pdftoppm -r 110 -png REPORT.pdf /an/existing/qa-folder/page
```

The script requires ReportLab, pypdf, PyMuPDF, Poppler's `pdftoppm`, and
Liberation Sans regular/bold at `/usr/share/fonts/truetype/liberation/`.
Installed versions are written in the receipt. It performs document layout,
figure drawing and artifact hashing only. It runs no field, elliptic-curve,
resultant, solver or benchmark computations.

The mathematical source `MATHEMATICAL_NOTE.md` remains byte-for-byte identical
to the pinned note, SHA-256
`948d36ca3d72661aa4cf16786ebed30fcb2ba146afb0df4420767a91e8f35ddc`.
The independent review and other attestation-pinned sources are also read
only. The PDF is a curated presentation of those sources, not a proof kernel.

## Visual review

All seven pages and the standalone figure have been inspected after raster
rendering. The report has readable embedded fonts, separated superscripts
and subscripts, complete equation/table lines, intact page margins and
consistent page numbers. The early draft's unsupported set-membership glyph
was replaced by a readable word in the displayed sum. Later changes retained
the exact equations and added the model-specific m=4 query budget.

User-facing dates use 22 November 2026, America/Los_Angeles. Internal UTC
dates in the pinned mathematical sources are preserved. The current report
retains the initial original-root-lock failure before the new tests executed,
and marks the restored-source rerun pending until its actual receipt is read.
Native cubic-residual issuance and replay remain pending; prepared controls
are not reported as observations.
PCC tracking records remain unverified.

## Updating evidence after CI

Read the actual run/job log and retained receipt before changing status.
Record the checked-out source, source hashes, test counts, original-lock
commands and identity issuance/replay at their actual scope. Update both
`REPORT.md` and `report-validation.json`, then rerun the renderer. Reinspect
the changed pages and confirm that the output is still seven pages and below
64 KiB. The script deliberately resets `visual_check` to pending on every
render; only the reviewer may replace it after inspecting that PDF hash.

Do not upgrade the mathematical standing or add a speedup claim merely
because producer correctness CI passes. An evaluated identity with
`proof:null` does not formally certify the tree or completeness theorem.

## Growing-family appendix

The seventh page distinguishes selecting a curve for a fixed symmetric fibre
from selecting a fibre for an already specified odd-order curve. It gives the
Linnik smooth-p+1 construction, the capped power-of-two fibre size, the exact
regular-fibre sampling scope and the written factor-base setup cost. Proper
subgroup membership, a growing prime-order curve family, relation rank and
target-dependent root solving remain separate obligations. Small-order leaf
exclusions on composite-order curves require additional membership guards,
which are outside the frozen iteration-six counts.

The appendix is a presentation of reviewed written deductions. It adds no
finite-field computation, measurement or performance claim to the rendering
script. The full mathematical proofs live in the linked iteration-seven
notes; their claim standing remains unverified.
