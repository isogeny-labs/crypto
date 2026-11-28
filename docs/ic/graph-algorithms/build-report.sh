#!/usr/bin/env bash
# Document rendering only; no research algorithm, experiment or measurement.
set -euo pipefail
cd -- "$(dirname -- "$0")"
dot -Tsvg pipeline.dot -o pipeline.svg
pandoc README.md SOURCES.md \
  --from=markdown+tex_math_dollars+pipe_tables \
  --to=typst --standalone \
  --template=report-template.typ \
  --lua-filter=report-layout.lua \
  --output=report.typ
if command -v typst >/dev/null 2>&1; then
  typst compile report.typ report.pdf
else
  # The typst wheel is a document renderer, not a research implementation.
  # Install with: python3 -m pip install typst==0.15.2
  python3 -c 'import typst; typst.compile("report.typ", output="report.pdf")'
fi
