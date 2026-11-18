#!/usr/bin/env python3
"""Dependency-free one-page PDF for R117."""
from pathlib import Path
root=Path(__file__).resolve().parent

def label(x,y,size,value):
 value=value.replace('\\','\\\\').replace('(','\\(').replace(')','\\)')
 return f'BT /F1 {size} Tf {x} {y} Td ({value}) Tj ET\n'
c='0 0 0 rg\n'+label(40,750,18,'R117: G51 coefficient orders at x=0')
c+=label(40,725,10,'5 November 2026 | exact marked-23 degree-7 base equation')
for i,line in enumerate([
 'All 29 actual G51 terms were evaluated on four base places.',
 'Two e=1 Newton edges are certified by exact coefficient orders.',
 'y0/e1: lambda order -10; y1/e1: lambda order 0.',
 'Both initial edge polynomials equal (1+T+T^2)^8.',
 'y0/e3 and y1/e2 retain exact coefficient precision gaps.',
 'Next: extend base precision and lift repeated lambda roots.',
]):c+=label(45,670-36*i,11,line)
for i,line in enumerate([
 'Fresh replay, altered-source rejection, independent rational expansion: pass.',
 'Lean finite support arithmetic: pass.',
 'Lean checked both exact edges and the repeated polynomial.',
 'Result SHA-256:',
 'b1786c05cca9509a8e9c0dd21c781c1110ced4b0a545b15c08120edbb467f393',
]):c+=label(40,390-25*i,10,line)
stream=c.encode('ascii')
objs=[b'<< /Type /Catalog /Pages 2 0 R >>',b'<< /Type /Pages /Kids [3 0 R] /Count 1 >>',b'<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>',b'<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>',f'<< /Length {len(stream)} >>\nstream\n'.encode()+stream+b'endstream']
pdf=bytearray(b'%PDF-1.4\n');offsets=[0]
for n,obj in enumerate(objs,1):offsets.append(len(pdf));pdf+=f'{n} 0 obj\n'.encode()+obj+b'\nendobj\n'
xref=len(pdf);pdf+=f'xref\n0 {len(offsets)}\n0000000000 65535 f \n'.encode()
for off in offsets[1:]:pdf+=f'{off:010d} 00000 n \n'.encode()
pdf+=f'trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n'.encode()
(root/'report.pdf').write_bytes(pdf)
