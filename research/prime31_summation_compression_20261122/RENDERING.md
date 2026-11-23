# Report rendering and visual verification

This is the editable rendering record for the eight-page `REPORT.pdf`. The report covers the first two iterations: the exact Dickson/S3 resultant component, its retained negative operation diagnostic, and the full-image bidegree theorem. All mathematical and computational standing remains `pcc:unverified`.

## Deliverables

- `REPORT.pdf`: 8 US Letter pages, embedded readable font subsets, vector equations and chart, and linked references.
- `render_report.py`: ReportLab layout, recorded-data figure creation, and lossless PDF stream compression.
- `figures/dickson-resultant-flow.svg` and `.png`: exact flow from composed membership to signed group replay.
- `figures/kernel-counts.svg` and `.png`: the retained regular depth-six diagnostic, with operation units and both comparison arms.
- `figures/kernel-counts.json`: chart values copied from the retained native JSON, with its SHA-256.
- `render-receipt.json`: output sizes, hashes, versions, commands, page count, and visual-check record.

Every listed deliverable is smaller than 64 KiB. Keep page-by-page raster QA and runtime caches outside the published research folder.

## Reproduce

From the workspace root:

```sh
python3 repo-audit/research/prime31_summation_compression_20261122/render_report.py
mkdir -p pdf-qa-prime31
pdftoppm -r 110 -png repo-audit/research/prime31_summation_compression_20261122/REPORT.pdf pdf-qa-prime31/page
```

The renderer needs ReportLab, Matplotlib, pypdf, PyMuPDF, Poppler's `pdftoppm`, and Liberation Sans regular/bold TTF files at `/usr/share/fonts/truetype/liberation/`. Exact installed package versions are in the receipt. PDF creation timestamps may change the file hash between otherwise identical renders.

Python is used only for document layout, plotting existing recorded counts, and artifact verification. It does not run field, elliptic-curve, resultant, or benchmark computations. The report is a curated snapshot of the supplied research prose; it does not automatically incorporate future mathematical changes.

## Visual QA

All eight pages were rendered and visually inspected. The final report has legible native text and exponents, adequate formula spacing, aligned tables, complete chart labels, and unobstructed page margins. Both exported figure PNGs were also inspected. A standard-font substitution problem in an early draft was corrected by embedding font subsets; the final file was compressed without rasterizing its text or equations.

The PDF preserves the source qualifications: the stronger sparse-product control wins the recorded scalar-operation diagnostic; kernel counts omit setup and extraction; development tests used a separate cached-dependency adapter; identity records and in-session review do not provide a signed PCC verdict. No novelty or asymptotic speedup is asserted.
