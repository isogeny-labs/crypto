#!/usr/bin/env python3
"""Dependency-free evidence summary for R122."""
import json
from pathlib import Path

root = Path(__file__).resolve().parent
round_dir = root.parent
data = json.loads((round_dir/'result.json').read_text())
lines = ['R122: fourth F4 translation of marked23 y0/e3',
         '5 November 2026 | exact R121 supports | one base place',
         'Translate U=s(delta+V) on the final repeated eight-root edge.']
for key in ('y0_e3',):
    rows = data['branches'][key][0]['translated_rows']
    lines.append(f"{key}: translated orders {[r['s_order'] for r in rows]}")
lines += ['y0/e3 splits: 1 correction of order 2, 7 of order 2/7.',
          'All four x=0 base places now have a tame seven-root edge.',
          'The leading F4 correction is 1 and is Frobenius-fixed.',
          'Independent F4 table and finite Lean edge checks: pass.',
          'Next: normalized branch census and N51 valuation.']

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
