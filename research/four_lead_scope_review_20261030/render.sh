#!/bin/sh
set -eu
cd "$(dirname "$0")"
rsvg-convert -f pdf -o obligations-figure.pdf obligations.svg
pdflatex -interaction=nonstopmode -halt-on-error -no-shell-escape report.tex
pdflatex -interaction=nonstopmode -halt-on-error -no-shell-escape report.tex
