"""Plots recorded/derived values from cells.json (written by ../analyze_all.py from raw/*.jsonl). No new computation."""
import json, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
d = json.load(open("cells.json"))
arms = [("b50", "b50 learned half", "#2166ac"), ("c", "c learned row order", "#2166ac"), ("d", "d trace transfer", "#2166ac"),
        ("r50", "r50 random half (null of b50)", "#1b7837"), ("p", "p random row order (null of c)", "#1b7837")]
cols = {"17": "#d95f02", "19": "#7570b3", "21": "#e7298a"}
names = {"17": "(17,4,3)", "19": "(19,4,3)", "21": "(21,5,3)"}
fig, (ax, bx) = plt.subplots(1, 2, figsize=(12, 4.8), gridspec_kw={"width_ratios": [3, 2]})
for j, (k, lab, _) in enumerate(arms):
    for ci, cell in enumerate(["17", "19", "21"]):
        a = d[cell]["arms"][k]; r, lo, hi, _m = a["w"]["0"]
        y = j + (ci - 1) * 0.22
        ax.errorbar(r, y, xerr=[[r - lo], [hi - r]], fmt="o", color=cols[cell], capsize=2, label=names[cell] if j == 0 else None)
ax.set_yticks(range(len(arms))); ax.set_yticklabels([l for _, l, _ in arms], fontsize=8); ax.invert_yaxis()
ax.axvline(1, color="k", lw=0.8); ax.axvline(1.3, color="k", lw=0.8, ls=":")
ax.set_xscale("log"); ax.set_xlim(0.5, 10)
ax.set_xlabel("sum touches(a) / sum touches(arm), 95% paired bootstrap; w = 0.1 and 1 change every ratio by under 0.01")
ax.set_title("Measured vs incumbent a, cells 17 / 19 / 21 (dotted = frozen 1.3 threshold)", fontsize=9); ax.legend(fontsize=8, loc="lower right")
cells = ["17", "19", "21"]; x = range(3)
bx.bar([i - 0.2 for i in x], [d[c]["null"]["b50"]["w"]["0"][0] for c in cells], 0.4, color="#2166ac", label="sum cost(r50) / sum cost(b50)")
bx.bar([i + 0.2 for i in x], [d[c]["null"]["c"]["w"]["0"][0] for c in cells], 0.4, color="#888888", label="sum cost(p) / sum cost(c)")
for i, c in enumerate(cells):
    for off, k in ((-0.2, "b50"), (0.2, "c")):
        v, lo, hi = d[c]["null"][k]["w"]["0"]
        bx.errorbar(i + off, v, yerr=[[v - lo], [hi - v]], color="k", capsize=2)
bx.axhline(1, color="k", lw=0.8); bx.set_xticks(list(x)); bx.set_xticklabels([names[c] for c in cells])
bx.set_ylabel("ratio (above 1: learned beats its null)"); bx.legend(fontsize=8)
bx.set_title("Matched nulls: b50 beats r50 only at (21,5,3)", fontsize=9)
fig.suptitle("Item 09 learned F4 policy, third pass. Cell (13,3,4) trained, not evaluated. Host L0, toy scale, counted touches.", fontsize=9)
fig.tight_layout(); fig.savefig("ratios_r3.svg"); fig.savefig("ratios_r3.png", dpi=150)
