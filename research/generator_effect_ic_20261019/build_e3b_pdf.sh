#!/bin/sh
# Rebuild E3b.typ/E3b.pdf and the run report's PDF from Markdown (pandoc + typst).
set -e
cd "$(dirname "$0")"
pandoc E3b.md -o E3b.typ
python3 - <<'PY'
import re
s = open('E3b.typ').read()
s = re.sub(r'#figure\(image\("(e3b/e3b_(?:ladder|f4)\.svg)"', r'#figure(image("\1", width: 100%', s)
s = re.sub(r'(#figure\(image\("e3b/e3b_ladder\.svg".*?\n\)\n)\n(#emph\[.*?\])\n',
           lambda m: '#page(flipped: true, margin: 1.2cm)[\n' + m.group(1) + '\n' + m.group(2) + '\n]\n', s, flags=re.S)
open('E3b.typ', 'w').write('#set page(margin: 1.8cm)\n#set text(size: 8.5pt)\n' + s)
PY
typst compile --root . E3b.typ E3b.pdf
for d in reports/run-*-e3b; do
  [ -d "$d" ] || continue
  (cd "$d"
   pandoc README.md -o report.typ
   python3 - <<'PY'
import re
s = open('report.typ').read()
s = re.sub(r'#figure\(image\("(e3b_ladder\.svg)"', r'#figure(image("\1", width: 100%', s)
s = re.sub(r'(#figure\(image\("e3b_ladder\.svg".*?\n\)\n)\n(#emph\[.*?\])\n',
           lambda m: '#page(flipped: true, margin: 1.2cm)[\n' + m.group(1) + '\n' + m.group(2) + '\n]\n', s, flags=re.S)
open('report.typ', 'w').write('#set page(margin: 1.8cm)\n#set text(size: 8.5pt)\n' + s)
PY
   typst compile --root . report.typ report.pdf)
done
