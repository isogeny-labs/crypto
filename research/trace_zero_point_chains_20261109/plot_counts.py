"""Render already recorded circuit counts; no arithmetic or solver research."""
import json
from pathlib import Path

root = Path(__file__).resolve().parent
data = json.loads((root / "counts.json").read_text())
svg = ['<svg xmlns="http://www.w3.org/2000/svg" width="1080" height="420" viewBox="0 0 1080 420">',
       '<rect width="1080" height="420" fill="white"/>',
       '<g font-family="DejaVu Sans,sans-serif" fill="#1a3342">',
       '<text x="25" y="28" font-size="23">Constant specialization: exact circuit allocations</text>',
       '<text x="25" y="55" font-size="15">Constructed SAT variables (count) · m3 singleton correctness controls</text>']
for panel, row in enumerate(data["specialization"]):
    left = 75 + 530 * panel
    svg.append(f'<text x="{left}" y="88" font-size="18">n={row["n"]}</text>')
    for tick in [0, 40000, 80000, 120000]:
        y = 330 - tick / 120000 * 210
        svg.append(f'<path d="M{left} {y}h430" stroke="#dde5eb"/>')
        svg.append(f'<text x="{left-8}" y="{y+4}" text-anchor="end" font-size="11">{tick:,}</text>')
    for i, (label, key, color) in enumerate([
        ("Default", "default", "#778899"),
        ("Fixed target", "target", "#3682a0"),
        ("Fixed domain", "fixed_domain", "#45a586"),
    ]):
        value = row[key]
        height = value / 120000 * 210
        x = left + 25 + i * 140
        svg.append(f'<rect x="{x}" y="{330-height}" width="95" height="{height}" fill="{color}"/>')
        svg.append(f'<text x="{x+47}" y="{322-height}" text-anchor="middle" font-size="15">{value:,}</text>')
        svg.append(f'<text x="{x+47}" y="351" text-anchor="middle" font-size="14">{label}</text>')
svg.extend(['<text x="25" y="390" font-size="13">One construction per bar; no runtime or uncertainty sample. Source: checks/native-tests.txt · 2026-11-09</text>', '</g></svg>'])
(root / "counts.svg").write_text("\n".join(svg) + "\n")
