#!/bin/sh
# Rebuild analysis, figures and PDF from a recorded run (needs matplotlib, numpy, typst wheels).
set -e
RUN=${1:-results/run-20261019T1730Z}
python3 analyze.py "$RUN" A1
python3 plot.py "$RUN" figures
python3 -c "import typst; typst.compile('REPORT.typ', output='REPORT.pdf')"
