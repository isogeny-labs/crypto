#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"
for graph in typed_correspondence typed_transfer_context; do
  dot -Tsvg "$graph.dot" -o "$graph.svg"
  dot -Tpng -Gdpi=150 "$graph.dot" -o "$graph.png"
  dot -Tpdf "$graph.dot" -o "$graph.pdf"
done
pdflatex -interaction=nonstopmode -halt-on-error report.tex
pdflatex -interaction=nonstopmode -halt-on-error report.tex
