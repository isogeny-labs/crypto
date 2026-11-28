#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
export XDG_CACHE_HOME="${TMPDIR:-/tmp}/aps-cost-report-font-cache"
mkdir -p "$XDG_CACHE_HOME"
dot -Tsvg pipeline.dot -o pipeline.svg
dot -Tpdf pipeline.dot -o pipeline.pdf
pandoc README.md --from=markdown+tex_math_dollars --standalone \
  --variable geometry:margin=22mm --variable fontsize=10pt \
  --variable colorlinks=true --variable urlcolor=blue \
  --lua-filter=report-images.lua --to=latex --output=report.tex
pdflatex -interaction=nonstopmode -halt-on-error report.tex >/dev/null
pdflatex -interaction=nonstopmode -halt-on-error report.tex >/dev/null
rm -f report.aux report.log report.out
