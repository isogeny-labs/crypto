#!/usr/bin/env bash
# Document layout only; this script does not run research or recompute results.
set -euo pipefail
cd "$(dirname "$0")"
dot -Tsvg figures/qualification.dot -o figures/qualification.svg
pandoc REPORT.md --from=markdown+tex_math_single_backslash --to=typst \
  --standalone --template=report-template.typ --wrap=none --include-in-header=report-style.typ \
  -V papersize=a4 -V mainfont='DejaVu Sans' -V fontsize=9pt \
  --output=REPORT.typ
if command -v typst >/dev/null 2>&1; then
  typst compile REPORT.typ REPORT.pdf
else
  python3 -c 'import typst; typst.compile("REPORT.typ", output="REPORT.pdf")'
fi
