#!/usr/bin/env python3
"""Render the source/Lean coefficient-slice certificate for independent review."""

import csv
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parent
packet = json.loads((ROOT / "coefficient-slice.json").read_text())
assert packet["schema"] == "projective-law-coefficient-slice/v1"
rows = packet["rows"]
assert len(rows) == 7 and all(row["equal"] is True for row in rows)

with (ROOT / "coefficient-slice.csv").open("w", newline="") as handle:
    writer = csv.DictWriter(
        handle,
        fieldnames=("rust", "lean", "monomials", "equal"),
        extrasaction="ignore",
        lineterminator="\n",
    )
    writer.writeheader()
    writer.writerows(rows)

bars = []
labels = []
for index, row in enumerate(rows):
    y = 62 + index * 42
    width = row["monomials"] * 28
    bars.append(f'<rect x="170" y="{y}" width="{width}" height="25" fill="#2b7287"/>')
    labels.append(f'<text x="25" y="{y + 18}" font-size="17">{row["rust"]} = {row["lean"]}</text>')
    labels.append(f'<text x="{180 + width}" y="{y + 18}" font-size="16">{row["monomials"]}</text>')
svg = (
    '<svg xmlns="http://www.w3.org/2000/svg" width="720" height="395" viewBox="0 0 720 395">\n'
    '<rect width="720" height="395" fill="white"/>\n'
    '<text x="25" y="32" font-size="20" font-weight="bold">Source-derived 2+2 coefficient slice</text>\n'
    + "\n".join(bars + labels)
    + '\n<text x="25" y="375" font-size="14">Bars: integer-polynomial monomial count; all seven equal. No group-law claim.</text>\n'
    '</svg>\n'
)
(ROOT / "coefficient-slice.svg").write_text(svg)

md = """# Source-derived common 2+2 coefficient slice

Schema: `projective-law-coefficient-slice/v1`. The restricted parser extracts
seven straight-line field expressions from `add22_weighted_common` and the
seven corresponding `let` expressions in `generic22Runtime`. It substitutes
earlier assignments, expands each expression over the integers, and requires
exact polynomial equality. The checked mapping is `q1c` to `q₁`, etc.; the
weighted-to-dehomogenized interpretation is outside this check.

All seven coefficient pairs match. Subtraction-to-addition mutations in
Rust `bcoef` and Lean `B` are rejected. The [CSV](coefficient-slice.csv) records each pair,
monomial count, and equality result; the [graph](coefficient-slice.svg) shows
the counts. [Certificate](coefficient-slice.json) records both source hashes
and every expanded monomial with its integer coefficient.

This is a source-derived algebraic check for a narrow fragment. It does not
model Rust control flow, field representation, weighted-coordinate validity,
the remaining addition branches, or equality of the returned divisor class.
The existing full-cohort prices and their relation to matched rho are
unchanged. Run `python3 check_coefficient_slice.py --check` to verify the
frozen certificate; run `python3 check_coefficient_slice.py` and
`python3 render_coefficient_slice.py` to reproduce these artifacts.
"""
(ROOT / "COEFFICIENT_SLICE.md").write_text(md)

tex = r"""\documentclass[11pt]{article}
\usepackage[margin=0.9in]{geometry}
\usepackage{graphicx}
\usepackage{booktabs}
\begin{document}
\section*{Source-derived common 2+2 coefficient slice}
\texttt{projective-law-coefficient-slice/v1}. A restricted parser extracts
seven field expressions from the Rust common 2+2 function and their seven
Lean counterparts. It substitutes earlier assignments and compares expanded
integer polynomials. All seven pairs match; subtraction-to-addition
mutations in the Rust and Lean $B$ coefficients are rejected.

\begin{center}
\begin{tabular}{lrr}
\toprule
Rust / Lean & Monomials & Equal\\
\midrule
"""
for row in rows:
    tex += f'{row["rust"]} / {row["lean"]} & {row["monomials"]} & yes\\\\\n'
tex += r"""\bottomrule
\end{tabular}
\end{center}
\noindent This check covers the named coefficient slice only. It does not
establish Rust operational semantics, weighted-coordinate validity, or the
Jacobian group law. The existing 1,260-target cost ratios remain unchanged.
The accompanying JSON records source hashes; CSV retains one row per pair.
\end{document}
"""
(ROOT / "COEFFICIENT_SLICE.tex").write_text(tex)
print("coefficient-slice report sources rendered")
