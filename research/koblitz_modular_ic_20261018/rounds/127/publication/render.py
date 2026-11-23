#!/usr/bin/env python3
"""Render the R127 source-order factor census with only stdlib."""
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
r=json.loads((HERE.parent/'result.json').read_text())
assert r['rational_roots']==[[90,2],[123,1]]
labels=[('Y+123',1,'simple F128 root'),('Y+90',2,'double F128 root'),('Q4',4,'irreducible quartic')]
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="760" height="330" viewBox="0 0 760 330">',
     '<rect width="760" height="330" fill="white"/>',
     '<text x="35" y="35" font-size="20" font-family="sans-serif">R127: degree-7 base fiber over F128</text>',
     '<text x="35" y="56" font-size="12" font-family="sans-serif">Bars: factor degree times source-order multiplicity; total 7</text>']
for i,(name,n,desc) in enumerate(labels):
    y=87+i*65
    svg += [f'<text x="35" y="{y+20}" font-size="16" font-family="monospace">{name}</text>',
            f'<rect x="160" y="{y}" width="{n*90}" height="30" fill="{("#be7654" if i==1 else "#397e9d")}"/>',
            f'<text x="{170+n*90}" y="{y+20}" font-size="14" font-family="sans-serif">{n}: {desc}</text>']
svg += ['<text x="35" y="305" font-size="12" font-family="sans-serif">Source: rounds/127/result.json; normalized ramification remains to be computed.</text>','</svg>']
(HERE/'fiber.svg').write_text('\n'.join(svg)+'\n')

def esc(s):return s.replace('\\','\\\\').replace('(','\\(').replace(')','\\)')
cmd=[]
def txt(x,y,s,size=12):cmd.append(f'BT /F1 {size} Tf {x} {y} Td ({esc(s)}) Tj ET')
txt(45,740,'R127: degree-7 marked-23 base fiber',18)
txt(45,711,'F128 = F2[a] / (a^7+a^6+a^3+a+1)')
txt(45,684,'f23(a,Y) = (Y+123)(Y+90)^2 Q4(Y)')
txt(45,658,'Q4 = Y^4+66Y^3+83Y^2+38Y+25; irreducible over F128',11)
txt(45,625,'Factor',12);txt(190,625,'Length',12);txt(400,625,'Interpretation',12)
for i,(name,n,desc) in enumerate(labels):
    y=586-i*57
    txt(45,y+8,name,13)
    cmd.append('0.72 0.37 0.25 rg' if i==1 else '0.20 0.47 0.61 rg')
    cmd.append(f'190 {y} {n*35} 25 re f')
    txt(205+n*35,y+8,str(n),13)
    txt(400,y+8,desc,11)
txt(45,360,'Source-order length 1+2+4=7 over F128; normalized ramification unresolved.',11)
txt(45,335,'Independent GF128 check, Lean product, byte replay, changed-source gate passed.',10)
txt(45,310,'Result SHA-256: 87b87127fab784f9bf48239081194a9ebcf67456cb4204c3e1d53956e9221e30',8)
txt(45,280,'Next: Newton expansion at the double root Y=90.',11)
content=('\n'.join(cmd)+'\n').encode()
objs=[b'<< /Type /Catalog /Pages 2 0 R >>',b'<< /Type /Pages /Kids [3 0 R] /Count 1 >>',
      b'<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>',
      b'<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>',
      f'<< /Length {len(content)} >>\nstream\n'.encode()+content+b'endstream']
pdf=bytearray(b'%PDF-1.4\n');offsets=[0]
for i,obj in enumerate(objs,1):
    offsets.append(len(pdf));pdf.extend(f'{i} 0 obj\n'.encode()+obj+b'\nendobj\n')
xref=len(pdf);pdf.extend(f'xref\n0 {len(offsets)}\n0000000000 65535 f \n'.encode())
for off in offsets[1:]:pdf.extend(f'{off:010d} 00000 n \n'.encode())
pdf.extend(f'trailer << /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n'.encode())
(HERE/'report.pdf').write_bytes(pdf)
print('FIBER127_REPORT_RENDER_PASS')
