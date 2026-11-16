#!/bin/sh
# Rebuild E5c.pdf and the E5c run report PDF from their Markdown (pandoc + typst).
set -e
cd "$(dirname "$0")"
for f in E5c reports/run-20261019T2240Z/README; do
  pandoc "$f.md" -o "$f.typ"
  python3 - "$f.typ" <<'PY'
import sys
p = sys.argv[1]
s = open(p).read()
s = s.replace('#figure(image("e5/e5c.svg"', '#figure(image("e5/e5c.svg", width: 100%')
s = s.replace('#figure(image("../../e5/e5c.svg"', '#figure(image("../../e5/e5c.svg", width: 100%')
s = '#set page(margin: 1.6cm)\n#set text(size: 9.5pt)\n' + s
open(p, 'w').write(s)
PY
  typst compile --root ../.. "$f.typ" "$f.pdf"
done
