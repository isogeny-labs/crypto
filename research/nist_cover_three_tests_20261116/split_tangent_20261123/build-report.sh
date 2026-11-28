#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
python3 controls.py
dot -Tsvg pipeline.dot -o pipeline.svg
dot -Tpdf pipeline.dot -o pipeline.pdf
pandoc REPORT.md --from markdown+tex_math_dollars --pdf-engine=pdflatex -V colorlinks=true --resource-path=. -o REPORT.pdf
