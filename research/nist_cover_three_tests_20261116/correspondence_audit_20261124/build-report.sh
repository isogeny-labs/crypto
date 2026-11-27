#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
dot -Tsvg chain.dot -o chain.svg
dot -Tpdf chain.dot -o chain.pdf
pandoc REPORT.md --from markdown+tex_math_dollars --pdf-engine=pdflatex \
    -V colorlinks=true --resource-path=. -o REPORT.pdf
