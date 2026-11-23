#!/usr/bin/env bash
# Document rendering only; REPORT.md and DIAGRAM.dot are the editable sources.
set -eu
cd "$(dirname "$0")"
dot -Tsvg DIAGRAM.dot -o DIAGRAM.svg
dot -Tpdf DIAGRAM.dot -o DIAGRAM.graph.pdf
pandoc REPORT.md --standalone --from=gfm --to=latex \
  -V geometry:margin=18mm -V fontsize=10pt --output REPORT.tex
node --input-type=module <<'JS'
import fs from 'node:fs';
const file='REPORT.tex';
let tex=fs.readFileSync(file,'utf8');
tex=tex.replace('\\includesvg{DIAGRAM.svg}','\\begin{center}\\includegraphics[width=0.62\\linewidth]{DIAGRAM.graph.pdf}\\end{center}');
tex=tex.replace('\\begin{longtable}[]{@{}ll@{}}','\\begin{longtable}[]{@{}p{0.28\\linewidth}p{0.68\\linewidth}@{}}');
fs.writeFileSync(file,tex);
JS
pdflatex -interaction=nonstopmode -halt-on-error REPORT.tex
pdflatex -interaction=nonstopmode -halt-on-error REPORT.tex
