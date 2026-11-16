# Document checks

This is producer document/development checking only. [pcc:unverified]

- Markdown and retrieval prose lint: zero findings after correcting four
  paragraphs that triggered the strong-word linter. No claim standing inferred.
- Diagram: generated with Graphviz from `pipeline.dot`. An initial reserved
  node name caused a syntax error; renamed before accepting SVG output.
- PDF: generated from all of `REPORT.md` plus the SVG with Typst 0.15.1.
  Rasterized and visually read all five pages: text, source pins, diagram
  labels and references are legible and within the page. No old PDF reused.
- `render.py` only lays out recorded prose and the static visual. It performs
  no field arithmetic, solver, simulation, benchmark or result analysis.
- Native source inputs remain pinned to the feature snapshots in REPORT.md.
  No native source was edited or requalified by this document-only task.
- `benchrec.py gate` was attempted. First, sparse metadata lacked the
  existing integrity-exclusions file; the raw refusal is retained in
  `checks/benchrec-without-exclusions.txt`. After materializing current
  records, progress and that exclusions file, the gate still could not
  qualify this sparse checkout: historical raw/report directories are not
  materialized. Its complete output is losslessly retained in
  `checks/benchrec-sparse-checkout.txt.xz`. These are not hosted failures or
  scientific outcomes; no unrelated record was edited to obtain a pass.
  Full-checkout hosted records qualification remains pending.

Rebuild the documents (using a scratch environment, not a scientific runner):

```sh
dot -Tsvg pipeline.dot -o pipeline.svg
# In a scratch virtualenv with mistune and typst installed:
python render.py
pdftoppm -scale-to 1400 -png report.pdf /tmp/point-hash-report-page
```

Fresh renderer binaries can change PDF bytes. Check and inspect the new
render rather than asserting cross-version PDF reproducibility. Report
source, diagram source and a SHA256SUMS manifest accompany the committed PDF.
