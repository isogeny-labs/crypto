#!/usr/bin/env python3
"""Render R133 derivative-support y-degree partitions as vector SVG and PDF."""
from pathlib import Path
import json

here=Path(__file__).resolve().parent
data=json.loads((here.parent/'result.json').read_text())
assert [(f['zero_derivative_y_degree'],f['nonzero_derivative_y_degree']) for f in data['fibers']]==[(1,6),(1,6)]
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="700" height="260" viewBox="0 0 700 260">',
     '<rect width="700" height="260" fill="white"/>',
     '<text x="34" y="38" font-family="sans-serif" font-size="20" font-weight="bold">R133 lambda derivative support</text>',
     '<text x="34" y="61" font-family="sans-serif" font-size="12">Base y-degree at each discriminant x-point; red requires local normalization</text>']
pdf=['1 1 1 rg 0 0 700 260 re f','0 0 0 rg BT /F1 19 Tf 34 222 Td (R133 lambda derivative support) Tj ET',
     'BT /F1 11 Tf 34 199 Td (Base y-degree; red requires local normalization) Tj ET']
for i,label in enumerate(('P11','P22')):
    y=100+70*i
    svg += [f'<text x="34" y="{y+24}" font-family="sans-serif" font-size="16">{label}</text>',
            f'<rect x="110" y="{y}" width="70" height="34" fill="#bd4d43"/>',
            f'<rect x="180" y="{y}" width="420" height="34" fill="#327b91"/>',
            f'<text x="137" y="{y+23}" font-family="sans-serif" font-size="15" fill="white">1</text>',
            f'<text x="378" y="{y+23}" font-family="sans-serif" font-size="15" fill="white">6</text>']
    py=260-y-34
    pdf += [f'0 0 0 rg BT /F1 16 Tf 34 {260-y-24} Td ({label}) Tj ET',
            f'0.741 0.302 0.263 rg 110 {py} 70 34 re f',
            f'0.196 0.482 0.569 rg 180 {py} 420 34 re f',
            f'1 1 1 rg BT /F1 15 Tf 137 {py+11} Td (1) Tj ET',
            f'BT /F1 15 Tf 378 {py+11} Td (6) Tj ET']
svg += ['<text x="34" y="237" font-family="sans-serif" font-size="11">Derivative-zero y-degree 1; derivative-unit y-degree 6 at each x-point.</text>','</svg>']
pdf += ['0 0 0 rg BT /F1 10 Tf 34 23 Td (Zero degree 1; unit degree 6 at each x-point.) Tj ET']
(here/'support133.svg').write_text('\n'.join(svg)+'\n')
stream='\n'.join(pdf).encode()
objects=[b'<< /Type /Catalog /Pages 2 0 R >>',b'<< /Type /Pages /Kids [3 0 R] /Count 1 >>',
         b'<< /Type /Page /Parent 2 0 R /MediaBox [0 0 700 260] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>',
         b'<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>',
         f'<< /Length {len(stream)} >>\nstream\n'.encode()+stream+b'\nendstream']
out=bytearray(b'%PDF-1.4\n');offsets=[0]
for i,obj in enumerate(objects,1):
    offsets.append(len(out));out+=f'{i} 0 obj\n'.encode()+obj+b'\nendobj\n'
xref=len(out);out+=f'xref\n0 {len(offsets)}\n0000000000 65535 f \n'.encode()
for offset in offsets[1:]:out+=f'{offset:010d} 00000 n \n'.encode()
out+=f'trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n'.encode()
(here/'support133.pdf').write_bytes(out)
print('RENDER133_PASS')
