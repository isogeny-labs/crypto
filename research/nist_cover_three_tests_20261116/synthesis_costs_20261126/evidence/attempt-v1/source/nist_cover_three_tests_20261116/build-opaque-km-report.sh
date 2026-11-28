#!/bin/sh
set -eu
cd "$(dirname "$0")"
dot -Tsvg figures/opaque-km.dot -o figures/opaque-km.svg
dot -Tpdf figures/opaque-km.dot -o figures/opaque-km.pdf
sed 's@figures/opaque-km.svg@figures/opaque-km.pdf@g' OPAQUE_KM.md |
    pandoc --from markdown+tex_math_dollars --pdf-engine=pdflatex \
    -V geometry:margin=0.7in -V fontsize=10pt -V colorlinks=true \
    --resource-path=. -o OPAQUE_KM.pdf
