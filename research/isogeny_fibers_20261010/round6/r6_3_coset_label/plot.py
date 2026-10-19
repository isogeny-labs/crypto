"""Plot recorded R6.3 values only (no computation of results).

Reads exhaustive.json, sample.json, bench.json in this directory and writes
labels.svg, cost.svg and plot_data.json.
"""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import Patch

D = os.path.dirname(os.path.abspath(__file__))
ld = lambda f: json.load(open(os.path.join(D, f)))
ex, sa, be = ld("exhaustive.json"), ld("sample.json"), ld("bench.json")

TXT, SEC, GRID, SURF = "#0b0b0b", "#52514e", "#e4e3df", "#fcfcfb"
CAT = ["#2a78d6", "#eb6834", "#1baf7a", "#eda100"]  # labels 00, 01, 10, 11
plt.rcParams.update({"font.size": 9, "text.color": TXT, "axes.labelcolor": SEC,
                     "xtick.color": SEC, "ytick.color": SEC, "axes.edgecolor": GRID,
                     "svg.fonttype": "none"})

rows = ex["per_n"] + sa["per_n"]
ns = [r["n"] for r in rows]
grid = []
for r in rows:
    col = []
    for c in r["cosets"]:
        lab = c["label"]
        col.append(None if lab is None else 2 * lab[0] + lab[1])
    grid.append(col)

fig, ax = plt.subplots(figsize=(9.5, 2.9), facecolor=SURF)
ax.set_facecolor(SURF)
for i, (r, col) in enumerate(zip(rows, grid)):
    for k, v in enumerate(col):
        color = "#9a9994" if v is None else CAT[v]
        ax.add_patch(plt.Rectangle((i + 0.04, k + 0.04), 0.92, 0.92, color=color, lw=0))
        t = "mixed" if v is None else f"{v >> 1}{v & 1}"
        ax.text(i + 0.5, k + 0.5, t, ha="center", va="center", fontsize=7.5,
                color="#ffffff" if v in (0, 2) else TXT)
ax.set_xlim(0, len(rows)); ax.set_ylim(0, 4)
ax.set_xticks([i + 0.5 for i in range(len(rows))])
ax.set_xticklabels([f"{n}\n({n % 8})" for n in ns])
ax.set_yticks([0.5, 1.5, 2.5, 3.5])
ax.set_yticklabels(["k=0 (4E)", "k=1 (P4+4E)", "k=2 (T+4E)", "k=3 (-P4+4E)"])
ax.axvline(len(ex["per_n"]), color=TXT, lw=1)
ax.set_xlabel("n (n mod 8);  left of the rule: exhaustive, every point;  right: sampled, >= 2000 points per coset")
ax.set_title("Measured label (Tr x, Tr(y + x H(x))) on each coset of 4E, P4 = (1,0); one label per cell = constant on the coset",
             fontsize=9, loc="left")
ax.legend(handles=[Patch(color=CAT[i], label=f"{i >> 1}{i & 1}") for i in range(4)],
          ncol=4, loc="upper right", bbox_to_anchor=(1, 1.32), frameon=False, title=None)
for s in ax.spines.values():
    s.set_visible(False)
fig.tight_layout()
fig.savefig(os.path.join(D, "labels.svg"), facecolor=SURF)
fig.savefig(os.path.join(D, "labels.png"), facecolor=SURF, dpi=160)

# Cost: two panels, separate axes (wall contended; counted word ops), members of 4E.
bn = [r["n"] for r in be["per_n"]]
arms = [("tab", "table (VT, this field)"), ("ct_mask", "CT masked H"), ("ct_fong", "CT even-fold H")]
def pop(r, name):
    return [p for p in r["populations"] if p["population"] == name][0]
fig, axs = plt.subplots(1, 2, figsize=(9.5, 3.3), facecolor=SURF)
w = 0.26
out = {"n": bn}
for j, (key, lab) in enumerate(arms):
    med = [pop(r, "members_of_4E")["wall_contended"][key]["median_ns"] for r in be["per_n"]]
    lo = [pop(r, "members_of_4E")["wall_contended"][key]["q10_ns"] for r in be["per_n"]]
    hi = [pop(r, "members_of_4E")["wall_contended"][key]["q90_ns"] for r in be["per_n"]]
    wo = [pop(r, "members_of_4E")["counted"][key]["word_ops_per_call"] for r in be["per_n"]]
    xs = [i + (j - 1) * w for i in range(len(bn))]
    axs[0].bar(xs, med, w * 0.92, color=CAT[j], label=lab,
               yerr=[[m - l for m, l in zip(med, lo)], [h - m for m, h in zip(med, hi)]],
               error_kw={"elinewidth": 1, "ecolor": SEC, "capsize": 0})
    axs[1].bar(xs, wo, w * 0.92, color=CAT[j], label=lab)
    out[key] = {"median_ns": med, "q10_ns": lo, "q90_ns": hi, "word_ops": wo}
libm = [pop(r, "members_of_4E")["wall_contended"]["lib_is_in_4e"]["median_ns"] for r in be["per_n"]]
axs[0].plot(range(len(bn)), libm, "o", color=TXT, ms=4, label="library is_in_4e (reference)")
out["lib_median_ns"] = libm
for a, t in zip(axs, ["wall per call, ns (contended; median, q10-q90 of reps)",
                      "counted word operations per call"]):
    a.set_facecolor(SURF)
    a.set_xticks(range(len(bn))); a.set_xticklabels([str(n) for n in bn])
    a.set_xlabel("n"); a.set_title(t, fontsize=9, loc="left")
    a.grid(axis="y", color=GRID, lw=0.6); a.set_axisbelow(True)
    for s in ("top", "right"):
        a.spines[s].set_visible(False)
axs[0].legend(frameon=False, fontsize=7.5, loc="upper left")
fig.suptitle("4E test on members of 4E (no early exit), bench.json", fontsize=9, x=0.01, ha="left")
fig.tight_layout()
fig.savefig(os.path.join(D, "cost.svg"), facecolor=SURF)
fig.savefig(os.path.join(D, "cost.png"), facecolor=SURF, dpi=160)
json.dump({"labels": {"n": ns, "coset_label_index": grid}, "cost_members": out},
          open(os.path.join(D, "plot_data.json"), "w"), indent=1)
print("ok")
