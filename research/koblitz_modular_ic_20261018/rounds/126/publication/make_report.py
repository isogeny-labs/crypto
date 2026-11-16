#!/usr/bin/env python3
"""Render the R126 result as an editable SVG and a one-page vector PDF."""
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
rounddir = HERE.parent
r = json.loads((rounddir / 'result.json').read_text())
items = [(key, entries[0]['n51_normalized_order']) for key, entries in r['branches'].items()]
assert items == [('y0_e1', -141), ('y0_e3', -78), ('y1_e1', 42), ('y1_e2', 49)]

svg = ['<svg xmlns="http://www.w3.org/2000/svg" width="760" height="340" viewBox="0 0 760 340">',
       '<rect width="760" height="340" fill="white"/>',
       '<text x="38" y="36" font-family="sans-serif" font-size="20">R126: normalized N51 orders at x=0 tame-seven places</text>',
       '<text x="38" y="59" font-family="sans-serif" font-size="12">Exact local orders; F4 conjugate roots agree, n=2 roots per place</text>',
       '<line x1="370" y1="85" x2="370" y2="285" stroke="#444" stroke-width="1"/>']
for i, (key, value) in enumerate(items):
    y = 94 + i * 49
    x = 370 + value * 1.65
    svg += [f'<text x="38" y="{y+20}" font-family="monospace" font-size="16">{key}</text>',
            f'<rect x="{min(x,370):.1f}" y="{y}" width="{abs(x-370):.1f}" height="26" fill="{"#b55243" if value<0 else "#2b779d"}"/>',
            f'<text x="{x+(-32 if value<0 else 8):.1f}" y="{y+19}" font-family="sans-serif" font-size="14">{value}</text>']
svg += ['<text x="38" y="315" font-family="sans-serif" font-size="12">Source: rounds/126/result.json (SHA-256 40cb7e16...); units: normalized t-valuation</text>', '</svg>']
(HERE / 'orders.svg').write_text('\n'.join(svg) + '\n')

def esc(s): return s.replace('\\', '\\\\').replace('(', '\\(').replace(')', '\\)')
commands = []
def label(x, y, s, size=12):
    commands.append(f'BT /F1 {size} Tf {x} {y} Td ({esc(s)}) Tj ET')
label(45, 750, 'R126: N51 on the x=0 tame-seven places', 18)
label(45, 724, 'Exact normalized local orders; two F4 roots per residue-degree-2 place')
commands.append('0.25 0.25 0.25 RG 350 502 m 350 693 l S')
for i, (key, value) in enumerate(items):
    y = 661 - i * 48
    x = 350 + value * 1.2
    label(45, y+7, key, 14)
    commands.append('0.71 0.32 0.26 rg' if value < 0 else '0.17 0.47 0.62 rg')
    commands.append(f'{min(x,350):.1f} {y} {abs(x-350):.1f} 23 re f')
    label(round(x-32 if value < 0 else x+8), y+7, str(value), 13)
label(45, 465, 'Combined with R125 simple orders: x=0 degree-weighted contribution = -358')
label(45, 435, 'Checks: independent F4 lift, byte replay, changed-source rejection, Lean finite proof')
label(45, 410, 'Result SHA-256: 40cb7e162ee63d0ff61c3774e2a1c1e0893b7a61e7b18d4a4fd32e3437318672', 9)
label(45, 385, 'Source/protocol commit: a56e7b28ca9ededc8d7e62be2e4955b29a929445', 10)
label(45, 350, 'Next: full normalized N51 divisor at the other finite supports and infinity.', 11)
content = ('\n'.join(commands) + '\n').encode()
objects = [
    b'<< /Type /Catalog /Pages 2 0 R >>',
    b'<< /Type /Pages /Kids [3 0 R] /Count 1 >>',
    b'<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>',
    b'<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>',
    f'<< /Length {len(content)} >>\nstream\n'.encode() + content + b'endstream',
]
pdf = bytearray(b'%PDF-1.4\n')
offsets = [0]
for i, obj in enumerate(objects, 1):
    offsets.append(len(pdf))
    pdf.extend(f'{i} 0 obj\n'.encode() + obj + b'\nendobj\n')
xref = len(pdf)
pdf.extend(f'xref\n0 {len(offsets)}\n0000000000 65535 f \n'.encode())
for offset in offsets[1:]: pdf.extend(f'{offset:010d} 00000 n \n'.encode())
pdf.extend(f'trailer << /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n'.encode())
(HERE / 'report.pdf').write_bytes(pdf)
print('R126_REPORT_RENDER_PASS')
