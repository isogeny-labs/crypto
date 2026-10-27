#!/bin/sh
# Rebuild E5.typ and E5.pdf from E5.md (pandoc + typst); the figure goes on a landscape page.
set -e
cd "$(dirname "$0")"
pandoc E5.md -o E5.typ
python3 - <<'PY'
import re
s = open('E5.typ').read()
s = s.replace('#figure(image("e5/e5.svg", alt:', '#figure(image("e5/e5.svg", height: 84%, alt:')
s = re.sub(r'(#figure\(image\("e5/e5\.svg".*?\n\)\n)\n(#emph\[.*?\])\n',
           lambda m: '#page(flipped: true, margin: 1.2cm)[\n' + m.group(1) + '\n' + m.group(2) + '\n]\n', s, flags=re.S)
s = '#set page(margin: 1.6cm)\n#set text(size: 9.5pt)\n' + s
open('E5.typ', 'w').write(s)
PY
typst compile --root ../.. E5.typ E5.pdf
