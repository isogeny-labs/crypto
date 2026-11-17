#!/usr/bin/env python3
"""Render recorded chain-exact Dreg rows as an SVG and adjacent CSV."""

from __future__ import annotations

import csv
import json
import sys
from pathlib import Path


COLORS = {"koblitz": "#1f77b4", "binary": "#d62728"}
SHAPES = {
    ("koblitz", "natural"): "circle",
    ("koblitz", "planted"): "square",
    ("binary", "natural"): "triangle",
    ("binary", "planted"): "diamond",
}


def parse_run_id(run_id: str) -> tuple[str, int, str]:
    # IDs are spike38-{family}-m{m}-{natural|planted}.
    parts = run_id.split("-")
    if len(parts) != 4 or parts[0] != "spike38" or not parts[2].startswith("m"):
        raise ValueError(f"unexpected run id: {run_id}")
    return parts[1], int(parts[2][1:]), parts[3]


def marker(shape: str, x: float, y: float, color: str) -> str:
    if shape == "circle":
        return f'<circle cx="{x:.1f}" cy="{y:.1f}" r="5" fill="{color}" stroke="#222" stroke-width="0.8"/>'
    if shape == "square":
        return f'<rect x="{x-4:.1f}" y="{y-4:.1f}" width="8" height="8" fill="{color}" stroke="#222" stroke-width="0.8"/>'
    if shape == "triangle":
        return f'<path d="M {x:.1f} {y-5:.1f} L {x+5:.1f} {y+4:.1f} L {x-5:.1f} {y+4:.1f} Z" fill="{color}" stroke="#222" stroke-width="0.8"/>'
    return f'<path d="M {x:.1f} {y-5:.1f} L {x+5:.1f} {y:.1f} L {x:.1f} {y+5:.1f} L {x-5:.1f} {y:.1f} Z" fill="{color}" stroke="#222" stroke-width="0.8"/>'


def main(export_path: Path, out_dir: Path) -> None:
    export = json.loads(export_path.read_text(encoding="utf-8"))
    rows = []
    for run in export["runs"]:
        counted = run["measurements"]["counted"]
        if run["outcome_class"] != "exact_original_top" or counted.get("dreg_exact") is None:
            continue
        family, m, target = parse_run_id(run["run_id"])
        rows.append(
            {
                "run_id": run["run_id"],
                "family": family,
                "m": m,
                "target_kind": target,
                "dreg_exact_degree": int(counted["dreg_exact"]),
                "predicted_degree": 83 * (m - 2) + 28,
            }
        )
    if len(rows) < 2:
        raise SystemExit("fewer than two exact comparable values; use the pipeline diagram instead")
    out_dir.mkdir(parents=True, exist_ok=True)
    csv_path = out_dir / "plot-data.csv"
    with csv_path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)

    width, height = 900, 560
    left, right, top, bottom = 88, 36, 62, 90
    plot_w, plot_h = width - left - right, height - top - bottom
    y_min, y_max = 90, 630

    def sx(m: int, jitter: float = 0.0) -> float:
        return left + (m - 3 + jitter) / 6 * plot_w

    def sy(degree: int) -> float:
        return top + (y_max - degree) / (y_max - y_min) * plot_h

    elements = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
        '<rect width="100%" height="100%" fill="white"/>',
        '<style>text{font-family:Arial,sans-serif;fill:#222}.small{font-size:12px}.label{font-size:14px}.title{font-size:18px;font-weight:bold}</style>',
        '<text class="title" x="450" y="27" text-anchor="middle">N83 spike38 exact original-top Dreg by m</text>',
        f'<text class="small" x="450" y="46" text-anchor="middle">{len(rows)} exact cases; 4 cases per m; deterministic certificates; no uncertainty interval</text>',
    ]
    for degree in range(100, 601, 50):
        y = sy(degree)
        elements.append(f'<line x1="{left}" y1="{y:.1f}" x2="{width-right}" y2="{y:.1f}" stroke="#ddd"/>')
        elements.append(f'<text class="small" x="{left-12}" y="{y+4:.1f}" text-anchor="end">{degree}</text>')
    for m in range(3, 10):
        x = sx(m)
        elements.append(f'<line x1="{x:.1f}" y1="{top}" x2="{x:.1f}" y2="{height-bottom}" stroke="#eee"/>')
        elements.append(f'<text class="label" x="{x:.1f}" y="{height-bottom+24}" text-anchor="middle">{m}</text>')
        pred = 83 * (m - 2) + 28
        py = sy(pred)
        elements.append(f'<line x1="{x-20:.1f}" y1="{py:.1f}" x2="{x+20:.1f}" y2="{py:.1f}" stroke="#333" stroke-width="2"/>')
    elements.extend(
        [
            f'<line x1="{left}" y1="{top}" x2="{left}" y2="{height-bottom}" stroke="#222"/>',
            f'<line x1="{left}" y1="{height-bottom}" x2="{width-right}" y2="{height-bottom}" stroke="#222"/>',
            f'<text class="label" transform="translate(22 {height/2}) rotate(-90)" text-anchor="middle">Exact original-top degree of regularity (degree)</text>',
            f'<text class="label" x="{(left+width-right)/2}" y="{height-18}" text-anchor="middle">Summation-polynomial parameter m</text>',
        ]
    )
    jitter_map = {
        ("koblitz", "natural"): -0.18,
        ("koblitz", "planted"): -0.06,
        ("binary", "natural"): 0.06,
        ("binary", "planted"): 0.18,
    }
    for row in rows:
        family, target = row["family"], row["target_kind"]
        elements.append(marker(SHAPES[(family, target)], sx(row["m"], jitter_map[(family, target)]), sy(row["dreg_exact_degree"]), COLORS[family]))
    legend_y = height - bottom + 50
    legend = [
        ("koblitz", "natural", "Koblitz natural"),
        ("koblitz", "planted", "Koblitz planted"),
        ("binary", "natural", "Binary natural"),
        ("binary", "planted", "Binary planted"),
    ]
    x0 = left + 8
    for family, target, label in legend:
        elements.append(marker(SHAPES[(family, target)], x0, legend_y - 4, COLORS[family]))
        elements.append(f'<text class="small" x="{x0+10}" y="{legend_y}">{label}</text>')
        x0 += 170
    elements.append(f'<text class="small" x="{width-right}" y="{height-4}" text-anchor="end">Horizontal ticks: predicted 83(m−2)+28. Run: {export["export_id"]}</text>')
    elements.append("</svg>")
    (out_dir / "dreg-by-m.svg").write_text("\n".join(elements) + "\n", encoding="utf-8")
    print(json.dumps({"svg": str(out_dir / "dreg-by-m.svg"), "csv": str(csv_path), "n": len(rows)}, sort_keys=True))


if __name__ == "__main__":
    if len(sys.argv) != 3:
        raise SystemExit(f"usage: {Path(sys.argv[0]).name} EXPORT_JSON OUTPUT_DIR")
    main(Path(sys.argv[1]), Path(sys.argv[2]))
