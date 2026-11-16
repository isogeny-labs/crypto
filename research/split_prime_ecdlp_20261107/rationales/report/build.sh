#!/usr/bin/env bash
# Render report/REPORT.pdf from report/REPORT.typ with the figures in ../figures.
# Rendering only (AGENTS.md "no Python": Typst as a Python wheel is document rendering).
# Env: REPORT_PY (python with the typst wheel; default /tmp/report-venv/bin/python).
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PY="${REPORT_PY:-/tmp/report-venv/bin/python}"
"$PY" -c "import typst; typst.compile('$HERE/REPORT.typ', output='$HERE/REPORT.pdf', root='$HERE/..')"
echo "wrote $HERE/REPORT.pdf"
