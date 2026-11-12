#!/usr/bin/env python3
"""cells.tsv -> fig_m3.svg/png: W / W_PKM and W / W_null against t at m = 3."""
import csv, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
D = os.path.dirname(os.path.abspath(__file__))
W = {(r["arm"], r["control"], int(r["t"])): int(r["W"])
     for r in csv.DictReader(open(os.path.join(D, "cells.tsv")), delimiter="\t") if r["censored"] == "0"}
arms = [("quad", "deg 2, genus 2", "#d97706"), ("cubic", "deg 3, genus 4", "#c2410c"),
        ("generic3", "deg 3 rational, genus 5", "#7c2d12"), ("quart", "deg 4, genus 5", "#6b7280")]
fig, (a1, a2) = plt.subplots(1, 2, figsize=(11, 4.2), sharey=True)
for arm, lab, c in arms:
    ts = [t for t in (2, 3, 4) if (arm, "tower", t) in W]
    a1.plot(ts, [W[(arm, "tower", t)] / W[("none", "tower", t)] for t in ts], "-o", color=c, ms=3, label=lab)
    a2.plot(ts, [W[(arm, "tower", t)] / W[(arm, "null", t)] for t in ts], "-o", color=c, ms=3, label=lab)
a1.axhline(0.8, color="#15803d", ls="--", lw=1, label="H1 threshold 0.8")
a1.axhline(1.0, color="#374151", lw=0.8)
a2.axhline(1.0, color="#374151", lw=0.8)
a1.set_title("m = 3: W(cover) / W(PKM tower), equal |F|")
a2.set_title("m = 3: W(cover) / W(its null)")
for a in (a1, a2):
    a.set_xlabel("tower depth t  (|F| = 2^t)"); a.set_xticks([2, 3, 4]); a.grid(alpha=0.3); a.legend(fontsize=7)
a1.set_ylabel("width ratio")
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(os.path.join(D, f"fig_m3.{ext}"), dpi=150)
