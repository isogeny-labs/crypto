# Lays out recorded values from ../../run1/R14.txt; computes nothing new.
import re, sys, matplotlib
matplotlib.use(sys.argv[3] if len(sys.argv) > 3 else "svg")
import matplotlib.pyplot as plt
rows = []
for line in open(sys.argv[1]):
    m = re.match(r"\s*(\d)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+([\d.]+)%\s+(\d+)\s+[\d.]+\s+[\d.]+\s+[\d.]+\s+[\d.]+%\s+[yN]", line)
    if m:
        rows.append((int(m[3]), int(m[4]), int(m[5]), float(m[6]), int(m[7])))
rows.sort()
n = [r[0] for r in rows]
fig, (ax, bx) = plt.subplots(1, 2, figsize=(10, 4.2))
ax.scatter(n, [r[2] / r[1] for r in rows], marker="o", facecolors="none", edgecolors="#c05621", label="folded, no early escape (round 13)", s=24)
ax.scatter(n, [r[4] / r[1] for r in rows], marker="s", color="#2b6cb0", label="folded, active cycle window", s=24)
ax.axhline(0.7071, color="#555", ls="--", lw=1); ax.text(1.1e3, 0.62, "ideal 1/√2", color="#555", fontsize=8)
ax.axhline(1.0, color="#999", ls=":", lw=1)
ax.set_xscale("log"); ax.set_yscale("log"); ax.set_xlabel("subgroup order N"); ax.set_ylabel("folded / plain walk ops")
ax.legend(fontsize=7, loc="upper left"); ax.set_title("Fold vs plain full-width walk", fontsize=9)
bx.scatter(n, [r[3] for r in rows], color="#c05621", s=24)
bx.axhline(50, color="#555", ls="--", lw=1); bx.text(1.1e3, 52, "registered C14-1 bound (two largest rows)", color="#555", fontsize=7)
bx.set_xscale("log"); bx.set_xlabel("subgroup order N"); bx.set_ylabel("% of folded walk steps inside a short cycle")
bx.set_title("Passive window: trapped steps", fontsize=9)
fig.suptitle("Round fourteen, CARD-HEIC-ROUND14-1", fontsize=10); fig.tight_layout(); fig.savefig(sys.argv[2])
