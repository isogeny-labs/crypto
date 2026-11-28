#!/usr/bin/env python3
"""README.md -> report.html (tiny Markdown subset), figure inlined. Then: node render_pw.js -> report.pdf."""
import re, html
md = open('README.md').read().replace('![figure](fig_sibling.png)', '@@FIG@@')
def inline(s):
    s = html.escape(s)
    s = re.sub(r'\*\*(.+?)\*\*', r'<b>\1</b>', s)
    return re.sub(r'`([^`]+)`', r'<code>\1</code>', s)
lines = md.split('\n'); out = []; i = 0
while i < len(lines):
    l = lines[i]
    if l.startswith('|'):
        rows = []
        while i < len(lines) and lines[i].startswith('|'):
            rows.append(lines[i]); i += 1
        out.append('<table>')
        for k, r in enumerate(rows):
            if k == 1: continue
            tag = 'th' if k == 0 else 'td'
            out.append('<tr>' + ''.join(f'<{tag}>{inline(c.strip())}</{tag}>' for c in r.strip('|').split('|')) + '</tr>')
        out.append('</table>'); continue
    if l.startswith('- '):
        out.append('<ul>'); depth = 0
        while i < len(lines) and (lines[i].startswith('- ') or lines[i].startswith('  ')):
            s = lines[i]
            if s.startswith('  - '):
                if depth == 0: out[-1] = out[-1][:-5]; out.append('<ul>'); depth = 1
                out.append('<li>' + inline(s[4:]) + '</li>')
            elif s.startswith('- '):
                if depth == 1: out.append('</ul></li>'); depth = 0
                out.append('<li>' + inline(s[2:]) + '</li>')
            else:
                out[-1] = out[-1][:-5] + ' ' + inline(s.strip()) + '</li>'
            i += 1
        if depth == 1: out.append('</ul></li>')
        out.append('</ul>'); continue
    if l.startswith('#'):
        n = len(l) - len(l.lstrip('#')); out.append(f'<h{n}>' + inline(l[n+1:]) + f'</h{n}>'); i += 1; continue
    if l.strip() == '@@FIG@@':
        out.append(open('fig_sibling.svg').read().replace('<svg ', '<svg style="width:100%;height:auto" ', 1)); i += 1; continue
    if l.strip():
        para = []
        while i < len(lines) and lines[i].strip() and not lines[i].startswith(('#', '|', '- ', '@@')):
            para.append(lines[i].strip()); i += 1
        out.append('<p>' + inline(' '.join(para)) + '</p>'); continue
    i += 1
css = '''body{font-family:Helvetica,Arial,sans-serif;font-size:10.5px;line-height:1.35;color:#222;background:#fff;max-width:900px;margin:auto}
h1{font-size:17px}h2{font-size:14px;margin-top:16px}h3{font-size:12px}table{border-collapse:collapse;margin:6px 0;width:100%}
td,th{border:1px solid #ccc;padding:2px 4px;text-align:left;vertical-align:top}th{background:#f1f1f1}code{font-size:9.5px;background:#f5f5f5}li{margin:1px 0}'''
open('report.html', 'w').write(f'<!doctype html><html><head><meta charset="utf-8"><title>Genus 2/3 sibling walk</title><style>{css}</style></head><body>' + '\n'.join(out) + '</body></html>')
