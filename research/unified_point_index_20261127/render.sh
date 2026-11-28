#!/usr/bin/env bash
set -euo pipefail
report_dir=$(cd "$(dirname "$0")" && pwd)
dot -Tsvg "$report_dir/point-index.dot" -o "$report_dir/point-index.svg"
dot -Tpng -Gdpi=120 "$report_dir/point-index.dot" -o "$report_dir/point-index.png"
pdf_source=$(mktemp "$report_dir/.pdf-source.XXXXXX.md")
trap 'rm -f "$pdf_source"' EXIT
sed 's/point-index\.svg/point-index.png/' "$report_dir/REPORT.md" > "$pdf_source"
pandoc "$pdf_source" --from gfm --resource-path="$report_dir" --pdf-engine=xelatex \
  -V geometry:margin=0.7in -V fontsize=10pt -V mainfont='DejaVu Sans' \
  -o "$report_dir/REPORT.pdf"
