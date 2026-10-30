#!/usr/bin/env python3
"""Render only the frozen CSV's aggregate mismatch counts."""
import csv
from pathlib import Path

here=Path(__file__).resolve().parent
with (here/'ROWS.csv').open(newline='') as f: rows=list(csv.DictReader(f))
assert len(rows)==25
parts=['<svg xmlns="http://www.w3.org/2000/svg" width="780" height="430" viewBox="0 0 780 430">',
 '<rect width="780" height="430" fill="white"/>',
 '<text x="35" y="32" font-family="sans-serif" font-size="21">F5 source-expression mismatch by W coordinate</text>',
 '<text x="35" y="55" font-family="sans-serif" font-size="13">All 390,625 tuples; each W bar aggregates all Z values and 78,125 rows.</text>',
 '<path d="M80 330 H740 M80 85 V330" stroke="#444" stroke-width="2" fill="none"/>']
for value in range(0,101,20):
    y=330-2.4*value
    parts += [f'<path d="M80 {y:.1f} H740" stroke="#ddd"/>',f'<text x="40" y="{y+5:.1f}" font-family="sans-serif" font-size="12">{value}%</text>']
series=[('mutation_n6_mismatch','#bd531e','W¹ N6'),('mutation_a_mismatch','#6d5087','W¹ A'),('mutation_c_mismatch','#2b7d66','W¹ C'),('mismatch','#1d6aa7','author/Rust W²')]
for w in range(5):
    group=[r for r in rows if int(r['w'])==w]
    x=150+120*w
    for j,(key,color,_) in enumerate(series):
        pct=sum(int(r[key]) for r in group)/sum(int(r['tested']) for r in group)*100
        y=330-2.4*pct
        parts.append(f'<rect x="{x+16*j}" y="{y:.1f}" width="13" height="{330-y:.1f}" fill="{color}"/>')
    parts.append(f'<text x="{x+20}" y="352" font-family="sans-serif" font-size="14">{w}</text>')
parts.append('<text x="670" y="352" font-family="sans-serif" font-size="13">W</text>')
for j,(_,color,label) in enumerate(series):
    x=55+180*j
    parts += [f'<rect x="{x}" y="385" width="14" height="14" fill="{color}"/>',
              f'<text x="{x+20}" y="397" font-family="sans-serif" font-size="13">{label}</text>']
parts.append('</svg>')
(here/'GRAPH.svg').write_text('\n'.join(parts)+'\n')
