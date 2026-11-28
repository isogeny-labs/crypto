#!/usr/bin/env python3
"""Render a bounded page-use chart from the validated result rows."""

import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
data = json.loads((HERE / "RESULTS.json").read_text())
assert data["status"] == "passed"
rows = data["cells"]
width, height = 860, 340
parts = [
    f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
    '<rect width="100%" height="100%" fill="#ffffff"/>',
    '<text x="35" y="39" font-family="Helvetica" font-size="21" font-weight="bold" fill="#17364a">Pages touched by actual pair streams</text>',
    '<text x="35" y="62" font-family="Helvetica" font-size="13" fill="#526876">16-word pages; width is share of preallocated capacity</text>',
]
for i, row in enumerate(rows):
    y = 100 + i * 70
    used = row["used_pages"]
    capacity = row["page_capacity"]
    frac = used / capacity
    bar_x, bar_w = 125, 560
    parts.extend([
        f'<text x="35" y="{y + 20}" font-family="Helvetica" font-size="17" font-weight="bold" fill="#17364a">K{row["field_degree"]}</text>',
        f'<rect x="{bar_x}" y="{y}" width="{bar_w}" height="29" rx="5" fill="#e8edf0"/>',
        f'<rect x="{bar_x}" y="{y}" width="{bar_w * frac:.1f}" height="29" rx="5" fill="#267eaa"/>',
        f'<text x="705" y="{y + 20}" font-family="Helvetica" font-size="14" fill="#17364a">{used:,} / {capacity:,}</text>',
    ])
parts.extend([
    '<text x="35" y="313" font-family="Helvetica" font-size="12" fill="#526876">Capacity is preallocated in the prototype. This is key coverage, not RSS or elapsed time.</text>',
    '</svg>',
])
(HERE / "page_use.svg").write_text("\n".join(parts) + "\n")
