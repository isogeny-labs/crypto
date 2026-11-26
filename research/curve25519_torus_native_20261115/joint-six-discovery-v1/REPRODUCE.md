# Review and render this written analysis

DERIVATION.md states every mathematical hypothesis and the full induction;
PROOF_MAP.md distinguishes written laws from missing native/formal
instances. PRODUCT_JET_LEAD.md is a conditional algebraic reformulation
and next-work contract. No scientific executable or new Lean source is
part of this version. Rechecking prose, hashes or PDF layout does not
independently verify the mathematical argument. [pcc:unverified]

From this directory, Graphviz and pdfLaTeX render the editable sources:

```sh
dot -Tpdf flow.dot -o flow.pdf
dot -Tsvg flow.dot -o flow.svg
pdflatex -interaction=nonstopmode -halt-on-error report.tex
pdftoppm -scale-to 1600 -png report.pdf page
```

The recorded final PDF has two inspected pages and no overfull/underfull
boxes. Renderer timestamps mean a rebuilt PDF need not be byte-identical.
The committed PDF and raw renderer logs are the attested subjects. The
first overfull layout source/diagram/log/stdout remain in report-development.
No frozen source, evidence, criterion or previous report is edited to make
this report. Graph normalization occurred before hashing. [pcc:unverified]
