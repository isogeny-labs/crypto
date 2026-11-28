#!/usr/bin/env python3
"""Render the exact R102 x+1 coefficient-order histogram as SVG."""
import json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
data = json.loads((root / "result.json").read_text())
rows = [(int(order), count) for order, count in data["coefficient_order_histogram"].items()]
rows.sort()
parts = ['<svg xmlns="http://www.w3.org/2000/svg" width="850" height="390" viewBox="0 0 850 390">',
         '<rect width="850" height="390" fill="white"/>',
         '<text x="35" y="34" font-size="20" font-family="sans-serif">R102: all 105 N51 coefficient orders at x=1</text>',
         '<line x1="60" y1="320" x2="810" y2="320" stroke="black"/>']
for i, (order, count) in enumerate(rows):
    x = 75 + i * 91
    height = count * 8
    parts += [f'<rect x="{x}" y="{320-height}" width="54" height="{height}" fill="#285f9f"/>',
              f'<text x="{x+14}" y="{309-height}" font-size="16" font-family="sans-serif">{count}</text>',
              f'<text x="{x+10}" y="347" font-size="16" font-family="sans-serif">{order}</text>']
parts += ['<text x="290" y="377" font-size="15" font-family="sans-serif">Coefficient order v_(x+1)</text>', '</svg>']
(root / "publication" / "diagram.svg").write_text("\n".join(parts) + "\n")
