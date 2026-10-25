"""Presentation only: draw already-recorded status counts from summary.json."""
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
packet = json.loads((HERE / "results/summary.json").read_text())
models = packet["models"]
rows = []
for model in models:
    statuses = model["lift_counts"]
    failed = sum(count for name, count in statuses.items() if name.endswith(":false"))
    fallback = statuses.get("enumerated_exception_lift:true", 0)
    rows.append((model["p"], model["r"], failed, fallback))

width, height = 840, 390
scale = 1.5
parts = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
         '<rect width="100%" height="100%" fill="white"/>',
         '<text x="32" y="35" font-family="sans-serif" font-size="19" font-weight="bold">Concrete Kummer: complete 16-bit subgroup replay</text>',
         '<text x="32" y="58" font-family="sans-serif" font-size="12">All 119,573 states: xADD mismatch or undefined = 0. Signed-lift outcomes by model (counts).</text>']
for x, label, color in [(250, "Signed lift returned no divisor", "#b45309"), (515, "Enumerated exception lift succeeded", "#2563eb")]:
    parts += [f'<rect x="{x}" y="72" width="14" height="14" fill="{color}"/>',
              f'<text x="{x+21}" y="84" font-family="sans-serif" font-size="12">{label}</text>']
for i, (p, r, failed, fallback) in enumerate(rows):
    y = 137 + i * 75
    parts.append(f'<text x="32" y="{y+12}" font-family="sans-serif" font-size="14">F{p}, r={r}</text>')
    for j, (value, color) in enumerate([(failed, "#b45309"), (fallback, "#2563eb")]):
        yy = y - 7 + j * 23
        parts.append(f'<rect x="250" y="{yy}" width="{value*scale:.1f}" height="16" fill="{color}"/>')
        parts.append(f'<text x="{260+value*scale:.1f}" y="{yy+13}" font-family="sans-serif" font-size="12">{value}</text>')
parts += ['<line x1="250" y1="330" x2="730" y2="330" stroke="#444"/>',
          '<text x="250" y="349" font-family="sans-serif" font-size="11">0</text>',
          '<text x="400" y="349" font-family="sans-serif" font-size="11">100</text>',
          '<text x="550" y="349" font-family="sans-serif" font-size="11">200</text>',
          '<text x="700" y="349" font-family="sans-serif" font-size="11">300</text>',
          '<text x="32" y="378" font-family="sans-serif" font-size="11">KUMMER-N13 / 2026-10-25; xADD is 3 calls per state, lift is 1 call per state. No larger-model bound.</text>',
          '</svg>']
(HERE / "status-graph.svg").write_text("\n".join(parts) + "\n")
