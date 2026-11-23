#!/usr/bin/env python3
"""Render the R98 recorded denominator-exponent counts as SVG."""
import collections
import json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
result = json.loads((root / "result.json").read_text())
counts = collections.Counter(result["denominator_exponents"])
parts = ['<svg xmlns="http://www.w3.org/2000/svg" width="900" height="400" viewBox="0 0 900 400">',
         '<rect width="900" height="400" fill="white"/>',
         '<text x="40" y="34" font-size="20" font-family="sans-serif">G51 denominator exponents (28 source tokens)</text>',
         '<line x1="65" y1="310" x2="860" y2="310" stroke="black"/>']
for i, exponent in enumerate(sorted(counts)):
    x = 76 + i * 64
    height = counts[exponent] * 51
    parts += [f'<rect x="{x}" y="{310-height}" width="42" height="{height}" fill="#285f9f"/>',
              f'<text x="{x+10}" y="{300-height}" font-size="15" font-family="sans-serif">{counts[exponent]}</text>',
              f'<text x="{x}" y="333" font-size="14" font-family="sans-serif">{exponent}</text>']
parts += ['<text x="390" y="375" font-size="15" font-family="sans-serif">Exponent e in denominator x^e</text>',
          '</svg>']
(root / "publication" / "diagram.svg").write_text("\n".join(parts) + "\n")
