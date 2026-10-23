#!/bin/bash
# REPORT.md -> report.typ -> report.pdf (pandoc + typst wheel in /tmp/report-venv; research-visuals "Rendering")
cd "$(dirname "$0")/.." || exit 1
python3 -I analysis/build_report.py
printf '%s\n' '#set page(paper: "a4", margin: 1.6cm)' '#set text(size: 9pt)' '#set table(stroke: 0.4pt + gray, inset: 3pt)' '#show table: set text(size: 7pt)' > report.typ
pandoc REPORT.md --wrap=none -f markdown+pipe_tables -t typst >> report.typ
/tmp/report-venv/bin/python -c "import typst; typst.compile('report.typ', output='report.pdf')"
