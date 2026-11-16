#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")"
dot -Tsvg setup.dot -o setup.svg
dot -Tpdf setup.dot -o setup.pdf
pdflatex -interaction=nonstopmode -halt-on-error report.tex > /tmp/hashed-ecdl-hash-prime-report-build.txt
gs -q -dNOPAUSE -dBATCH -sDEVICE=pdfwrite -dPDFSETTINGS=/screen \
  -dEmbedAllFonts=true -dSubsetFonts=true -sOutputFile=.compact.pdf report.pdf
mv -- .compact.pdf report.pdf
