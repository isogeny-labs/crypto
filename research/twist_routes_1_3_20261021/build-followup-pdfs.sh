#!/usr/bin/env bash
set -euo pipefail
study_dir="$(cd -- "$(dirname -- "$0")" && pwd)"
report_dir="$study_dir/reports/followup-20261023"
cd -- "$report_dir"
export XDG_CACHE_HOME=/tmp/twist-followup-tex-cache
export XDG_CONFIG_HOME=/tmp/twist-followup-tex-config
if test "${1:-}" != "--combined-only"; then
for twist_svg in ./*.svg; do
  inkscape "$twist_svg" --export-type=pdf --export-filename="${twist_svg%.svg}.graph.pdf" >"${twist_svg%.svg}.inkscape.stdout" 2>"${twist_svg%.svg}.inkscape.stderr"
done
for twist_md in E*.md; do
  pandoc "$twist_md" --standalone --lua-filter="$study_dir/followup-pdf-images.lua" \
    --pdf-engine=xelatex -V geometry:margin=0.65in -V fontsize=10pt \
    -V mainfont='DejaVu Sans' -V colorlinks=true -o "${twist_md%.md}.pdf" \
    >"${twist_md%.md}.tex.stdout" 2>"${twist_md%.md}.tex.stderr"
done
fi
cp RESULTS.md RESULTS-COMBINED.md
for twist_md in E*.md; do
  printf '\n\\newpage\n\n' >> RESULTS-COMBINED.md
  cat "$twist_md" >> RESULTS-COMBINED.md
done
pandoc RESULTS-COMBINED.md --standalone --lua-filter="$study_dir/followup-pdf-images.lua" \
  --pdf-engine=xelatex -V geometry:margin=0.65in -V fontsize=10pt \
  -V mainfont='DejaVu Sans' -V colorlinks=true -o RESULTS.pdf \
  >RESULTS.tex.stdout 2>RESULTS.tex.stderr
