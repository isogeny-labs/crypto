# Visual review: KeyAI integration report

**Reviewed:** 2026-11-23 UTC, for the 2026-11-22 America/Los_Angeles integration.
**Status:** final five-page report rendered and visually inspected on every page.
**Scope:** document layout, source presentation, and accurate labeling of the
recorded evidence. This review does not certify mathematical contracts, native
execution, or Lean sources. [pcc:unverified]

## Reviewed artifacts and evidence boundary

The report and diagram incorporate the frozen scoped qualification outcomes:
4/4 example tests, 5/5 separately authored adversarial tests, and 6/6 selected
original support-module tests passed. The full-library release gate exited 101
before any tests because of delimiter errors in an untouched baseline source
file. The report records this unresolved full-library qualification explicitly.
The scoped record `PCC-20261123-08e731` and baseline-gate record
`PCC-20261123-db257a` both have **unverified** standing.
[pcc:PCC-20261123-08e731] [pcc:PCC-20261123-db257a] [pcc:unverified]

Implementation and receipt links use the published immutable snapshot
[`988280cc865c46f0d45cbbfa84fa84ebc05c2852`](https://github.com/isogeny-labs/crypto/tree/988280cc865c46f0d45cbbfa84fa84ebc05c2852).
Upstream source and prior-art links retain their separate original pins. The
Markdown and editable SVG were rendered into the final PDF. Every PDF page was
opened individually as a 100-dpi PNG after the final content changes, and the
standalone SVG render was also inspected. [pcc:unverified]

| Page | Visual and content checks |
|:--|:--|
| 1 | Title, local/UTC date distinction, scoped passes, full-library blocker, unverified standing, support equivalence, input guards, and characteristic counterexample are readable. |
| 2 | Jacobian formula and matrix orientation are legible. The entire diagram fits, including the separate extraction obligation, signed-witness stage, normalized row, independent rank, and anchor/replay endpoint. The recorded fixture is clearly separate from a measured funnel. |
| 3 | Signed/cofactor normalization, fixed nonzero-target assumption, target determination, and declared Prime31/F13 fixture inputs fit without clipping. Equations remain distinguishable from prose. |
| 4 | Frozen-execution scope, all three test counts, the 2,830-case comparison, fixture-accounting table, anchor recovery, and preserved first-attempt wrapper failure are readable. |
| 5 | Full-library failure table and baseline identity, unverified PCC records, immutable implementation and evidence links, original source pins, RR9/useful-rank attribution, and explicit KB gap fit on the final page. |

No clipped text, missing glyphs, overlapping content blocks, or orphaned
near-empty pages were observed in the final render. The PDF has five pages and
30 clickable link annotations; all link destinations are HTTPS URLs. Page
numbers, running labels, source references, and the final paragraph were
checked. [pcc:unverified]

## Rendering path

The initial Pandoc/pdflatex attempt failed because `pdflatex.fmt` was missing.
The installed Tectonic fallback failed because its bundle was not cached and
the download returned HTTP 403. Neither layout failure is a scientific test
outcome.

The successful layout path used Pandoc's Markdown AST, ReportLab for paragraphs,
tables, and links, Matplotlib for mathematical typography, Inkscape for SVG to
PDF conversion, and PyMuPDF to place the vector figure. DejaVu Sans and Mono
fonts are embedded. Display equations and the figure remain vector graphics;
inline math is rendered at 360 dpi. Python was used for document layout and
inspection only, without scientific calculations or a native-test substitute.

| Tool | Version |
|:--|:--|
| Pandoc | 3.1.11.1 |
| Inkscape | 1.4 |
| ReportLab | 4.4.9 |
| Matplotlib | 3.10.8 |
| PyMuPDF | 1.26.6 |
| Pillow | 13.0.0 |

The report's Markdown and SVG remain the editable source artifacts. The README
includes a content-rebuild recipe for an environment with a complete LaTeX
installation; that route may paginate differently from this renderer.

## Reviewed hashes

| Artifact | Bytes | SHA-256 |
|:--|--:|:--|
| README.md | 6226 | `930baf12a82ddec2e373eabb051cfbe68d609e0ed221c439198938fe371a82cd` |
| REPORT.md | 19161 | `39f2796b814832473744956868bbae3842dd1e685c3bfe68ac45f4901c52def0` |
| ACCOUNTING.svg | 5318 | `f31e7260207033a7a291d7f6d333775e77a3cc01018ce4a93c6a5ca7c3e8b079` |
| REPORT.pdf | 350664 | `617ffd4b4045044b30164c053af96a1e3368fe9c6c04ca4a4e4f39e77bb7b016` |

The diagram is a protocol illustration with recorded correctness-fixture
counts, not a performance funnel or cost series. Canonical performance and
leaderboard charts are unchanged. Any subsequent change to the report or
diagram requires regenerating the PDF, reviewing its pages, and refreshing
these hashes together.
