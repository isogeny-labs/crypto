#!/usr/bin/env python3
"""Lay out recorded audit values; does not recompute the research model."""
import hashlib
import html
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
provenance = json.loads((ROOT / "provenance.json").read_text())
receipts = {}
for name, expected in provenance["data_sha256"].items():
    raw = (ROOT / name).read_bytes()
    if hashlib.sha256(raw).hexdigest() != expected:
        raise SystemExit(f"receipt hash mismatch: {name}")
    receipts[name] = json.loads(raw)
old = {row["curve_id"]: row for row in receipts["previous-audit.json"]["curves"]}
current = receipts["current-audit.json"]["curves"]
svg = ['<svg xmlns="http://www.w3.org/2000/svg" width="960" height="490" viewBox="0 0 960 490">',
       '<rect width="960" height="490" fill="white"/>',
       '<g font-family="Arial, sans-serif" fill="#202a38">',
       '<text x="35" y="30" font-size="21" font-weight="bold">Conditional genus-128 formula prices</text>',
       '<text x="35" y="54" font-size="14">Recorded sweeps; review refresh 2026-10-19. No timed samples or empirical intervals.</text>']
colors = {"old": "#727b88", "current": "#007e87", "rho": "#be600b"}
for panel, row in enumerate(current):
    left, top, width, height = 65 + panel * 465, 115, 370, 275
    x = lambda bound: left + (bound - 1) / 83 * width
    y = lambda value: top + height - (value - 50) / 350 * height
    svg.append(f'<text x="{left}" y="90" font-size="18" font-weight="bold">{html.escape(row["name"])}</text>')
    for value in (50, 100, 150, 200, 250, 300, 350, 400):
        yy = y(value)
        svg.append(f'<path d="M{left},{yy:.2f}h{width}" stroke="#e3e7eb"/>')
        svg.append(f'<text x="{left-8}" y="{yy+4:.2f}" text-anchor="end" font-size="11">{value}</text>')
    for bound in (1, 15, 30, 45, 60, 84):
        svg.append(f'<text x="{x(bound):.2f}" y="{top+height+20}" text-anchor="middle" font-size="11">{bound}</text>')
    for tag, audit in (("old", old[row["curve_id"]]["audit"]), ("current", row["audit"])):
        points = " ".join(f'{x(p["degree_bound"]):.2f},{y(p["log2_total"]):.2f}' for p in audit["sweep"])
        svg.append(f'<polyline points="{points}" fill="none" stroke="{colors[tag]}" stroke-width="2"/>')
        best = audit["best"]
        svg.append(f'<circle cx="{x(best["degree_bound"]):.2f}" cy="{y(best["log2_total"]):.2f}" r="4" fill="{colors[tag]}"/>')
    yy = y(row["reference_rho_log2"])
    svg.append(f'<path d="M{left},{yy:.2f}h{width}" stroke="{colors["rho"]}" stroke-width="2" stroke-dasharray="6 4"/>')
    svg.append(f'<text x="{left+width/2}" y="431" text-anchor="middle" font-size="13">Smoothness degree bound (84 recorded bounds)</text>')
    svg.append(f'<text x="{left+10}" y="{top+20}" font-size="12">Current minimum: {row["audit"]["best"]["log2_total"]:.6f} at b=15</text>')
svg.append('<text x="16" y="268" text-anchor="middle" transform="rotate(-90 16 268)" font-size="13">log2 modeled bit-operation cost</text>')
for i, (tag, label) in enumerate((("old", "Previous audit"), ("current", "Current audit"), ("rho", "Matched formula rho"))):
    xx = 200 + 210 * i
    svg.append(f'<path d="M{xx},464h25" stroke="{colors[tag]}" stroke-width="3"/>')
    svg.append(f'<text x="{xx+32}" y="469" font-size="13">{label}</text>')
svg.append('</g></svg>')
(ROOT / "cost-sweep.svg").write_text("\n".join(svg) + "\n")
