#!/usr/bin/env bash
set -euo pipefail

report_dir="$(cd "$(dirname "$0")" && pwd)"
export SOURCE_DATE_EPOCH=1794960000

pandoc "$report_dir/H115_REPORT.html" \
  --from=html \
  --lua-filter="$report_dir/H115_REPORT_RENDER.lua" \
  --include-in-header="$report_dir/H115_REPORT_HEADER.tex" \
  --pdf-engine=xelatex \
  --resource-path="$report_dir" \
  -V geometry:margin=17mm \
  -V papersize:a4 \
  -V fontsize=10pt \
  -V mainfont="DejaVu Sans" \
  -V monofont="DejaVu Sans Mono" \
  -o "$report_dir/H115_REPORT.pdf"

# xdvipdfmx otherwise emits a fresh random trailer ID on every invocation.
# Use the first 128 bits of the frozen report-source SHA-256 as the stable ID;
# replacement is length-preserving, so PDF byte offsets remain unchanged.
perl -0pi -e \
  's{/ID\[<[0-9a-f]{32}><[0-9a-f]{32}>\]}{/ID[<a1d17a4882a80529fdf33cfe09ddd4f9><a1d17a4882a80529fdf33cfe09ddd4f9>]}' \
  "$report_dir/H115_REPORT.pdf"
