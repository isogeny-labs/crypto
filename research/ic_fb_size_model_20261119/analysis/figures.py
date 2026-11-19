"""Measured S(w=0) against factor-base size, with the fitted model and rho."""
import json, math
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
D = Path(__file__).resolve().parent.parent
fit = json.load(open(D / "analysis" / "fit.json"))
fig, ax = plt.subplots(figsize=(7.2, 4.4))
colors = {"p24": "#2a6fdb", "p28": "#d9622b"}
for c in ["p24", "p28"]:
    o = fit[c]; N = o["N"]; a, b = o["a"], o["b"]
    Fs = [t["F"] for t in o["rows"]]; S = [t["mean_S"] for t in o["rows"]]
    xs = [2 ** (k / 20) for k in range(int(20 * math.log2(Fs[0] / 1.5)), int(20 * math.log2(Fs[-1] * 1.5)))]
    ax.plot(xs, [(a * x * x + b * N / (4 * x)) / math.sqrt(N) for x in xs], color=colors[c], lw=1, alpha=0.6)
    ax.plot(Fs, S, "o", color=colors[c], label=f"{c}: measured IC (fit a={a:.2f}, b={b:.2f})")
    ax.axhline(o["rho_S_w0"], color=colors[c], ls="--", lw=1)
    ax.axvline(o["Fstar_model"], color=colors[c], ls=":", lw=1)
ax.text(40, 1.1, "rho.negation (dashed)", fontsize=8)
ax.set_xscale("log", base=2); ax.set_yscale("log")
ax.set_xlabel("factor-base size |F| (prime-abscissa)"); ax.set_ylabel("S = GAE / sqrt(r), w = 0")
ax.set_title("IC cost = |F|^2 (pair table) + N/(4|F|) (walk trials); dotted: N^(1/3)/2")
ax.legend(fontsize=8, loc="upper center"); fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(D / "figures" / f"size_sweep.{ext}", dpi=150)
