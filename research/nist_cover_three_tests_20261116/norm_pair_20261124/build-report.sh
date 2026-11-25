#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
python3 closure.py
pandoc REPORT.md --from markdown+tex_math_dollars --pdf-engine=pdflatex -V colorlinks=true --resource-path=. -o REPORT.pdf
