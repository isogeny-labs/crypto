#!/usr/bin/env python3
"""Dependency-free one-page x=0 Newton audit PDF."""
from pathlib import Path
root=Path(__file__).resolve().parent

def label(x,y,size,value):
 value=value.replace('\\','\\\\').replace('(','\\(').replace(')','\\)')
 return f'BT /F1 {size} Tf {x} {y} Td ({value}) Tj ET\n'
c='0 0 0 rg\n'+label(40,750,18,'R111: x=0 Newton entry for the degree-112 source')
c+=label(40,725,10,'5 November 2026 | exact F2 support from the marked-23 donor')
for i,line in enumerate([
 'The base fiber is y^4 (y+1)^3.',
 'Near y=0: lower hull (0,2), (1,1), (4,0), (7,0).',
 'Its positive slopes are 1 and 1/3; edge supports 1+w and 1+w^3.',
 'Near y=1: lower hull (0,3), (3,0), (7,0).',
 'Its slope-1 edge has residual (1+w)^3, a repeated root.',
 'Further blowup is required before normalized branch counts.',
]):c+=label(45,670-36*i,10,line)
for i,line in enumerate([
 'G coefficient values here are termwise lower bounds, not valuations.',
 'Fresh replay, altered-source rejection, independent support check: pass.',
 'Lean finite edge factorizations: pass.',
 'Result SHA-256:',
 'e35e271cb6c86fc440676f1884709430eb091482f8531635294bc7dbdf49c4e5',
]):c+=label(40,390-25*i,10,line)
stream=c.encode('ascii')
objs=[b'<< /Type /Catalog /Pages 2 0 R >>',b'<< /Type /Pages /Kids [3 0 R] /Count 1 >>',b'<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>',b'<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>',f'<< /Length {len(stream)} >>\nstream\n'.encode()+stream+b'endstream']
pdf=bytearray(b'%PDF-1.4\n');offsets=[0]
for n,obj in enumerate(objs,1):offsets.append(len(pdf));pdf+=f'{n} 0 obj\n'.encode()+obj+b'\nendobj\n'
xref=len(pdf);pdf+=f'xref\n0 {len(offsets)}\n0000000000 65535 f \n'.encode()
for off in offsets[1:]:pdf+=f'{off:010d} 00000 n \n'.encode()
pdf+=f'trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n'.encode()
(root/'report.pdf').write_bytes(pdf)
