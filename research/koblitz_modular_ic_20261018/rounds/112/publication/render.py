#!/usr/bin/env python3
"""Dependency-free one-page PDF for R112."""
from pathlib import Path
root=Path(__file__).resolve().parent

def label(x,y,size,value):
 value=value.replace('\\','\\\\').replace('(','\\(').replace(')','\\)')
 return f'BT /F1 {size} Tf {x} {y} Td ({value}) Tj ET\n'
c='0 0 0 rg\n'+label(40,750,18,'R112: first blowup of the x=0 repeated edge')
c+=label(40,725,10,'5 November 2026 | exact marked-23 degree-7 base equation')
for i,line in enumerate([
 'Set y=1+x(1+w); divide f51 by x^3.',
 'The 24-term quotient has special fiber w^3.',
 'Its lower hull: (0,7), (1,2), (3,0), (7,4).',
 'Slope 5 residual 1+w is simple.',
 'Slope 1 residual 1+w^2=(1+w)^2 remains repeated.',
 'Next: lift the simple root and blow up w=x(1+w1).',
]):c+=label(45,670-36*i,11,line)
for i,line in enumerate([
 'Fresh replay, altered-source rejection, independent substitution: pass.',
 'Lean finite edge arithmetic: pass.',
 'Normalized branches and N51 valuations remain to be determined.',
 'Result SHA-256:',
 '0ca85dd551bb1e288aea9f3d2175061750a19b58e9f94d1dbc58f4eb61171fab',
]):c+=label(40,390-25*i,10,line)
stream=c.encode('ascii')
objs=[b'<< /Type /Catalog /Pages 2 0 R >>',b'<< /Type /Pages /Kids [3 0 R] /Count 1 >>',b'<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>',b'<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>',f'<< /Length {len(stream)} >>\nstream\n'.encode()+stream+b'endstream']
pdf=bytearray(b'%PDF-1.4\n');offsets=[0]
for n,obj in enumerate(objs,1):offsets.append(len(pdf));pdf+=f'{n} 0 obj\n'.encode()+obj+b'\nendobj\n'
xref=len(pdf);pdf+=f'xref\n0 {len(offsets)}\n0000000000 65535 f \n'.encode()
for off in offsets[1:]:pdf+=f'{off:010d} 00000 n \n'.encode()
pdf+=f'trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n'.encode()
(root/'report.pdf').write_bytes(pdf)
