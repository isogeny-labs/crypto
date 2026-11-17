#!/bin/sh
# Rebuild DRYRUN.typ and DRYRUN.pdf from DRYRUN.md (pandoc + typst); the figures go on landscape pages.
set -e
cd "$(dirname "$0")"
pandoc DRYRUN.md -o DRYRUN.typ
python3 - <<'PY'
import re
s = open('DRYRUN.typ').read()
s = s.replace('#figure(image("dryrun/dryrun.svg", alt:', '#figure(image("dryrun/dryrun.svg", width: 100%, alt:')
s = s.replace('#figure(image("dryrun/dryrun2.svg", alt:', '#figure(image("dryrun/dryrun2.svg", width: 100%, alt:')
s = re.sub(r'(#figure\(image\("dryrun/dryrun2?\.svg".*?\n\)\n)\n(#emph\[.*?\](?: #strong\[.*?\])?)\n',
           lambda m: '#page(flipped: true, margin: 1.2cm)[\n' + m.group(1) + '\n' + m.group(2) + '\n]\n', s, flags=re.S)
open('DRYRUN.typ', 'w').write(s)
PY
typst compile --root . DRYRUN.typ DRYRUN.pdf
