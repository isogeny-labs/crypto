#!/usr/bin/env bash
# Native document reproduction; no scientific computation or benchmark.
set -euo pipefail
out=${1:?fresh absolute report output directory}
report_dir=$(cd "$(dirname "$0")" && pwd)
mkdir "$out"
cp "$report_dir/REPORT.tex" "$report_dir/first-kernel-cycle.dot" "$out/"
cd "$out"
dot -Tpdf first-kernel-cycle.dot -o first-kernel-cycle.pdf
dot -Tsvg first-kernel-cycle.dot -o first-kernel-cycle.svg
pdflatex -interaction=nonstopmode -halt-on-error REPORT.tex > render.stdout 2> render.stderr
pdfinfo REPORT.pdf > PDF_INFO.txt
