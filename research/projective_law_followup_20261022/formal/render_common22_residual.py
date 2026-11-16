#!/usr/bin/env python3
"""Render the exact common-2+2 residual identity packet."""

import csv
import json
import math
from pathlib import Path


ROOT = Path(__file__).resolve().parent
packet = json.loads((ROOT / "common22-residual.json").read_text())
assert packet["schema"] == "projective-law-common22-residual/v1"
rows = packet["rows"]
assert len(rows) == 5 and all(row["equal"] is True for row in rows)

with (ROOT / "common22-residual-summary.csv").open("w", newline="") as handle:
    writer = csv.DictWriter(
        handle,
        fieldnames=("obligation", "monomials", "equal", "polynomial_sha256"),
        extrasaction="ignore",
        lineterminator="\n",
    )
    writer.writeheader()
    writer.writerows(rows)

bars = [
    '<svg xmlns="http://www.w3.org/2000/svg" width="850" height="365" viewBox="0 0 850 365">',
    '<rect width="850" height="365" fill="white"/>',
    '<text x="20" y="32" font-size="20" font-weight="bold">Common 2+2 output polynomial terms</text>',
    '<text x="20" y="55" font-size="14">Bar length uses log10(count + 1); labels show exact counts.</text>',
]
for index, row in enumerate(rows):
    y = 78 + 48 * index
    width = round(125 * math.log10(row["monomials"] + 1))
    bars += [
        f'<text x="20" y="{y + 20}" font-size="15">{row["obligation"]}</text>',
        f'<rect x="330" y="{y}" width="{width}" height="25" fill="#2b7287"/>',
        f'<text x="{340 + width}" y="{y + 19}" font-size="15">{row["monomials"]}</text>',
    ]
bars += [
    '<text x="20" y="342" font-size="13">Exact equality and the cleared residual congruence are checked separately.</text>',
    '</svg>',
]
(ROOT / "common22-residual.svg").write_text("\n".join(bars) + "\n")

md = """# Common 2+2 returned-ordinate congruence

Schema: `projective-law-common22-residual/v1`. Lean proves the exact cleared
polynomial identity `C⁵N + S C²X + T = U[-C⁴X + C³(-Cq₁-A) + C²Q]` over any
commutative ring. This is the cleared congruence between the source-shaped
ordinate numerators and cubic numerator modulo the residual quadratic; it
uses no field division. In the unit-weight affine specialization, nonzero
`B,C` allow the usual negative-interpolant interpretation. Normalization of
arbitrary weighted outputs remains unproved here. The [build log](common22-residual-build.log) records the kernel
check and foundational axiom output.

The source checker extracts the common Rust 2+2 branch, verifies its return
tuple, proves that the source `r` has a `C²` factor, and compares the source
`qdelta`, `rdelta`, `s`, and `t` with the theorem's definitions after exact
polynomial substitution. All five obligations match. Mutating the `s` sign
on either side is rejected. The [certificate](common22-residual.json) binds
both source hashes and five polynomial digests; the [summary CSV](common22-residual-summary.csv)
and [graph](common22-residual.svg) retain the exact monomial counts. The
[PDF](COMMON22_RESIDUAL.pdf) and 3.3 MB [term-level CSV](common22-residual-terms.csv)
are stored through the repository artifact manifest in this directory; use
`python3 tools/artifacts/artifacts.py pull research/projective_law_followup_20261022/formal`
from the repository root before checking it.

This proves the common branch's cleared ordinate congruence and its stated
source obligations. It does not yet prove that `U` is the residual support
for a valid smooth curve, that the interpolation function has the required
principal divisor, or that every exceptional runtime branch returns the
Jacobian sum. The full 1,260-target operation-price ratios are unchanged.
"""
(ROOT / "COMMON22_RESIDUAL.md").write_text(md)

tex = r"""\documentclass[11pt]{article}
\usepackage[margin=0.9in]{geometry}
\usepackage{booktabs}
\begin{document}
\section*{Common 2+2 returned-ordinate congruence}
\texttt{projective-law-common22-residual/v1}. Lean proves the cleared
identity $C^5N + SC^2X + T = U[-C^4X + C^3(-Cq_1-A) + C^2Q]$ over a
commutative ring. Source-derived integer polynomials meet five obligations
connecting the Rust return tuple to the theorem's ordinate definitions.
Changing the sign of the $s$ term on either side is rejected.

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
\noindent The term-level CSV retains every expanded integer coefficient. This
cleared polynomial statement does not establish weighted normalization, the
residual support's curve equation, the principal-divisor step, exceptional
branches, or a cost change.
\end{document}
"""
(ROOT / "COMMON22_RESIDUAL.tex").write_text(tex)
print("common22-residual report sources rendered")
