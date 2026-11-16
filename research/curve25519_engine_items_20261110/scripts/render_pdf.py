#!/usr/bin/env python3
"""README.md -> REPORT.typ (pandoc -t typst) -> REPORT.pdf (typst python module).
Usage: render_pdf.py <study dir>. Figures referenced as PNG from the README are embedded."""
import os, re, subprocess, sys
import typst
d = os.path.abspath(sys.argv[1])
body = subprocess.run(['pandoc', os.path.join(d, 'README.md'), '-t', 'typst', '--wrap=none'],
                      check=True, capture_output=True, text=True).stdout
head = ('#set page(paper: "a4", margin: 1.8cm, numbering: "1")\n#set text(size: 9.5pt)\n'
        '#set table(inset: 4pt, stroke: 0.4pt + gray)\n#show table: set text(size: 7.5pt)\n'
        '#show figure: set block(breakable: true)\n')
body = re.sub(r'#image\("([^"]+)"\)', lambda m: '#image("%s", width: 100%%)' % m.group(1), body)
open(os.path.join(d, 'REPORT.typ'), 'w').write(head + body)
typst.compile(os.path.join(d, 'REPORT.typ'), output=os.path.join(d, 'REPORT.pdf'), root=d)
print('wrote', os.path.join(d, 'REPORT.pdf'), os.path.getsize(os.path.join(d, 'REPORT.pdf')), 'bytes')
