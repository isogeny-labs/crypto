# Lays out recorded values from ../../run1/R12.txt; computes nothing new.
import re, sys, matplotlib
matplotlib.use(sys.argv[3] if len(sys.argv) > 3 else "svg")
import matplotlib.pyplot as plt
rows = []
for line in open(sys.argv[1]):
    m = re.match(r"\s*(\d)\s+(\d+)\s+(\d+)\s+([\d.]+)\s+([\d.]+)\s+([\d.]+)\s+([\d.]+)\s+\d+\s+[yN]\s*$", line)
    if m:
        rows.append((int(m[3]), float(m[4]), float(m[5]), float(m[6]), float(m[7])))
rows.sort()
n = [r[0] for r in rows]
fig, ax = plt.subplots(figsize=(7, 4.2))
ax.scatter(n, [r[1] for r in rows], marker="o", color="#c05621", label="Jacobian, 8-bit coefficients", s=24)
ax.scatter(n, [r[3] for r in rows], marker="s", facecolors="none", edgecolors="#c05621", label="integer model, 8-bit", s=24)
ax.scatter(n, [r[2] for r in rows], marker="o", color="#2b6cb0", label="Jacobian, full-width coefficients", s=24)
ax.scatter(n, [r[4] for r in rows], marker="s", facecolors="none", edgecolors="#2b6cb0", label="integer model, full-width", s=24)
ax.axhline(1.2533, color="#555", ls="--", lw=1)
ax.text(2.5e5, 1.17, "random-map ideal √(π/2) = 1.2533", color="#555", fontsize=8)
ax.set_xscale("log"); ax.set_xlabel("subgroup order N"); ax.set_ylabel("S_walk = walk ops / √N (mean of 40 seeds)")
ax.set_title("Round twelve, CARD-HEIC-ROUND12-1: DP rho, 8-bit vs full-width branch coefficients", fontsize=9.5)
ax.legend(fontsize=7, loc="upper left"); fig.tight_layout(); fig.savefig(sys.argv[2])
