"""Presentation only: graph counts already validated in summary.json."""
import json
import math
from pathlib import Path

here = Path(__file__).resolve().parent
models = json.loads((here / "results/summary.json").read_text())["models"]
parts = ['<svg xmlns="http://www.w3.org/2000/svg" width="920" height="850" viewBox="0 0 920 850">',
         '<rect width="920" height="850" fill="white"/>',
         '<text x="32" y="35" font-family="sans-serif" font-size="21" font-weight="bold">N15: bounded signed-lift fallback by frozen model</text>',
         '<text x="32" y="58" font-family="sans-serif" font-size="12">Three 16-bit subgroups exhaustive; 18 larger models sampled at 1,029 states each. Bars use log2(count + 1).</text>',
         '<text x="32" y="85" font-family="sans-serif" font-size="12">Model / field p</text>',
         '<text x="265" y="85" font-family="sans-serif" font-size="12">Fast failures recovered by bounded fallback</text>',
         '<text x="785" y="85" font-family="sans-serif" font-size="12">Final nulls</text>']
for i, model in enumerate(models):
    y = 100 + 32 * i
    if i and model["bits"] != models[i - 1]["bits"]:
        parts.append(f'<line x1="32" x2="884" y1="{y-7}" y2="{y-7}" stroke="#dddddd"/>')
    count = model["lift_accounting"]["enumerated_fallback_succeeded"]
    null = model["lift_accounting"]["final_no_divisor"]
    width = 48 * math.log2(count + 1)
    parts.extend([f'<text x="32" y="{y+14}" font-family="sans-serif" font-size="12">{model["bits"]}-bit / {model["p"]}</text>',
                  f'<rect x="265" y="{y}" width="{width:.1f}" height="18" fill="#2563eb"/>',
                  f'<text x="{271+width:.1f}" y="{y+14}" font-family="sans-serif" font-size="12">{count}</text>',
                  f'<text x="814" y="{y+14}" font-family="sans-serif" font-size="12">{null}</text>'])
parts.extend(['<text x="32" y="818" font-family="sans-serif" font-size="11">138,095 checked states; 1,125 recovered fast failures; zero final nulls in this replay. No all-state source bound or rho ratio.</text>',
              '</svg>'])
(here / "status-graph.svg").write_text("\n".join(parts) + "\n")
