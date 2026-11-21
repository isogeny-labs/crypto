"""E8/E9a run report figures from ../../tables.json (examples/isogeny_gap_round6_tables.rs).
Lays out recorded/derived values only."""
import json, os
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt
here = os.path.dirname(os.path.abspath(__file__))
t = json.load(open(os.path.join(here, "../../tables.json")))

fig, ax = plt.subplots(1, 2, figsize=(10, 4.2))
# Left: P-E8d, saving against solvable share.
for r in t["e8"]:
    coset = r["config"].endswith(":coset")
    ax[0].scatter(100 * r["solvable_share"], 100 * r["saving"], marker="s" if coset else "o",
                  color="#3b6ea8" if coset else "#c0632b", s=30)
ax[0].plot([0, 100], [0, 100], color="black", lw=0.8, ls=":")
ax[0].axhline(0, color="grey", lw=0.6)
ax[0].scatter([], [], marker="o", color="#c0632b", label="default base")
ax[0].scatter([], [], marker="s", color="#3b6ea8", label="trace-coset base")
ax[0].set_xlabel("share of full-F4 work in solvable systems (%)")
ax[0].set_ylabel("saving of f4-f2-early, 1 − Σearly/Σfull (%)")
ax[0].set_title("E8: early stop saves at most the solvable share", fontsize=9)
ax[0].legend(fontsize=7, loc="upper left"); ax[0].grid(alpha=0.3)
# Right: P-E9a, yield ratio against 1/f2.
for r in t["e9a"]:
    y = r["yield_ratio"]
    if y[0] is None or r["yield_default"] < 30:
        continue
    inv = 1 / r["f2_default"]
    ax[1].errorbar(inv, y[0], yerr=[[y[0] - y[1]], [y[2] - y[0]]], fmt="o" if r["a"] == 0 else "s",
                   color="#3b6ea8" if r["a"] == 1 else "#c0632b", capsize=3, ms=5)
    ax[1].annotate(f"n{r['n']} d{r['d']}", (inv, y[0]), fontsize=6, xytext=(4, 2), textcoords="offset points")
ax[1].plot([1.7, 2.4], [1.7, 2.4], color="black", lw=0.8, ls=":", label="ratio = 1/f₂")
ax[1].errorbar([], [], fmt="o", color="#c0632b", label="a = 0")
ax[1].errorbar([], [], fmt="s", color="#3b6ea8", label="a = 1 (affine coset)")
ax[1].set_xlabel("1 / f₂ (default base's fraction in 2E)")
ax[1].set_ylabel("yield, coset ÷ default (95 % Poisson)")
ax[1].set_title("E9a: the coset's yield gain tracks 1/f₂ (rows with ≥ 30 default hits)", fontsize=9)
ax[1].legend(fontsize=7); ax[1].grid(alpha=0.3)
fig.tight_layout(); fig.savefig(os.path.join(here, "e8_e9a.svg")); print("ok")
