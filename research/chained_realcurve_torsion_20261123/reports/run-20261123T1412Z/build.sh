#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
export SOURCE_DATE_EPOCH=1795442400
dot -Tsvg chain.dot -o chain.svg
dot -Tpdf chain.dot -o chain.pdf
inkscape coverage.svg --export-type=pdf --export-filename=coverage.pdf
pdflatex -interaction=nonstopmode -halt-on-error -file-line-error REPORT.tex
pdflatex -interaction=nonstopmode -halt-on-error -file-line-error REPORT.tex
