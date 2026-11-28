"""Plot recorded A7 numbers only: dependent rate (B = 8) with 95% Wilson intervals
per sampler, read from results/summary.json (written by the Rust binary), plus the
published 317/100000 line and its Wilson interval. Display only; computes nothing new."""
import json
import matplotlib; matplotlib.use("svg")
import matplotlib.pyplot as plt

s = json.load(open("results/summary.json"))
BLUE, INK, MUTED, BAND = "#2a78d6", "#0b0b0b", "#8a8984", "#e8e6e1"
fig, ax = plt.subplots(figsize=(7.5, 4.6))
pub = s["published"]
ax.axhspan(100 * pub["wilson95"][0], 100 * pub["wilson95"][1], color=BAND, zorder=0)
ax.axhline(100 * pub["rate"], color=MUTED, lw=1.5, ls="--", zorder=1)
names = []
for i, row in enumerate(s["per_sampler"]):
    r = 100 * row["dependent_rate"]
    lo, hi = (100 * v for v in row["dependent_rate_wilson95"])
    ax.errorbar(i, r, yerr=[[r - lo], [hi - r]], fmt="o", color=BLUE, ms=8, capsize=6, lw=2, zorder=3)
    ax.text(i + 0.08, max(r, 0.12), f"{r:.3f}%\n({row['dependent']}/{row['n']})", va="bottom", ha="left", fontsize=9, color=INK)
    names.append(row["sampler"])
ax.text(len(names) - 0.55, 100 * pub["rate"], "published 317/100000 = 0.317%\n(Wilson 95% band shaded)",
        va="bottom", ha="right", fontsize=8.5, color=INK)
ax.set_xticks(range(len(names)), names)
ax.set_xlim(-0.5, len(names) - 0.3)
ax.set_ylim(bottom=0)
ax.set_ylabel("dependent runs, % (relation with |n_i| ≤ 8, verified over Q)")
ax.set_title("xedni p=17 Experiment A: dependent rate per sampler, 95% Wilson intervals", fontsize=10)
ax.grid(axis="y", color="#dddbd5", lw=0.6)
for sp in ("top", "right"):
    ax.spines[sp].set_visible(False)
fig.tight_layout()
fig.savefig("a7_dependent_rate.svg")
