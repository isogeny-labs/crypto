#!/bin/sh
# Rebuild E0_E4.typ and E0_E4.pdf from E0_E4.md (pandoc + typst); the figure goes on a landscape page.
set -e
cd "$(dirname "$0")"
pandoc E0_E4.md -o E0_E4.typ
python3 - <<'PY'
import re
s = open('E0_E4.typ').read()
s = s.replace('#figure(image("e4/e4.svg", alt:', '#figure(image("e4/e4.svg", width: 100%, alt:')
s = re.sub(r'(#figure\(image\("e4/e4\.svg".*?\n\)\n)\n(#emph\[.*?\])\n',
           lambda m: '#page(flipped: true, margin: 1.0cm)[\n' + m.group(1) + '\n' + m.group(2) + '\n]\n', s, flags=re.S)
s = '#set page(margin: 1.6cm)\n#set text(size: 9.5pt)\n' + s
open('E0_E4.typ', 'w').write(s)
PY
typst compile --root . E0_E4.typ E0_E4.pdf
