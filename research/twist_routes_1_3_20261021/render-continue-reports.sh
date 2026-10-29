#!/usr/bin/env bash
# Document/graph rendering only; all numerical results come from native audits.
set -euo pipefail
root=research/twist_routes_1_3_20261021/reports/continue-20261026
only=${1:-all}
dot -Tsvg "$root/CONNECTIONS.dot" -o "$root/CONNECTIONS.svg"
for report in "$root" "$root"/*/; do
  test -f "$report/RESULTS.md" || continue
  if [ "$only" != all ] && [ "$(basename -- "$report")" != "$only" ]; then continue; fi
  for svg in "$report"/*.svg; do
    graph=${svg%.svg}
    inkscape "$svg" --export-type=pdf --export-filename="$graph.graph.pdf"
    inkscape "$svg" --export-type=png --export-dpi=150 --export-filename="$graph.png"
  done
  ( cd "$report"
    pandoc RESULTS.md --standalone --from=gfm --to=latex -V geometry:margin=18mm -V fontsize=10pt --output RESULTS.tex
    node --input-type=module <<'JS'
import fs from 'node:fs';
let tex=fs.readFileSync('RESULTS.tex','utf8');
tex=tex.replace(/\\includesvg\{([^}]+)\.svg\}/g,(_,p)=>'\\begin{center}\\includegraphics[width=0.98\\linewidth]{'+p+'.graph.pdf}\\end{center}');
tex=tex.replaceAll('\\begin{longtable}[]{@{}lll@{}}','\\begin{longtable}[]{@{}p{0.56\\linewidth}p{0.18\\linewidth}p{0.18\\linewidth}@{}}');
tex=tex.replaceAll('\\begin{longtable}[]{@{}ll@{}}','\\begin{longtable}[]{@{}p{0.54\\linewidth}p{0.40\\linewidth}@{}}');
tex=tex.replaceAll('\\begin{longtable}[]{@{}llll@{}}','\\begin{longtable}[]{@{}p{0.50\\linewidth}p{0.12\\linewidth}p{0.13\\linewidth}p{0.17\\linewidth}@{}}');
tex=tex.replace('\\begin{document}','\\emergencystretch=4em\n\\begin{document}');
fs.writeFileSync('RESULTS.tex',tex);
JS
    pdflatex -interaction=nonstopmode -halt-on-error RESULTS.tex
    pdflatex -interaction=nonstopmode -halt-on-error RESULTS.tex
  )
done
