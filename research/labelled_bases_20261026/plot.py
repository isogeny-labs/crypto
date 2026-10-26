"""Evaluates the closed-form lower bound of README.md (derived, not measured) and lays it out."""
import math
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

logN = 60.0
N = 2.0 ** logN
fig, ax = plt.subplots(figsize=(8, 4.3))
for m in (2, 3, 4):
    for logs, ls in ((40, ":"), (50, "--"), (60, "-")):
        s = 2.0 ** logs
        xs, ys = [], []
        for k in range(0, int(logs * 10) + 1):
            B = 2.0 ** (k / 10)
            c = B * N / s + (math.factorial(m) * N / B ** m) * max(1.0, s ** m / (math.factorial(m) * N))
            xs.append(k / 10); ys.append(math.log2(c / math.sqrt(N)))
        ax.plot(xs, ys, ls=ls, label=f"m={m}, s=2^{logs}")
ax.axhline(1.0, color="#b33", lw=1); ax.text(1, 1.3, "2√N (the bound)", color="#b33", fontsize=8)
ax.axhline(math.log2(0.886), color="#555", lw=1, ls="-."); ax.text(1, -1.9, "rho 0.886√N", color="#555", fontsize=8)
ax.set_xlabel("log2 B (labelled base size)"); ax.set_ylabel("log2 (cost / √N)")
ax.set_ylim(-3, 30); ax.legend(fontsize=7, ncol=3)
ax.set_title("Labelled-base cost model, N = 2^60 (derived from README.md, no measurement)", fontsize=9)
fig.tight_layout(); fig.savefig("bound.svg"); fig.savefig("bound.png", dpi=150)
