#!/usr/bin/env python3
"""Lay out recorded fixed-stream counts; no result arithmetic is done here."""

import json
from pathlib import Path

here = Path(__file__).resolve().parent
study = here.parents[1]
cells = json.loads((study / "RESULTS.json").read_text())["cells"]

width, height = 1040, 490
colors = {"first": "#879eab", "candidate": "#287caa", "all": "#c58a4a"}
parts = [
    f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
    '<rect width="100%" height="100%" fill="white"/>',
    '<style>text{font-family:Helvetica Neue,Arial,sans-serif;fill:#213847}.small{font-size:14px}.label{font-size:17px;font-weight:600}.title{font-size:24px;font-weight:700}</style>',
    '<text x="24" y="36" class="title">First-independent spends more modeled additions</text>',
    '<text x="24" y="62" class="small">Run 20261116T1342Z; deterministic fixed streams, one execution, no interval.</text>',
    '<text x="150" y="102" class="label">Target queries to full rank</text>',
    '<text x="550" y="102" class="label">Modeled group additions to full rank</text>',
]
for row, cell in enumerate(cells):
    y = 130 + row * 105
    values = {arm: cell[f"{arm}_full_rank"] for arm in colors}
    parts.append(f'<text x="24" y="{y+36}" class="label">K{cell["field_degree"]}</text>')
    for x0, metric, scale in ((150, "queries", 250), (550, "modeled_group_adds", 250)):
        maximum = max(value[metric] if metric == "queries" else value["scan"][metric]
                      for value in values.values())
        for offset, (arm, value) in enumerate(values.items()):
            count = value[metric] if metric == "queries" else value["scan"][metric]
            yy = y + offset * 27
            bar = max(2, round(scale * count / maximum))
            parts.append(f'<rect x="{x0}" y="{yy}" width="{bar}" height="18" rx="3" fill="{colors[arm]}"/>')
            parts.append(f'<text x="{x0+bar+7}" y="{yy+15}" class="small">{count:,}</text>')
for x, arm, label in ((150, "first", "first-hit"), (290, "candidate", "first-independent"), (515, "all", "all-hit")):
    parts.append(f'<rect x="{x}" y="458" width="17" height="12" fill="{colors[arm]}"/>')
    parts.append(f'<text x="{x+25}" y="469" class="small">{label}</text>')
parts.append('</svg>')
(here / "tradeoff.svg").write_text("\n".join(parts) + "\n")
