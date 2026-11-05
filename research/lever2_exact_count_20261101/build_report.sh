#!/bin/sh
# Rebuild the figure and tables from runs/m*/summary.json, then REPORT.pdf from REPORT.md.
# Document rendering only (pandoc + the Typst wheel in /tmp/report-venv).
set -e
cd "$(dirname "$0")"
PY=/tmp/report-venv/bin/python
$PY plot_counts.py .
pandoc REPORT.md -f gfm -t typst --wrap=none -o body.typ
sed -i '/^<[^>]*>$/d' body.typ   # pandoc's heading labels: Typst rejects non-ASCII label names
sed -i 's#^\#image("\([^"]*\)")#\#page(flipped: true, margin: 1.2cm)[\#figure(image("\1", width: 100%), caption: [Largest closure count over the threshold (top) and coverage (bottom), by base size; m = 2 left, m = 3 right. Exact counts from the summary.json of runs/m2 and runs/m3.])]#' body.typ
printf '%s\n' '#set page(paper: "a4", margin: 1.6cm, numbering: "1")' '#set text(font: "DejaVu Sans", size: 8.4pt)' '#show raw: set text(font: "DejaVu Sans Mono", size: 7.2pt)' '#show table: set text(size: 6.8pt)' '#include "body.typ"' > REPORT.typ
$PY -c "import typst; typst.compile('REPORT.typ', output='REPORT.pdf')"
