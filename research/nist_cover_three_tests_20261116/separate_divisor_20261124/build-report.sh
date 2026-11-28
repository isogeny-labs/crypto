#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
dot -Tsvg audit.dot -o audit.svg
dot -Tpdf audit.dot -o audit.pdf
pandoc REPORT.md --from markdown+tex_math_dollars --pdf-engine=pdflatex \
    -V colorlinks=true --resource-path=. -o REPORT.pdf
