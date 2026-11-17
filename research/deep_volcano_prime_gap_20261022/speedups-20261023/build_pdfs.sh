#!/usr/bin/env bash
# Render the speed-up results report and its two run reports to PDF:
# pandoc (gfm -> typst) of each README, then typst with the study's preamble.
# usage: build_pdfs.sh PANDOC PYTHON   (PYTHON has the `typst` package)
set -euo pipefail
pandoc=$1 py=$2
study=$(cd "$(dirname "$0")/.." && pwd)
for dir in speedups-20261023 reports/run-20261023T0032Z reports/run-20261023T0121Z; do
  d="$study/$dir"
  rel=$(realpath --relative-to="$d" "$study")
  "$pandoc" -f gfm -t typst "$d/README.md" -o "$d/report-body.typ"
  printf '// Build: speedups-20261023/build_pdfs.sh (pandoc -f gfm -t typst README.md, then this file).\n#include "%s/report-preamble.typ"\n#include "report-body.typ"\n' "$rel" > "$d/report.typ"
  "$py" -c "import sys, typst; typst.compile(sys.argv[1], output=sys.argv[2], root=sys.argv[3])" "$d/report.typ" "$d/report.pdf" "$study"
  echo "wrote $dir/report.pdf"
done
