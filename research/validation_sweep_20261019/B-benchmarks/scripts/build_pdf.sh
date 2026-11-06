#!/bin/sh
# REPORT.md -> REPORT.typ -> REPORT.pdf (pandoc + typst wheel in /tmp/report-venv, see research-visuals skill)
cd "$(dirname "$0")/.."
/tmp/report-venv/bin/python scripts/build_report.py
pandoc REPORT.md -s -f gfm -t typst -V papersize=a4 -V fontsize=8pt -o REPORT.typ
sed -i 's/margin: (x: 1.25in, y: 1.25in),/margin: (x: 1.4cm, y: 1.5cm),/; s/font: (),/font: ("DejaVu Sans",),/' REPORT.typ
/tmp/report-venv/bin/python -c "import typst; typst.compile('REPORT.typ', output='REPORT.pdf')"
