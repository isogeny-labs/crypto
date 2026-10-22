#!/usr/bin/env bash
set -euo pipefail
study_dir="$(cd -- "$(dirname -- "$0")" && pwd)"
report_root="${1:-$study_dir/reports/execution-20261022T0032Z}"
report_root="$(cd -- "$report_root" && pwd)"
export XDG_CACHE_HOME=/tmp/twist-report-cache
export XDG_CONFIG_HOME=/tmp/twist-report-config
export GSETTINGS_BACKEND=memory
mkdir -p "$XDG_CACHE_HOME" "$XDG_CONFIG_HOME"
bash "$study_dir/build-execution-reports.sh" "$report_root"
(
  cd -- "$study_dir"
  dot -Tsvg PROGRAM.dot -o PROGRAM.svg
  dot -Tpdf PROGRAM.dot -o PROGRAM.pdf
  pandoc EXPERIMENTS.md --standalone --to=latex --include-in-header=latex-header.tex \
    -V mainfont='DejaVu Serif' -V monofont='DejaVu Sans Mono' -V fontsize=10pt \
    -V geometry:margin=0.65in --resource-path=.:reports -o EXPERIMENTS.tex
  xelatex -halt-on-error -interaction=nonstopmode EXPERIMENTS.tex > reports/experiments-build.stdout
  xelatex -halt-on-error -interaction=nonstopmode EXPERIMENTS.tex >> reports/experiments-build.stdout
)
(
  cd -- "$study_dir/reports"
  pandoc EXECUTION_RESULTS.md --standalone --to=latex --include-in-header=../latex-header.tex \
    --lua-filter=../report-figure-pages.lua \
    -V mainfont='DejaVu Serif' -V monofont='DejaVu Sans Mono' -V fontsize=10pt \
    -V geometry:margin=0.65in -o EXECUTION_RESULTS.tex
  xelatex -jobname=EXECUTION_RESULTS-TEXT -halt-on-error -interaction=nonstopmode EXECUTION_RESULTS.tex > execution-build.stdout
  xelatex -jobname=EXECUTION_RESULTS-TEXT -halt-on-error -interaction=nonstopmode EXECUTION_RESULTS.tex >> execution-build.stdout
  pdfunite EXECUTION_RESULTS-TEXT.pdf "$report_root/SENSITIVITY.pdf" \
    "$report_root/isogeny-20261022T0054Z-v2/MAPS.pdf" EXECUTION_RESULTS.new.pdf
  mv EXECUTION_RESULTS.new.pdf EXECUTION_RESULTS.pdf
)
