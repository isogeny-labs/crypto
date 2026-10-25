#!/usr/bin/env bash
# Recorded-value document rendering only.
set -euo pipefail
cd "$(dirname "$0")/reports/block-20261025"
dot -Tsvg COUNTS.dot -o COUNTS.svg
dot -Tpdf COUNTS.dot -o COUNTS.graph.pdf
dot -Gdpi=150 -Tpng COUNTS.dot -o COUNTS.png
pandoc RESULTS.md --standalone --from=gfm --to=latex \
  -V geometry:margin=18mm -V fontsize=10pt --output RESULTS.tex
node --input-type=module <<'JS'
import fs from 'node:fs';
let tex=fs.readFileSync('RESULTS.tex','utf8');
tex=tex.replaceAll('\\includesvg{COUNTS.svg}','\\begin{center}\\includegraphics[width=0.98\\linewidth]{COUNTS.graph.pdf}\\end{center}');
tex=tex.replaceAll('\\begin{longtable}[]{@{}lllll@{}}','\\begin{longtable}[]{@{}p{0.34\\linewidth}p{0.12\\linewidth}p{0.12\\linewidth}p{0.12\\linewidth}p{0.20\\linewidth}@{}}');
tex=tex.replaceAll('\\begin{longtable}[]{@{}lll@{}}','\\begin{longtable}[]{@{}p{0.43\\linewidth}p{0.26\\linewidth}p{0.25\\linewidth}@{}}');
tex=tex.replaceAll('\\begin{longtable}[]{@{}llll@{}}','\\begin{longtable}[]{@{}p{0.48\\linewidth}p{0.14\\linewidth}p{0.14\\linewidth}p{0.14\\linewidth}@{}}');
tex=tex.replace('\\begin{document}','\\emergencystretch=4em\n\\begin{document}');
fs.writeFileSync('RESULTS.tex',tex);
JS
pdflatex -interaction=nonstopmode -halt-on-error RESULTS.tex
pdflatex -interaction=nonstopmode -halt-on-error RESULTS.tex
