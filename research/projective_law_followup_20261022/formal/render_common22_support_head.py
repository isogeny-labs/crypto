#!/usr/bin/env python3
"""Render common-2+2 residual high-coefficient evidence."""

import csv
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parent
packet = json.loads((ROOT / "common22-support-head.json").read_text())
assert packet["schema"] == "projective-law-common22-support-head/v1"
rows = packet["rows"]
assert len(rows) == 8 and all(row["equal"] is True for row in rows)

with (ROOT / "common22-support-head.csv").open("w", newline="") as handle:
    writer = csv.DictWriter(
        handle,
        fieldnames=("obligation", "monomials", "equal"),
        extrasaction="ignore",
        lineterminator="\n",
    )
    writer.writeheader()
    writer.writerows(rows)

svg = [
    '<svg xmlns="http://www.w3.org/2000/svg" width="720" height="445" viewBox="0 0 720 445">',
    '<rect width="720" height="445" fill="white"/>',
    '<text x="20" y="30" font-size="20" font-weight="bold">Residual-support head coefficients</text>',
    '<text x="20" y="51" font-size="14">Bars show expanded integer-polynomial monomial counts.</text>',
]
for index, row in enumerate(rows):
    y = 68 + index * 43
    width = row["monomials"] * 3
    svg += [
        f'<text x="20" y="{y + 18}" font-size="16">{row["obligation"]}</text>',
        f'<rect x="190" y="{y}" width="{width}" height="24" fill="#2b7287"/>',
        f'<text x="{200 + width}" y="{y + 18}" font-size="15">{row["monomials"]}</text>',
    ]
svg += [
    '<text x="20" y="430" font-size="13">Exact theorem: e6 = 0, e5 = B², e4 = 0.</text>',
    '</svg>',
]
(ROOT / "common22-support-head.svg").write_text("\n".join(svg) + "\n")

md = """# Common 2+2 residual-support head

Schema: `projective-law-common22-support-head/v1`. The Lean theorem proves
that the `X⁶`, `X⁵`, and `X⁴` coefficients of `N²-u₁u₂U` are respectively
`0`, `B²`, and `0`, with `B,A,C,Q,Rbar` defined by the unit-weight common
2+2 formulas. It is an exact identity over any commutative ring. The
[build log](common22-support-head-build.log) records the kernel check.

The source checker compares the Rust `B,A,C,Q` polynomials and the factored
`Rbar` with the Lean definitions, then independently expands the three head
coefficients. All eight obligations match; sign mutations of `Q` in Rust and
Lean are rejected. The [certificate](common22-support-head.json) retains
every expanded term, the [CSV](common22-support-head.csv) has one row per
obligation, and the [graph](common22-support-head.svg) shows exact term counts.
The [PDF](COMMON22_SUPPORT_HEAD.pdf) is hash-bound in this directory's
artifact manifest; pull the directory with `tools/artifacts/artifacts.py`
before opening it in a fresh checkout.

This establishes the high-degree shape required for the residual-support
argument in the unit-weight branch. It does not prove the lower coefficients
of `N²-u₁u₂U=B²F`, the input curve congruences, coprime-support uniqueness,
arbitrary weighted normalization, exceptional branches, or the final
Jacobian-class statement. The 1,260-target cost ratios are unchanged.
"""
(ROOT / "COMMON22_SUPPORT_HEAD.md").write_text(md)

tex = r"""\documentclass[11pt]{article}
\usepackage[margin=0.9in]{geometry}
\usepackage{booktabs}
\begin{document}
\section*{Common 2+2 residual-support head}
\texttt{projective-law-common22-support-head/v1}. Lean proves the $X^6$,
$X^5$, and $X^4$ coefficients of $N^2-u_1u_2U$ are $0$, $B^2$, and $0$
over any commutative ring. Five source expressions and three coefficient
identities match after exact integer-polynomial expansion. Rust and Lean
$Q$-sign mutations are rejected.

\begin{center}
\begin{tabular}{lrr}
\toprule
Obligation & Monomials & Equal\\
\midrule
"""
for row in rows:
    label = row["obligation"].replace("_", "\\_")
    tex += f'{label} & {row["monomials"]} & yes\\\\\n'
tex += r"""\bottomrule
\end{tabular}
\end{center}
\noindent This is a high-degree polynomial boundary in the unit-weight
common branch. Lower curve coefficients, weighted normalization, and the
divisor-class theorem remain unproved here. The existing cost ratios do not
change.
\end{document}
"""
(ROOT / "COMMON22_SUPPORT_HEAD.tex").write_text(tex)
print("common22-support-head report sources rendered")
