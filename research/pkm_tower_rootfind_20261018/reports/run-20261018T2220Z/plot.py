"""Draw the recorded values of the root-finder round as SVG.

Plots only numbers already written to ../../ANALYSIS.json by the native
analyser (examples/pkm_rootfind_analyze.rs): per-arm medians, minima and
maxima.  Nothing is computed here beyond choosing what to draw.
Usage: python plot.py   (from this directory)
"""
import json
import matplotlib

matplotlib.use("svg")
import matplotlib.pyplot as plt

A = json.load(open("../../ANALYSIS.json"))
CELLS = [(k, A["sessions"][k]["cells"][0]) for k in ("P5", "P6", "P7", "P8") if k in A["sessions"]]
ARMS = [("f4", "tower F4", "#7a7a7a"), ("sig-f5", "signature F5", "#b04a8a"),
        ("f6", "F6 analogue", "#d0702a"), ("enum", "enum", "#2a6fb0"),
        ("rootfind-subtract", "rootfind-subtract (m=2)", "#3a9a5a"),
        ("rootfind-mitm", "rootfind-mitm (m=2)", "#9ab83a")]


def bars(field, ylabel, title, fname):
    fig, ax = plt.subplots(figsize=(9.5, 4.4))
    x, ticks, labels = 0, [], []
    for key, c in CELLS:
        start = x
        for arm, lab, col in ARMS:
            v = (c["arms"].get(arm) or {}).get(field)
            if v and v["median"] and v["median"] > 0:
                ax.bar(x, v["median"], color=col, width=0.8)
                ax.errorbar(x, v["median"], yerr=[[v["median"] - v["min"]], [v["max"] - v["median"]]],
                            color="black", capsize=2, linewidth=0.8)
            x += 1
        ticks.append((start + x - 1) / 2)
        labels.append(f"{key}: {c['bits']}-bit")
        x += 1
    ax.set_yscale("log")
    ax.set_xticks(ticks, labels, fontsize=8)
    ax.set_ylabel(ylabel)
    ax.set_title(title, fontsize=10)
    for _, lab, col in ARMS:
        ax.bar(0, 0, color=col, label=lab)
    ax.legend(fontsize=7, loc="upper center", bbox_to_anchor=(0.5, -0.1), ncol=3)
    fig.tight_layout()
    fig.savefig(fname, bbox_inches="tight")


bars("s", "S = charged GAE / sqrt(r) (log scale)",
     "S per one-target run, median and range (8 targets, 4 at P8)\n"
     "root finders charge every addition; tower arms' S omits their multiply-adds", "s_per_arm.svg")
bars("fp_multiplications", "F_p multiplications per run (derived; log scale)",
     "F_p multiplications per run: 3*adds + 7*doubles + solver multiply-adds\n"
     "(PrimeCurve's fixed per-operation costs; inversions in the next graph)", "fp_multiplications.svg")
bars("inversions", "F_p inversions per run (derived; log scale)",
     "F_p inversions per run: one per affine addition or doubling, plus native inversions", "inversions.svg")
print("wrote s_per_arm.svg fp_multiplications.svg inversions.svg")
