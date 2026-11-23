#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
aps_report_tmp=$(mktemp -d /tmp/aps-instrument-report.XXXXXX)
trap 'if [ "$?" -eq 0 ]; then rm -rf -- "$aps_report_tmp"; else printf "Render failed; logs retained at %s\n" "$aps_report_tmp" >&2; fi' EXIT
dot -Tsvg instrument.dot -o instrument.svg
dot -Tpdf instrument.dot -o instrument.pdf
SOURCE_DATE_EPOCH=$(date -u -d '2026-10-30 00:00:00 UTC' +%s) \
  pdflatex -interaction=nonstopmode -halt-on-error \
  -output-directory "$aps_report_tmp" report.tex > "$aps_report_tmp/latex.log" 2>&1
# Standard vector-PDF font subsetting; all text and figure content is retained.
gs -q -sDEVICE=pdfwrite -dCompatibilityLevel=1.5 -dPDFSETTINGS=/prepress \
  -dSubsetFonts=true -dCompressFonts=true -dNOPAUSE -dBATCH \
  -sOutputFile="$aps_report_tmp/report-fonts.pdf" "$aps_report_tmp/report.pdf" \
  > "$aps_report_tmp/fonts.log" 2>&1
pdftotext "$aps_report_tmp/report.pdf" "$aps_report_tmp/before.txt"
pdftotext "$aps_report_tmp/report-fonts.pdf" "$aps_report_tmp/after.txt"
cmp "$aps_report_tmp/before.txt" "$aps_report_tmp/after.txt"
cp "$aps_report_tmp/report-fonts.pdf" report.pdf
