#!/usr/bin/env bash
set -euo pipefail
proposal_dir="$(cd -- "$(dirname -- "$0")" && pwd)"
proposal_build_dir="$(mktemp -d /tmp/twist-followup-pdf.XXXXXX)"
trap 'rm -rf -- "$proposal_build_dir"' EXIT
export XDG_CACHE_HOME=/tmp/twist-report-cache
export XDG_CONFIG_HOME=/tmp/twist-report-config
export GSETTINGS_BACKEND=memory
mkdir -p "$XDG_CACHE_HOME" "$XDG_CONFIG_HOME"
cd -- "$proposal_dir"
dot -Tsvg PROGRAM.dot -o PROGRAM.svg
dot -Tpdf PROGRAM.dot -o PROGRAM.pdf
pandoc README.md --from=markdown+tex_math_dollars --standalone --to=latex \
  --lua-filter=layout.lua \
  --include-in-header=../latex-header.tex \
  -V mainfont='DejaVu Serif' -V monofont='DejaVu Sans Mono' \
  -V fontsize=10pt -V geometry:margin=0.7in \
  -o "$proposal_build_dir/REPORT.tex"
xelatex -output-directory="$proposal_build_dir" -halt-on-error -interaction=nonstopmode \
  "$proposal_build_dir/REPORT.tex" > "$proposal_build_dir/first-pass.stdout"
xelatex -output-directory="$proposal_build_dir" -halt-on-error -interaction=nonstopmode \
  "$proposal_build_dir/REPORT.tex" > "$proposal_build_dir/build.stdout"
if rg -n 'Overfull|Missing character|undefined references|LaTeX Error' "$proposal_build_dir/build.stdout"; then
  printf '%s\n' 'PDF layout/reference check failed.' >&2
  exit 1
fi
pdfunite "$proposal_build_dir/REPORT.pdf" PROGRAM.pdf "$proposal_build_dir/combined.pdf"
cp "$proposal_build_dir/REPORT.tex" REPORT.tex
cp "$proposal_build_dir/combined.pdf" REPORT.pdf
pdfinfo REPORT.pdf
