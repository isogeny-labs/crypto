"""E32 figures: layout of recorded values only (experiments/36_handover_ladder/summary.json,
written by examples/e32_tables.rs).  The fitted lines are drawn from the recorded fit
parameters (alpha, beta); nothing is computed here beyond evaluating those lines."""
import json
import math
import os
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
SUMMARY = sys.argv[1] if len(sys.argv) > 1 else os.path.join(
    HERE, "../../../experiments/36_handover_ladder/summary.json")
s = json.load(open(SUMMARY))
curves = s["curves"]
fits = s["fits"]
W = [("0", "0 GAE", "#1f77b4"), ("0.1", "0.1 GAE", "#ff7f0e"), ("1", "1 GAE", "#2ca02c")]
plt.rcParams.update({"font.size": 9})


def num(v):
    return math.inf if v == "inf" else v


def save(fig, name):
    fig.tight_layout()
    fig.savefig(os.path.join(HERE, name + ".svg"))
    fig.savefig(os.path.join(HERE, name + ".png"), dpi=150)
    plt.close(fig)


# 1. distance against size, with the M1 and M2 lines run out to D = 1
fig, ax = plt.subplots(figsize=(7.2, 4.6))
for g, label, col in W:
    xs = [c["log2n"] for c in curves]
    ys = [math.log2(c["D"][g]) for c in curves]
    ax.scatter(xs, ys, s=12, color=col, label=f"curves, {label}", zorder=3)
    for m, ls in (("M1", "-"), ("M2", "--")):
        f = fits[g][m]
        if not f:
            continue
        a, b = f["alpha"], f["beta"]
        ns = num(f["log2_nstar"])
        x1 = min(ns, 140) if math.isfinite(ns) else 140
        X = [22, x1]
        ax.plot(X, [a + b * (x - 32) for x in X], ls, color=col, lw=1,
                label=f"{m} {label}: log2 n* = {ns:.1f}" if math.isfinite(ns) else f"{m} {label}: no crossover")
ax.axhline(0, color="k", lw=0.8)
ax.axvspan(min(c["log2n"] for c in curves), s["reached_log2n"], color="0.9", zorder=0, label="measured range")
ax.set_xlabel("log2 n")
ax.set_ylabel("log2 (S_IC / S_rho)")
ax.set_title("E32: distance to the matched <±psi> rho (walk (iii)), arm B, solver A, Karatsuba")
ax.legend(fontsize=6.5, ncol=2, loc="upper right")
save(fig, "distance")

# 2. crossover order with 95 % bootstrap intervals
fig, ax = plt.subplots(figsize=(7.2, 3.4))
yl, k = [], 0
for g, label, col in W:
    for m in ("M1", "M1_all", "M2"):
        f = fits[g][m]
        if not f:
            continue
        ns = num(f["log2_nstar"])
        lo, hi = (num(v) for v in f["ci95"]["log2_nstar"])
        hi_d = min(hi, 200)
        ax.plot([lo, hi_d], [k, k], color=col, lw=2)
        if math.isfinite(ns):
            ax.plot([ns], [k], "o", color=col)
        if not math.isfinite(hi):
            ax.annotate("→ ∞", (hi_d, k), fontsize=7, va="center")
        yl.append(f"{m}, {label}")
        k += 1
ax.axvline(s["reached_log2n"], color="k", ls=":", lw=1, label=f"largest measured n (2^{s['reached_log2n']:.1f})")
ax.set_yticks(range(len(yl)))
ax.set_yticklabels(yl, fontsize=7)
ax.set_xlabel("crossover order log2 n*  (point and 95 % stratified cluster bootstrap, 10,000 replicates)")
ax.legend(fontsize=7, loc="upper left")
save(fig, "nstar")

# 3. components: S / p, rho per sqrt(n), yield
fig, axs = plt.subplots(1, 3, figsize=(7.4, 2.8))
xs = [c["log2n"] for c in curves]
axs[0].scatter(xs, [c["S"] / c["p"] for c in curves], s=10)
axs[0].set_yscale("log")
axs[0].set_title("S / p  (F_p mults)")
axs[1].scatter(xs, [c["rho"] / math.sqrt(c["n"]) for c in curves], s=10, color="#d62728")
axs[1].set_title("rho / sqrt(n)")
axs[2].scatter(xs, [c["y"] for c in curves], s=10, color="#9467bd")
axs[2].set_yscale("log")
axs[2].set_title("targets per relation y")
for a in axs:
    a.set_xlabel("log2 n")
save(fig, "components")
print("figures written")
