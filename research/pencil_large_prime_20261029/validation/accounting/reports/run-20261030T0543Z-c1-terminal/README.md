# C1 terminal observation report

Observations only; no performance ranking or mathematical verdict. `review-packet.pdf` contains the coverage graph followed by the unchanged native four-page report. `report.tex` is editable source; `report.json` binds the native analysis and inputs. The graph is an editable SVG with source counts in `coverage-counts.json`.

Seven of18 scheduled executions were retained, including two pencil timeouts. Four of15 measured executions were retained. No candidate solve or successful candidate/reference pair is established. All36 cost-ratio cells are null, never zero. S2 in the CSV is twice S/M; N10 is ten times the native-event price. The inversion column is I/M. Missing observations, unqualified clocks and unknown active worker/spending totals remain explicit.

Raw checkpoint18 files were published at ed12fa3d15281a81e6e76850618f7e37b0811de9 before this report. Native analysis returned0; canonical packaging and semantic checking returned1 with the retained completeness findings. These failures are expected evidence of an incomplete gate, not suppressed checks. Independent terminal scientific replay and review remain unperformed. Root inspected all five PDF pages for legibility and checked all36 null ratio cells; this is not independent review.

Reproduction uses the pinned native binaries and committed JSON requests in `runs/p186379-gate-20261030-c1/terminal-g18-observations/native-analysis/`. Run `analyze_cohort analysis-manifest.json --p186379-target0`, the canonical exporter with `package-request.json --out export.json`, then `check-export` against the original v4 protocol. Render `report-request.json` into a fresh output directory. Render the coverage SVG with `rsvg-convert -f pdf`, then prepend it to the native report with `pdfunite`. Preserve nonzero export/check findings.

PDFs and raw TeX logs use content-addressed artifact storage; hydrate using the adjacent `.artifacts.json`. Earlier reports and data remain unchanged. The expired campaign authorization is not renewed by reporting.
