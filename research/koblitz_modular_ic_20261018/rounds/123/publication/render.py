#!/usr/bin/env python3
"""Dependency-free evidence summary for R123."""
import json
from pathlib import Path

root = Path(__file__).resolve().parent
round_dir = root.parent
data = json.loads((round_dir/'result.json').read_text())
lines = ['R123: normalized x=0 branch census',
         '5 November 2026 | exact R116 and R119-R122 inputs',
         'Each F2 base place has one simple and one tame-seven place.']
for key in ('y0_e1','y0_e3','y1_e1','y1_e2'):
    row=data['base_places'][key]
    lines.append(f"{key}: base e={row['x_order_on_base']}, x degree={row['x_divisor_degree']}")
lines += ['Both places have F2 residue degree 2.',
          'Eight normalized places; x=0 divisor degree 112.',
          'Seven-edge residual relation is W^7=1 over F4.',
          'Independent degree audit and finite Lean arithmetic: pass.',
          'Next: evaluate N51 on these eight places.']

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
