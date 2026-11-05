#!/usr/bin/env python3
"""Render the R131 twelve-place unit census as SVG and PDF."""
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
r=json.loads((HERE.parent/'result.json').read_text())
groups=[('Y+123, deg 8',2),('Y+90, deg 8',2),('Q4, deg 1',4),('Q4, deg 3',4)]
assert r['unit_place_count']==sum(n for _,n in groups)==12 and r['zero_residue_place_count']==0
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="760" height="350" viewBox="0 0 760 350">',
     '<rect width="760" height="350" fill="white"/>',
     '<text x="35" y="35" font-family="sans-serif" font-size="20">R131: N51 residues at twelve degree-7 places</text>',
     '<text x="35" y="58" font-family="sans-serif" font-size="12">Blue bars count certified unit places; zero-residue count is zero in each group</text>']
for i,(name,n) in enumerate(groups):
    y=80+i*57
    svg += [f'<text x="35" y="{y+20}" font-family="monospace" font-size="15">{name}</text>',
            f'<rect x="220" y="{y}" width="{n*95}" height="28" fill="#27779a"/>',
            f'<text x="{230+n*95}" y="{y+20}" font-family="sans-serif" font-size="14">{n} / {n} units</text>']
svg += ['<text x="35" y="327" font-family="sans-serif" font-size="12">All 105 rational denominators are units; degree-7 contribution to div(N51) is zero.</text>','</svg>']
(HERE/'n51-units.svg').write_text('\n'.join(svg)+'\n')
def esc(s):return s.replace('\\','\\\\').replace('(','\\(').replace(')','\\)')
c=[]
def txt(x,y,s,size=12):c.append(f'BT /F1 {size} Tf {x} {y} Td ({esc(s)}) Tj ET')
txt(45,741,'R131: N51 is a unit at all twelve degree-7 places',17)
txt(45,712,'105 rational terms; every denominator is a unit at the x-residue')
txt(45,682,'Factor group',12);txt(248,682,'Certified unit places',12)
for i,(name,n) in enumerate(groups):
    y=636-i*62
    txt(45,y+9,name,12)
    c.append(f'0.16 0.47 0.60 rg 248 {y} {n*50} 26 re f')
    txt(260+n*50,y+9,f'{n}/{n}',12)
txt(45,337,'Total: 12/12 units, 0 zero residues; local divisor contribution = 0.',11)
txt(45,311,'Checks: independent rational evaluation and gcd, byte replay, Lean counts.',10)
txt(45,284,'Result SHA-256: 2b3180791d30cc2e659b02f6b738b2cfd25b619e2fd5ca7e1e1d63cf49d5f00b',8)
txt(45,257,'Next: degree-11 and degree-22 supports, then infinity.',11)
content=('\n'.join(c)+'\n').encode()
objs=[b'<< /Type /Catalog /Pages 2 0 R >>',b'<< /Type /Pages /Kids [3 0 R] /Count 1 >>',
      b'<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>',
      b'<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>',
      f'<< /Length {len(content)} >>\nstream\n'.encode()+content+b'endstream']
pdf=bytearray(b'%PDF-1.4\n');off=[0]
for i,obj in enumerate(objs,1):off.append(len(pdf));pdf.extend(f'{i} 0 obj\n'.encode()+obj+b'\nendobj\n')
xref=len(pdf);pdf.extend(f'xref\n0 {len(off)}\n0000000000 65535 f \n'.encode())
for n in off[1:]:pdf.extend(f'{n:010d} 00000 n \n'.encode())
pdf.extend(f'trailer << /Size {len(off)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n'.encode())
(HERE/'report.pdf').write_bytes(pdf)
print('RESIDUE131_REPORT_RENDER_PASS')
