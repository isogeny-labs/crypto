#!/usr/bin/env bash
set -euo pipefail
study=research/factor_base_heldout_20261025
python3 "$study/render_graph.py"
convert -background white -density 150 "$study/factor-base-heldout.svg" "$study/factor-base-heldout.png"
convert -background white -density 150 "$study/evaluation-by-size.svg" "$study/evaluation-by-size.png"
python3 "$study/render_report.py"
(
  cd "$study"
  pdflatex -interaction=nonstopmode -halt-on-error REPORT.tex > REPORT.build.log
)
pdfinfo "$study/REPORT.pdf" | rg '^(Pages|Page size|File size):'
