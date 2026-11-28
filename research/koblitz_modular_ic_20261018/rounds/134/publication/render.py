#!/usr/bin/env python3
"""Render R134 residue-degree groups as an SVG and a compact vector PDF."""
from pathlib import Path
import json

here=Path(__file__).resolve().parent
r=json.loads((here.parent/'result.json').read_text())
rows=[]
for fiber in r['fibers']:
    for part in fiber['degree_parts']:
        rows.append((fiber['x_factor'],fiber['base_residue_degree_over_F2'],part['count'],part['residue_degree_over_F2']))
assert rows==[('P11',11,4,44),('P11',55,4,55),('P11',55,4,165),('P22',44,4,176),('P22',88,2,176),('P22',88,2,528)]
color={'P11':'#286d9f','P22':'#b5682c'}
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="780" height="440" viewBox="0 0 780 440">',
     '<rect width="780" height="440" fill="white"/>',
     '<text x="30" y="38" font-family="sans-serif" font-size="20" font-weight="bold">R134 ordinary lambda-place residue degrees</text>',
     '<text x="30" y="61" font-family="sans-serif" font-size="12">Bar length is F2 residue degree; label gives x-point, base degree and place count</text>']
pdf=['1 1 1 rg 0 0 780 440 re f',
     '0 0 0 rg BT /F1 19 Tf 30 402 Td (R134 ordinary lambda-place residue degrees) Tj ET',
     'BT /F1 11 Tf 30 379 Td (Bar length is F2 residue degree; label gives base degree and count.) Tj ET']
for i,(point,base,count,degree) in enumerate(rows):
    y=93+i*49
    label=f'{point}/{base}: {count} x {degree}'
    width=degree*0.76
    col=color[point]
    svg += [f'<text x="30" y="{y+20}" font-family="sans-serif" font-size="13">{label}</text>',
            f'<rect x="310" y="{y}" width="{width:.1f}" height="27" fill="{col}"/>']
    rr,gg,bb=(int(col[k:k+2],16)/255 for k in (1,3,5))
    pdf += [f'0 0 0 rg BT /F1 13 Tf 30 {440-y-20} Td ({label}) Tj ET',
            f'{rr:.4f} {gg:.4f} {bb:.4f} rg 310 {440-y-27} {width:.1f} 27 re f']
svg += ['<text x="30" y="420" font-family="sans-serif" font-size="11">20 places; degree-weighted total 3168. Exceptional base places remain separate.</text>','</svg>']
pdf += ['0 0 0 rg BT /F1 10 Tf 30 20 Td (20 places; weighted degree 3168. Exceptional places remain separate.) Tj ET']
(here/'ordinary134.svg').write_text('\n'.join(svg)+'\n')
stream='\n'.join(pdf).encode()
objects=[b'<< /Type /Catalog /Pages 2 0 R >>',b'<< /Type /Pages /Kids [3 0 R] /Count 1 >>',
         b'<< /Type /Page /Parent 2 0 R /MediaBox [0 0 780 440] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>',
         b'<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>',
         f'<< /Length {len(stream)} >>\nstream\n'.encode()+stream+b'\nendstream']
out=bytearray(b'%PDF-1.4\n');offsets=[0]
for i,obj in enumerate(objects,1):
    offsets.append(len(out));out+=f'{i} 0 obj\n'.encode()+obj+b'\nendobj\n'
xref=len(out);out+=f'xref\n0 {len(offsets)}\n0000000000 65535 f \n'.encode()
for offset in offsets[1:]:out+=f'{offset:010d} 00000 n \n'.encode()
out+=f'trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n'.encode()
(here/'ordinary134.pdf').write_bytes(out)
print('RENDER134_PASS')
