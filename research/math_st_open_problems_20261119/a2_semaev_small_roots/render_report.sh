#!/bin/sh
# Render REPORT.md (+ success_vs_alpha.svg) to REPORT.pdf. Rendering only.
# Needs pandoc >= 3.1 and the typst Python wheel ($PY, default python3).
set -eu
cd "$(dirname "$0")"
tmp=$(mktemp -d)
pandoc REPORT.md -s -t typst -o "$tmp/report.typ"
sed -i 's/  font: (),/  font: ("Libertinus Serif", "New Computer Modern", "DejaVu Sans"),/; s/  fontsize: 11pt,/  fontsize: 9pt,/' "$tmp/report.typ"
printf '#set page(flipped: true)\n#set par(justify: false)\n' | cat - "$tmp/report.typ" > "$tmp/r.typ"
cp success_vs_alpha.svg "$tmp/"
"${PY:-python3}" -c "import typst,sys; typst.compile(sys.argv[1], output=sys.argv[2])" "$tmp/r.typ" REPORT.pdf
rm -rf "$tmp"
