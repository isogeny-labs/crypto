#!/bin/bash
set -eu
cd "$(dirname "$0")"
python3 render_relation_pipeline.py
sed 's@figures/relation-pipeline.svg@figures/relation-pipeline.pdf@g' RELATION_PIPELINE.md |
    pandoc --from markdown+tex_math_dollars --pdf-engine=pdflatex \
    -V geometry:margin=0.7in -V fontsize=10pt -V colorlinks=true \
    --resource-path=. -o RELATION_PIPELINE.pdf
