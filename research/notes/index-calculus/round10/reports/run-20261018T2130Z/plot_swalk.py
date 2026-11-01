# Lays out recorded values from ../../run1/R10.txt; computes nothing new.
import re, sys, matplotlib
matplotlib.use(sys.argv[3] if len(sys.argv) > 3 else "svg")
import matplotlib.pyplot as plt
rows = []
for line in open(sys.argv[1]):
    m = re.match(r"\s*(\d)\s+(\d+)\s+(\d+)\s+([\d.]+)\s+([\d.]+)\s+([\d.]+)\s+([\d.]+)\s+[\d.]+\s+[\d.]+\s+[yN]\s*$", line)
    if m:
        rows.append((int(m[3]), float(m[4]), float(m[5]), float(m[6]), float(m[7])))
rows.sort()
n = [r[0] for r in rows]
fig, ax = plt.subplots(figsize=(7, 4.2))
for i, (lab, c, mk, fill) in enumerate((("plain", "#2b6cb0", "o", True), ("fold", "#c05621", "o", False),
                                        ("plain+mix", "#2b6cb0", "s", False), ("fold+mix", "#c05621", "s", True))):
    ys = [r[i + 1] for r in rows]
    if fill:
        ax.scatter(n, ys, marker=mk, color=c, label=lab, s=22)
    else:
        ax.scatter(n, ys, marker=mk, facecolors="none", edgecolors=c, label=lab, s=22)
ax.axhline(1.2533, color="#2b6cb0", ls="--", lw=1)
ax.axhline(0.8862, color="#c05621", ls="--", lw=1)
ax.text(1.1e3, 1.28, "plain ideal √(π/2)", color="#2b6cb0", fontsize=8)
ax.text(1.1e3, 0.91, "folded ideal √(π/4)", color="#c05621", fontsize=8)
ax.set_xscale("log"); ax.set_xlabel("subgroup order N"); ax.set_ylabel("S_walk = walk group ops / √N (mean of 40 seeds)")
ax.set_title("Round ten, CARD-HEIC-ROUND10-1: DP rho walk cost, old hash vs SplitMix64-mixed", fontsize=10)
ax.legend(fontsize=7, ncol=2); fig.tight_layout(); fig.savefig(sys.argv[2])
