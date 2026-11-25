#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
dot -Tsvg mechanism.dot -o mechanism.svg
dot -Tpdf mechanism.dot -o mechanism.pdf
pandoc REPORT.md --from markdown+tex_math_dollars --pdf-engine=pdflatex -V colorlinks=true --resource-path=. -o REPORT.pdf
