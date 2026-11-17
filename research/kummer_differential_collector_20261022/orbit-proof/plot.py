#!/usr/bin/env python3
import csv
import math
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent
rows = list(csv.DictReader((ROOT / "results/bounds.csv").open()))
models = {}
for row in rows:
    models.setdefault((int(row["bits"]), int(row["p"])), int(row["minimum_offset_labels"]))

W, H = 1120, 480
panels = [(70, 75, 455, 325), (625, 75, 455, 325)]
bits_min, bits_max = 15, 41


def sx(bits, panel):
    x, _y, w, _h = panel
    return x + (bits - bits_min) / (bits_max - bits_min) * w


def sy(value, panel, lo, hi):
    _x, y, _w, h = panel
    t = (math.log10(value) - math.log10(lo)) / (math.log10(hi) - math.log10(lo))
    return y + h - t * h


svg = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}">',
       '<rect width="100%" height="100%" fill="white"/>',
       '<style>text{font-family:DejaVu Sans,Arial,sans-serif;fill:#222}.axis{stroke:#333;stroke-width:1.2}.grid{stroke:#ddd;stroke-width:1}.pt{fill:#2463a7;stroke:white;stroke-width:1}.pt2{fill:#a4492f;stroke:white;stroke-width:1}</style>',
       '<text x="560" y="28" text-anchor="middle" font-size="18" font-weight="bold">Fixed-offset sidecar orbit bounds — 21 models × 16 jumps</text>']

for panel, title, ylabel, lo, hi in [
    (panels[0], "Exact orbit size (one aggregate/model)", "minimum stored offsets", 1e4, 1e12),
    (panels[1], "Current explicit [u64;4] state", "state size (GiB)", 1e-3, 1e5),
]:
    x, y, w, h = panel
    svg += [f'<line class="axis" x1="{x}" y1="{y+h}" x2="{x+w}" y2="{y+h}"/>',
            f'<line class="axis" x1="{x}" y1="{y}" x2="{x}" y2="{y+h}"/>',
            f'<text x="{x+w/2}" y="{y-16}" text-anchor="middle" font-size="14" font-weight="bold">{title}</text>',
            f'<text x="{x+w/2}" y="{y+h+52}" text-anchor="middle" font-size="13">cohort size class (bits)</text>',
            f'<text x="{x-52}" y="{y+h/2}" text-anchor="middle" font-size="13" transform="rotate(-90 {x-52} {y+h/2})">{ylabel}</text>']
    for bit in [16, 20, 24, 28, 32, 36, 40]:
        px = sx(bit, panel)
        svg += [f'<line class="grid" x1="{px}" y1="{y}" x2="{px}" y2="{y+h}"/>',
                f'<text x="{px}" y="{y+h+22}" text-anchor="middle" font-size="11">{bit}</text>']
    for exponent in range(math.floor(math.log10(lo)), math.ceil(math.log10(hi)) + 1, 2):
        val = 10 ** exponent
        py = sy(val, panel, lo, hi)
        svg += [f'<line class="grid" x1="{x}" y1="{py}" x2="{x+w}" y2="{py}"/>',
                f'<text x="{x-8}" y="{py+4}" text-anchor="end" font-size="10">10^{exponent}</text>']

for (bits, _p), labels in sorted(models.items()):
    svg.append(f'<circle class="pt" cx="{sx(bits, panels[0]):.2f}" cy="{sy(labels, panels[0], 1e4, 1e12):.2f}" r="5"/>')
    gib = 32 * labels / 1024**3
    svg.append(f'<circle class="pt2" cx="{sx(bits, panels[1]):.2f}" cy="{sy(gib, panels[1], 1e-3, 1e5):.2f}" r="5"/>')

svg += ['<text x="560" y="462" text-anchor="middle" font-size="11" fill="#555">Each point aggregates one model; all 16 nonzero jumps have the same orbit in its prime-order subgroup.</text>', '</svg>']
(ROOT / "orbit-bounds.svg").write_text("\n".join(svg) + "\n")
subprocess.run(["convert", str(ROOT / "orbit-bounds.svg"), str(ROOT / "orbit-bounds.png")], check=True)
