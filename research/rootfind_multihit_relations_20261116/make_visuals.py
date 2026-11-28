#!/usr/bin/env python3
"""Render the two observed scan/row tradeoffs from RESULTS.json."""

import json
from pathlib import Path

here = Path(__file__).resolve().parent
data = json.loads((here / "RESULTS.json").read_text())
cells = data["cells"]
width, height = 920, 425
parts = [
    f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
    '<rect width="100%" height="100%" fill="white"/>',
    '<style>text{font-family:Helvetica Neue,Arial,sans-serif;fill:#233c4b} .small{font-size:14px} .label{font-size:16px;font-weight:600} .title{font-size:23px;font-weight:700}</style>',
    '<text x="30" y="40" class="title">All-hit: fewer queries, more pair probes</text>',
    '<text x="30" y="64" class="small">Fixed deterministic residual streams; counts through each arm’s full-rank target.</text>',
    '<text x="158" y="100" class="label">Target queries</text>',
    '<text x="535" y="100" class="label">Pair lookups</text>',
]
for row, cell in enumerate(cells):
    y = 135 + row * 90
    n = cell["field_degree"]
    first = cell["first_full_rank"]
    all_ = cell["all_full_rank"]
    fq, aq = first["queries"], all_["queries"]
    fl, al = first["scan"]["lookups"], all_["scan"]["lookups"]
    parts.append(f'<text x="30" y="{y+26}" class="label">K{n}</text>')
    for x0, max_value, values in [(158, max(fq, aq), [(fq, "#8da9b8"), (aq, "#2580a1")]),
                                   (535, max(fl, al), [(fl, "#8da9b8"), (al, "#2580a1")])]:
        for offset, (value, color) in enumerate(values):
            yy = y + offset * 28
            bar = int(225 * value / max_value)
            parts.append(f'<rect x="{x0}" y="{yy}" width="{bar}" height="18" rx="3" fill="{color}"/>')
            parts.append(f'<text x="{x0+bar+8}" y="{yy+14}" class="small">{value:,}</text>')
parts.extend([
    '<rect x="160" y="397" width="17" height="12" fill="#8da9b8"/><text x="184" y="408" class="small">first-hit</text>',
    '<rect x="290" y="397" width="17" height="12" fill="#2580a1"/><text x="314" y="408" class="small">all-hit</text>',
    '</svg>',
])
(here / "tradeoff.svg").write_text("\n".join(parts) + "\n")
