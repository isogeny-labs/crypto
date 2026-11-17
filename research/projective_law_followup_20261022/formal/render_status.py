#!/usr/bin/env python3
"""Render the recorded N9 formal-boundary status table without Python extras."""

from __future__ import annotations

import csv
import html
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent
WIDTH, HEIGHT = 1080, 620
LEFT, RIGHT, TOP, ROW = 410, 70, 125, 52
MAX_COUNT = 7


def main() -> None:
    with (ROOT / "formal-status.csv").open(newline="") as handle:
        rows = list(csv.DictReader(handle))
    plot_width = WIDTH - LEFT - RIGHT
    pieces = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{WIDTH}" height="{HEIGHT}" viewBox="0 0 {WIDTH} {HEIGHT}">',
        '<rect width="100%" height="100%" fill="#ffffff"/>',
        '<style>text{font-family:DejaVu Sans,Arial,sans-serif;fill:#243447}.title{font-size:25px;font-weight:700}.sub{font-size:13px;fill:#475569}.label{font-size:14px}.count{font-size:13px;font-weight:700}.axis{stroke:#cbd5e1;stroke-width:1}</style>',
        '<text x="32" y="42" class="title">N9 formal proof boundary</text>',
        '<text x="32" y="68" class="sub">Green: kernel checked at stated scope   Orange: explicitly not established</text>',
    ]
    for tick in range(MAX_COUNT + 1):
        x = LEFT + plot_width * tick / MAX_COUNT
        pieces.append(f'<line x1="{x:.1f}" y1="96" x2="{x:.1f}" y2="520" class="axis"/>')
        pieces.append(f'<text x="{x:.1f}" y="544" text-anchor="middle" class="sub">{tick}</text>')
    for index, row in enumerate(rows):
        y = TOP + index * ROW
        value = int(row["count"])
        width = plot_width * value / MAX_COUNT
        color = "#2f855a" if row["class"] == "mechanized" else "#c05621"
        label = html.escape(row["item"])
        pieces.append(f'<text x="{LEFT - 14}" y="{y + 18}" text-anchor="end" class="label">{label}</text>')
        pieces.append(f'<rect x="{LEFT}" y="{y}" width="{width:.1f}" height="27" rx="2" fill="{color}" stroke="#243447" stroke-width="0.6"/>')
        pieces.append(f'<text x="{LEFT + width + 8:.1f}" y="{y + 19}" class="count">{value}</text>')
    pieces.extend([
        '<text x="540" y="570" text-anchor="middle" class="sub">Named obligations in the packet</text>',
        '<text x="32" y="602" class="sub">Counts describe the inventory only; they do not measure proof effort or universal completeness.</text>',
        '</svg>',
    ])
    svg = ROOT / "formal-status.svg"
    svg.write_text("\n".join(pieces) + "\n")
    subprocess.run(
        ["convert", "-background", "white", str(svg), str(ROOT / "formal-status.png")],
        check=True,
    )


if __name__ == "__main__":
    main()
