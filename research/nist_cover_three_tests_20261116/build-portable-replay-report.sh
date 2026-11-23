#!/bin/bash
set -eu
cd "$(dirname "$0")"
python3 render_portable_replay.py
sed 's@figures/portable-replay.svg@figures/portable-replay.pdf@g' PORTABLE_REPLAY.md |
    pandoc --from markdown+tex_math_dollars --pdf-engine=pdflatex \
    -V geometry:margin=0.7in -V fontsize=10pt -V colorlinks=true \
    --resource-path=. -o PORTABLE_REPLAY.pdf
