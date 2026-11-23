#!/usr/bin/env python3
"""Draw requested backing bytes from the audited three-cell result."""

import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
data = json.loads((HERE / "RESULTS.json").read_text())
assert data["status"] == "passed"
rows = data["cells"]
assert len(rows) == 3

width, height = 900, 410
x0, x1 = 210, 835
lo, hi = 10, 23


def x_of(value):
    return x0 + (math.log2(value) - lo) / (hi - lo) * (x1 - x0)


parts = [
    f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
    '<rect width="100%" height="100%" fill="#ffffff"/>',
    '<text x="35" y="34" font-family="Helvetica" font-size="21" font-weight="bold" fill="#18384d">Requested pair-index backing</text>',
    '<text x="35" y="57" font-family="Helvetica" font-size="13" fill="#586d78">Bytes; logarithmic horizontal scale. K13/K19/K23 actual pair streams, source c3ad96f.</text>',
]
for exponent in (10, 12, 14, 16, 18, 20, 22):
    x = x_of(1 << exponent)
    parts.append(f'<line x1="{x:.1f}" y1="82" x2="{x:.1f}" y2="345" stroke="#e1e8ec"/>')
    parts.append(f'<text x="{x:.1f}" y="364" text-anchor="middle" font-family="Helvetica" font-size="11" fill="#586d78">2^{exponent}</text>')
for i, row in enumerate(rows):
    y = 100 + i * 84
    n = row["field_degree"]
    inline = row["inline_requested_backing_bytes"]
    paged = row["paged_requested_backing_bytes"]
    parts.extend([
        f'<text x="35" y="{y + 22}" font-family="Helvetica" font-size="17" font-weight="bold" fill="#18384d">K{n}</text>',
        f'<rect x="{x0}" y="{y}" width="{x_of(inline)-x0:.1f}" height="21" rx="3" fill="#2584a7"/>',
        f'<text x="{x_of(inline)+7:.1f}" y="{y+16}" font-family="Helvetica" font-size="12" fill="#18384d">inline {inline:,}</text>',
        f'<rect x="{x0}" y="{y+29}" width="{x_of(paged)-x0:.1f}" height="21" rx="3" fill="#a7b8c4"/>',
        f'<text x="{x_of(paged)+7:.1f}" y="{y+45}" font-family="Helvetica" font-size="12" fill="#18384d">paged {paged:,}</text>',
    ])
parts.extend([
    '<text x="35" y="393" font-family="Helvetica" font-size="12" fill="#586d78">Derived from directory words and reserved spill/page capacity; excludes allocator overhead and RSS.</text>',
    '</svg>',
])
(HERE / "requested_backing.svg").write_text("\n".join(parts) + "\n")
