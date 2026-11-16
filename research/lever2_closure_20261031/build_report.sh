#!/bin/sh
# Rebuild the figure from evidence.json and the study PDF from REPORT.md.
# Document rendering only (pandoc + the Typst wheel in /tmp/report-venv).
set -e
cd "$(dirname "$0")"
PY=/tmp/report-venv/bin/python
$PY plot_degree.py evidence.json degree_growth
pandoc REPORT.md -f gfm -t typst --wrap=none -o body.typ
sed -i '/^<[^>]*>$/d' body.typ   # pandoc's heading labels: Typst rejects non-ASCII label names
sed -i 's#^\#image("\([^"]*\)")#\#page(flipped: true, margin: 1.2cm)[\#figure(image("\1", width: 100%), caption: [Recorded F4 solving degrees (left) and widest matrix at m = 2 (right). Values copied from the tower thread; open markers are a registered prediction.])]#' body.typ
printf '%s\n' '#set page(paper: "a4", margin: 1.8cm, numbering: "1")' '#set text(font: "DejaVu Sans", size: 8.6pt)' '#show raw: set text(font: "DejaVu Sans Mono", size: 7.4pt)' '#show table: set text(size: 7.4pt)' '#include "body.typ"' > REPORT.typ
$PY -c "import typst; typst.compile('REPORT.typ', output='REPORT.pdf')"
