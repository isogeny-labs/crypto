#!/bin/sh
# Render A3_P5.2_Q025.md (+ a3_reduction.svg) to A3_P5.2_Q025.pdf. Rendering only.
# Needs pandoc >= 3.1 and the typst Python wheel ($PY, default python3).
set -eu
cd "$(dirname "$0")"
tmp=$(mktemp -d)
pandoc A3_P5.2_Q025.md -s -t typst -o "$tmp/report.typ"
sed -i 's/  font: (),/  font: ("Libertinus Serif", "New Computer Modern", "DejaVu Sans"),/; s/  fontsize: 11pt,/  fontsize: 9pt,/; s/angle\.l/chevron.l/g; s/angle\.r/chevron.r/g' "$tmp/report.typ"
printf '#set page(flipped: true)\n#set par(justify: false)\n' | cat - "$tmp/report.typ" > "$tmp/r.typ"
cp a3_reduction.svg "$tmp/"
"${PY:-python3}" -c "import typst,sys; typst.compile(sys.argv[1], output=sys.argv[2])" "$tmp/r.typ" A3_P5.2_Q025.pdf
rm -rf "$tmp"
