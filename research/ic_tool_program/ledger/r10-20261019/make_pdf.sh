#!/usr/bin/env bash
# Rebuild ledger.svg/png and report.pdf from README.md and data/ (run rerate.sh first).
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"; t="$(mktemp -d)"
MPLCONFIGDIR="$t" python3 "$here/plot.py"
sed 's/ledger\.svg/ledger.png/; s/\^/\\^/g' "$here/README.md" > "$t/r10.md"
cp "$here/ledger.png" "$t/"
pandoc "$t/r10.md" -t typst -o "$t/body.typ"
{ printf '#set page(paper: "a4", flipped: true, margin: 1.2cm)\n#set text(size: 8pt)\n#set table(inset: 3pt)\n#show figure: set block(breakable: true)\n'; cat "$t/body.typ"; } > "$t/report.typ"
typst compile "$t/report.typ" "$here/report.pdf"
