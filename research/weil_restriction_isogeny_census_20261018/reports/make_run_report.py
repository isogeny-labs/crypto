"""Run report for a results/run-* directory (layout of recorded values only).

Usage: python make_run_report.py <run-dir> <out.typ>
Reads env.txt, tool_log.txt, checks_g.txt, checks_h.txt, summary.csv,
kernels.csv (n = 3 rows) and by_class.csv; embeds the figures from
../figures when they exist.
"""

import csv
import sys
from pathlib import Path

run = Path(sys.argv[1])
out = Path(sys.argv[2])
figdir = run.parent.parent / "figures"


def text(name):
    p = run / name
    return p.read_text() if p.exists() else "(missing)"


def rows(name):
    p = run / name
    if not p.exists():
        return []
    with open(p, newline="") as f:
        return list(csv.DictReader(f))


def esc(s):
    return s.replace("\\", "\\\\").replace("#", "\\#").replace("@", "\\@").replace("$", "\\$").replace("_", "\\_").replace("*", "\\*")


parts = []
parts.append(
    """#set page(paper: "a4", margin: (x: 1.5cm, y: 1.4cm), numbering: "1 / 1")
#set text(size: 8.6pt)
#set par(justify: true, leading: 0.5em)
#show table: set text(size: 7pt)
#show raw: set text(size: 7.2pt)
"""
)
parts.append(f"= Run report: {esc(run.name)}\n\n")
parts.append("== Environment and command\n\n```\n" + text("env.txt").strip() + "\n```\n\n")
parts.append("== Tool log\n\n```\n" + text("tool_log.txt").strip() + "\n```\n\n")
parts.append("== Checks\n\n```\n" + text("checks_g.txt").strip() + "\n" + text("checks_h.txt").strip() + "\n```\n\n")

summ = rows("summary.csv")
if summ:
    parts.append("== Part H summary (summary.csv)\n\n")
    hdr = ["q", "classes", "prime_order", "cofactor_2", "full_2_torsion", "ell", "with_tian", "prime_with_tian", "cof2_with_tian", "prime_any_odd_ell", "all_any_odd_ell"]
    parts.append("#table(columns: %d, align: right,\n" % len(hdr))
    parts.append("  " + ", ".join(f"[*{esc(h)}*]" for h in hdr) + ",\n")
    for r in summ:
        parts.append("  " + ", ".join(f"[{esc(r[h])}]" for h in hdr) + ",\n")
    parts.append(")\n\n")

kern = [r for r in rows("kernels.csv") if r["n"] == "3"]
if kern:
    parts.append("== Part G, n = 3 (kernels.csv)\n\n")
    hdr = ["ell", "qbar", "class", "tr", "det", "eigenvalue_one", "submodules_dim_n", "kernels", "tian_kernels", "types_d01"]
    parts.append("#table(columns: %d, align: right,\n" % len(hdr))
    parts.append("  " + ", ".join(f"[*{esc(h)}*]" for h in hdr) + ",\n")
    for r in kern:
        parts.append("  " + ", ".join(f"[{esc(r[h])}]" for h in hdr) + ",\n")
    parts.append(")\n\n")

byc = [r for r in rows("by_class.csv") if r["tian"] == "1" and r["ell"] != "2"]
if byc:
    parts.append("== Part H: classes with a Tian kernel at odd ℓ (by_class.csv)\n\n")
    hdr = ["q", "ell", "qbar", "class", "tr", "det", "curves", "prime_order_curves", "cofactor_2_curves"]
    parts.append("#table(columns: %d, align: right,\n" % len(hdr))
    parts.append("  " + ", ".join(f"[*{esc(h)}*]" for h in hdr) + ",\n")
    for r in byc:
        parts.append("  " + ", ".join(f"[{esc(r[h])}]" for h in hdr) + ",\n")
    parts.append(")\n\n")

for fig, cap in [("affected", "Curves with a Tian kernel by q and ℓ"), ("kernel_map", "n = 3: Tian kernels by class of φ"), ("n2n4", "Kernel census by (n, ℓ)")]:
    if (figdir / f"{fig}.svg").exists():
        parts.append(f"== Graph: {esc(cap)}\n\n#image(\"/figures/{fig}.svg\", width: 100%)\n\n")

out.write_text("".join(parts))
print("wrote", out)
