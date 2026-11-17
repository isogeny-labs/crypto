#!/bin/sh
set -eu
cd "$(dirname "$0")"
python3 render_figures.py
sed -e 's@figures/ranking.svg@figures/ranking.pdf@g' \
    -e 's@figures/cover-flow.svg@figures/cover-flow.pdf@g' README.md |
  pandoc --from markdown+tex_math_dollars --pdf-engine=pdflatex \
    -V geometry:margin=0.7in -V fontsize=10pt -V colorlinks=true \
    --resource-path=. -o REPORT.pdf
