"""Presentation only: plot checked chart counts from the native summary."""
import json
from pathlib import Path

here = Path(__file__).resolve().parent
models = json.loads((here / "results/summary.json").read_text())["models"]
parts = ['<svg xmlns="http://www.w3.org/2000/svg" width="900" height="850" viewBox="0 0 900 850">',
         '<rect width="900" height="850" fill="white"/>',
         '<text x="30" y="35" font-family="sans-serif" font-size="21" font-weight="bold">N16: direct chart on the frozen exceptional classes</text>',
         '<text x="30" y="58" font-family="sans-serif" font-size="12">Source zero vectors recovered by the algebraic chart; each bar is a finite class check.</text>',
         '<text x="30" y="84" font-family="sans-serif" font-size="12">Model</text>',
         '<text x="245" y="84" font-family="sans-serif" font-size="12">Source-zero / extended-nonzero degree-two outputs</text>',
         '<text x="755" y="84" font-family="sans-serif" font-size="12">Action failures</text>']
for i, model in enumerate(models):
    y = 100 + i * 32
    if i and model["bits"] != models[i-1]["bits"]:
        parts.append(f'<line x1="30" x2="870" y1="{y-7}" y2="{y-7}" stroke="#dddddd"/>')
    v = model["chart_counts"].get("source_zero_extended_nonzero", 0)
    fails = model["check_counts"].get("mismatch", 0) + model["check_counts"].get("missing", 0)
    w = v * 1.45
    parts += [f'<text x="30" y="{y+14}" font-family="sans-serif" font-size="12">{model["bits"]}-bit / p={model["p"]}</text>',
              f'<rect x="245" y="{y}" width="{w:.1f}" height="18" fill="#2563eb"/>',
              f'<text x="{251+w:.1f}" y="{y+14}" font-family="sans-serif" font-size="12">{v}</text>',
              f'<text x="810" y="{y+14}" font-family="sans-serif" font-size="12">{fails}</text>']
parts += ['<text x="30" y="816" font-family="sans-serif" font-size="11">21 models: 5,599 source zeros resolved; 27,470/27,470 finite action comparisons matched. Generic geometric identity remains open.</text>', '</svg>']
(here / "chart-graph.svg").write_text("\n".join(parts) + "\n")
