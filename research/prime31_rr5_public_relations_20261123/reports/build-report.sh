#!/usr/bin/env bash
# Document layout only. It reads the recorded report and fixed SVG figures.
set -euo pipefail
cd "$(dirname "$0")/.."
pandoc REPORT.md --from markdown+tex_math_single_backslash --to typst \
  --lua-filter reports/images.lua \
  --output reports/body.typ
cat reports/style.typ reports/title.typ reports/body.typ > reports/rr5-public-relations.typ
"${PDF_RENDER_PYTHON:-python3}" reports/render_report.py
