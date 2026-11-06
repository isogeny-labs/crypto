#!/usr/bin/env python3
"""Render the r29 fixed-base census and exploratory model result without dependencies."""
import json
from pathlib import Path
import subprocess

study = Path(__file__).resolve().parents[2]
evidence = study / "evidence"
base = evidence / "n83-factorbase-variants/r29-attempt-02/factor-base-output/index-summary.json"
result = evidence / "model-expansion-20261031/run-02/result.json"
out = Path(__file__).resolve().parent
summary, model = json.loads(base.read_text()), json.loads(result.read_text())

W, H = 1250, 550
parts = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}">',
         '<rect width="100%" height="100%" fill="white"/>',
         '<style>text{font-family:DejaVu Sans,Arial,sans-serif;fill:#18212b}.title{font-size:24px;font-weight:bold}.label{font-size:16px}.small{font-size:13px}.bar{fill:#2878a8}.axis{stroke:#445;stroke-width:1}</style>',
         '<text x="50" y="38" class="title">N83 r29 fixed bases and seven-base model diagnostic</text>',
         '<text x="50" y="68" class="label">Enumerated points per family; each base reused for m=2–9</text>']
families = summary["families"]
max_points = max(x["point_count"] for x in families)
left, top, barw, barh, gap = 60, 100, 360, 46, 24
for i, row in enumerate(families):
    y = top + i * (barh + gap)
    width = barw * row["point_count"] / max_points
    parts += [f'<text x="{left}" y="{y-8}" class="label">{row["family"].capitalize()}</text>',
              f'<rect x="{left}" y="{y}" width="{width:.1f}" height="{barh}" rx="5" class="bar"/>',
              f'<text x="{left+width+10:.1f}" y="{y+29}" class="small">{row["point_count"]:,} points</text>']
parts += ['<text x="700" y="68" class="label">Grouped leave-one-base-out MAE</text>',
          '<text x="700" y="93" class="small">matched-configuration baseline: 0.000</text>']
x0, y0, scale = 700, 120, 95
for i, row in enumerate(model["folds"]):
    y = y0 + i * 43
    width = row["ridge_mae"] * scale
    parts += [f'<text x="{x0}" y="{y+17}" class="small">r{row["heldout_base"]}</text>',
              f'<rect x="{x0+45}" y="{y}" width="{width:.1f}" height="25" rx="3" class="bar"/>',
              f'<text x="{x0+55+width:.1f}" y="{y+18}" class="small">{row["ridge_mae"]:.3f}</text>']
parts += ['<line x1="50" y1="420" x2="1050" y2="420" class="axis"/>',
          f'<text x="50" y="454" class="label">{model["rows"]} systems; {model["finite_interval_rows"]} interval-midpoint proxies; {model["positive_dimensional_rows"]} positive-dimensional outcomes</text>',
          f'<text x="50" y="483" class="label">Matched configuration outcomes identical across all bases: {model["identical_configuration_outcomes_across_bases"]}/48</text>',
          '<text x="50" y="518" class="small">Exploratory/post-label; interval midpoint is not exact Dreg, FFD, or LFD. No elite-base lead.</text>',
          '</svg>']
svg = out / "r29-model-diagnostic.svg"
svg.write_text("\n".join(parts) + "\n")
png = out / "r29-model-diagnostic.png"
subprocess.run(["rsvg-convert", "-o", str(png), str(svg)], check=True)

tex = r'''\documentclass[10pt]{article}
\usepackage[margin=0.65in]{geometry}
\usepackage{graphicx}
\usepackage[T1]{fontenc}
\usepackage{lmodern}
\begin{document}
\section*{N83 r29 fixed factor bases and model diagnostic}
The fixed-base enumeration covered 134,217,728 candidate x-coordinates for
each of the Koblitz, general binary, and prime families. It retained
134,195,671, 134,226,535, and 134,206,522 affine points, respectively, and
reused each exact base for m=2 through 9. The 384 payload/receipt shard pairs
cover contiguous selector ranges; the local index validation and all 48 PDP
input membership checks passed.

The exploratory seven-base training set has 336 matched rows. Its target is
the midpoint of a finite top-form degree interval, not exact degree of
regularity, FFD, or LFD. The grouped leave-one-base-out ridge fit has aggregate
MAE 2.5022 against 0.0000 for a matched-configuration baseline and wins zero
of seven held-out bases. All 48 matched top-form outcomes are identical across
the bases, so this fit finds no elite-base lead.

The diagnostic identifies a sampling limitation: planted tuples use common
support in the low 26 coordinates, and all seven bases repeat the same point
tuples in these PDP inputs. A lead-generating follow-up must sample
base-specific targets across the full 134-million-selector domain while retaining
the present common-support panel as a control. This is a post-label diagnostic
and does not establish a general base-quality result.

\begin{center}\includegraphics[width=\textwidth]{r29-model-diagnostic.png}\end{center}
\noindent Source files: factor-base index summary SHA-256
\texttt{f6787fb737fb988bcd5089fb1cb755b81f8745a7016e93d37d7c15bf87874236};
model protocol SHA-256
\texttt{2141d9f99a8bc07641b63db1a89d8f7fcaae9d29b4dfee6607e76592ec38e114}.
\end{document}
'''
(out / "report.tex").write_text(tex)
subprocess.run(["pdflatex", "-interaction=nonstopmode", "-halt-on-error", "report.tex"], cwd=out, check=True, stdout=subprocess.DEVNULL)
