"""Render only statistics already emitted by the frozen Rust analyzer."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parent
groups = {g["group"]: g for g in json.loads((ROOT / "run/summary.json").read_text())["groups"]}
cases = [
    ("coset:word62:m8192", "Coset · word62 / 8192"),
    ("coset:word62:m16384", "Coset · word62 / 16384"),
    ("coset:p224:m2048", "Coset · P224 / 2048"),
    ("rational:word62:m2048", "Rational · word62 / 2048"),
    ("rational:p224:m128", "Rational · P224 / 128"),
    ("rational:p224:m512", "Rational · P224 / 512"),
]
plt.rcParams.update({"font.size": 9, "svg.fonttype": "none"})

def save(fig, name):
    fig.savefig(ROOT / (name + ".svg"), bbox_inches="tight")
    fig.savefig(ROOT / (name + ".png"), dpi=60, bbox_inches="tight")
    plt.close(fig)

fig, axes = plt.subplots(1, 2, figsize=(10.4, 4.5), sharey=True)
for ax, phase, title in zip(axes, ("work", "query_work"), ("Fresh construction + query", "Query only")):
    for j, (key, _) in enumerate(cases):
        g = groups.get(key + ":conditioned-sum:finite")
        if g is None:
            ax.text(.1, j, "No completed finite cell", transform=ax.get_yaxis_transform())
            continue
        for arm, label, color, offset in (("newton", "Newton / Euclid", "#bd542b", -.18), ("half", "Half-GCD", "#177957", 0), ("lookup", "Lookup", "#226aaa", .18)):
            v = g["arms"][arm]["metrics"][phase]["muls"]
            if v["n"]:
                ax.hlines(j + offset, v["min"], v["max"], color=color)
                ax.plot(v["median"], j + offset, "o", color=color, markersize=5, label=label if j == 0 else None)
            else:
                ax.text(.1, j + offset, arm + ": no success", color=color, transform=ax.get_yaxis_transform(), fontsize=7)
    ax.set_xscale("log")
    ax.set(title=title, xlabel="Primitive field multiplications", yticks=range(len(cases)), yticklabels=[c[1] for c in cases])
    ax.grid(axis="x", alpha=.25)
axes[0].invert_yaxis()
axes[0].legend(fontsize=8, loc="lower left")
fig.suptitle("Conditioned finite targets: native medians and observed ranges\nAll arms executed on identical inputs; clocks below L2", fontsize=10)
fig.tight_layout()
save(fig, "counts")

fig, ax = plt.subplots(figsize=(9.2, 3.9))
for j, (key, _) in enumerate(cases):
    g = groups.get(key + ":conditioned-sum:finite")
    if g is None:
        continue
    for phase, label, color, offset, marker in (("work", "Construction + query", "#177957", -.12, "o"), ("query_work", "Query only", "#bd542b", .12, "s")):
        v = g["paired_ratios"][phase + ":newton/half"]
        if v["n"]:
            ax.hlines(j + offset, v["min"], v["max"], color=color)
            ax.plot(v["median"], j + offset, marker, color=color, markersize=5, label=label if j == 0 else None)
    for name, marker, color, label in (("work:half/half-aa", "+", "#8253a0", "A/A count ratio"), ("work:lookup/lookup-fast", "x", "#226aaa", "Unused-backend lookup ratio")):
        v = g["paired_ratios"][name]
        if v["n"]:
            ax.plot(v["median"], j, marker, color=color, label=label if j == 0 else None)
ax.axvline(1, color="gray", linewidth=1, linestyle="--")
ax.set(yticks=range(len(cases)), yticklabels=[c[1] for c in cases], xlabel="Paired Newton / Half-GCD multiplication-count ratio")
ax.invert_yaxis()
ax.grid(axis="x", alpha=.25)
ax.legend(fontsize=8, loc="best")
fig.tight_layout()
save(fig, "ratios")
