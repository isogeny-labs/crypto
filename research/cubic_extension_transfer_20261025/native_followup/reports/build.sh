#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")"
pdflatex -interaction=nonstopmode -halt-on-error REPORT.tex > REPORT.log.txt
pdflatex -interaction=nonstopmode -halt-on-error REPORT.tex >> REPORT.log.txt
gs -q -dNOPAUSE -dBATCH -sDEVICE=pdfwrite -dPDFSETTINGS=/screen \
  -dEmbedAllFonts=false -dSubsetFonts=true -sOutputFile=REPORT-final.pdf REPORT.pdf
mv REPORT-final.pdf REPORT.pdf
