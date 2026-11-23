#!/usr/bin/env python3
"""REPORT.md -> REPORT.typ (light converter) -> REPORT.pdf with Typst from the venv:
/tmp/report-venv/bin/python make_report.py"""
import os, re

HERE = os.path.dirname(os.path.abspath(__file__))
md = open(os.path.join(HERE, "REPORT.md")).read().split("\n")


def esc(t):
    out = []
    for i, seg in enumerate(re.split(r"(`[^`]*`)", t)):
        if i % 2 == 1:
            out.append(seg)
            continue
        seg = seg.replace("\\", "\\\\")
        for ch in "*_$#@<>[]~":
            seg = seg.replace(ch, "\\" + ch)
        out.append(seg)
    return "".join(out)


L = ['#set page(paper: "a4", margin: (x: 1.8cm, y: 1.8cm), numbering: "1")', '#set text(size: 9.5pt)', '#set par(justify: false)', '#show raw: set text(size: 8pt)', '#set table(stroke: 0.4pt + gray, inset: 3pt)', ""]
i = 0
placed = False
while i < len(md):
    ln = md[i]
    if ln.startswith("# "):
        L.append("#align(center)[#text(size: 15pt, weight: \"bold\")[%s]]" % esc(ln[2:]))
    elif ln.startswith("## "):
        L.append("== %s" % esc(ln[3:]))
        if ln.startswith("## Predictions") and not placed:
            pass
    elif ln.startswith("|"):
        rows = []
        while i < len(md) and md[i].startswith("|"):
            cells = [c.strip() for c in md[i].strip().strip("|").split("|")]
            if not all(re.fullmatch(r":?-+:?", c) for c in cells):
                rows.append(cells)
            i += 1
        n = len(rows[0])
        L.append("#table(columns: %d, align: left, fill: (x, y) => if y == 0 { luma(235) }," % n)
        for r in rows:
            L.append("  " + " ".join("[%s]," % esc(c) for c in r))
        L.append(")")
        continue
    elif ln.startswith("- "):
        L.append("- %s" % esc(ln[2:]))
    elif re.match(r"^\d+\. ", ln):
        L.append("+ %s" % esc(re.sub(r"^\d+\. ", "", ln)))
    elif ln.strip() == "":
        L.append("")
    else:
        L.append(esc(ln) + (" \\" if i + 1 < len(md) and md[i + 1].strip() and not md[i + 1].startswith(("|", "-", "#")) and not re.match(r"^\d+\. ", md[i + 1]) else ""))
    if ln.startswith("## Results"):
        L.append("")
    i += 1
# graph after the Results section heading block: append before limitations
txt = "\n".join(L)
txt = txt.replace("== Predictions against outcomes", '#figure(image("graph.svg", width: 100%), caption: [S and measured rho on J_H against p, three prices (GAE_H, single target); log-log. \\[pcc:unverified\\]])\n\n== Predictions against outcomes', 1)
open(os.path.join(HERE, "REPORT.typ"), "w").write(txt + "\n")
import typst
os.chdir(HERE)
typst.compile("REPORT.typ", output="REPORT.pdf")
typst.compile("REPORT.typ", output="/tmp/claude-0/-home-user/ff829140-7de0-585a-9053-fa1c3d46ba86/scratchpad/page{n}.png", format="png", ppi=70)
print("ok")
