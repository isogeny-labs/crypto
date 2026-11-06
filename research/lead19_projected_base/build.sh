#!/bin/sh
# Render README.md to README.typ and README.pdf (research-visuals skill,
# "Rendering"): pandoc (Markdown -> Typst) and the Typst compiler from a
# Python interpreter that imports `typst`.  Document tooling only.
#
#   sh build.sh [path-to-python-with-typst]
set -eu
cd "$(dirname "$0")"
PY="${1:-python3}"
pandoc --wrap=none README.md -t typst -o body.typ.tmp
{
  printf '#set page(margin: 1.5cm)\n'
  printf '#set text(size: 9pt)\n'
  printf '#show table: set text(size: 6.5pt)\n'
  printf '#set table(stroke: 0.4pt + gray, inset: 3pt)\n'
  printf '#set image(width: 100%%)\n'
  printf '#show figure.where(kind: image): set figure(supplement: none)\n'
  cat body.typ.tmp
} > README.typ
rm -f body.typ.tmp
"$PY" -c "import typst; typst.compile('README.typ', output='README.pdf')"
echo "wrote README.typ and README.pdf"
