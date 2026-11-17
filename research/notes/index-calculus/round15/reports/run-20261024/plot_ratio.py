# Lays out recorded values from ../../run1/R15.txt; computes nothing new.
import re, sys, matplotlib
matplotlib.use(sys.argv[3] if len(sys.argv) > 3 else "svg")
import matplotlib.pyplot as plt
rows = []
for line in open(sys.argv[1]):
    m = re.match(r"\s*(\d)\s+(\d+)\s+(\d+)\s+\d+\s+\d+\s+\d+\s+([\d.]+)\s+([\d.]+)\s+[\d.]+\s+[\d.]+%\s+[yN]", line)
    if m:
        rows.append((int(m[3]), float(m[4]), float(m[5])))
rows.sort()
n = [r[0] for r in rows]
fig, ax = plt.subplots(figsize=(7, 4.2))
ax.scatter(n, [r[1] for r in rows], marker="s", facecolors="none", edgecolors="#c05621", label="folded, active window (round 14)", s=26)
ax.scatter(n, [r[2] for r in rows], marker="o", color="#2b6cb0", label="folded, collide-first window", s=26)
ax.axhline(0.7071, color="#555", ls="--", lw=1); ax.text(1.1e3, 0.68, "ideal 1/√2", color="#555", fontsize=8)
ax.axhline(1.0, color="#999", ls=":", lw=1)
ax.set_xscale("log"); ax.set_xlabel("subgroup order N"); ax.set_ylabel("folded / plain walk ops (mean of 40 seeds)")
ax.set_title("Round fifteen, CARD-HEIC-ROUND15-1: collide-first cycle window", fontsize=10)
ax.legend(fontsize=8, loc="upper right"); fig.tight_layout(); fig.savefig(sys.argv[2])
