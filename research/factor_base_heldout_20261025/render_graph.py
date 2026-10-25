#!/usr/bin/env python3
"""Layout only: draw ratios already recorded by the native analyzers."""
import json
from html import escape
from pathlib import Path

ROOT = Path(__file__).parent
selection = json.loads((ROOT / "results/selection.json").read_text())
evaluation = json.loads((ROOT / "results/evaluation-summary.json").read_text())
training = selection["candidate_scores"]
eval_all = next(item for item in evaluation["summary"] if item["scope"] == "all")
eval_ratio = eval_all["representative"]["candidate_over_rho"]
eval_text = "null" if eval_ratio is None else f"{eval_ratio:.6f}"
width, height = 1080, 610
left, right, top, bottom = 100, 1030, 85, 510
vmax = max(5.0, max((item["representative_ratio"] or 0) for item in training) * 1.08, (eval_ratio or 0) * 1.08)
scale = (bottom - top) / vmax
y = lambda v: bottom - v * scale
parts = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
         '<rect width="100%" height="100%" fill="white"/>',
         '<text x="100" y="36" font-size="24" font-family="sans-serif" font-weight="bold">N12 model-held-out factor-base tuning</text>',
         '<text x="100" y="61" font-size="14" font-family="sans-serif">GAE candidate / matched rho, S/M=0.5, I/M=10, logical price=0.1</text>']
for tick in range(int(vmax) + 1):
    yy = y(tick)
    parts += [f'<line x1="{left}" y1="{yy:.2f}" x2="{right}" y2="{yy:.2f}" stroke="#e5e7eb"/>',
              f'<text x="{left-12}" y="{yy+5:.2f}" text-anchor="end" font-size="13" font-family="sans-serif">{tick}</text>']
parts.append(f'<line x1="{left}" y1="{y(1):.2f}" x2="{right}" y2="{y(1):.2f}" stroke="#be123c" stroke-width="2" stroke-dasharray="7,5"/>')
bar_width = 86
step = 125
for index, item in enumerate(training):
    x = left + 40 + index * step
    value = item["representative_ratio"]
    color = "#2563eb" if item["policy"] == selection["selected_policy"] else "#93c5fd"
    if value is None:
        parts.append(f'<text x="{x+bar_width/2}" y="{bottom-12}" text-anchor="middle" font-size="13" font-family="sans-serif">null</text>')
    else:
        parts += [f'<rect x="{x}" y="{y(value):.2f}" width="{bar_width}" height="{bottom-y(value):.2f}" fill="{color}"/>',
                  f'<text x="{x+bar_width/2}" y="{y(value)-9:.2f}" text-anchor="middle" font-size="13" font-family="sans-serif">{value:.3f}</text>']
    parts.append(f'<text x="{x+bar_width/2}" y="{bottom+24}" text-anchor="middle" font-size="12" font-family="sans-serif">{escape(item["policy"])}</text>')
parts += [f'<text x="{left}" y="{height-42}" font-size="14" font-family="sans-serif">Training: 14 models, 840 targets per policy. Blue bar: frozen choice.</text>',
          f'<text x="{left}" y="{height-18}" font-size="14" font-family="sans-serif">Evaluation: 7 unseen models, 420 targets, selected ratio {eval_text}; all failures retained.</text>',
          '</svg>']
(ROOT / "factor-base-heldout.svg").write_text("\n".join(parts) + "\n")

sizes = (16, 20, 24, 28, 32, 36, 40)
size_rows = [next(item for item in evaluation["summary"] if item["scope"] == f"bits-{bits}") for bits in sizes]
values = [row["representative"]["candidate_over_rho"] for row in size_rows]
valid = [v for v in values if v is not None]
ymax = max(3.0, max(valid, default=1.0) * 1.12)
top2, bottom2 = 75, 475
yy = lambda v: bottom2 - v * (bottom2 - top2) / ymax
parts2 = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="600" viewBox="0 0 {width} 600">',
          '<rect width="100%" height="100%" fill="white"/>',
          '<text x="100" y="36" font-size="24" font-family="sans-serif" font-weight="bold">N12 held-out evaluation by size</text>',
          '<text x="100" y="60" font-size="14" font-family="sans-serif">Frozen policy; 60 new public targets on one unseen model per size; representative GAE ratio</text>']
for tick in range(int(ymax) + 1):
    z = yy(tick)
    parts2 += [f'<line x1="100" y1="{z:.2f}" x2="1030" y2="{z:.2f}" stroke="#e5e7eb"/>',
               f'<text x="87" y="{z+5:.2f}" text-anchor="end" font-size="13" font-family="sans-serif">{tick}</text>']
parts2.append(f'<line x1="100" y1="{yy(1):.2f}" x2="1030" y2="{yy(1):.2f}" stroke="#be123c" stroke-width="2" stroke-dasharray="7,5"/>')
for index, (bits, value, row) in enumerate(zip(sizes, values, size_rows)):
    x = 145 + index * 140
    parts2.append(f'<text x="{x}" y="{bottom2+28}" text-anchor="middle" font-size="14" font-family="sans-serif">{bits} bits</text>')
    if value is not None:
        parts2 += [f'<circle cx="{x}" cy="{yy(value):.2f}" r="7" fill="#2563eb"/>',
                   f'<text x="{x}" y="{yy(value)-12:.2f}" text-anchor="middle" font-size="13" font-family="sans-serif">{value:.3f}</text>']
    else:
        parts2.append(f'<text x="{x}" y="{bottom2-20}" text-anchor="middle" font-size="13" font-family="sans-serif">null</text>')
parts2 += [f'<text x="100" y="555" font-size="14" font-family="sans-serif">Matched source-negation rho; {evaluation["verified"]}/420 candidate and {evaluation["rho_verified"]}/420 rho answers verified.</text>', '</svg>']
(ROOT / "evaluation-by-size.svg").write_text("\n".join(parts2) + "\n")
