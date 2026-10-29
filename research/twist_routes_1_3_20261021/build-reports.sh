#!/usr/bin/env bash
set -euo pipefail
study_dir="$(cd -- "$(dirname -- "$0")" && pwd)"
cd "$study_dir"
export XDG_CACHE_HOME="${XDG_CACHE_HOME:-/tmp/twist-report-cache}"
mkdir -p "$XDG_CACHE_HOME"
dot -Tsvg PROGRAM.dot -o PROGRAM.svg
dot -Tpdf PROGRAM.dot -o PROGRAM.pdf
for document in EXPERIMENTS reports/RESULTS; do
  pandoc "$document.md" --standalone --to=latex \
    --include-in-header=latex-header.tex \
    -V mainfont='DejaVu Serif' -V monofont='DejaVu Sans Mono' \
    -V fontsize=10pt -V geometry:margin=0.65in \
    --resource-path=.:reports --output="$document.tex"
  if [[ "$document" == reports/* ]]; then
    # Make relative graph/receipt paths resolve from the report's own directory.
    (cd reports && xelatex -halt-on-error -interaction=nonstopmode RESULTS.tex > /tmp/twist-results-tex.log)
    (cd reports && xelatex -halt-on-error -interaction=nonstopmode RESULTS.tex >> /tmp/twist-results-tex.log)
  else
    xelatex -halt-on-error -interaction=nonstopmode "$document.tex" > /tmp/twist-experiments-tex.log
    xelatex -halt-on-error -interaction=nonstopmode "$document.tex" >> /tmp/twist-experiments-tex.log
  fi
done
