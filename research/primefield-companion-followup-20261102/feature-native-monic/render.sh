#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
"${REPORT_PYTHON:-/home/ec2-user/primefield-ecdlp-research-20261028/.venv/bin/python}" -c 'import typst; typst.compile("report.typ", output="report.pdf")'
