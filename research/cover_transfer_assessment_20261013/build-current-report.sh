#!/usr/bin/env bash
set -euo pipefail
report_dir="$(cd -- "$(dirname -- "$0")" && pwd)"
study_dir="$report_dir/../twist_routes_1_3_20261021"
render_dir="$(mktemp -d /tmp/cover-assessment-render.XXXXXX)"
export XDG_CACHE_HOME=/tmp/twist-report-cache
export XDG_CONFIG_HOME=/tmp/twist-report-config
export GSETTINGS_BACKEND=memory
cd -- "$report_dir"
for base in typed_correspondence typed_transfer_context; do
  dot -Tsvg "$base.dot" -o "$base.svg"
  dot -Tpng "$base.dot" -o "$base.png"
  dot -Tpdf "$base.dot" -o "$render_dir/$base.pdf"
done
pandoc README.md --from=markdown+tex_math_single_backslash --standalone --to=latex --include-in-header="$study_dir/latex-header.tex" \
  --lua-filter="$study_dir/report-figure-pages.lua" \
  -V mainfont='DejaVu Serif' -V monofont='DejaVu Sans Mono' -V fontsize=10pt \
  -V geometry:margin=0.7in -o "$render_dir/REPORT.tex"
xelatex -output-directory="$render_dir" -halt-on-error -interaction=nonstopmode "$render_dir/REPORT.tex" > "$render_dir/build.stdout"
xelatex -output-directory="$render_dir" -halt-on-error -interaction=nonstopmode "$render_dir/REPORT.tex" >> "$render_dir/build.stdout"
pdfunite "$render_dir/REPORT.pdf" "$render_dir/typed_correspondence.pdf" \
  "$render_dir/typed_transfer_context.pdf" "$render_dir/joined.pdf"
node - "$render_dir/metadata.ps" <<'NODE'
const fs=require('fs');
const subject=fs.readFileSync('README.md','utf8').match(/^subject: "(.*)"$/m)[1];
const escape=s=>s.replaceAll('\\','\\\\').replaceAll('(','\\(').replaceAll(')','\\)');
fs.writeFileSync(process.argv[2],`[ /Title (Catalog-wide curve-cover transfer assessment) /Subject (${escape(subject)}) /DOCINFO pdfmark\n`);
NODE
gs -q -dBATCH -dNOPAUSE -sDEVICE=pdfwrite -dCompatibilityLevel=1.4 -sOutputFile="$render_dir/final.pdf" \
  "$render_dir/joined.pdf" "$render_dir/metadata.ps"
cp "$render_dir/final.pdf" cover_transfer_assessment_20261013.pdf
printf '%s\n' "$render_dir"
