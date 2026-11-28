#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
dot -Tsvg supports.dot -o supports.svg
dot -Tpdf supports.dot -o supports.pdf
python3 figures.py
pandoc REPORT.md --from markdown+tex_math_dollars --pdf-engine=pdflatex -V colorlinks=true --resource-path=. -o REPORT.pdf
