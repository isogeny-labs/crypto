#!/usr/bin/env python3
"""Render fixed-stream query and group-operation proxy counts."""

import json
from pathlib import Path

here = Path(__file__).resolve().parent
cells = json.loads((here / "RESULTS.json").read_text())["cells"]
width, height = 1040, 475
parts = [
    f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
    '<rect width="100%" height="100%" fill="white"/>',
    '<style>text{font-family:Helvetica Neue,Arial,sans-serif;fill:#233c4b}.small{font-size:14px}.label{font-size:17px;font-weight:600}.title{font-size:24px;font-weight:700}</style>',
    '<text x="28" y="39" class="title">Block tail did not advance full rank</text>',
    '<text x="28" y="63" class="small">Fixed holdout streams; complete hit blocks charged to first-hit and block-tail arms.</text>',
    '<text x="155" y="98" class="label">Targets to full rank</text>',
    '<text x="555" y="98" class="label">Modeled group additions</text>',
]
colors = {"first": "#8da9b8", "tail": "#2580a1", "all": "#cc9658"}
for row, cell in enumerate(cells):
    y = 130 + row * 103
    n = cell["field_degree"]
    values = {arm: cell[f"{arm}_full_rank"] for arm in colors}
    parts.append(f'<text x="28" y="{y+35}" class="label">K{n}</text>')
    for x0, key, scale in [(155, "queries", 260), (555, "modeled_group_adds", 270)]:
        maximum = max(v[key] if key == "queries" else v["scan"][key]
                      for v in values.values())
        for offset, (arm, value) in enumerate(values.items()):
            count = value[key] if key == "queries" else value["scan"][key]
            yy = y + 26 * offset
            bar = max(2, int(scale * count / maximum))
            parts.append(f'<rect x="{x0}" y="{yy}" width="{bar}" height="17" rx="3" fill="{colors[arm]}"/>')
            parts.append(f'<text x="{x0+bar+7}" y="{yy+14}" class="small">{count:,}</text>')
for x, arm in [(155, "first"), (265, "tail"), (375, "all")]:
    parts.append(f'<rect x="{x}" y="449" width="17" height="12" fill="{colors[arm]}"/>')
    parts.append(f'<text x="{x+24}" y="460" class="small">{arm}</text>')
parts.append('</svg>')
(here / "tradeoff.svg").write_text("\n".join(parts) + "\n")
