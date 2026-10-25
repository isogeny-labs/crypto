#!/usr/bin/env python3
"""Render the bounded common-2+2 uniqueness proof packet."""

import csv
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
SCHEMA = "projective-law-common22-uniqueness/v1"
ROWS = [
    ("support_coprime", "premise", "kernel_checked_conditional", "B nonzero implies IsCoprime u1 u2"),
    ("runtime_interpolation", "source_link", "open_source_link", "runtime execution instantiates proof-side numerator and interpolation"),
    ("first_residue", "premise", "kernel_checked_conditional", "interpolation and valid input imply first curve residue"),
    ("second_residue", "premise", "kernel_checked_conditional", "interpolation and valid input imply second curve residue"),
    ("generic_head", "implication", "kernel_checked_conditional", "generic cubic and quadratic residual has recorded coefficients at degrees six five four"),
    ("source_head", "implication", "kernel_checked_conditional", "proof-side source-style residual has head coefficients zero B squared zero"),
    ("degree_bound", "implication", "kernel_checked_conditional", "matching quintic heads imply difference degree below support product"),
    ("crt_uniqueness", "implication", "kernel_checked_conditional", "the four premises imply residual equals scale times curve"),
    ("divisor_class", "conclusion", "open_geometry", "residual curve equation yields the intended Pic0 class"),
]

packet = {
    "schema": SCHEMA,
    "theorems": [
        "Common22Coprime.determinant_bezout",
        "Common22Coprime.supports_coprime",
        "Common22Coprime.firstSupport_natDegree",
        "Common22Coprime.secondSupport_natDegree",
        "Common22Coprime.firstSupport_nonzero",
        "Common22Coprime.secondSupport_nonzero",
        "Common22Residues.first_residue",
        "Common22Residues.second_residue",
        "Common22Residues.residual_curve_eq",
        "Common22Degree.cleared_difference_degree_lt_four",
        "Common22Degree.cleared_difference_degree_lt_supports",
        "Common22Degree.natDegree_le_five_of_sextic_head_zero",
        "Common22Degree.cleared_difference_degree_lt_supports_of_sextic_head",
        "Common22PolynomialHead.generic_expansion",
        "Common22PolynomialHead.generic_coeff_six",
        "Common22PolynomialHead.generic_coeff_five",
        "Common22PolynomialHead.generic_coeff_four",
        "Common22PolynomialHead.generic_natDegree_le_six",
        "Common22SourceHead.sourceE_coeff_six",
        "Common22SourceHead.sourceE_coeff_five",
        "Common22SourceHead.sourceE_coeff_four",
        "Common22SourceHead.canonical_n_eq_interpolant",
        "Common22SourceHead.sourceE_natDegree_le_six",
        "Common22SourceHead.sourceE_strict_degree",
        "Common22Uniqueness.zero_of_coprime_supports",
        "Common22Uniqueness.residual_eq_curve",
    ],
    "axioms": ["propext", "Classical.choice", "Quot.sound"],
    "rows": [
        {"obligation": name, "kind": kind, "status": status, "statement": statement}
        for name, kind, status, statement in ROWS
    ],
    "scope": "conditional coprimeness, residues, proof-side source-style polynomial head and strict degree, and curve equation over a field; runtime semantic correspondence and geometry remain open",
    "cost_ratio_change": None,
}
(ROOT / "common22-uniqueness.json").write_text(json.dumps(packet, indent=2) + "\n")
with (ROOT / "common22-uniqueness.csv").open("w", newline="") as handle:
    writer = csv.DictWriter(handle, fieldnames=("obligation", "kind", "status", "statement"), lineterminator="\n")
    writer.writeheader()
    writer.writerows(packet["rows"])

svg = [
    '<svg xmlns="http://www.w3.org/2000/svg" width="900" height="550" viewBox="0 0 900 550">',
    '<rect width="900" height="550" fill="white"/>',
    '<style>text{font-family:DejaVu Sans,Arial,sans-serif;fill:#243447}.title{font-size:23px;font-weight:bold}.label{font-size:15px}.note{font-size:13px;fill:#475569}</style>',
    '<text x="28" y="38" class="title">Common 2+2 curve-equation proof boundary</text>',
    '<text x="28" y="61" class="note">Green: kernel-checked conditional step. Orange: source or geometry still to connect.</text>',
]
for index, (name, _, status, _) in enumerate(ROWS):
    y = 78 + index * 48
    color = "#2f855a" if status == "kernel_checked_conditional" else "#c05621"
    svg.extend([
        f'<rect x="29" y="{y}" width="20" height="20" fill="{color}"/>',
        f'<text x="60" y="{y + 16}" class="label">{name.replace("_", " ")}</text>',
        f'<text x="590" y="{y + 16}" class="note">{status.replace("_", " ")}</text>',
    ])
svg.extend([
    '<text x="28" y="532" class="note">This diagram counts obligations; it does not measure proof effort or cost.</text>',
    '</svg>',
])
(ROOT / "common22-uniqueness.svg").write_text("\n".join(svg) + "\n")

(ROOT / "COMMON22_UNIQUENESS.md").write_text("""# Common 2+2 coprime-support uniqueness

Schema: `projective-law-common22-uniqueness/v1`. The pinned Lean build proves
an explicit Bézout identity for the two source-style quadratics, proves their
degree-two and nonzero properties, then proves that nonzero `B` makes them
coprime. Given interpolation and valid-input curve
congruences, it proves both residual residues. It also proves that a polynomial
below `degree (u1*u2)` is zero if divisible by both coprime supports. Applied
to the cleared residual difference, this gives the conditional curve equation.
A separate Lean lemma proves the strict degree bound from matching head
coefficients and quadratic-support hypotheses. It covers a degree-at-most-six
residual whose sixth coefficient vanishes.
Four additional Lean theorems expand a generic cubic numerator against two
quadratic supports and a quadratic residual support, then read its coefficients
at degrees six, five, and four. Four more substitute the proof-side
source-style coefficients and show `0`, `B²`, `0`; they also equate the
canonical numerator with its interpolation expression. The semantic link
from Rust execution to that proof-side polynomial remains open.
The source-style residual is also proved at most sextic, and its cleared
difference from a monic quintic with zero fourth coefficient has degree
strictly below the two-support product.
The [coprime](common22-coprime-build.log), [residue](common22-residues-build.log),
[degree](common22-degree-build.log),
[polynomial-head](common22-polynomial-head-build.log),
[source-head](common22-source-head-build.log), and
[uniqueness](common22-uniqueness-build.log) kernel logs record all twenty-six
theorem names and their foundational axioms.

The [CSV](common22-uniqueness.csv) and [JSON](common22-uniqueness.json) list
each premise and conclusion, while the [graph](common22-uniqueness.svg) makes
the current proof boundary visible. The [PDF](COMMON22_UNIQUENESS.pdf) is a
printable copy for independent review. It is rendered from
`COMMON22_UNIQUENESS.tex` with `pdflatex` and optimized with Ghostscript
`pdfwrite`; the checksum manifest binds the final bytes.

The proof-side head is exact, but Rust-to-Lean semantic correspondence has not
been mechanically established for all valid inputs. The
principal-divisor and `Pic^0` bridge remain open. No new DLP experiment ran;
the recorded cost ratios are unchanged.
""")

(ROOT / "COMMON22_UNIQUENESS.tex").write_text(r"""\documentclass[11pt]{article}
\usepackage[margin=0.9in]{geometry}
\usepackage{booktabs}
\begin{document}
\section*{Common 2+2 coprime-support uniqueness}
\texttt{projective-law-common22-uniqueness/v1}. A pinned Lean check first proves
an explicit Bezout identity: the common-branch determinant $B$ is a linear
combination of the two source-style supports. Lean also proves those supports
have degree two and are nonzero. Thus $B\ne0$ makes them coprime.
Given interpolation and valid-input curve congruences, Lean proves that each
support divides $H=E-B^2F$. If $\deg H<\deg(u_1u_2)$, then $H=0$ and
$E=B^2F$. A separate theorem derives this degree bound when $E$ has degree
at most six with zero sixth coefficient, $F$ has degree at most five, their
scaled coefficients match in degrees five and four, and $u_1,u_2$ are nonzero
quadratics. These are conditional polynomial statements.
Four more theorems expand a generic cubic-minus-quadratic product and read its
coefficients in degrees six, five, and four. Four source-style theorems show
those coefficients are $0,B^2,0$ and equate the cubic's canonical and
interpolation forms. Two further theorems prove the proof-side residual is
at most sextic and obtain the strict degree bound for a monic quintic with
zero fourth coefficient. Rust execution is not translated into Lean here.

\begin{center}
\begin{tabular}{ll}
\toprule
Obligation & Recorded status\\
\midrule
Support coprimeness from $B\ne0$ & Kernel checked, conditional\\
Runtime interpolation link & Open source link\\
First and second curve residues & Kernel checked, conditional\\
Generic polynomial head & Kernel checked, conditional\\
Proof-side source-style head & Kernel checked, conditional\\
Degree-bound implication & Kernel checked, conditional\\
Coprime low-degree implication & Kernel checked, conditional\\
Divisor-class conclusion & Open geometry\\
\bottomrule
\end{tabular}
\end{center}

\noindent The CSV and JSON enumerate the obligations separately. This is a
polynomial equation bridge, not a universal group-law proof. No DLP run or
price changed in this packet.
\end{document}
""")

print("common22-uniqueness packet rendered")
