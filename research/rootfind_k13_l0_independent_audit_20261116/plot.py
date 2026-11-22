#!/usr/bin/env python3
"""Render the independently recomputed, descriptive K13 L0 ratios."""

import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
data = json.loads((HERE / "RESULTS.json").read_text())
labels = [
    ("matrix-f4", "matrix F4"),
    ("matrix-f5", "matrix F5"),
    ("inherited-f4", "inherited F4"),
    ("f6-ic", "F6-IC"),
    ("rootfind-m3", "scalar rootfinding"),
    ("batch-m3-aa", "batch A/A"),
    ("rho-strong", "strong rho"),
]

left, width = 245, 580
lo, hi = -3.2, 0.25


def x_at(ratio):
    return left + (math.log10(ratio) - lo) * width / (hi - lo)


lines = [
    '<svg xmlns="http://www.w3.org/2000/svg" width="930" height="455" viewBox="0 0 930 455">',
    '<rect width="930" height="455" fill="white"/>',
    '<text x="24" y="34" font-family="Helvetica" font-size="20" font-weight="bold">K13 complete-process ratios, L0 only</text>',
    '<text x="24" y="57" font-family="Helvetica" font-size="13">Batch rootfinding / reference; four exact public target-round pairs</text>',
]
for tick in (0.001, 0.01, 0.1, 1):
    x = x_at(tick)
    lines.append(f'<line x1="{x:.1f}" y1="86" x2="{x:.1f}" y2="389" stroke="#c8d2dc" stroke-width="1"/>')
    lines.append(f'<text x="{x:.1f}" y="410" text-anchor="middle" font-family="Helvetica" font-size="12">{tick:g}</text>')
for i, (key, label) in enumerate(labels):
    ratio = data["summed_wall_ratios"][key]
    y = 106 + 40 * i
    x = x_at(ratio)
    color = "#3068a1" if key in ("matrix-f4", "matrix-f5", "inherited-f4", "f6-ic") else "#ad7636"
    lines.append(f'<text x="24" y="{y+15}" font-family="Helvetica" font-size="14">{label}</text>')
    lines.append(f'<line x1="{left}" y1="{y+10}" x2="{x:.1f}" y2="{y+10}" stroke="{color}" stroke-width="12" stroke-linecap="round"/>')
    lines.append(f'<circle cx="{x:.1f}" cy="{y+10}" r="6" fill="{color}"/>')
    text_x = min(x + 15, 843)
    lines.append(f'<text x="{text_x:.1f}" y="{y+15}" font-family="Helvetica" font-size="13">{ratio:.4g}</text>')
lines.append('<text x="24" y="440" font-family="Helvetica" font-size="12" fill="#555">Raw-record arithmetic check; source timing, process integrity and L2 qualification remain open.</text>')
lines.append('</svg>')
(HERE / "ratios.svg").write_text("\n".join(lines) + "\n")
