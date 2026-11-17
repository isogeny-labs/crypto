#!/usr/bin/env python3
"""Dependency-free one-page PDF for R110."""
from pathlib import Path
root=Path(__file__).resolve().parent

def label(x,y,size,value):
 value=value.replace('\\','\\\\').replace('(','\\(').replace(')','\\)')
 return f'BT /F1 {size} Tf {x} {y} Td ({value}) Tj ET\n'
c='0 0 0 rg\n'+label(40,750,19,'R110: exact degree-7 N51 valuations')
c+=label(40,725,10,'5 November 2026 | marked-23 x=1 local branches')
for i,line in enumerate([
 'Set s=r^7 and u=r^23 v. The initial G equation is v+v^8.',
 'The simple root v(0)=1 lifts uniquely modulo r^40.',
 'y=0: first nonzero N51 term is alpha*r^35.',
 'y=1: first nonzero N51 term is alpha*r^28.',
 'Both orders are below the certified r^40 G precision bound.',
 'Each place has residue degree 2; local zero degree = 126.',
]):c+=label(45,670-37*i,11,line)
for i,line in enumerate([
 'Fresh replay, altered-source rejection, independent source rebuild: pass.',
 'Lean finite-order and local-degree arithmetic: pass.',
 'Other finite supports and infinity remain for the global divisor.',
 'Result SHA-256:',
 '4b8445ffb9243c369010078602fd5e59caee60b1373f3af31dffc3cfb1c7c738',
]):c+=label(40,380-25*i,10,line)
stream=c.encode('ascii')
objs=[b'<< /Type /Catalog /Pages 2 0 R >>',b'<< /Type /Pages /Kids [3 0 R] /Count 1 >>',b'<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>',b'<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>',f'<< /Length {len(stream)} >>\nstream\n'.encode()+stream+b'endstream']
pdf=bytearray(b'%PDF-1.4\n');offsets=[0]
for n,obj in enumerate(objs,1):offsets.append(len(pdf));pdf+=f'{n} 0 obj\n'.encode()+obj+b'\nendobj\n'
xref=len(pdf);pdf+=f'xref\n0 {len(offsets)}\n0000000000 65535 f \n'.encode()
for off in offsets[1:]:pdf+=f'{off:010d} 00000 n \n'.encode()
pdf+=f'trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n'.encode()
(root/'report.pdf').write_bytes(pdf)
