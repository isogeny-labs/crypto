"""Draw the recorded values of the prime-field PKM tower round as SVG.

Plots only numbers already written to ../../ANALYSIS.json by the native
analyser (examples/pkm_tower_prime_analyze.rs): per-arm medians, minima,
maxima and per-target values.  Nothing is computed here beyond choosing
what to draw.
Usage: python plot.py   (from this directory)
"""
import json
import matplotlib

matplotlib.use("svg")
import matplotlib.pyplot as plt

A = json.load(open("../../ANALYSIS.json"))
CELLS = []
for key in ("P5", "P6", "P7", "P8"):
    s = A["sessions"].get(key)
    if s and s["cells"]:
        CELLS.append((key, s["cells"][0]))
TOWER = [("f4", "tower F4", "#7a7a7a"), ("sig-f5", "signature F5", "#b04a8a"), ("f6", "F6 analogue (cap 4)", "#d0702a")]
ALL = [("rho-negation", "rho, negation", "#9ab83a")] + TOWER + [("enum", "enum (no algebra)", "#2a6fb0")]


# 1. Counted F_p multiply-adds per run, by tower engine and cell.
fig, ax = plt.subplots(figsize=(8.5, 4.2))
x, ticks, labels = 0, [], []
for key, c in CELLS:
    start = x
    for arm, lab, col in TOWER:
        v = (c["arms"].get(arm) or {}).get("muladds")
        if v:
            ax.bar(x, v["median"], color=col, width=0.8)
            ax.errorbar(x, v["median"], yerr=[[v["median"] - v["min"]], [v["max"] - v["median"]]],
                        color="black", capsize=2, linewidth=0.8)
            ax.text(x, v["max"] * 1.4, f"n={v['n']}", ha="center", fontsize=6)
        x += 1
    ticks.append((start + x - 1) / 2)
    labels.append(f"{key}: {c['bits']}-bit")
    x += 1
ax.set_yscale("log")
ax.set_xticks(ticks, labels, fontsize=8)
ax.set_ylabel("F_p multiply-adds per run (counted; log scale)")
ax.set_title("Tower-engine work per one-target run, median and range\n"
             "enum does no algebra (0 multiply-adds) and is not drawn", fontsize=10)
for _, lab, col in TOWER:
    ax.bar(0, 0, color=col, label=lab)
ax.legend(fontsize=8, loc="upper center", bbox_to_anchor=(0.5, -0.1), ncol=3)
fig.tight_layout()
fig.savefig("muladds_per_run.svg", bbox_inches="tight")

# 2. S per arm and cell, with the floor.
fig, ax = plt.subplots(figsize=(9, 4.4))
x, ticks, labels = 0, [], []
for key, c in CELLS:
    start = x
    for arm, lab, col in ALL:
        v = (c["arms"].get(arm) or {}).get("s")
        if v is None:
            x += 1
            continue
        ax.bar(x, v["median"], color=col, width=0.8)
        ax.errorbar(x, v["median"], yerr=[[v["median"] - v["min"]], [v["max"] - v["median"]]],
                    color="black", capsize=2, linewidth=0.8)
        x += 1
    ax.hlines(c["floor_s"], start - 0.5, x - 0.5, color="red", linestyle=":", linewidth=1.2)
    ticks.append((start + x - 1) / 2)
    labels.append(f"{key}: {c['bits']}-bit")
    x += 1
ax.set_yscale("log")
ax.set_xticks(ticks, labels, fontsize=8)
ax.set_ylabel("S = charged GAE / sqrt(r) (log scale)")
ax.set_title("Cost per one-target run in ecbench's unit, median and range\n"
             "tower arms' S is a lower bound (multiply-adds unpriced); red dotted line = floor sqrt(pi/4)", fontsize=10)
for _, lab, col in ALL:
    ax.bar(0, 0, color=col, label=lab)
ax.legend(fontsize=7, loc="upper center", bbox_to_anchor=(0.5, -0.1), ncol=5)
fig.tight_layout()
fig.savefig("s_per_arm.svg", bbox_inches="tight")

# 3. Per-target engine ratios: W(sig-F5)/W(F4) and log2 W(F4)/W(F6).
fig, (a1, a2) = plt.subplots(1, 2, figsize=(9, 3.6))
for i, (key, c) in enumerate(CELLS):
    q = c["questions"]
    if "q1_sig_f5_over_f4_muladds" in q:
        vals = q["q1_sig_f5_over_f4_muladds"]["ratio"]["values"]
        a1.scatter([i + (j - len(vals) / 2) * 0.04 for j in range(len(vals))], vals, s=14, color="#b04a8a")
    if "q2_log2_f4_over_f6_muladds" in q:
        vals = q["q2_log2_f4_over_f6_muladds"]["log2_ratio"]["values"]
        a2.scatter([i + (j - len(vals) / 2) * 0.04 for j in range(len(vals))], vals, s=14, color="#d0702a")
for a in (a1, a2):
    a.set_xticks(range(len(CELLS)), [k for k, _ in CELLS], fontsize=8)
a1.axhline(1.0, color="#444", linewidth=0.8)
a1.set_ylabel("W(signature F5) / W(F4)")
a1.set_title("Q1: same tree; < 1 favours F5", fontsize=9)
a2.axhline(0.1, color="#444", linestyle="--", linewidth=0.8)
a2.axhspan(-0.05, 0.05, color="#dddddd", zorder=0)
a2.set_ylabel("log2( W(F4) / W(F6) )")
a2.set_title("Q2: dashed = 0.1 rule, grey = 'no saving'", fontsize=9)
fig.suptitle("Per-target engine ratios (counted F_p multiply-adds)", fontsize=10)
fig.tight_layout()
fig.savefig("engine_ratios.svg", bbox_inches="tight")
print("wrote muladds_per_run.svg s_per_arm.svg engine_ratios.svg")
