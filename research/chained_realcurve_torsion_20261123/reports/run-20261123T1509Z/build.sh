#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
export SOURCE_DATE_EPOCH=1795442400
dot -Tsvg models.dot -o models.svg
dot -Tpdf models.dot -o models.pdf
inkscape isomorphisms.svg --export-type=pdf --export-filename=isomorphisms.pdf
pdflatex -interaction=nonstopmode -halt-on-error -file-line-error REPORT.tex
pdflatex -interaction=nonstopmode -halt-on-error -file-line-error REPORT.tex
