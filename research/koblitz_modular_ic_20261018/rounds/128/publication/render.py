#!/usr/bin/env python3
"""Render R128 Newton data as SVG and one-page vector PDF."""
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
r=json.loads((HERE.parent/'result.json').read_text())
assert r['hull']==[[0,1],[2,0],[7,0]]
points=[(e['u_exponent'],e['s_order']) for e in r['by_u']]
X=lambda j:100+70*j
Y=lambda v:210-80*v
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="760" height="330" viewBox="0 0 760 330">',
     '<rect width="760" height="330" fill="white"/>',
     '<text x="36" y="35" font-size="20" font-family="sans-serif">R128: F128 Newton hull at the degree-7 double root</text>',
     '<text x="36" y="56" font-size="12" font-family="sans-serif">Horizontal: u exponent; vertical: first s order</text>',
     '<line x1="100" y1="210" x2="620" y2="210" stroke="#555"/>',
     '<line x1="100" y1="210" x2="100" y2="92" stroke="#555"/>',
     '<polyline points="100,130 240,210 590,210" fill="none" stroke="#be5b45" stroke-width="3"/>']
for j,v in points:
    svg.append(f'<circle cx="{X(j)}" cy="{Y(v)}" r="5" fill="#267397"/>')
    svg.append(f'<text x="{X(j)-3}" y="237" font-size="12" font-family="sans-serif">{j}</text>')
svg += ['<text x="110" y="95" font-size="13" font-family="sans-serif">edge: 84 + 110 Z², slope -1/2</text>',
        '<text x="36" y="289" font-size="12" font-family="sans-serif">Transverse coefficient 84 is a unit; direct parameter candidate u=y+90.</text>','</svg>']
(HERE/'newton.svg').write_text('\n'.join(svg)+'\n')

def esc(s):return s.replace('\\','\\\\').replace('(','\\(').replace(')','\\)')
c=[]
def txt(x,y,s,size=12):c.append(f'BT /F1 {size} Tf {x} {y} Td ({esc(s)}) Tj ET')
txt(45,740,'R128: Newton expansion at the degree-7 double root',17)
txt(45,713,'F(s,u)=f23(a+s,90+u) over F128; 52 nonzero terms')
txt(45,684,'First s-orders by u-degree: 1,1,0,0,0,1,0,0')
txt(45,661,'Lower hull: (0,1) -> (2,0) -> (7,0)')
txt(45,635,'Positive edge residual: 84 + 110 Z^2; slope -1/2')
c += ['0.3 0.3 0.3 RG 90 440 m 90 600 l S','90 440 m 550 440 l S',
      '0.75 0.35 0.27 RG 90 560 m 210 440 l 510 440 l S']
for j,v in points:
    x=90+60*j;y=440+120*v
    c.append(f'0.17 0.45 0.60 rg {x-4} {y-4} 8 8 re f')
    txt(x-2,421,str(j),10)
txt(45,385,'Transverse derivative coefficient: 84 != 0 in F128; 84*6 = 110.')
txt(45,358,'Next: lift s(u) directly, then certify normalized ramification and lambda branches.',10)
txt(45,332,'Checks: independent Horner, byte replay, changed-source rejection, Lean finite claims.',9)
txt(45,307,'Result SHA-256: cce71416b01d600abbeedbd86f56f647c6c91b7d2ee536d6f730d5e9c79a739d',8)
content=('\n'.join(c)+'\n').encode()
objs=[b'<< /Type /Catalog /Pages 2 0 R >>',b'<< /Type /Pages /Kids [3 0 R] /Count 1 >>',
      b'<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>',
      b'<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>',
      f'<< /Length {len(content)} >>\nstream\n'.encode()+content+b'endstream']
pdf=bytearray(b'%PDF-1.4\n');offset=[0]
for i,obj in enumerate(objs,1):offset.append(len(pdf));pdf.extend(f'{i} 0 obj\n'.encode()+obj+b'\nendobj\n')
xref=len(pdf);pdf.extend(f'xref\n0 {len(offset)}\n0000000000 65535 f \n'.encode())
for o in offset[1:]:pdf.extend(f'{o:010d} 00000 n \n'.encode())
pdf.extend(f'trailer << /Size {len(offset)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n'.encode())
(HERE/'report.pdf').write_bytes(pdf)
print('NEWTON128_REPORT_RENDER_PASS')
