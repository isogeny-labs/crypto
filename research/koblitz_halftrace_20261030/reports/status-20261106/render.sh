#!/usr/bin/env bash
set -euo pipefail
report_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
report_scratch=$(mktemp -d)
trap 'rm -rf -- "$report_scratch"' EXIT
"${TYPST_BIN:-typst}" compile "$report_dir/REPORT.typ" "$report_scratch/typeset.pdf"
# Preserve the vector report and embedded font subsets in a compact PDF.
gs -q -sDEVICE=pdfwrite -dCompatibilityLevel=1.7 -dNOPAUSE -dBATCH \
  -dPDFSETTINGS=/prepress -sOutputFile="$report_dir/REPORT.pdf" \
  "$report_scratch/typeset.pdf"
(cd "$report_dir" && sha256sum REPORT.pdf REPORT.typ pipeline.svg SNAPSHOT.json > REPORT.sha256)
