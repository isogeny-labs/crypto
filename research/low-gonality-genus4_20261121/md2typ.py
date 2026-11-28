#!/usr/bin/env python3
"""Tiny converter from the constrained REPORT.md dialect to Typst, embedding figure.svg. usage: python3 md2typ.py"""
import re, os
HERE = os.path.dirname(os.path.abspath(__file__))
md = open(os.path.join(HERE, "REPORT.md")).read().split("\n")
def esc(t):
    t = t.replace("\\", "\\\\").replace("#", "\\#").replace("$", "\\$").replace("@", "\\@").replace("<", "\\<").replace(">", "\\>")
    t = t.replace("*", "\\*").replace("_", "\\_")
    t = re.sub(r"`([^`]*)`", lambda m: "#raw(\"" + m.group(1).replace("\\\\", "\\").replace("\\_", "_").replace("\\#", "#").replace("\\*", "*").replace("\\$", "$").replace("\\@", "@").replace("\\<", "<").replace("\\>", ">").replace('"', '\\"') + "\")", t)
    return t
out = ['#set page(paper: "a4", margin: 1.6cm)', '#set text(size: 9pt)', '#set par(justify: false)']
i = 0
while i < len(md):
    l = md[i]
    if l.startswith("|"):
        rows = []
        while i < len(md) and md[i].startswith("|"):
            cells = [c.strip() for c in md[i].strip().strip("|").split("|")]
            if not all(re.fullmatch(r":?-+:?", c) for c in cells):
                rows.append(cells)
            i += 1
        n = len(rows[0])
        out.append("#block(breakable: false)[#set text(size: 6.5pt)\n#table(columns: %d, stroke: 0.3pt, inset: 2pt," % n)
        for r in rows:
            r = (r + [""] * n)[:n]
            out.append(" " + ", ".join("[" + esc(c) + "]" for c in r) + ",")
        out.append(")]")
        continue
    m = re.match(r"(#+) (.*)", l)
    if m:
        out.append("%s %s" % ("=" * len(m.group(1)), esc(m.group(2))))
    elif l.startswith("- "):
        out.append("- " + esc(l[2:]))
    elif l.strip() == "[[FIGURE]]":
        out.append('#figure(image("figure.svg", width: 100%), caption: [Figure from RESULTS.json by plot.py; all numbers [pcc:unverified].])')
    elif l.strip() == "":
        out.append("")
    else:
        out.append(esc(l))
    i += 1
open(os.path.join(HERE, "REPORT.typ"), "w").write("\n".join(out) + "\n")
