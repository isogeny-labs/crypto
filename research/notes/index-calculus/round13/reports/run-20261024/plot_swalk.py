# Lays out recorded values from ../../run1/R13.txt; computes nothing new.
import re, sys, matplotlib
matplotlib.use(sys.argv[3] if len(sys.argv) > 3 else "svg")
import matplotlib.pyplot as plt
rows = []
for line in open(sys.argv[1]):
    m = re.match(r"\s*(\d)\s+(\d+)\s+(\d+)\s+\d+\s+\d+\s+[\d.]+\s+[\d.]+%\s+([\d.]+)\s+([\d.]+)", line)
    if m:
        rows.append((int(m[3]), float(m[4]), float(m[5])))
rows.sort()
n = [r[0] for r in rows]
fig, ax = plt.subplots(figsize=(7, 4.2))
ax.scatter(n, [r[1] for r in rows], marker="o", color="#2b6cb0", label="plain full-width walk", s=24)
ax.scatter(n, [r[2] for r in rows], marker="o", facecolors="none", edgecolors="#c05621", label="±-folded full-width walk", s=24)
ax.axhline(1.2533, color="#2b6cb0", ls="--", lw=1); ax.axhline(0.8862, color="#c05621", ls="--", lw=1)
ax.text(1.1e3, 1.33, "plain ideal √(π/2)", color="#2b6cb0", fontsize=8)
ax.text(1.1e3, 0.70, "folded ideal √(π/4)", color="#c05621", fontsize=8)
ax.set_xscale("log"); ax.set_yscale("log"); ax.set_xlabel("subgroup order N"); ax.set_ylabel("S_walk = walk ops / √N (mean of 40 seeds)")
ax.set_title("Round thirteen, CARD-HEIC-ROUND13-1: negation fold on the full-width walk", fontsize=10)
ax.legend(fontsize=8, loc="upper left"); fig.tight_layout(); fig.savefig(sys.argv[2])
