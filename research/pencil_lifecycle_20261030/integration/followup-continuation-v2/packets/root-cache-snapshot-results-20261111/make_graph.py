#!/usr/bin/env python3
"""Render an editable SVG from graph-data.csv using only the standard library."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parent
rows = list(csv.DictReader((ROOT / "graph-data.csv").open(encoding="utf-8")))


def panel(name: str) -> list[dict[str, str]]:
    return [row for row in rows if row["panel"] == name]


elapsed = panel("elapsed")
inventory = panel("inventory")
budget = panel("budget")

parts = [
    '<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="680" viewBox="0 0 1200 680">',
    '<rect width="1200" height="680" fill="#f7f7f4"/>',
    '<style>text{font-family:Inter,Arial,sans-serif;fill:#17202a}.title{font-size:28px;font-weight:700}.sub{font-size:15px;fill:#52606d}.lab{font-size:16px}.val{font-size:15px;font-weight:700}.axis{stroke:#9aa5b1;stroke-width:1}.bar{fill:#28666e}.inv{fill:#7c3aed}.budget{fill:#d97706}</style>',
    '<text x="55" y="48" class="title">Root cache snapshot — terminal receipt metrics</text>',
    '<text x="55" y="76" class="sub">Elapsed time, inventory comparison, and bounded campaign accounting</text>',
    '<text x="55" y="122" class="lab">Observed elapsed time (seconds)</text>',
    '<line x1="210" y1="310" x2="560" y2="310" class="axis"/>',
]

max_elapsed = 4.0
for i, row in enumerate(elapsed):
    value = float(row["value"])
    y = 150 + i * 52
    width = 350 * value / max_elapsed
    parts.extend([
        f'<text x="55" y="{y + 20}" class="lab">{row["label"]}</text>',
        f'<rect x="210" y="{y}" width="{width:.2f}" height="28" rx="4" class="bar"/>',
        f'<text x="{220 + width:.2f}" y="{y + 20}" class="val">{value:.3f}</text>',
    ])

parts.extend([
    '<text x="650" y="122" class="lab">Path comparison (count)</text>',
    '<line x1="795" y1="360" x2="1125" y2="360" class="axis"/>',
])
max_inventory = max(float(row["value"]) for row in inventory)
for i, row in enumerate(inventory):
    value = float(row["value"])
    y = 150 + i * 48
    width = 310 * value / max_inventory
    parts.extend([
        f'<text x="650" y="{y + 19}" class="lab">{row["label"]}</text>',
        f'<rect x="795" y="{y}" width="{width:.2f}" height="26" rx="4" class="inv"/>',
        f'<text x="{805 + width:.2f}" y="{y + 19}" class="val">{int(value)}</text>',
    ])

parts.extend([
    '<text x="55" y="405" class="lab">Campaign accounting (worker-seconds)</text>',
    '<rect x="55" y="438" width="1070" height="42" rx="5" fill="#e5e7eb"/>',
])
total = sum(float(row["value"]) for row in budget)
x = 55.0
colors = ["#475569", "#d97706", "#d7e3fc"]
for i, row in enumerate(budget):
    value = float(row["value"])
    width = 1070 * value / total
    parts.append(f'<rect x="{x:.2f}" y="438" width="{width:.2f}" height="42" fill="{colors[i]}"/>')
    x += width

for i, row in enumerate(budget):
    parts.extend([
        f'<rect x="{55 + i * 330}" y="510" width="16" height="16" fill="{colors[i]}"/>',
        f'<text x="{79 + i * 330}" y="524" class="lab">{row["label"]}: {int(float(row["value"])):,}</text>',
    ])

parts.extend([
    '<text x="55" y="585" class="sub">Manifest: 2,165 files · 415 directories · 2,618,767,578 aggregate bytes</text>',
    '<text x="55" y="612" class="sub">The cache body is local and is not included in this packet; the exact current-cache manifest is preserved.</text>',
    '<text x="55" y="640" class="sub">Source: graph-data.csv and preserved receipts. No dollar cost was present in the receipts.</text>',
    '</svg>',
])

(ROOT / "timing.svg").write_text("\n".join(parts) + "\n", encoding="utf-8")
