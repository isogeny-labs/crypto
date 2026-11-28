#!/bin/sh
set -eu
cd "$(dirname "$0")"
dot -Tsvg figures/complete-maps.dot -o figures/complete-maps.svg
dot -Tpdf figures/complete-maps.dot -o figures/complete-maps.pdf
sed 's@figures/complete-maps.svg@figures/complete-maps.pdf@g' MAPS_STATUS.md |
    pandoc --from markdown+tex_math_dollars --pdf-engine=pdflatex \
    -V geometry:margin=0.7in -V fontsize=10pt -V colorlinks=true \
    --resource-path=. -o MAPS_STATUS.pdf
