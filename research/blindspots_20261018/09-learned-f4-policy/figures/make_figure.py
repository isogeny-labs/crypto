"""Plots recorded/derived values from figures/cell17.json (produced by ../analyze_cell17.py from raw/*.jsonl).
No new computation."""
import json, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
d = json.load(open("cell17.json"))
names = [("r50", "r50 (random half)"), ("r25", "r25 (random quarter)"), ("b25", "b25 learned"), ("b50", "b50 learned"),
         ("c", "c row order"), ("p", "p random order (null)"), ("d", "d trace transfer")]
fig, (ax, bx) = plt.subplots(1, 2, figsize=(11, 4.6), gridspec_kw={"width_ratios": [3, 2]})
for i, (k, lab) in enumerate(names):
    a = d["arms"][k]; r = a["ratio_w0"]
    col = "#1b7837" if k.startswith("r") else ("#2166ac" if k.startswith("b") else "#888888")
    ax.errorbar(r, i, xerr=[[r - a["lo"]], [a["hi"] - r]], fmt="o", color=col, capsize=3)
    ax.text(r * 1.06, i + 0.18, f"{r:.2f}  rows_a/rows_X {a['rows_ratio']:.2f}", fontsize=8)
ax.errorbar(d["o_ratio_3targets"], len(names), fmt="D", color="#b2182b")
ax.text(d["o_ratio_3targets"] * 1.06, len(names) + 0.18, f"{d['o_ratio_3targets']:.2f}  (3 targets)", fontsize=8)
ax.set_yticks(range(len(names) + 1)); ax.set_yticklabels([l for _, l in names] + ["o oracle subset"])
ax.axvline(1, color="k", lw=0.8); ax.axvline(1.3, color="k", lw=0.8, ls=":")
ax.set_xscale("log"); ax.set_xlim(0.6, 12)
ax.set_xlabel("sum touches(a) / sum touches(arm), 32 usable test targets (95% bootstrap CI)\nw = 0, 0.1, 1 change every ratio by under 0.01")
ax.set_title("Cell (17,4,3) measured vs incumbent a; dotted = frozen 1.3 threshold", fontsize=9)
t = d["cell13_train"]
bx.bar([x["idx"] for x in t], [x["touches"] / 1e9 for x in t], color="#c51b7d")
bx.axhline(3.0, color="k", ls="--", lw=1); bx.text(0, 3.05, "frozen cap 3e9", fontsize=8)
bx.set_xlabel("train target index"); bx.set_ylabel("incumbent touches (1e9)")
bx.set_title("Cell (13,3,4) train: 13 of 13 incumbent\nruns end cap_touch (BLOCKED)", fontsize=9)
fig.suptitle("Item 09 learned F4 policy, repair pass r2. Cells 19 and 21 trained, not evaluated. Host L0, toy scale.", fontsize=9)
fig.tight_layout()
fig.savefig("ratios.svg"); fig.savefig("ratios.png", dpi=150)
