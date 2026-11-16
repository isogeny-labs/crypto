"""Presentation only: graph the recorded model-level lift statuses."""
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
models = json.loads((HERE / "results/summary.json").read_text())["models"]
width, height = 880, 720
parts = [
    f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
    '<rect width="100%" height="100%" fill="white"/>',
    '<text x="34" y="34" font-family="sans-serif" font-size="20" font-weight="bold">Concrete Kummer: frozen state sample, 18 further models</text>',
    '<text x="34" y="57" font-family="sans-serif" font-size="12">1,029 signed states per model; all 55,566 xADD outputs matched the Jacobian oracle.</text>',
]
legend = [
    ("general_conversion_inapplicable:false", "#a16207", "General conversion null"),
    ("Delta_zero:false", "#7c3aed", "D denominator zero"),
    ("b1_coefficient_zero:false", "#dc2626", "b1 coefficient zero"),
]
for i, (_, color, label) in enumerate(legend):
    x = 33 + i * 250
    parts += [f'<rect x="{x}" y="75" width="14" height="14" fill="{color}"/>',
              f'<text x="{x+21}" y="87" font-family="sans-serif" font-size="12">{label}</text>']
scale = 65
for i, model in enumerate(models):
    y = 112 + i * 29
    if i and model["bits"] != models[i - 1]["bits"]:
        parts.append(f'<line x1="33" x2="840" y1="{y-8}" y2="{y-8}" stroke="#dddddd"/>')
    parts.append(f'<text x="34" y="{y+11}" font-family="sans-serif" font-size="13">{model["bits"]}-bit / F{model["p"]}</text>')
    x = 240
    for key, color, _ in legend:
        value = model["lift_counts"].get(key, 0)
        if value:
            parts.append(f'<rect x="{x}" y="{y}" width="{value*scale}" height="17" fill="{color}"/>')
            x += value * scale
    parts.append(f'<text x="{x+8}" y="{y+13}" font-family="sans-serif" font-size="12">{model["lift_accounting"]["final_no_divisor"]}</text>')
parts += [
    '<line x1="240" y1="650" x2="695" y2="650" stroke="#444"/>',
    '<text x="240" y="668" font-family="sans-serif" font-size="11">0</text>',
    '<text x="305" y="668" font-family="sans-serif" font-size="11">1</text>',
    '<text x="370" y="668" font-family="sans-serif" font-size="11">2</text>',
    '<text x="435" y="668" font-family="sans-serif" font-size="11">3</text>',
    '<text x="500" y="668" font-family="sans-serif" font-size="11">4</text>',
    '<text x="565" y="668" font-family="sans-serif" font-size="11">5</text>',
    '<text x="630" y="668" font-family="sans-serif" font-size="11">6</text>',
    '<text x="695" y="668" font-family="sans-serif" font-size="11">7</text>',
    '<text x="34" y="702" font-family="sans-serif" font-size="11">KUMMER-N14 / 2026-10-25; counts are final nulls after the source fallback. All-state bound remains unknown.</text>',
    '</svg>',
]
(HERE / "status-graph.svg").write_text("\n".join(parts) + "\n")
