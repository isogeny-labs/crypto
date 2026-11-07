#!/usr/bin/env python3
"""Lay out the aggregate values recorded by the native analyzer as SVG."""
import csv
from pathlib import Path

ROOT = Path(__file__).resolve().parent
RESULTS = ROOT / "results"
with (RESULTS / "graph-data.csv").open(newline="", encoding="utf-8") as handle:
    graph_rows = list(csv.DictReader(handle))
with (RESULTS / "original-vs-replication.csv").open(newline="", encoding="utf-8") as handle:
    comparison = list(csv.DictReader(handle))

labels = {"fb1-j16": "full / 16 jumps", "fb1-j8": "full / 8 jumps", "fb1-j32": "full / 32 jumps", "fb2-j16": "half / 16 jumps", "fb4-j16": "quarter / 16 jumps"}
colors = {"fb1-j16": "#0072B2", "fb1-j8": "#009E73", "fb1-j32": "#56B4E9", "fb2-j16": "#E69F00", "fb4-j16": "#D55E00"}
order = ["fb1-j16", "fb1-j8", "fb1-j32", "fb2-j16", "fb4-j16"]
width, height = 1200, 500
parts = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">', '<rect width="100%" height="100%" fill="white"/>', '<style>text{font-family:DejaVu Sans,Arial,sans-serif;fill:#222}.tick{font-size:11px}.label{font-size:12px}.title{font-size:14px;font-weight:600}.grid{stroke:#ddd;stroke-width:1}.axis{stroke:#333;stroke-width:1.2}</style>', '<text x="600" y="24" text-anchor="middle" class="title">Factor-base / graph replication · operation-count pricing · run 20261024</text>']

def line(x1, y1, x2, y2, cls="axis", extra=""):
    parts.append(f'<line x1="{x1:.1f}" y1="{y1:.1f}" x2="{x2:.1f}" y2="{y2:.1f}" class="{cls}" {extra}/>')

def text(x, y, value, anchor="middle", cls="tick", extra=""):
    parts.append(f'<text x="{x:.1f}" y="{y:.1f}" text-anchor="{anchor}" class="{cls}" {extra}>{value}</text>')

x0, y0, pw, ph = 70, 68, 480, 340
ymax = 4.5
for tick in [0, 1, 2, 3, 4]:
    y = y0 + ph - tick / ymax * ph
    line(x0, y, x0 + pw, y, "grid"); text(x0 - 10, y + 4, str(tick), "end")
for bit in [16, 20, 24, 28, 32, 36, 40]:
    x = x0 + (bit - 16) / 24 * pw
    line(x, y0 + ph, x, y0 + ph + 5); text(x, y0 + ph + 20, str(bit))
line(x0, y0, x0, y0 + ph); line(x0, y0 + ph, x0 + pw, y0 + ph)
text(x0 + pw / 2, y0 - 12, "Representative price by size (180 targets per point)", cls="title")
text(x0 + pw / 2, y0 + ph + 42, "Source subgroup size (bits)", cls="label")
text(18, y0 + ph / 2, "Candidate / matched rho cost", cls="label", extra=f'transform="rotate(-90 18 {y0 + ph / 2:.1f})"')
for arm in order:
    rows = sorted((r for r in graph_rows if r["arm"] == arm and r["slice"] != "all"), key=lambda r: int(r["slice"]))
    coords = [(x0 + (int(r["slice"]) - 16) / 24 * pw, y0 + ph - float(r["candidate_over_rho"]) / ymax * ph) for r in rows]
    parts.append('<polyline fill="none" stroke="{}" stroke-width="2" points="{}"/>'.format(colors[arm], " ".join(f"{x:.1f},{y:.1f}" for x, y in coords)))
    for x, y in coords: parts.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="3.4" fill="{colors[arm]}"/>')
line(x0, y0 + ph - ph / ymax, x0 + pw, y0 + ph - ph / ymax, "axis", 'stroke-dasharray="5 4"')
for idx, arm in enumerate(order):
    lx = x0 + (idx % 3) * 160; ly = 435 + (idx // 3) * 22
    line(lx, ly, lx + 22, ly, "axis", f'style="stroke:{colors[arm]};stroke-width:3"'); text(lx + 28, ly + 4, labels[arm], "start")

x0, y0, pw, ph = 670, 68, 480, 340
by_arm = {row["arm"]: row for row in comparison}
for tick in [0, 1, 2, 3, 4]:
    y = y0 + ph - tick / ymax * ph
    line(x0, y, x0 + pw, y, "grid"); text(x0 - 10, y + 4, str(tick), "end")
line(x0, y0, x0, y0 + ph); line(x0, y0 + ph, x0 + pw, y0 + ph)
text(x0 + pw / 2, y0 - 12, "Representative price, full 1,260-target cohorts", cls="title")
text(620, y0 + ph / 2, "Candidate / matched rho cost", cls="label", extra=f'transform="rotate(-90 620 {y0 + ph / 2:.1f})"')
group = pw / len(order); barw = 30
for idx, arm in enumerate(order):
    cx = x0 + group * (idx + .5)
    old = float(by_arm[arm]["original_candidate_over_rho"]); new = float(by_arm[arm]["replication_candidate_over_rho"])
    for x, value, color in [(cx - barw, old, "#999999"), (cx, new, colors[arm])]:
        y = y0 + ph - value / ymax * ph
        parts.append(f'<rect x="{x:.1f}" y="{y:.1f}" width="{barw}" height="{y0 + ph - y:.1f}" fill="{color}"/>')
    text(cx, y0 + ph + 17, labels[arm].split(" / ")[0]); text(cx, y0 + ph + 31, labels[arm].split(" / ")[1])
line(x0, y0 + ph - ph / ymax, x0 + pw, y0 + ph - ph / ymax, "axis", 'stroke-dasharray="5 4"')
parts.append('<rect x="790" y="449" width="14" height="14" fill="#999999"/>'); text(810, 461, "Original cohort", "start")
parts.append('<rect x="935" y="449" width="14" height="14" fill="#0072B2"/>'); text(955, 461, "Independent cohort", "start")
text(600, 492, "Dashed line: equal cost. Representative cell: S/M=0.5, I/M=10, logical=0.1 GAE.", cls="tick")
parts.append("</svg>")
(ROOT / "factor-base-replication.svg").write_text("\n".join(parts) + "\n", encoding="utf-8")
