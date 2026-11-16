#!/usr/bin/env bash
# Rebuild the dated review layout; never replace the frozen historical PDF.
set -euo pipefail
cd "$(dirname "$0")"
PY=${PY:-python3}
"$PY" tables.py review-20261020
"$PY" review-20261020/build_pdf.py
