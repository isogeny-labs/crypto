#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")"
dot -Tsvg implementation.dot -o implementation.svg
dot -Tpdf implementation.dot -o implementation.pdf
pdflatex -interaction=nonstopmode -halt-on-error report.tex > /tmp/hashed-finite-implementation-build.txt
gs -q -dNOPAUSE -dBATCH -sDEVICE=pdfwrite -dPDFSETTINGS=/screen \
  -dEmbedAllFonts=true -dSubsetFonts=true -sOutputFile=.compact.pdf report.pdf
mv -- .compact.pdf report.pdf
