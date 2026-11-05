#!/usr/bin/env python3
"""Render the R132 residue-degree census as SVG and a small vector PDF."""
from pathlib import Path
import json

here=Path(__file__).resolve().parent
rows=json.loads((here.parent/'result.json').read_text())['fibers']
items=[]
for fiber in rows:
    for part in fiber['base_factor_groups']:
        items.extend([(fiber['x_factor'],part['residue_degree_over_F2'])]*part['factor_count'])
assert items==[('P11',11),('P11',11),('P11',55),('P22',22),('P22',44),('P22',88)]
colors={'P11':'#226c9f','P22':'#b35d24'}
labels=['P11: 11','P11: 11','P11: 55','P22: 22','P22: 44','P22: 88']
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="720" height="430" viewBox="0 0 720 430">',
     '<rect width="720" height="430" fill="white"/>',
     '<text x="36" y="38" font-family="sans-serif" font-size="20" font-weight="bold">R132 base-place residue degrees</text>',
     '<text x="36" y="61" font-family="sans-serif" font-size="12">Six places over two lambda-discriminant x-points; bar length is degree over F2</text>']
pdf=['1 1 1 rg 0 0 720 430 re f','0 0 0 rg BT /F1 20 Tf 36 392 Td (R132 base-place residue degrees) Tj ET',
     'BT /F1 11 Tf 36 369 Td (Six places over P11 and P22; length is degree over F2) Tj ET']
for i,(point,value) in enumerate(items):
    y=100+i*47
    width=value*5.5
    color=colors[point]
    svg += [f'<text x="36" y="{y+19}" font-family="sans-serif" font-size="14">{labels[i]}</text>',
            f'<rect x="145" y="{y}" width="{width}" height="27" fill="{color}"/>']
    r,g,b=(int(color[j:j+2],16)/255 for j in (1,3,5))
    pdf += [f'0 0 0 rg BT /F1 14 Tf 36 {430-y-19} Td ({labels[i]}) Tj ET',
            f'{r:.4f} {g:.4f} {b:.4f} rg 145 {430-y-27} {width:.1f} 27 re f']
svg += ['<text x="36" y="409" font-family="sans-serif" font-size="11">Exact factorization; local base cover only.</text>','</svg>']
pdf += ['0 0 0 rg BT /F1 10 Tf 36 21 Td (Exact factorization; local base cover only.) Tj ET']
(here/'fibers132.svg').write_text('\n'.join(svg)+'\n')
stream='\n'.join(pdf).encode()
objects=[b'<< /Type /Catalog /Pages 2 0 R >>',
         b'<< /Type /Pages /Kids [3 0 R] /Count 1 >>',
         b'<< /Type /Page /Parent 2 0 R /MediaBox [0 0 720 430] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>',
         b'<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>',
         f'<< /Length {len(stream)} >>\nstream\n'.encode()+stream+b'\nendstream']
out=bytearray(b'%PDF-1.4\n')
offsets=[0]
for i,obj in enumerate(objects,1):
    offsets.append(len(out));out+=f'{i} 0 obj\n'.encode()+obj+b'\nendobj\n'
xref=len(out)
out+=f'xref\n0 {len(offsets)}\n0000000000 65535 f \n'.encode()
for offset in offsets[1:]:out+=f'{offset:010d} 00000 n \n'.encode()
out+=f'trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n'.encode()
(here/'fibers132.pdf').write_bytes(out)
print('CHART132_PASS')
