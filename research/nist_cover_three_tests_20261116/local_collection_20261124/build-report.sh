#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
python3 figures.py
dot -Tsvg incidence.dot -o incidence.svg
dot -Tpdf incidence.dot -o incidence.pdf
pandoc REPORT.md --from markdown+tex_math_dollars --pdf-engine=pdflatex -V colorlinks=true --resource-path=. -o REPORT.pdf
