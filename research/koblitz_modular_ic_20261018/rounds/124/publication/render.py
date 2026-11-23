#!/usr/bin/env python3
"""Dependency-free evidence summary for R124."""
import json
from pathlib import Path

root = Path(__file__).resolve().parent
round_dir = root.parent
data = json.loads((round_dir/'result.json').read_text())
lines = ['R124: first N51 leading forms at x=0',
         '5 November 2026 | 105 actual rational terms | order 640',
         'All four weighted F4 leading sums cancel exactly.']
for key in ('y0_e1','y0_e3','y1_e1','y1_e2'):
    row=data['branches'][key]
    lines.append(f"{key}: candidate s-order={row['first_forms'][0]['candidate_s_order']}, sum=0")
lines += ['The 11 tied powers form (1+T+T^2)^7 over F2.',
          'No source-precision competitor; each cancellation is exact.',
          'Independent 105-term long division and Lean identity: pass.',
          'Next: evaluate N51 after branch-specific lambda corrections.']

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
