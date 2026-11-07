#!/bin/sh
# Rebuild the figure from the rows, then REPORT.pdf from REPORT.md.
# Document rendering only (pandoc + the Typst wheel in /tmp/report-venv).
set -e
cd "$(dirname "$0")"
PY=/tmp/report-venv/bin/python
$PY plot_cost.py .
pandoc REPORT.md -f gfm -t typst --wrap=none -o body.typ
sed -i '/^<[^>]*>$/d' body.typ
sed -i 's#^\#image("\([^"]*\)")#\#figure(image("\1", width: 100%), caption: [Wall time and largest dense block of one capped F4 run against t. Rows of runs/ and preflight/.])#' body.typ
printf '%s\n' '#set page(paper: "a4", margin: 1.6cm, numbering: "1")' '#set text(font: "DejaVu Sans", size: 8.4pt)' '#show raw: set text(font: "DejaVu Sans Mono", size: 7.2pt)' '#show table: set text(size: 7pt)' '#include "body.typ"' > REPORT.typ
$PY -c "import typst; typst.compile('REPORT.typ', output='REPORT.pdf')"
