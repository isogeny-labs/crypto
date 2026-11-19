#!/bin/bash
set -eu
cd "$(dirname "$0")"
dot -Tsvg transfer-flow.dot -o transfer-flow.svg
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
dot -Tpng -Gdpi=180 transfer-flow.dot -o "$scratch/transfer-flow.png"
sed "s#(transfer-flow.svg)#($scratch/transfer-flow.png)#" REPORT.md > "$scratch/REPORT.md"
pandoc "$scratch/REPORT.md" --pdf-engine=xelatex \
  -V geometry:margin=0.8in -V fontsize=10pt -o REPORT.pdf
