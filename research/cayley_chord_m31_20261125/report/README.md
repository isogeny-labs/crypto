# Publication report

[REPORT.pdf](REPORT.pdf) is a three-page presentation of the frozen M31 component evidence and the written product-label counting argument. Scientific claims retain `[pcc:unverified]` standing; this presentation supplies no signed PCC verdict, Lean proof or `identity.certificate/v1` record. [pcc:unverified]

## Files

| File | Role |
| --- | --- |
| [render_report.py](render_report.py) | Editable layout source; reads recorded evidence and checks its SHA-256 |
| [REPORT.pdf](REPORT.pdf) | Source-linked report, with the two figures embedded as vectors |
| [extraction-flow.svg](extraction-flow.svg) | Signed chord extraction diagram |
| [recorded-case-counts.svg](recorded-case-counts.svg) | Composition of the recorded correctness panel |

The figures describe the extraction stages and exact recorded case counts. They contain no timing comparison, performance projection or independent-relation estimate. The PDF distinguishes the uniform-target, target-independent-label counting bound from a runtime lower bound. [pcc:unverified]

## Reproduce the presentation

Use Python 3, ReportLab and the DejaVu Sans regular/bold fonts at `/usr/share/fonts/truetype/dejavu/`. The publication render used ReportLab 4.4.9. Run from this directory:

```sh
python3 -B render_report.py
```

The input defaults to `../evidence/result-run1.json`. An explicit path can be supplied with `--summary`. The renderer refuses an input whose SHA-256 differs from:

```text
a4f1782166fd21ae3ef9211fd4158601940d4884a2450495c81e8dd95eb23ae9
```

The source fixes the default PDF metadata epoch to 2026-11-26 00:00:00 UTC. Set `SOURCE_DATE_EPOCH` explicitly if a different publication build epoch is needed. Byte-level reproduction also depends on the ReportLab and font versions; this is a presentation build, not a scientific replay.

The renderer performs no finite-field research computation, benchmark, solver run or replay. It lays out the frozen summary, curated proof narrative and source references. The scientific files remain separate from these presentation outputs. [pcc:unverified]

## Visual review

All three final pages were rasterized with Poppler and inspected for clipped text, overlaps, missing glyphs, legible formulas, source links and table alignment. The report is three US Letter pages; the PDF and both SVGs are each below 64 KiB. Raster review images are disposable build products and are not included.

```sh
pdftoppm -r 110 -png REPORT.pdf /tmp/cayley-report-page
pdfinfo REPORT.pdf
```

The parent [study README](../README.md), [protocol](../PROTOCOL.md), [review](../REVIEW.md), [source record](../SOURCE.json) and [frozen summary](../evidence/result-run1.json) define the research scope and qualifications. PR [#5232](https://github.com/isogeny-labs/crypto/pull/5232) contains the publication package. No new scientific sample was generated for this report. [pcc:unverified]
