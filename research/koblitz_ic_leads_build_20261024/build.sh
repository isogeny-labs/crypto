#!/bin/sh
# Build report.pdf from README.md (layout only). Needs pandoc and the typst
# Python wheel (python3 -m venv /tmp/report-venv && /tmp/report-venv/bin/pip install typst matplotlib).
# Typst consumes a ';' that directly follows an embedded #link(...)[...]
# expression, so "]; " is rewritten as "]\; " to keep it as text.
set -e
cd "$(dirname "$0")"
pandoc README.md -f gfm -t typst --wrap=none \
  | sed -E '/^<[^>]*>$/d' \
  | perl -pe 's/\x5d\x3b\x20/\x5d\x5c\x3b\x20/g' > body.typ
"${PY:-/tmp/report-venv/bin/python}" -c "import typst; typst.compile('report.typ', output='report.pdf')"
