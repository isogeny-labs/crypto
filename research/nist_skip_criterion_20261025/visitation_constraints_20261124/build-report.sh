#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
export MPLCONFIGDIR=/tmp/nist-visitation-matplotlib
python3 render.py
pdflatex -interaction=nonstopmode -halt-on-error REPORT.tex
if grep -q 'Overfull' REPORT.log; then exit 1; fi
compact_pdf=$(mktemp /tmp/nist-visitation-pdf.XXXXXX.pdf)
trap 'rm -f "$compact_pdf"' EXIT
gs -sDEVICE=pdfwrite -dCompatibilityLevel=1.6 -dPDFSETTINGS=/ebook -dNOPAUSE -dBATCH -dQUIET -sOutputFile="$compact_pdf" REPORT.pdf
cp "$compact_pdf" REPORT.pdf
