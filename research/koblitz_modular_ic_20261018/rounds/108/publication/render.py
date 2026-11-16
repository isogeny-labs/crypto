#!/usr/bin/env python3
"""R108 support-window counts as SVG and one-page PDF."""
import json
from pathlib import Path

root=Path(__file__).resolve().parent
r=json.loads((root.parent/'result.json').read_text())
windows=[(0,31),(32,63),(64,95),(96,127)]
counts={key:[sum(lo<=i<=hi for i in r[key]['z_exponents']) for lo,hi in windows] for key in ('y0','y1')}
assert sum(counts['y0'])==71 and sum(counts['y1'])==80
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="850" height="330" viewBox="0 0 850 330">',
 '<rect width="850" height="330" fill="white"/>',
 '<text x="30" y="34" font-size="20" font-family="sans-serif">R108: nonzero z(s) coefficients per 32-degree window</text>']
for j,(lo,hi) in enumerate(windows):
 for k,key in enumerate(('y0','y1')):
  val=counts[key][j];x=70+190*j+65*k;color='#28649c' if k==0 else '#b56528'
  svg += [f'<rect x="{x}" y="{260-7*val}" width="52" height="{7*val}" fill="{color}"/>',
          f'<text x="{x+15}" y="{250-7*val}" font-size="14" font-family="sans-serif">{val}</text>']
 svg.append(f'<text x="{70+190*j}" y="284" font-size="13" font-family="sans-serif">{lo}-{hi}</text>')
svg += ['<path d="M45 260 L825 260" stroke="black"/>',
 '<text x="60" y="313" font-size="13" font-family="sans-serif">Blue: y=0; orange: y=1. Exact R108 series; fresh replay.</text>','</svg>']
(root/'diagram.svg').write_text('\n'.join(svg)+'\n')


def label(x,y,size,value):
 value=value.replace('\\','\\\\').replace('(','\\(').replace(')','\\)')
 return f'BT /F1 {size} Tf {x} {y} Td ({value}) Tj ET\n'


c='0 0 0 rg\n'+label(40,750,18,'R108: base series extended to s^128')
c+=label(40,724,10,'5 November 2026 | marked-23 Koblitz local series')
for i,line in enumerate([
 'Both original f residuals vanish modulo s^128; all first 32 bits match R106.',
 'The recorded z(s) supports contain 71 and 80 nonzero coefficients.',
 'R102 denominator orders pull back to at most 27 or 36 in s.',
]):c+=label(40,684-18*i,10,line)
c+=label(40,600,12,'Nonzero z(s) coefficients by 32-degree window')
for j,(lo,hi) in enumerate(windows):
 for k,key in enumerate(('y0','y1')):
  val=counts[key][j];x=50+140*j+48*k
  c+=('0.16 0.39 0.61 rg\n' if k==0 else '0.71 0.39 0.16 rg\n')
  c+=f'{x} 345 32 {6*val} re f\n'
  c+='0 0 0 rg\n'+label(x+8,355+6*val,9,str(val))
 c+=label(50+140*j,325,9,f'{lo}-{hi}')
c+='0 0 0 RG 45 345 m 595 345 l S\n'
for i,line in enumerate([
 'Blue: y=0. Orange: y=1. Counted from exact result JSON.',
 'Fresh replay, changed hash, independent bit arithmetic, and Lean passed.',
 'Next: N51 valuation on degree-1 and degree-7 lambda branches.',
 'Raw archive SHA-256:',
 '99dcbbc8daaecdb718715b7ae978431d6723ada3108f94d3e99f63f1cab79a87',
]):c+=label(40,280-18*i,10,line)
stream=c.encode('ascii')
objects=[b'<< /Type /Catalog /Pages 2 0 R >>',b'<< /Type /Pages /Kids [3 0 R] /Count 1 >>',
 b'<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>',
 b'<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>',
 f'<< /Length {len(stream)} >>\nstream\n'.encode()+stream+b'endstream']
pdf=bytearray(b'%PDF-1.4\n');offsets=[0]
for n,obj in enumerate(objects,1):offsets.append(len(pdf));pdf+=f'{n} 0 obj\n'.encode()+obj+b'\nendobj\n'
xref=len(pdf);pdf+=f'xref\n0 {len(offsets)}\n0000000000 65535 f \n'.encode()
for off in offsets[1:]:pdf+=f'{off:010d} 00000 n \n'.encode()
pdf+=f'trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n'.encode()
(root/'report.pdf').write_bytes(pdf)
