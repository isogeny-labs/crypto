#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")"
pdflatex -interaction=nonstopmode -halt-on-error report.tex > /tmp/hashed-finite-refresh-build.txt
gs -q -dNOPAUSE -dBATCH -sDEVICE=pdfwrite -dPDFSETTINGS=/screen \
  -dEmbedAllFonts=true -dSubsetFonts=true -sOutputFile=.compact.pdf report.pdf
mv -- .compact.pdf report.pdf
