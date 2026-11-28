#!/usr/bin/env python3
"""Dependency-free one-page PDF for R113."""
from pathlib import Path
root=Path(__file__).resolve().parent

def label(x,y,size,value):
 value=value.replace('\\','\\\\').replace('(','\\(').replace(')','\\)')
 return f'BT /F1 {size} Tf {x} {y} Td ({value}) Tj ET\n'
c='0 0 0 rg\n'+label(40,750,18,'R113: second blowup of the x=0 wild edge')
c+=label(40,725,10,'5 November 2026 | exact marked-23 degree-7 base equation')
for i,line in enumerate([
 'Set y=1+x+x^2(1+v); divide f51 by x^6.',
 'The 34-term quotient has special fiber v^2(1+v).',
 'Its lower hull: (0,3), (2,0), (3,0), (7,8).',
 'Positive slope 3/2 has residual 1+T^2.',
 'The residual is (1+T)^2, repeated in characteristic two.',
 'Next: set x=r^2, v=r^3(1+h) and lift higher terms.',
]):c+=label(45,670-36*i,11,line)
for i,line in enumerate([
 'Fresh replay, altered-source rejection, independent R112 composition: pass.',
 'Lean finite support arithmetic: pass.',
 'The normalized place count and N51 valuations remain open.',
 'Result SHA-256:',
 '41996e050fa85958218d4d205cb5cdeec599d59e4176fa0e11db38f6c467ef26',
]):c+=label(40,390-25*i,10,line)
stream=c.encode('ascii')
objs=[b'<< /Type /Catalog /Pages 2 0 R >>',b'<< /Type /Pages /Kids [3 0 R] /Count 1 >>',b'<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>',b'<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>',f'<< /Length {len(stream)} >>\nstream\n'.encode()+stream+b'endstream']
pdf=bytearray(b'%PDF-1.4\n');offsets=[0]
for n,obj in enumerate(objs,1):offsets.append(len(pdf));pdf+=f'{n} 0 obj\n'.encode()+obj+b'\nendobj\n'
xref=len(pdf);pdf+=f'xref\n0 {len(offsets)}\n0000000000 65535 f \n'.encode()
for off in offsets[1:]:pdf+=f'{off:010d} 00000 n \n'.encode()
pdf+=f'trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n'.encode()
(root/'report.pdf').write_bytes(pdf)
