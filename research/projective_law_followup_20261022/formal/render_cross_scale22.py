#!/usr/bin/env python3
"""Render the cross-scaled-input certificate as standard data and visuals."""

import csv
import json
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parent
packet = json.loads((ROOT / "cross-scale22.json").read_text())
assert packet["schema"] == "projective-law-cross-scale22/v1"
all_rows = packet["rows"]
rows = [row for row in all_rows if row["kind"] == "cross_input"]
assert len(all_rows) == 23 and len(rows) == 8 and all(row["equal"] is True for row in all_rows)
weight_rows = json.loads((ROOT / "cross-scale22-weights.json").read_text())["rows"]

with (ROOT / "cross-scale22.csv").open("w", newline="") as handle:
    writer = csv.DictWriter(
        handle,
        fieldnames=("rust", "lean", "kind", "z_power", "w_power", "equal"),
        extrasaction="ignore",
        lineterminator="\n",
    )
    writer.writeheader()
    writer.writerows(all_rows)

svg = [
    '<svg xmlns="http://www.w3.org/2000/svg" width="900" height="620" viewBox="0 0 900 620">',
    '<rect width="900" height="620" fill="white"/>',
    '<text x="20" y="31" font-size="20" font-weight="bold">Common 2+2 cross-scaled inputs</text>',
    '<rect x="490" y="18" width="14" height="14" fill="#2b7287"/>',
    '<text x="510" y="31" font-size="14">common Z exponent</text>',
    '<rect x="490" y="39" width="14" height="14" fill="#cc7b3d"/>',
    '<text x="510" y="52" font-size="14">common W exponent</text>',
]
for index, row in enumerate(rows):
    y = 78 + 42 * index
    svg += [
        f'<text x="25" y="{y + 18}" font-size="17">{row["rust"]}</text>',
        f'<rect x="130" y="{y}" width="{row["z_power"] * 75}" height="13" fill="#2b7287"/>',
        f'<rect x="130" y="{y + 15}" width="{row["w_power"] * 75}" height="10" fill="#cc7b3d"/>',
        f'<text x="{145 + row["z_power"] * 75}" y="{y + 14}" font-size="14">Z^{row["z_power"]} W^{row["w_power"]}</text>',
    ]
svg += ['<text x="20" y="428" font-size="16" font-weight="bold">Derived coefficient weights (Lean)</text>']
for index, row in enumerate(weight_rows):
    y = 445 + 43 * index
    svg += [
        f'<text x="25" y="{y + 18}" font-size="17">{row["coefficient"]}</text>',
        f'<rect x="130" y="{y}" width="{row["z_power"] * 56}" height="13" fill="#2b7287"/>',
        f'<rect x="130" y="{y + 15}" width="{row["w_power"] * 56}" height="10" fill="#cc7b3d"/>',
        f'<text x="{145 + row["z_power"] * 56}" y="{y + 14}" font-size="14">Z^{row["z_power"]} W^{row["w_power"]}</text>',
    ]
svg += [
    '<text x="20" y="602" font-size="13">Source expression checks and conditional algebraic weights are separate evidence.</text>',
    '</svg>',
]
(ROOT / "cross-scale22.svg").write_text("\n".join(svg) + "\n")
subprocess.run([
    "convert", "-background", "white", "-density", "144",
    str(ROOT / "cross-scale22.svg"), "-resize", "900x620",
    "-depth", "8", "-colors", "32", "-strip", f"PNG8:{ROOT / 'cross-scale22.png'}",
], check=True)

md = """# Weighted 2+2 cross-scaling boundary

Schema: `projective-law-cross-scale22/v1`. The restricted checker extracts
eight power intermediates and eight cross-scaled input expressions from the
common Rust 2+2 branch and the corresponding Lean `crossScale22` definition.
It then substitutes those inputs into seven Rust and Lean coefficient
expressions. It compares the expanded integer polynomials and rejects a
multiplication-to-addition mutation on each side and a composed-coefficient
subtraction mutation. All 23 expressions match. The
[certificate](cross-scale22.json) records exact source hashes and every
expanded term; the [CSV](cross-scale22.csv) and [graph](cross-scale22.svg)
show all 23 checked rows and the eight input weights respectively.
The two-page [PDF](CROSS_SCALE22.pdf) embeds the source-linked graph.

The Lean theorem `crossScale22_represents_affine` proves that, when the
weighted numerator equations hold, the eight cross-scaled inputs have common
weights `Z=z1*z2` and `W=w1*w2`: `q` has `Z²`, `r` has `Z⁴`, `s` has `Z³W`,
and `t` has `Z⁵W`. The theorem is a polynomial statement over any
commutative ring. Five further Lean theorems derive common weights
`B=Z⁸ B_aff`, `A=Z⁹W A_aff`, and `C=Z⁷W C_aff`, then prove that nonzero
`Z,W` preserve the `B≠0` and `C≠0` branch conditions. The [build
log](cross-scale22-build.log) records the kernel and axiom output. The exact
exponents and theorem names are in the [weights CSV](cross-scale22-weights.csv)
and [JSON](cross-scale22-weights.json). Rust-to-Lean runtime correspondence
and the returned divisor-class statement remain open.

This extends the [seven-coefficient slice](COEFFICIENT_SLICE.md), but does not
establish the full Rust control flow, exceptional branches, or the concrete
Mumford/Pic⁰ addition theorem. The 1,260-target cost ratios are unchanged.

Recheck from this directory with `python3 check_cross_scale22.py --check`,
`lake env lean CrossScale22.lean`, and `python3 render_cross_scale22.py`.
The five additional theorem receipts and weight table have a native checker:
from the parent study directory run `cargo run --release --bin
validate-cross-scale-weights`.
"""
(ROOT / "CROSS_SCALE22.md").write_text(md)

tex = r"""\documentclass[11pt]{article}
\usepackage[margin=0.9in]{geometry}
\usepackage{booktabs}
\usepackage{graphicx}
\usepackage{pdflscape}
\begin{document}
\section*{Weighted 2+2 cross-scaling boundary}
\texttt{projective-law-cross-scale22/v1}. The source checker compares eight
power intermediates, eight cross-scaled inputs, and seven composed coefficient
expressions from Rust and Lean as expanded integer polynomials. All 23 match.
Multiplication-to-addition mutations on both sides and a composed-coefficient
subtraction mutation are rejected.

\begin{center}
\begin{tabular}{lccc}
\toprule
Input & Common $Z$ power & Common $W$ power & Equal\\
\midrule
"""
for row in rows:
    tex += f'{row["rust"]} & {row["z_power"]} & {row["w_power"]} & yes\\\\\n'
tex += r"""\bottomrule
\end{tabular}
\end{center}
\noindent The Lean theorem proves these common weights from the weighted
numerator equations over a commutative ring. Five further theorems derive
$B=Z^8B_{aff}$, $A=Z^9WA_{aff}$, and $C=Z^7WC_{aff}$, plus nonzero
$B,C$ equivalence when $Z,W$ are nonzero. It does not establish the
returned Jacobian class, exceptional branches, or a cost change. The
accompanying CSV and JSON retain all 23 rows and all polynomial terms.
\clearpage
\begin{landscape}
\section*{Input and coefficient weights}
\begin{center}
\includegraphics[width=0.94\linewidth]{cross-scale22.png}
\end{center}
\end{landscape}
\end{document}
"""
(ROOT / "CROSS_SCALE22.tex").write_text(tex)
print("cross-scale22 report sources rendered")
