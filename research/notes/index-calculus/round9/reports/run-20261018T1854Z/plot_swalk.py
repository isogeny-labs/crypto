# Lays out recorded values from ../../run5/R9.txt; computes nothing new.
import re, sys, matplotlib
matplotlib.use(sys.argv[3] if len(sys.argv) > 3 else "svg")
import matplotlib.pyplot as plt
rows = []
for line in open(sys.argv[1]):
    m = re.match(r"\s*(\d)\s+(\d+)\s+(\d+)\s+\d+\s+\d+\s+[\d.]+\s+[\d.]+%\s+([\d.]+)\s+([\d.]+)", line)
    if m:
        rows.append((int(m[1]), int(m[3]), float(m[4]), float(m[5])))
fig, ax = plt.subplots(figsize=(7, 4.2))
for g, mk in ((2, "o"), (3, "s"), (4, "^")):
    r = [x for x in rows if x[0] == g]
    ax.scatter([x[1] for x in r], [x[2] for x in r], marker=mk, color="#2b6cb0", label=f"plain, genus {g}")
    ax.scatter([x[1] for x in r], [x[3] for x in r], marker=mk, facecolors="none", edgecolors="#c05621", label=f"folded, genus {g}")
ax.axhline(1.2533, color="#2b6cb0", ls="--", lw=1)
ax.axhline(0.8862, color="#c05621", ls="--", lw=1)
ax.text(1.1e3, 1.28, "plain ideal √(π/2)", color="#2b6cb0", fontsize=8)
ax.text(1.1e3, 0.91, "folded ideal √(π/4)", color="#c05621", fontsize=8)
ax.set_xscale("log"); ax.set_xlabel("subgroup order N"); ax.set_ylabel("S_walk = walk group ops / √N (mean of 40 seeds)")
ax.set_title("Round nine, CARD-HEIC-ROUND9-5: DP rho walk cost, plain vs ±-folded", fontsize=10)
ax.legend(fontsize=7, ncol=2); fig.tight_layout(); fig.savefig(sys.argv[2])
