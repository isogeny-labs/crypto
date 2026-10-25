#!/usr/bin/env python3
"""Render the bounded common-2+2 uniqueness proof packet."""

import csv
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
SCHEMA = "projective-law-common22-uniqueness/v1"
ROWS = [
    ("support_coprime", "premise", "open_source_link", "B nonzero implies IsCoprime u1 u2"),
    ("first_residue", "premise", "open_source_link", "u1 divides residual minus scale times curve"),
    ("second_residue", "premise", "open_source_link", "u2 divides residual minus scale times curve"),
    ("degree_bound", "premise", "open_source_link", "difference degree is below degree of u1 times u2"),
    ("crt_uniqueness", "implication", "kernel_checked_conditional", "the four premises imply residual equals scale times curve"),
    ("divisor_class", "conclusion", "open_geometry", "residual curve equation yields the intended Pic0 class"),
]

packet = {
    "schema": SCHEMA,
    "theorems": [
        "Common22Uniqueness.zero_of_coprime_supports",
        "Common22Uniqueness.residual_eq_curve",
    ],
    "axioms": ["propext", "Classical.choice", "Quot.sound"],
    "rows": [
        {"obligation": name, "kind": kind, "status": status, "statement": statement}
        for name, kind, status, statement in ROWS
    ],
    "scope": "conditional polynomial uniqueness over a field; no source-to-premise or geometric proof",
    "cost_ratio_change": None,
}
(ROOT / "common22-uniqueness.json").write_text(json.dumps(packet, indent=2) + "\n")
with (ROOT / "common22-uniqueness.csv").open("w", newline="") as handle:
    writer = csv.DictWriter(handle, fieldnames=("obligation", "kind", "status", "statement"), lineterminator="\n")
    writer.writeheader()
    writer.writerows(packet["rows"])

svg = [
    '<svg xmlns="http://www.w3.org/2000/svg" width="900" height="390" viewBox="0 0 900 390">',
    '<rect width="900" height="390" fill="white"/>',
    '<style>text{font-family:DejaVu Sans,Arial,sans-serif;fill:#243447}.title{font-size:23px;font-weight:bold}.label{font-size:15px}.note{font-size:13px;fill:#475569}</style>',
    '<text x="28" y="38" class="title">Common 2+2 curve-equation proof boundary</text>',
    '<text x="28" y="61" class="note">Green: kernel-checked implication. Orange: premise or geometry still to connect.</text>',
]
for index, (name, _, status, _) in enumerate(ROWS):
    y = 78 + index * 47
    color = "#2f855a" if status == "kernel_checked_conditional" else "#c05621"
    svg.extend([
        f'<rect x="29" y="{y}" width="20" height="20" fill="{color}"/>',
        f'<text x="60" y="{y + 16}" class="label">{name.replace("_", " ")}</text>',
        f'<text x="590" y="{y + 16}" class="note">{status.replace("_", " ")}</text>',
    ])
svg.extend([
    '<text x="28" y="371" class="note">This diagram counts obligations; it does not measure proof effort or cost.</text>',
    '</svg>',
])
(ROOT / "common22-uniqueness.svg").write_text("\n".join(svg) + "\n")

(ROOT / "COMMON22_UNIQUENESS.md").write_text("""# Common 2+2 coprime-support uniqueness

Schema: `projective-law-common22-uniqueness/v1`. The pinned Lean build proves
that a polynomial of degree below `degree (u1*u2)` is zero if it is divisible
by both coprime supports. Applied to `residual - scale*curve`, this gives the
conditional equality `residual = scale*curve`. The [kernel log](common22-uniqueness-build.log)
records the two theorem names and their foundational axioms.

The [CSV](common22-uniqueness.csv) and [JSON](common22-uniqueness.json) list
each premise and conclusion, while the [graph](common22-uniqueness.svg) makes
the current proof boundary visible. The [PDF](COMMON22_UNIQUENESS.pdf) is a
printable copy for independent review.

The premise that `B != 0` makes the supports coprime, both source-derived
residues, and the degree bound have not been mechanically connected to this
theorem. The principal-divisor and `Pic^0` bridge also remain open. No new
DLP experiment ran and the recorded cost ratios are unchanged.
""")

(ROOT / "COMMON22_UNIQUENESS.tex").write_text(r"""\documentclass[11pt]{article}
\usepackage[margin=0.9in]{geometry}
\usepackage{booktabs}
\begin{document}
\section*{Common 2+2 coprime-support uniqueness}
\texttt{projective-law-common22-uniqueness/v1}. A pinned Lean check proves:
if $u_1,u_2$ are coprime, both divide $H$, and
$\deg H<\deg(u_1u_2)$, then $H=0$. Substituting
$H=E-B^2F$ yields $E=B^2F$ under those same explicit premises.

\begin{center}
\begin{tabular}{ll}
\toprule
Obligation & Recorded status\\
\midrule
Support coprimeness from source & Open source link\\
First and second curve residues & Open source links\\
Degree bound from source & Open source link\\
Coprime low-degree implication & Kernel checked, conditional\\
Divisor-class conclusion & Open geometry\\
\bottomrule
\end{tabular}
\end{center}

\noindent The CSV and JSON enumerate the obligations separately. This is a
polynomial uniqueness lemma, not a universal group-law proof. No DLP run or
price changed in this packet.
\end{document}
""")

print("common22-uniqueness packet rendered")
