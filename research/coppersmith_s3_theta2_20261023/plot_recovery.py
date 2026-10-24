#!/usr/bin/env python3
"""Lay out the recorded cells of a run's run.json (no computation beyond
reading the recorded values and the protocol's reference lines).
Usage: /tmp/report-venv/bin/python plot_recovery.py reports/run-.../run.json
Writes recovery.svg and recovery.png beside run.json."""
import json, pathlib, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

src = pathlib.Path(sys.argv[1]).resolve()
d = json.loads(src.read_text())
cells = [c for c in d["cells"] if c["arm"] == "planted"]
bits_list = sorted({c["bits"] for c in cells})
ells = sorted({c["ell"] for c in cells})
beta_det = {int(r["ell"]): r["beta_det"] for r in d["beta_det"]}
colors = {1: "#ef8a62", 2: "#b2182b", 3: "#67001f"}

fig, axes = plt.subplots(1, len(bits_list), figsize=(3.6 * len(bits_list), 4.2), sharey=True)
if len(bits_list) == 1:
    axes = [axes]
for ax, bits in zip(axes, bits_list):
    for ell in ells:
        rows = sorted([c for c in cells if c["bits"] == bits and c["ell"] == ell], key=lambda c: c["beta"])
        x = [c["beta"] for c in rows]
        y = [c["recovered"] / c["trials"] for c in rows]
        ax.plot(x, y, "o-", color=colors.get(ell, "k"), ms=4, label=f"ℓ = {ell} (dim {rows[0]['dim']})")
        yh = [c["recovered_hg"] / c["trials"] for c in rows]
        ax.plot(x, yh, "x:", color=colors.get(ell, "k"), ms=4, alpha=0.6)
        ax.axvline(beta_det[ell], color=colors.get(ell, "k"), lw=0.8, ls="--", alpha=0.7)
    ax.axvline(d["beta_asymptotic"], color="black", lw=1.0, ls="-.")
    ax.axvline(d["beta_stop_rule"], color="black", lw=1.4)
    ax.text(d["beta_stop_rule"] + 0.005, 0.5, "β = 1/4\n(stop rule)", fontsize=7, va="center")
    ax.text(d["beta_asymptotic"] + 0.005, 0.85, "1/6\n(ℓ→∞)", fontsize=7, va="center")
    p = next(c["p"] for c in cells if c["bits"] == bits)
    ax.set_title(f"{bits}-bit p = {p}", fontsize=9)
    ax.set_xlabel("β  (planted x(Pᵢ) ≤ p^β)")
    ax.set_ylim(-0.05, 1.05)
    ax.set_xlim(min(d["betas"]) - 0.01, max(d["betas"]) + 0.01)
    ax.grid(alpha=0.3)
axes[0].set_ylabel(f"recovered fraction (N = {d['trials']} per cell)")
axes[0].legend(fontsize=7, loc="upper right")
fig.suptitle("Planted two-summand decomposition recovered by the S₃ Coppersmith lattice. "
             "Solid: any pair of the first 6 rows; dotted ×: Howgrave-Graham rows only; "
             "dashed: β_det(ℓ) = 1/18, 1/10, 5/42.", fontsize=8)
fig.tight_layout()
fig.savefig(src.parent / "recovery.svg")
fig.savefig(src.parent / "recovery.png", dpi=150)
print("wrote", src.parent / "recovery.svg")
