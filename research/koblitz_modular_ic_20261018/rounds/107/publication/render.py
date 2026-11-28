#!/usr/bin/env python3
"""Exact R107 lambda Newton polygons as SVG and a one-page PDF."""
from pathlib import Path

root=Path(__file__).resolve().parent
panels=[("y=0",[28,23,23,0,0],"#2d6198"),("y=1",[27,23,23,0,0],"#b16124")]
degrees=[0,1,4,8,16]
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="390" viewBox="0 0 1000 390">',
 '<rect width="1000" height="390" fill="white"/>',
 '<text x="35" y="33" font-size="21" font-family="sans-serif">R107: exact lambda Newton polygons over the two x=1 base places</text>',
 '<text x="35" y="365" font-size="14" font-family="sans-serif">R107 lambda107-01 and fresh replay; orders exact below s^32; u = lambda + alpha over F4</text>']
for idx,(name,orders,color) in enumerate(panels):
 ox=45+idx*485
 svg += [f'<text x="{ox+15}" y="62" font-size="17" font-family="sans-serif" fill="{color}">{name}: slopes {5 if idx==0 else 4}, 23/7</text>',
         f'<path d="M{ox+25} 295 L{ox+435} 295 M{ox+25} 295 L{ox+25} 75" stroke="black" fill="none"/>']
 pts=[(ox+25+24*d,295-7*v) for d,v in zip(degrees,orders)]
 for a,b in ((0,1),(1,3),(3,4)):
  svg.append(f'<path d="M{pts[a][0]} {pts[a][1]} L{pts[b][0]} {pts[b][1]}" stroke="{color}" stroke-width="3"/>')
 for (x,y),d,v in zip(pts,degrees,orders):
  svg += [f'<circle cx="{x}" cy="{y}" r="5" fill="{color}"/>',
          f'<text x="{x-7}" y="316" font-size="13" font-family="sans-serif">{d}</text>']
 svg.append(f'<text x="{ox+330}" y="336" font-size="14" font-family="sans-serif">u exponent</text>')
svg.append('</svg>')
(root/'diagram.svg').write_text('\n'.join(svg)+'\n')


def label(x,y,size,value):
 value=value.replace('\\','\\\\').replace('(','\\(').replace(')','\\)')
 return f'BT /F1 {size} Tf {x} {y} Td ({value}) Tj ET\n'


c='0 0 0 rg\n'+label(40,750,18,'R107: four normalized places above x=1')
c+=label(40,724,10,'5 November 2026 | marked-23 Koblitz degree-112 source field')
for i,line in enumerate([
 'Exact u-coefficient orders: y=0 [28,23,23,0,0]; y=1 [27,23,23,0,0].',
 'Both polygons have a length-1 edge and a coprime-slope length-7 edge.',
 'Two F4-residue places lie over each base place: t-ramification 3,21,4,28.',
]):c+=label(40,685-18*i,10,line)
for idx,(name,orders,_) in enumerate(panels):
 ox=45+idx*280
 c+='0 0 0 RG 1 w\n'+f'{ox+18} 350 m {ox+255} 350 l S {ox+18} 350 m {ox+18} 520 l S\n'
 c+=('0.17 0.38 0.60 RG 2 w\n' if idx==0 else '0.69 0.36 0.14 RG 2 w\n')
 pts=[(ox+18+14*d,350+5*v) for d,v in zip(degrees,orders)]
 for a,b in ((0,1),(1,3),(3,4)):
  c+=f'{pts[a][0]} {pts[a][1]} m {pts[b][0]} {pts[b][1]} l S\n'
 for x,y in pts:c+=f'{x-2} {y-2} 4 4 re f\n'
 c+='0 0 0 rg\n'+label(ox+18,540,11,f'{name}: slope {5 if idx==0 else 4}, 23/7')
 c+=label(ox+18,330,9,'u exponents 0,1,4,8,16')
for i,line in enumerate([
 'Fresh replay, changed-hash rejection, independent Hasse composition,',
 'and Lean hull/degree arithmetic checked the result.',
 'Next: actual N51 valuation on degree-1 and degree-7 branches.',
 'Raw archive SHA-256:',
 '193db048167c2fd0e349a1af992095b008e8ea3425e9ee4243120d0c38f90e70',
]):c+=label(40,290-18*i,10,line)
stream=c.encode('ascii')
objects=[b'<< /Type /Catalog /Pages 2 0 R >>',b'<< /Type /Pages /Kids [3 0 R] /Count 1 >>',
 b'<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>',
 b'<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>',
 f'<< /Length {len(stream)} >>\nstream\n'.encode()+stream+b'endstream']
pdf=bytearray(b'%PDF-1.4\n');offsets=[0]
for n,obj in enumerate(objects,1):
 offsets.append(len(pdf));pdf+=f'{n} 0 obj\n'.encode()+obj+b'\nendobj\n'
xref=len(pdf);pdf+=f'xref\n0 {len(offsets)}\n0000000000 65535 f \n'.encode()
for off in offsets[1:]:pdf+=f'{off:010d} 00000 n \n'.encode()
pdf+=f'trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n'.encode()
(root/'report.pdf').write_bytes(pdf)
