#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
scratch=$(mktemp -d /tmp/atlas-report.XXXXXX)
trap 'rm -rf "$scratch"' EXIT
dot -Tpdf workflow.dot -o workflow.pdf
dot -Tsvg workflow.dot -o workflow.svg
pandoc README.md --pdf-engine=pdflatex -H report-header.tex -V fontfamily=mathptmx \
  -V geometry:margin=0.65in -V fontsize=10pt -o "$scratch/report.pdf"
gs -q -sDEVICE=pdfwrite -dCompatibilityLevel=1.7 -dPDFSETTINGS=/prepress \
  -dNOPAUSE -dBATCH -dSAFER -dEmbedAllFonts=false -sOutputFile=REPORT.pdf "$scratch/report.pdf"
