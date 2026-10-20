# PDF build status

`report.tex` is the reproducible report source. An offline build was attempted
with `tectonic --only-cached -o . report.tex`, which guarantees that the
compiler will not download resources. The local cache is incomplete: Tectonic
stops because `tuenc.def` is unavailable. The app LaTeX compiler did not return
diagnostics during this run, so it does not establish a successful compile or
produce an exportable PDF.

No PDF is included. The TeX source, SVG graph, 1200 × 760 PNG graph with 150 dpi
metadata, and frozen data copies are present. Rebuild the PDF in an environment
with a complete local LaTeX bundle, then verify the rendered pages before
adding a PDF artifact.
