#!/usr/bin/env python3
"""Render descriptive paired ratios on a log axis from the frozen analysis JSON."""
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
analysis = json.loads((HERE / "reports/repeat/analysis.json").read_text())
labels = [
    ("matrix-f4", "matrix F4"), ("matrix-f5", "matrix F5"),
    ("inherited-f4", "inherited F4"), ("f6-ic", "F6-IC"),
    ("rootfind-m3", "scalar rootfind"), ("batch-m3-aa", "batch A/A"),
    ("rho-strong", "strong rho"),
]
left, right = 235, 850
lo, hi = -4.2, 0.2


def x(value):
    return left + (math.log10(value) - lo) / (hi - lo) * (right - left)


lines = [
    '<svg xmlns="http://www.w3.org/2000/svg" width="1080" height="500" viewBox="0 0 1080 500">',
    '<rect width="1080" height="500" fill="white"/>',
    '<text x="24" y="32" font-family="Helvetica,sans-serif" font-size="19" font-weight="bold">K13 local L0: batch rootfinding / reference</text>',
    '<text x="24" y="55" font-family="Helvetica,sans-serif" font-size="12">Sum of complete-cold process wall over 4 verified exact target-round pairs per arm; session ECBS1hc322a24b6610</text>',
]
for tick, label in [(0.0001, "0.0001"), (0.001, "0.001"), (0.01, "0.01"), (0.1, "0.1"), (1, "1")]:
    xx = x(tick)
    lines += [f'<line x1="{xx:.1f}" y1="82" x2="{xx:.1f}" y2="408" stroke="#d9dfe5"/>',
              f'<text x="{xx:.1f}" y="431" text-anchor="middle" font-family="Helvetica,sans-serif" font-size="12">{label}</text>']
lines += [f'<line x1="{x(1):.1f}" y1="82" x2="{x(1):.1f}" y2="408" stroke="#8f3e2f" stroke-width="2"/>']
for i, (arm, label) in enumerate(labels):
    row = analysis["comparisons"][f"batch-m3/{arm}"]
    ratio = row["sum_process_wall_ratio_valid"]
    assert row["valid_pairs"] == 4 and not row["omitted_pairs"]
    yy = 110 + 44 * i
    color = "#0f567a" if ratio < 1 else "#a65a25"
    lines += [f'<text x="24" y="{yy+5}" font-family="Helvetica,sans-serif" font-size="14">{label}</text>',
              f'<line x1="{left}" y1="{yy}" x2="{right}" y2="{yy}" stroke="#edf0f2"/>',
              f'<circle cx="{x(ratio):.1f}" cy="{yy}" r="7" fill="{color}"/>',
              f'<text x="870" y="{yy+5}" font-family="Helvetica,sans-serif" font-size="13">{ratio:.4g}  (n=4)</text>']
lines += ['<text x="24" y="470" font-family="Helvetica,sans-serif" font-size="12">Ratio below 1 means lower observed wall in this epoch. Log axis. Local L0 descriptive data; independent timing validation pending.</text>', '</svg>']
(HERE / "reports/repeat/paired-ratios-log.svg").write_text("\n".join(lines) + "\n")
