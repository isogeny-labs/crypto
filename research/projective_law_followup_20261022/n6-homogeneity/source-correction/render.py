#!/usr/bin/env python3
"""Render the source-bound coefficient-weight table without recomputing evidence."""
import csv
from pathlib import Path
from xml.sax.saxutils import escape

here = Path(__file__).resolve().parent
with (here / 'scaling-weights.csv').open(newline='') as f:
    rows = list(csv.DictReader(f))
assert [r['coefficient'] for r in rows] == ['A', 'B', 'C']
parts = ['<svg xmlns="http://www.w3.org/2000/svg" width="800" height="400" viewBox="0 0 800 400">',
         '<rect width="800" height="400" fill="white"/>',
         '<text x="35" y="35" font-family="sans-serif" font-size="21">Weighted-to-affine coefficient exponents</text>',
         '<text x="35" y="59" font-family="sans-serif" font-size="13">Pinned author Magma and Rust use W² in N6; W¹ is a synthetic mutation.</text>',
         '<path d="M100 325 H755 M100 85 V325" stroke="#555" stroke-width="2" fill="none"/>',
         '<text x="30" y="95" font-family="sans-serif" font-size="13">w exponent</text>']
for n in range(4):
    y = 325 - 70*n
    parts += [f'<path d="M100 {y} H755" stroke="#e5e5e5"/>',
              f'<text x="80" y="{y+5}" font-family="sans-serif" font-size="13">{n}</text>']
for i,row in enumerate(rows):
    x=200+220*i
    w=int(row['author_rust_weight_w']); y=325-70*w
    parts += [f'<rect x="{x-38}" y="{y}" width="76" height="{325-y}" fill="#31688e"/>',
              f'<text x="{x-24}" y="{y-9}" font-family="sans-serif" font-size="15">z^{escape(row["author_rust_weight_z"])}w^{w}</text>',
              f'<text x="{x-6}" y="348" font-family="sans-serif" font-size="16">{escape(row["coefficient"])}</text>']
parts += ['<text x="35" y="382" font-family="sans-serif" font-size="12">Formal polynomial reductions over every commutative ring; no compiled-code or Pic⁰ claim.</text>', '</svg>']
(here/'scaling-weights.svg').write_text('\n'.join(parts)+'\n')
