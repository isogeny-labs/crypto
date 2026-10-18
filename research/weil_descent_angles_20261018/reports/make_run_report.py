"""Lay out one round-3 run's recorded files as its descriptive run report (Typst).

Usage: python make_run_report.py <run-dir> <out.typ> <figure.svg> <caption>
Copies env.txt, tool_log.txt, check/summary text and the head of each CSV
from the run directory. Computes nothing.
"""

import csv
import sys
from pathlib import Path

run, out, fig, cap = Path(sys.argv[1]), Path(sys.argv[2]), sys.argv[3], sys.argv[4]
TOOLS = {
    "item4": "examples/genus4_res_census.rs (Parts T, H4, K)",
    "item1D-mult": "examples/decomposition_coverage.rs, mode mult (Amendment D)",
    "item1D": "examples/decomposition_coverage.rs (Part D)",
    "item2P": "examples/pkm_coverage.rs (Part P)",
    "itemOJ": "examples/n2_routes.rs, mode run (Parts O/J)",
    "itemF": "examples/n2_routes.rs, mode control (Amendment F)",
}
tool = next(v for k, v in TOOLS.items() if run.name.startswith(k))


def esc(s):
    return (s.replace("\\", "\\\\").replace("#", "\\#").replace("_", "\\_").replace("*", "\\*")
            .replace("@", "\\@").replace("<", "\\<").replace("$", "\\$").replace("[", "\\[").replace("]", "\\]"))


def block(name):
    p = run / name
    if not p.exists():
        return ""
    t = p.read_text().strip()
    lines = t.splitlines()
    if len(lines) > 60:
        t = "\n".join(lines[:60] + [f"... ({len(lines) - 60} more lines in {name})"])
    return f"== {esc(name)}\n```\n{t}\n```\n"


def head(name, n=12):
    p = run / name
    with open(p, newline="") as f:
        r = list(csv.reader(f))
    hdr, body = r[0], r[1 : n + 1]
    cols = len(hdr)
    cells = ", ".join(f"[*{esc(h)}*]" for h in hdr)
    rows = ",\n".join(", ".join(f"[{esc(c)}]" for c in row) for row in body)
    return (f"== {esc(name)} (first {len(body)} of {len(r) - 1} rows)\n"
            f"#table(columns: {cols}, align: left, stroke: 0.4pt + rgb(\"#d9d8d4\"),\n{cells},\n{rows}\n)\n")


P = [f"""#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: {esc(run.name)}

Descriptive record of one recorded run of {esc(tool)} under
`research/weil_descent_angles_20261018/PROTOCOL.md` (Addendum A and
Amendments B–G, each committed before the run it governs). It lists what
ran, on what host, and what the tool recorded, file by file. Interpretation
is in the study's `REPORT.md`. Costs are counted operations
(F#sub[p] multiplications and inversions, group operations, native
operations), never wall-clock; the only times are the tool's own seconds.
Isolation: none earned (a shared cloud container, 4 threads); this
affects only the seconds, since every recorded value is an exact count or
a counted operation total.

"""]
P.append(block("env.txt"))
P.append(block("tool_log.txt"))
for name in ["checks.txt", "summary.txt", "control_summary.txt"]:
    P.append(block(name))
for p in sorted(run.glob("*.csv")):
    P.append(head(p.name))
P.append(f"""
== Graph
#figure(image("{fig}", width: 100%), caption: [{esc(cap)}])
""")
out.write_text("".join(P))
print("wrote", out)
