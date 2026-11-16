#!/usr/bin/env python3
"""Dependency-free one-page PDF for the exact R109 branch result."""
from pathlib import Path
root = Path(__file__).resolve().parent

def label(x, y, size, value):
    value = value.replace('\\', '\\\\').replace('(', '\\(').replace(')', '\\)')
    return f'BT /F1 {size} Tf {x} {y} Td ({value}) Tj ET\n'

c = '0 0 0 rg\n' + label(40, 750, 19, 'R109: N51 on the four x=1 branches')
c += label(40, 725, 10, '5 November 2026 | 105 exact rational terms')
for i, line in enumerate([
    'y=0, degree 1: unique leading u^0; v(n)=0.',
    'y=1, degree 1: unique leading u^0; v(n)=0.',
    'y=0, degree 7: u^0 and s^-23 u^7 cancel at weight zero.',
    'y=1, degree 7: u^0 and s^-23 u^7 cancel at weight zero.',
]):
    c += label(45, 670-38*i, 12, line)
c += label(40, 460, 12, 'Degree-7 valuations remain open; lift the local branch next.')
for i, line in enumerate([
    'Fresh replay, changed donor rejection, independent Hasse check: pass.',
    'Lean finite weighted-minimum and F2 cancellation lemmas: pass.',
    'First verifier indexing error corrected; producer/replay had no failure.',
    'Raw archive 24,344 bytes; SHA-256:',
    '263825efee003c4075a89f892765620c485cb7d6b10e675c6da0e2f9d20c4987',
]):
    c += label(40, 395-24*i, 10, line)
stream = c.encode('ascii')
objects = [b'<< /Type /Catalog /Pages 2 0 R >>', b'<< /Type /Pages /Kids [3 0 R] /Count 1 >>',
 b'<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>',
 b'<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>',
 f'<< /Length {len(stream)} >>\nstream\n'.encode() + stream + b'endstream']
pdf = bytearray(b'%PDF-1.4\n'); offsets = [0]
for n, obj in enumerate(objects, 1):
    offsets.append(len(pdf)); pdf += f'{n} 0 obj\n'.encode() + obj + b'\nendobj\n'
xref = len(pdf); pdf += f'xref\n0 {len(offsets)}\n0000000000 65535 f \n'.encode()
for off in offsets[1:]: pdf += f'{off:010d} 00000 n \n'.encode()
pdf += f'trailer\n<< /Size {len(offsets)} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n'.encode()
(root/'report.pdf').write_bytes(pdf)
