# PDF build status

`report.tex` is the reproducible report source. An offline build was attempted
with `tectonic --only-cached -o . report.tex`, which guarantees that the
compiler will not download resources. The local cache is incomplete: Tectonic
stops because `tuenc.def` is unavailable. The app LaTeX compiler did not return
diagnostics during this run, so it does not establish a successful compile or
produce an exportable PDF.

This historical TeX blocker was resolved on 2026-10-20 by writing the
equivalent [Typst source](report.typ) and compiling its
[one-page PDF](report.pdf) with the installed Typst wheel. The page was
opened and visually inspected; the SVG graph, labels, ratio values and
source links are legible. The TeX source is retained, and its offline build
remains blocked by the missing `tuenc.def` cache file.
