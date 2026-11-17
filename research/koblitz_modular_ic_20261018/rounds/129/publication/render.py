#!/usr/bin/env python3
"""Render the R129 normalized base fiber census in SVG and PDF."""
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
r=json.loads((HERE.parent/'result.json').read_text())
assert [(x['residue_degree_over_F2'],x['ramification_index']) for x in r['base_places']]==[(7,1),(7,2),(28,1)]
labels=[('Y+123',7,1),('Y+90',7,2),('Q4',28,1)]
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="760" height="340" viewBox="0 0 760 340">',
     '<rect width="760" height="340" fill="white"/>',
     '<text x="35" y="35" font-family="sans-serif" font-size="20">R129: normalized degree-7 base fiber</text>',
     '<text x="35" y="58" font-family="sans-serif" font-size="12">Bar width is residue degree times ramification index over F2; total 49</text>']
for i,(name,f,e) in enumerate(labels):
    y=88+i*70;w=f*e*11
    svg += [f'<text x="35" y="{y+20}" font-family="monospace" font-size="16">{name}</text>',
            f'<rect x="165" y="{y}" width="{w}" height="29" fill="{("#ba684a" if e==2 else "#2d789d")}"/>',
            f'<text x="{175+w}" y="{y+20}" font-family="sans-serif" font-size="14">f={f}, e={e}, ef={f*e}</text>']
svg += ['<text x="35" y="315" font-family="sans-serif" font-size="12">Direct lift: x-a = 6u² + 21u³ + ...; u=y+90 uniformizes the middle branch.</text>','</svg>']
(HERE/'base-fiber.svg').write_text('\n'.join(svg)+'\n')

def esc(s):return s.replace('\\','\\\\').replace('(','\\(').replace(')','\\)')
c=[]
def txt(x,y,s,size=12):c.append(f'BT /F1 {size} Tf {x} {y} Td ({esc(s)}) Tj ET')
txt(45,740,'R129: normalized degree-7 base fiber',18)
txt(45,712,'Direct lift: s=x-a=6u^2+21u^3+...; u=y+90')
txt(45,687,'F(s(u),u)=0 mod u^128; transverse derivative 84 is a unit')
txt(45,653,'Source factor',12);txt(175,653,'Weighted degree',12);txt(400,653,'(residue degree, e)',12)
for i,(name,f,e) in enumerate(labels):
    y=611-i*64;w=f*e*5
    txt(45,y+9,name,13)
    c.append('0.72 0.37 0.27 rg' if e==2 else '0.17 0.47 0.62 rg')
    c.append(f'175 {y} {w} 27 re f')
    txt(185+w,y+9,str(f*e),12)
    txt(400,y+9,f'({f},{e})',13)
txt(45,381,'Complete base-fiber degree: 7+14+28=49.',12)
txt(45,351,'Checks: independent direct-source lift, 128 residual coefficients, Lean arithmetic.',10)
txt(45,325,'First 16-update cap failed before output; amended source converged in 33 updates.',10)
txt(45,298,'Result SHA-256: 386eb38c51c8f0da0bb8d184ab91709bb1865235d6f6ea2068292713bd8d9e52',8)
txt(45,271,'Next: degree-16 lambda extension and N51 at these three base places.',10)
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
print('LIFT129_REPORT_RENDER_PASS')
