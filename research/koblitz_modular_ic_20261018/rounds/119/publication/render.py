#!/usr/bin/env python3
"""Dependency-free evidence summary for R119."""
import json
from pathlib import Path

root = Path(__file__).resolve().parent
round_dir = root.parent
data = json.loads((round_dir/'result.json').read_text())
lines = ['R119: F4 translation of marked23 lambda edges',
         '5 November 2026 | exact R118 coefficients | two conjugate F4 roots',
         'Translate T=alpha+Z after the first certified Newton edge.']
for key in ('y0_e1', 'y0_e3', 'y1_e1', 'y1_e2'):
    rows = data['branches'][key][0]['translated_rows']
    lines.append(f"{key}: translated orders {[r['s_order'] for r in rows]}")
lines += ['Near each F4 root: y0/e1 and y0/e3 have 8 corrections of order 1.',
          'y1/e2 has 8 corrections of order 2.',
          'y1/e1 splits: one correction of order 5, seven of order 23/7.',
          'Independent F4 bit planes and finite Lean edge checks: pass.',
          'Next: lift the repeated eight-root edges and evaluate N51.']

def label(x, y, size, value):
    value = value.replace('\\', '\\\\').replace('(', '\\(').replace(')', '\\)')
    return f'BT /F1 {size} Tf {x} {y} Td ({value}) Tj ET\n'

content = '0 0 0 rg\n'
for i, line in enumerate(lines):
    content += label(35, 750-40*i, 16 if i == 0 else 10, line)
stream = content.encode('ascii')
objs = [b'<< /Type /Catalog /Pages 2 0 R >>',
        b'<< /Type /Pages /Kids [3 0 R] /Count 1 >>',
        b'<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>',
        b'<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>',
        f'<< /Length {len(stream)} >>\nstream\n'.encode()+stream+b'endstream']
pdf = bytearray(b'%PDF-1.4\n')
offsets = [0]
for i, obj in enumerate(objs, 1):
    offsets.append(len(pdf))
    pdf += f'{i} 0 obj\n'.encode()+obj+b'\nendobj\n'
xref = len(pdf)
pdf += f'xref\n0 {len(offsets)}\n0000000000 65535 f \n'.encode()
for offset in offsets[1:]:
    pdf += f'{offset:010d} 00000 n \n'.encode()
pdf += f'trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n'.encode()
(root/'report.pdf').write_bytes(pdf)
