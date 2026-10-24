#!/bin/sh
# Rebuild REPORT.pdf from the note and the two SVGs. Document rendering only (AGENTS.md "no Python" exempts it).
set -e
cd "$(dirname "$0")"
/tmp/report-venv/bin/python plot_exponents.py
pandoc ../RESEARCH_LATTICE_ECDLP_ROUTES.md -f gfm -t typst --wrap=none -o body.typ
/tmp/report-venv/bin/python fix_typst.py
/tmp/report-venv/bin/python -c "import typst; typst.compile('report.typ', output='REPORT.pdf')"
