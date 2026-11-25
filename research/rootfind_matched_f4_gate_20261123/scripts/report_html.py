#!/usr/bin/env python3
"""README.md -> report.html (tiny markdown subset) with figure.svg inlined."""
import html, re
from pathlib import Path
R = Path(__file__).resolve().parents[1]
md = (R / 'README.md').read_text().splitlines()
def inline(t):
    t = html.escape(t)
    t = re.sub(r'\*\*(.+?)\*\*', r'<b>\1</b>', t)
    t = re.sub(r'`([^`]+)`', r'<code>\1</code>', t)
    return t
out, i, lst = [], 0, None
while i < len(md):
    l = md[i]
    if l.startswith('|'):
        rows = []
        while i < len(md) and md[i].startswith('|'):
            rows.append([c.strip() for c in md[i].strip('|').split('|')]); i += 1
        out.append('<table><tr>' + ''.join(f'<th>{inline(c)}</th>' for c in rows[0]) + '</tr>' +
                   ''.join('<tr>' + ''.join(f'<td>{inline(c)}</td>' for c in r) + '</tr>' for r in rows[2:]) + '</table>')
        continue
    m = re.match(r'^(#+) (.*)', l)
    if m:
        n = len(m.group(1)); out.append(f'<h{n}>{inline(m.group(2))}</h{n}>')
        if n == 1:
            out.append((R / 'figure.svg').read_text().replace('width="980" height="830"', 'width="100%"'))
    elif l.startswith('- ') or l.startswith('  '):
        if l.startswith('- '):
            out.append(f'<li>{inline(l[2:])}')
        else:
            out[-1] += ' ' + inline(l.strip())
    elif l.strip():
        para = [l]
        while i + 1 < len(md) and md[i+1].strip() and not re.match(r'^(#|\||- )', md[i+1]):
            i += 1; para.append(md[i])
        out.append('<p>' + inline(' '.join(para)) + '</p>')
    i += 1
body = '\n'.join(out)
body = re.sub(r'((?:<li>.*\n?)+)', lambda m: '<ul>' + m.group(1) + '</ul>', body)
(R / 'report.html').write_text(f'''<!doctype html><html><head><meta charset="utf-8"><title>Program B matched F4 gate</title>
<style>body{{font-family:Helvetica,Arial,sans-serif;font-size:11px;color:#18181b;background:#fff;max-width:900px;margin:auto}}
table{{border-collapse:collapse;margin:6px 0}}td,th{{border:1px solid #d4d4d8;padding:2px 6px;text-align:left}}code{{font-size:10px}}h1{{font-size:18px}}h2{{font-size:14px}}</style>
</head><body>{body}</body></html>''')
print('ok')
