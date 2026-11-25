#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
dot -Tsvg transport.dot -o transport.svg
dot -Tpdf transport.dot -o transport.pdf
pandoc REPORT.md --from markdown+tex_math_dollars --pdf-engine=pdflatex -V colorlinks=true --resource-path=. -o REPORT.pdf
