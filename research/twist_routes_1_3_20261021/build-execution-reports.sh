#!/usr/bin/env bash
set -euo pipefail
study_dir="$(cd -- "$(dirname -- "$0")" && pwd)"
report_root="${1:?pass the report root}"
report_root="$(cd -- "$report_root" && pwd)"
export XDG_CACHE_HOME=/tmp/twist-report-cache
export XDG_CONFIG_HOME=/tmp/twist-report-config
export GSETTINGS_BACKEND=memory
mkdir -p "$XDG_CACHE_HOME" "$XDG_CONFIG_HOME"
for directory in "$report_root"/*/; do
  [[ -f "$directory/REPORT.md" ]] || continue
  inkscape "$directory/METRICS.svg" --export-type=pdf \
    --export-filename="$directory/METRICS.pdf" > "$directory/render.stdout" 2> "$directory/render.stderr"
  (
    cd -- "$directory"
    pandoc REPORT.md --standalone --to=latex --include-in-header="$study_dir/latex-header.tex" \
      --lua-filter="$study_dir/report-figure-pages.lua" \
      -V mainfont='DejaVu Serif' -V monofont='DejaVu Sans Mono' -V fontsize=10pt \
      -V papersize=letter -V classoption=landscape -V geometry:margin=0.55in -o REPORT.tex
    xelatex -jobname=REPORT-TEXT -halt-on-error -interaction=nonstopmode REPORT.tex > build.stdout
    xelatex -jobname=REPORT-TEXT -halt-on-error -interaction=nonstopmode REPORT.tex >> build.stdout
    parts=(REPORT-TEXT.pdf METRICS.pdf)
    if [[ -f MAPS.dot ]]; then
      dot -Tsvg MAPS.dot -o MAPS.svg
      dot -Tpdf MAPS.dot -o MAPS.pdf
      parts+=(MAPS.pdf)
    fi
    pdfunite "${parts[@]}" REPORT.new.pdf
    mv REPORT.new.pdf REPORT.pdf
  )
done
