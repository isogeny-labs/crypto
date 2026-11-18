#!/bin/sh
set -eu
cd "$(dirname "$0")"
python3 render_opaque_boundary.py
sed 's@figures/opaque-boundary.svg@figures/opaque-boundary.pdf@g' OPAQUE_KM_BOUNDARIES.md |
    pandoc --from markdown+tex_math_dollars --pdf-engine=pdflatex \
    -V geometry:margin=0.7in -V fontsize=10pt -V colorlinks=true \
    --resource-path=. -o OPAQUE_KM_BOUNDARIES.pdf
