#!/usr/bin/env bash
# Build REPORT.pdf from REPORT.md (pandoc + pdfLaTeX). The PDF embeds the PDF
# renders of the same figures the Markdown shows as SVG; regenerate them first
# with figures/make_figures.py. Run from anywhere.
set -euo pipefail
cd "$(dirname "$0")/.."
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
sed -e 's#(figures/\(fig-[a-z-]*\)\.svg)#(figures/\1.pdf)#' REPORT.md > "$tmp/REPORT.md"
cat > "$tmp/head.tex" <<'TEX'
\usepackage{newunicodechar}
\newunicodechar{−}{\ensuremath{-}}
\newunicodechar{≥}{\ensuremath{\geq}}
\newunicodechar{≈}{\ensuremath{\approx}}
\newunicodechar{…}{\ldots}
\usepackage{float}
\floatplacement{figure}{H}
TEX
SOURCE_DATE_EPOCH=1794268800 pandoc "$tmp/REPORT.md" --resource-path=. \
  --pdf-engine=pdflatex -H "$tmp/head.tex" \
  -V fontsize=10pt -V geometry:margin=2cm -V colorlinks=true \
  -o REPORT.pdf
echo "wrote $(pwd)/REPORT.pdf"
