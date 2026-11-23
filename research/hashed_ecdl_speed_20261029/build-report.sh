#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")"
dot -Tsvg paths.dot -o paths.svg
dot -Tpdf paths.dot -o paths.pdf
dot -Tsvg witness-work.dot -o witness-work.svg
dot -Tpdf witness-work.dot -o witness-work.pdf
pdflatex -interaction=nonstopmode -halt-on-error report.tex > build.log
pdflatex -interaction=nonstopmode -halt-on-error report.tex >> build.log
gs -q -dNOPAUSE -dBATCH -sDEVICE=pdfwrite -dPDFSETTINGS=/screen \
  -dEmbedAllFonts=true -dSubsetFonts=true -sOutputFile=.compact.pdf report.pdf
mv -- .compact.pdf report.pdf
