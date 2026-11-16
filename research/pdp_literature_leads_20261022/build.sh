#!/bin/sh
# Render README.md to report.typ and report.pdf (research-visuals skill,
# "Rendering"). Document tooling only: pandoc (Markdown -> Typst) and the
# Typst compiler from a scratch virtualenv; computes nothing.
#
#   sh build.sh [path-to-python-with-typst]
#
# The Python interpreter must import `typst` (pip install typst). Figure 1 is
# placed on a landscape page so its labels stay legible.
set -eu
cd "$(dirname "$0")"
PY="${1:-python3}"
pandoc --wrap=none README.md -t typst -o body.typ.tmp
{
  printf '#set page(margin: 1.8cm)\n'
  printf '#set text(size: 9.5pt)\n'
  printf '#show table: set text(size: 7.5pt)\n'
  printf '#set table(stroke: 0.4pt + gray, inset: 3.5pt)\n'
  printf '#show image: it => align(center, it)\n'
  printf '#show figure.where(kind: image): set figure(supplement: none)\n'
  cat body.typ.tmp
} > report.typ
rm -f body.typ.tmp
# Put Figure 1 (the landscape diagram) on a flipped page at full width.
"$PY" - <<'EOF'
import re
src = open("report.typ").read()
pat = re.compile(r'#figure\(\[#image\("figures/pdp_landscape\.svg"\)\],\n  caption: \[(.*?)\]\n\)', re.S)
m = pat.search(src)
if not m:
    raise SystemExit("figure 1 block not found")
cap = m.group(1)
new = ('#page(flipped: true, margin: 1.2cm)[\n#figure([#image("figures/pdp_landscape.svg", width: 100%)],\n'
       '  caption: [' + cap + ']\n)\n]')
src = src[:m.start()] + new + src[m.end():]
src = src.replace('#figure([#image("figures/dreg_records.svg")]',
                  '#figure([#image("figures/dreg_records.svg", width: 100%)]')
open("report.typ", "w").write(src)
EOF
"$PY" -c "import typst; typst.compile('report.typ', output='report.pdf')"
echo "wrote report.typ and report.pdf"
