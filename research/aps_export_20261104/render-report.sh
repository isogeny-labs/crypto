#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
dot -Tsvg pipeline.dot -o pipeline.svg
dot -Tpdf pipeline.dot -o pipeline.pdf
pdflatex -interaction=nonstopmode -halt-on-error report.tex >/dev/null
report_tmp_dir=$(mktemp -d)
trap 'render_status=$?; if test "$render_status" -eq 0; then rm -rf "$report_tmp_dir"; else printf "Retained render comparison: %s\n" "$report_tmp_dir" >&2; fi' EXIT
mv report.pdf "$report_tmp_dir/full.pdf"
pdftotext "$report_tmp_dir/full.pdf" "$report_tmp_dir/full.txt"
gs -q -dBATCH -dNOPAUSE -sDEVICE=pdfwrite -dPDFSETTINGS=/ebook \
  -dSubsetFonts=true -dEmbedAllFonts=true -dCompressFonts=true \
  -sOutputFile=report.pdf "$report_tmp_dir/full.pdf"
pdftotext report.pdf "$report_tmp_dir/final.txt"
cmp "$report_tmp_dir/full.txt" "$report_tmp_dir/final.txt"
