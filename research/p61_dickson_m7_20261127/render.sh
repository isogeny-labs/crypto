#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"

dot -Tsvg pipeline.dot -o pipeline.svg
dot -Tpng -Gdpi=160 pipeline.dot -o pipeline.png
# Keep the PNG's pixels while giving Pandoc a readable 5.3-inch PDF figure.
convert pipeline.png -units PixelsPerInch -density 232 pipeline.png
pandoc REPORT.md -f markdown+tex_math_dollars -o REPORT.pdf \
  --pdf-engine=xelatex -V geometry:margin=0.75in -V fontsize=10pt
