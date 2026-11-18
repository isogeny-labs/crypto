#!/usr/bin/env python3
"""Dependency-free one-page PDF for R115."""
from pathlib import Path
root=Path(__file__).resolve().parent

def label(x,y,size,value):
 value=value.replace('\\','\\\\').replace('(','\\(').replace(')','\\)')
 return f'BT /F1 {size} Tf {x} {y} Td ({value}) Tj ET\n'
c='0 0 0 rg\n'+label(40,750,18,'R115: direct x=0 base uniformizer and four places')
c+=label(40,725,10,'5 November 2026 | exact marked-23 degree-7 base equation')
for i,line in enumerate([
 'Set t=v/x; E(x,t)=F(x,x*t)/x^2 has unit x derivative.',
 'Unique x(t) solves E through t^128; original f residual vanishes.',
 'The direct parameter has v_t(x)=2 and v_t(v)=3.',
 'Four base places have ramification e=1,3,1,2.',
 'Their local degrees sum to the degree-7 base field.',
 'Next: extend base precision beyond x^161 denominator pulls.',
]):c+=label(45,670-36*i,11,line)
for i,line in enumerate([
 'Fresh replay, altered-source rejection, independent source composition: pass.',
 'Lean finite support arithmetic: pass.',
 'Lambda and N51 valuations at x=0 remain open.',
 'Result SHA-256:',
 'fbbd1e37c16a2ef28d08410671eef8a1cd41837decaea49c8fb95753b7ffaff1',
]):c+=label(40,390-25*i,10,line)
stream=c.encode('ascii')
objs=[b'<< /Type /Catalog /Pages 2 0 R >>',b'<< /Type /Pages /Kids [3 0 R] /Count 1 >>',b'<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>',b'<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>',f'<< /Length {len(stream)} >>\nstream\n'.encode()+stream+b'endstream']
pdf=bytearray(b'%PDF-1.4\n');offsets=[0]
for n,obj in enumerate(objs,1):offsets.append(len(pdf));pdf+=f'{n} 0 obj\n'.encode()+obj+b'\nendobj\n'
xref=len(pdf);pdf+=f'xref\n0 {len(offsets)}\n0000000000 65535 f \n'.encode()
for off in offsets[1:]:pdf+=f'{off:010d} 00000 n \n'.encode()
pdf+=f'trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n'.encode()
(root/'report.pdf').write_bytes(pdf)
