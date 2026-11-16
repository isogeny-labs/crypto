#!/bin/sh
# Rebuild E2.typ and E2.pdf from E2.md (pandoc + typst); the figure goes on a landscape page.
set -e
cd "$(dirname "$0")"
pandoc E2.md -o E2.typ
python3 - <<'PY'
import re
s = open('E2.typ').read()
s = s.replace('#figure(image("e2/e2.svg", alt:', '#figure(image("e2/e2.svg", width: 100%, alt:')
s = re.sub(r'(#figure\(image\("e2/e2\.svg".*?\n\)\n)\n(#emph\[.*?\])\n',
           lambda m: '#page(flipped: true, margin: 1.2cm)[\n' + m.group(1) + '\n' + m.group(2) + '\n]\n', s, flags=re.S)
open('E2.typ', 'w').write(s)
PY
typst compile --root . E2.typ E2.pdf
