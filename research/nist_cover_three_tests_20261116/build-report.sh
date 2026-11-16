#!/bin/sh
set -eu
cd "$(dirname "$0")"
python3 render_figures.py
sed -e 's@figures/kernels.svg@figures/kernels.pdf@g' -e 's@figures/route.svg@figures/route.pdf@g' -e 's@figures/recovery.svg@figures/recovery.pdf@g' README.md |
  pandoc --from markdown+tex_math_dollars --pdf-engine=pdflatex -V geometry:margin=0.7in -V fontsize=10pt -V colorlinks=true --resource-path=. -o REPORT.pdf
