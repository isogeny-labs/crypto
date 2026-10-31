#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")"
dot -Tsvg routes.dot -o routes.svg
dot -Tpdf routes.dot -o routes.pdf
pdflatex -interaction=nonstopmode -halt-on-error report.tex > /tmp/hashed-ecdl-finite-size-report-build.txt
gs -q -dNOPAUSE -dBATCH -sDEVICE=pdfwrite -dPDFSETTINGS=/screen \
  -dEmbedAllFonts=true -dSubsetFonts=true -sOutputFile=.compact.pdf report.pdf
mv -- .compact.pdf report.pdf
