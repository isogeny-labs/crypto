#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
dot -Tsvg normalization.dot -o normalization.svg
dot -Tpdf normalization.dot -o normalization.pdf
pandoc REPORT.md --from markdown+tex_math_dollars --pdf-engine=pdflatex \
    -V colorlinks=true --resource-path=. -o REPORT.pdf
