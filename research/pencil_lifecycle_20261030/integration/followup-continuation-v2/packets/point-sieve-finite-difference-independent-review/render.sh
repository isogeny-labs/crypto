#!/bin/sh
set -eu
cd "$(dirname "$0")"
rsvg-convert -f pdf -o diagram.pdf diagram.svg
pdflatex -interaction=nonstopmode -halt-on-error report.tex
