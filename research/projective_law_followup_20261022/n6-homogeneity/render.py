"""Layout of the exact integer rows emitted by scaling.rs; no arithmetic result is computed here."""
import csv
from pathlib import Path

here = Path(__file__).resolve().parent
with (here / "scaling.csv").open(newline="") as stream:
    rows = list(csv.DictReader(stream))

def xy(row, key):
    return 90 + 150 * (int(row["mu"]) - 1), 410 - 2.7 * abs(int(row[key]))

parts = [
    '<svg xmlns="http://www.w3.org/2000/svg" width="800" height="490" viewBox="0 0 800 490">',
    '<rect width="800" height="490" fill="white"/>',
    '<text x="44" y="35" font-family="sans-serif" font-size="22">N6 weighted-coordinate scaling witness</text>',
    '<text x="44" y="58" font-family="sans-serif" font-size="13">Same affine divisor: S and W scale by μ; q=0, r=s=d=w=1, t=h=0</text>',
    '<path d="M90 72 V410 H690" fill="none" stroke="#444" stroke-width="2"/>',
    '<text x="27" y="94" font-family="sans-serif" font-size="13">|AA|</text>',
    '<text x="696" y="417" font-family="sans-serif" font-size="13">μ</text>',
]
for tick in (0, 25, 50, 75, 100, 125):
    y = 410 - 2.7 * tick
    parts.append(f'<path d="M85 {y:.1f} H690" stroke="#ddd"/>')
    parts.append(f'<text x="48" y="{y+5:.1f}" font-family="sans-serif" font-size="12">{tick}</text>')
for row in rows:
    x = 90 + 150 * (int(row["mu"]) - 1)
    parts.append(f'<text x="{x-4}" y="431" font-family="sans-serif" font-size="13">{row["mu"]}</text>')
for key, color, label, ylegend in [
    ("aa_squared_w", "#1d6aa7", "implemented W²: |AA|=μ³", 455),
    ("aa_linear_w", "#bd531e", "author-code W¹: |AA|=μ²", 477),
]:
    points = [xy(row, key) for row in rows]
    path = " ".join(("M" if i == 0 else "L") + f"{x:.1f},{y:.1f}" for i, (x, y) in enumerate(points))
    parts.append(f'<path d="{path}" fill="none" stroke="{color}" stroke-width="3"/>')
    for x, y in points:
        parts.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="5" fill="{color}"/>')
    parts.append(f'<rect x="440" y="{ylegend-12}" width="24" height="4" fill="{color}"/>')
    parts.append(f'<text x="472" y="{ylegend-4}" font-family="sans-serif" font-size="13">{label}</text>')
parts.append('</svg>')
(here / "scaling.svg").write_text("\n".join(parts) + "\n")
