#!/bin/sh
# Render this study's figures and REPORT.pdf (layout only; computes nothing).
# Needs pandoc and the `typst` wheel with matplotlib in /tmp/report-venv.
set -eu
cd "$(dirname "$0")"
PY=${PY:-/tmp/report-venv/bin/python}
"$PY" figures/plot.py results figures
pandoc REPORT.md -t typst --wrap=none -o /tmp/n2rm_body.typ
{ cat preamble.typ; echo; cat /tmp/n2rm_body.typ; } > REPORT.typ
"$PY" -c "import typst; typst.compile('REPORT.typ', output='REPORT.pdf', root='.')"
echo "built REPORT.pdf"
