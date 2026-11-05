#!/usr/bin/env python3
"""One-page SVG/PDF view of the R130 normalized lambda-fiber census."""
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
r=json.loads((HERE.parent/'result.json').read_text())
groups=[('Y+123: 2 x deg 8',2,56,1),('Y+90: 2 x deg 8',2,56,2),
        ('Q4: 4 x deg 1',4,28,1),('Q4: 4 x deg 3',4,84,1)]
assert sum(n*f*e for _,n,f,e in groups)==r['x_fiber_degree_check']==784
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="820" height="350" viewBox="0 0 820 350">',
     '<rect width="820" height="350" fill="white"/>',
     '<text x="35" y="35" font-family="sans-serif" font-size="20">R130: twelve degree-7 lambda places</text>',
     '<text x="35" y="57" font-family="sans-serif" font-size="12">Bar width: total weighted F2 degree per factor group; sum 784</text>']
for i,(name,n,f,e) in enumerate(groups):
    y=82+i*58;w=n*f*e*0.9
    svg += [f'<text x="35" y="{y+20}" font-family="monospace" font-size="14">{name}</text>',
            f'<rect x="245" y="{y}" width="{w:.1f}" height="28" fill="{("#b76547" if e==2 else "#2b789e")}"/>',
            f'<text x="{255+w:.1f}" y="{y+20}" font-family="sans-serif" font-size="13">{n} places, f={f}, e={e}</text>']
svg += ['<text x="35" y="326" font-family="sans-serif" font-size="12">All three lambda derivatives are nonzero; N51 valuations are the next calculation.</text>','</svg>']
(HERE/'lambda-fiber.svg').write_text('\n'.join(svg)+'\n')
def esc(s):return s.replace('\\','\\\\').replace('(','\\(').replace(')','\\)')
cmd=[]
def txt(x,y,s,size=12):cmd.append(f'BT /F1 {size} Tf {x} {y} Td ({esc(s)}) Tj ET')
txt(45,741,'R130: degree-7 lambda fiber of the degree-112 source',17)
txt(45,712,'Twelve normalized places; all lambda derivatives are units')
txt(45,680,'Group',11);txt(248,680,'Weighted F2 degree',11)
for i,(name,n,f,e) in enumerate(groups):
    y=634-i*65;w=n*f*e*0.55
    txt(45,y+9,name,11)
    cmd.append('0.72 0.38 0.27 rg' if e==2 else '0.17 0.47 0.62 rg')
    cmd.append(f'248 {y} {w:.1f} 26 re f')
    txt(round(258+w),y+9,str(n*f*e),11)
txt(45,344,'Total weighted x-fiber degree: 784 = 112*7.',12)
txt(45,318,'Checks: independent factor degrees, source coefficients, byte replay, Lean sums.',10)
txt(45,292,'Result SHA-256: 27a0c684f1dd41bb02f33e6d2943626a11ee27832a424c58eee24d0f69383cf2',8)
txt(45,264,'Next: N51 valuations on these twelve places.',11)
content=('\n'.join(cmd)+'\n').encode()
objects=[b'<< /Type /Catalog /Pages 2 0 R >>',b'<< /Type /Pages /Kids [3 0 R] /Count 1 >>',
         b'<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>',
         b'<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>',
         f'<< /Length {len(content)} >>\nstream\n'.encode()+content+b'endstream']
pdf=bytearray(b'%PDF-1.4\n');offset=[0]
for i,obj in enumerate(objects,1):offset.append(len(pdf));pdf.extend(f'{i} 0 obj\n'.encode()+obj+b'\nendobj\n')
xref=len(pdf);pdf.extend(f'xref\n0 {len(offset)}\n0000000000 65535 f \n'.encode())
for n in offset[1:]:pdf.extend(f'{n:010d} 00000 n \n'.encode())
pdf.extend(f'trailer << /Size {len(offset)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n'.encode())
(HERE/'report.pdf').write_bytes(pdf)
print('LAMBDA130_REPORT_RENDER_PASS')
