#!/usr/bin/env python3
"""Dependency-free one-page PDF for R114."""
from pathlib import Path
root=Path(__file__).resolve().parent

def label(x,y,size,value):
 value=value.replace('\\','\\\\').replace('(','\\(').replace(')','\\)')
 return f'BT /F1 {size} Tf {x} {y} Td ({value}) Tj ET\n'
c='0 0 0 rg\n'+label(40,750,18,'R114: radical x=0 lift repeats square residual')
c+=label(40,725,10,'5 November 2026 | exact marked-23 degree-7 base equation')
for i,line in enumerate([
 'Set x=r^2, v=r^3(1+h); divide R113 quotient by r^6.',
 'The 74-term quotient has special fiber h^2.',
 'Its lower hull: (0,3), (2,0), (3,3), (7,31).',
 'Positive slope 3/2 again has residual 1+T^2.',
 'The residual is (1+T)^2, repeated in characteristic two.',
 'Next: use t=v/x in the original separable branch.',
]):c+=label(45,670-36*i,11,line)
for i,line in enumerate([
 'Fresh replay, altered-source rejection, independent donor composition: pass.',
 'Lean finite support arithmetic: pass.',
 'The normalized place count and N51 valuations remain open.',
 'Result SHA-256:',
 'c7c5664c9956f9af5f709bd08cc6bf4ae91c2ba465224b5f0af3356b7dd71bc0',
]):c+=label(40,390-25*i,10,line)
stream=c.encode('ascii')
objs=[b'<< /Type /Catalog /Pages 2 0 R >>',b'<< /Type /Pages /Kids [3 0 R] /Count 1 >>',b'<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>',b'<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>',f'<< /Length {len(stream)} >>\nstream\n'.encode()+stream+b'endstream']
pdf=bytearray(b'%PDF-1.4\n');offsets=[0]
for n,obj in enumerate(objs,1):offsets.append(len(pdf));pdf+=f'{n} 0 obj\n'.encode()+obj+b'\nendobj\n'
xref=len(pdf);pdf+=f'xref\n0 {len(offsets)}\n0000000000 65535 f \n'.encode()
for off in offsets[1:]:pdf+=f'{off:010d} 00000 n \n'.encode()
pdf+=f'trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n'.encode()
(root/'report.pdf').write_bytes(pdf)
