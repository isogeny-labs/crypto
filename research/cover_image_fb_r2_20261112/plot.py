#!/usr/bin/env python3
"""cells.tsv -> fig_width.svg/png: W vs |F| per map, and W / W_null per map (m = 2, Kummer)."""
import csv, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
D = os.path.dirname(os.path.abspath(__file__))
rows = [r for r in csv.DictReader(open(os.path.join(D, "cells.tsv")), delimiter="\t") if r["m"] == "2"]
W = {(r["arm"], r["control"], int(r["t"])): (int(r["F"]), int(r["W"])) for r in rows}
arms = [("none", "PKM tower (no cover)", "#1f5fa8", "-"),
        ("quad", "deg 2, genus 2 (quad)", "#d97706", "-"),
        ("cubic", "deg 3, genus 4 (cubic)", "#c2410c", "-"),
        ("generic3", "deg 3 rational, genus 5", "#7c2d12", "-"),
        ("quart", "deg 4, genus 5 (quart)", "#6b7280", "-"),
        ("fourth", "u⁴ (positive control)", "#15803d", "--")]
fig, (a1, a2) = plt.subplots(1, 2, figsize=(11, 4.2))
for arm, lab, c, ls in arms:
    pts = sorted(W[k] for k in W if k[0] == arm and k[1] == "tower")
    a1.plot([p[0] for p in pts], [p[1] for p in pts], ls, color=c, marker="o", ms=3, label=lab)
    if arm in ("none", "fourth"):
        continue
    ts = sorted(k[2] for k in W if k[0] == arm and k[1] == "tower" and (arm, "null", k[2]) in W)
    a2.plot(ts, [W[(arm, "tower", t)][1] / W[(arm, "null", t)][1] for t in ts], ls, color=c,
            marker="o", ms=3, label=lab)
a2.axhspan(0.8, 1.25, color="#9ca3af", alpha=0.2, label="H0 band [0.8, 1.25]")
a2.axhline(1, color="#374151", lw=0.8)
a1.set_xscale("log", base=2); a1.set_yscale("log")
a1.set_xlabel("factor-base size |F|"); a1.set_ylabel("widest F4 matrix W")
a1.set_title("m = 2: W at equal |F|")
a2.set_xlabel("tower depth t"); a2.set_ylabel("W(cover) / W(its null)")
a2.set_title("cover system vs random coefficients, same support")
a2.set_ylim(0.6, 1.5)
for a in (a1, a2):
    a.grid(alpha=0.3); a.legend(fontsize=7)
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(os.path.join(D, f"fig_width.{ext}"), dpi=150)
