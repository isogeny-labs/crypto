#!/bin/sh
# Rebuild E3.typ/E3.pdf and the run report's PDF from Markdown (pandoc + typst).
set -e
cd "$(dirname "$0")"
pandoc E3.md -o E3.typ
python3 - <<'PY'
import re
s = open('E3.typ').read()
s = re.sub(r'#figure\(image\("(e3/e3(?:_f4)?\.svg)"', r'#figure(image("\1", width: 100%', s)
s = re.sub(r'(#figure\(image\("e3/e3\.svg".*?\n\)\n)\n(#emph\[.*?\])\n',
           lambda m: '#page(flipped: true, margin: 1.2cm)[\n' + m.group(1) + '\n' + m.group(2) + '\n]\n', s, flags=re.S)
open('E3.typ', 'w').write('#set page(margin: 1.8cm)\n#set text(size: 9.5pt)\n' + s)
PY
typst compile --root . E3.typ E3.pdf
cd reports/run-20261019T2334Z
pandoc README.md -o report.typ
python3 - <<'PY'
import re
s = open('report.typ').read()
s = re.sub(r'#figure\(image\("e3\.svg"', '#figure(image("e3.svg", width: 100%', s)
s = re.sub(r'(#figure\(image\("e3\.svg".*?\n\)\n)\n(#emph\[.*?\])\n',
           lambda m: '#page(flipped: true, margin: 1.2cm)[\n' + m.group(1) + '\n' + m.group(2) + '\n]\n', s, flags=re.S)
open('report.typ', 'w').write('#set page(margin: 1.8cm)\n#set text(size: 9.5pt)\n' + s)
PY
typst compile --root . report.typ report.pdf
