#!/usr/bin/env python3
"""cells.tsv -> fig_cols.svg/png: largest F4 matrix width vs |F| (m = 2, Kummer), and D vs t."""
import csv, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
D = os.path.dirname(os.path.abspath(__file__))
rows = list(csv.DictReader(open(os.path.join(D, "cells.tsv")), delimiter="\t"))
sel = lambda arm, ctl, m="2", kind="kummer": sorted(
    (int(r["F"]), int(r["t"]), int(r["max_cols"]), int(r["D_planted"]))
    for r in rows if r["m"] == m and r["kind"] == kind and r["arm"] == arm and r["control"] == ctl)
fig, (a1, a2) = plt.subplots(1, 2, figsize=(11, 4.2))
style = {("none", "tower"): ("PKM tower (no cover)", "#1f5fa8", "-"),
         ("quad", "tower"): ("cover image, genus 2 (quad)", "#c2410c", "-"),
         ("quad", "null"): ("quad, random-coefficient null", "#c2410c", ":"),
         ("generic", "tower"): ("cover image, genus 3 (generic)", "#6b7280", "-"),
         ("square", "tower"): ("square map (positive control)", "#15803d", "--")}
for (arm, ctl), (lab, c, ls) in style.items():
    pts = sel(arm, ctl)
    a1.plot([p[0] for p in pts], [p[2] for p in pts], ls, color=c, marker="o", ms=3, label=lab)
    a2.plot([p[1] for p in pts], [p[3] for p in pts], ls, color=c, marker="o", ms=3, label=lab)
a1.set_xscale("log", base=2); a1.set_yscale("log")
a1.set_xlabel("factor-base size |F|"); a1.set_ylabel("largest F4 matrix width (max_cols)")
a1.set_title("m = 2, Kummer: matrix width at equal |F|")
a2.set_xlabel("tower depth t"); a2.set_ylabel("solving_degree_max D")
a2.set_title("D is floored at the cover system's input degree 8")
for a in (a1, a2):
    a.grid(alpha=0.3)
a1.legend(fontsize=8)
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(os.path.join(D, f"fig_cols.{ext}"), dpi=150)
