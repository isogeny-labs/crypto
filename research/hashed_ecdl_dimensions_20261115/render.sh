#!/usr/bin/env bash
# Document rendering only; no research calculations or experiments.
set -euo pipefail
cd "$(dirname "$0")"
dot -Tsvg dimensions.dot -o dimensions.svg
# PYTHON selects a Python environment with the typst rendering wheel installed.
"${PYTHON:-python3}" -c "import typst; typst.compile('report.typ', output='report.pdf')"
