#!/usr/bin/env python3
"""Render the recorded Markdown and diagram; performs no research computation."""
from pathlib import Path
import json
import re
import subprocess
import typst

root = Path(__file__).resolve().parent
subprocess.run(['dot', '-Tsvg', str(root / 'dependency.dot'), '-o', str(root / 'dependency.svg')], check=True)
quote = lambda value: json.dumps(value, ensure_ascii=False)
def clean(value):
    value = re.sub(r'\[([^\]]+)\]\(([^)]+)\)', r'\1 (\2)', value)
    return value.replace('**', '').replace('`', '')
lines = ['#set page(paper: "a4", margin: 16mm, numbering: "1")',
         '#set text(font: "DejaVu Sans", size: 8.3pt)',
         '#set par(justify: false, leading: 0.55em)',
         '#show heading: set text(fill: rgb("#17324f"))']
for name in ['README.md', 'BRIDGE.md']:
    if name == 'BRIDGE.md':
        lines += ['#pagebreak()']
    src = (root / name).read_text().splitlines()
    i = 0
    while i < len(src):
        line = src[i]
        if not line.strip():
            i += 1
            continue
        if line.startswith('!['):
            lines += ['#figure(image("dependency.svg", width: 100%), caption: [Evidence dependencies; no timing data.])']
            i += 1
            continue
        if line.startswith('#'):
            level = len(line) - len(line.lstrip('#'))
            title = line[level:].strip()
            lines += ['#heading(level: ' + str(level) + ', ' + quote(title) + ')']
            i += 1
            continue
        if line.startswith('|'):
            rows = []
            while i < len(src) and src[i].startswith('|'):
                cells = [clean(v.strip()) for v in src[i].strip('|').split('|')]
                if not all(re.fullmatch(r'[-: ]+', v) for v in cells):
                    rows.append(cells)
                i += 1
            lines += ['#set text(size: 7.2pt)', '#table(columns: (' + ','.join('1fr' for _ in rows[0]) + '), inset: 4pt, stroke: 0.3pt + luma(180),']
            for index, row in enumerate(rows):
                cells = ', '.join('[' + '#text(' + quote(v) + ')' + ']' for v in row)
                lines += [('  table.header(' + cells + '),' if index == 0 else '  ' + cells + ',')]
            lines += [')', '#set text(size: 8.3pt)']
            continue
        para = [line]
        i += 1
        while i < len(src) and src[i].strip() and not src[i].startswith(('#', '|', '![')):
            para.append(src[i])
            i += 1
        lines += ['#text(' + quote(clean(' '.join(para))) + ')', '']
(root / 'report.typ').write_text('\n'.join(lines).rstrip() + '\n')
typst.compile(str(root / 'report.typ'), output=str(root / 'report.pdf'), root=str(root))
