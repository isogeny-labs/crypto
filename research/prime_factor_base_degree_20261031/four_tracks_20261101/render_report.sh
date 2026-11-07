#!/usr/bin/env bash
set -euo pipefail
task_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
task_raw=$(mktemp --suffix=.pdf)
trap 'rm -f -- "$task_raw"' EXIT
cd -- "$task_dir"
"${REPORT_PYTHON:-python3}" - "$task_raw" <<'PY'
import sys
import typst
typst.compile('report.typ', output=sys.argv[1])
PY
gs -q -dNOPAUSE -dBATCH -sDEVICE=pdfwrite -dCompatibilityLevel=1.7 \
  -dPDFSETTINGS=/printer -dDetectDuplicateImages=true \
  -sOutputFile=report.pdf "$task_raw"
