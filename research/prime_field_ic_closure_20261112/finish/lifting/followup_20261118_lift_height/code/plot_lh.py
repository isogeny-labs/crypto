#!/usr/bin/env python3
"""rho_G (max and p90 per cell) against bits, with the Lemma 3H thresholds.  [pcc:unverified]"""
import sys, json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

res = json.load(open(sys.argv[1]))
fig, axes = plt.subplots(1, 3, figsize=(13, 4.2), sharey=True)
THR = {7: 32.6, 8: 10.8, 9: 4.94}
for ax, r in zip(axes, (7, 8, 9)):
    for origin, col in (("O", "#1f6fb4"), ("tangent", "#c2571a")):
        cs = sorted([c for c in res["cells"] if c["r"] == r and c["origin"] == origin and c["arm"] == "X"
                     and c.get("rho_max")], key=lambda c: c["bits"])
        if not cs:
            continue
        b = [c["bits"] for c in cs]
        ax.plot(b, [c["rho_max"] for c in cs], "o-", color=col, label=f"{origin}-origin max")
        ax.plot(b, [c["rho_p90"] for c in cs], "s--", color=col, alpha=.6, label=f"{origin}-origin p90")
        if origin == "tangent" and any(c.get("rho_diff_max") for c in cs):
            ax.plot(b, [c.get("rho_diff_max") for c in cs], "^:", color=col, alpha=.8, label="tangent, difference basis max")
    ax.axhline(THR[r], color="k", lw=1)
    ax.text(8.5, THR[r] * 1.05, f"Lemma 3H threshold {THR[r]} (256 bits)", fontsize=8)
    ax.set_yscale("log")
    ax.set_title(f"r = {r}")
    ax.set_xlabel("toy prime size (bits)")
    ax.grid(alpha=.3)
axes[0].set_ylabel("rho_G = H / lambda_G")
axes[0].legend(fontsize=7, loc="upper right")
fig.suptitle("Height ratio on toy xedni lifts [pcc:unverified]", fontsize=10)
fig.tight_layout()
fig.savefig(sys.argv[2])
