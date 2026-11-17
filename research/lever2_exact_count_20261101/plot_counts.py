#!/usr/bin/env python3
"""Lay out the recorded exact counts of runs/m2 and runs/m3 (summary.json).

Document rendering only (AGENTS.md "Implementation language"): every number is read from
summary.json, which the Rust instrument wrote; nothing is computed or fitted here.

Writes counts.svg, counts.png and tables.md. Usage: plot_counts.py [studydir]
"""
import json
import os
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = sys.argv[1] if len(sys.argv) > 1 else "."
ALPHA_C = 0.01 / 90
FAMS = ["subgroup", "ap", "interval", "image", "dickson"]
STYLE = {
    "subgroup": dict(color="#1b6ca8", marker="o"),
    "ap": dict(color="#c4602a", marker="s"),
    "interval": dict(color="#2e8b57", marker="^"),
    "image": dict(color="#8a4fa0", marker="D"),
    "dickson": dict(color="#b8860b", marker="v"),
}
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 9, "axes.spines.top": False,
                     "axes.spines.right": False, "svg.fonttype": "none"})

summaries = {}
for m in (2, 3):
    path = os.path.join(root, f"runs/m{m}/summary.json")
    if os.path.exists(path):
        summaries[m] = json.load(open(path))["sizes"]


def fam(size, name):
    return next(f for f in size["families"] if f["family"] == name)


def verdict(t):
    if t["mw_p"] >= ALPHA_C:
        return "within"
    return "above" if t["cliff_delta"] > 0 else "below"


# ---- tables.md ------------------------------------------------------------
lines = [f"Bonferroni level alpha_c = 0.01/90 = {ALPHA_C:.2e}. `fires` counts instances with Cmax >= B^(m-1). "
         "Cmax and coverage are medians over the family's instances; p is the two-sided Mann-Whitney p against the "
         "400-instance null, d is Cliff's delta (positive: family larger).", ""]
for m, sizes in summaries.items():
    lines += [f"### m = {m}", "",
              "| B | p | threshold | family | fires | Cmax med [min, max] | null Cmax med | p (Cmax) | d | Cmax verdict | "
              "coverage med | null coverage | p (cov) | d | coverage verdict |",
              "|--:|--:|--:|:--|--:|:--|--:|--:|--:|:--|--:|--:|--:|--:|:--|"]
    for sz in sizes:
        for f in sz["families"]:
            c, v = f["cmax"], f["coverage"]
            if f["family"] == "null":
                continue
            lines.append(
                f"| {sz['B']} | {sz['p']} | {sz['threshold']} | {f['family']} | {f['fires']}/{f['instances']} | "
                f"{c['median']:g} [{c['min']:g}, {c['max']:g}] | {c['null_median']:g} | {c['mw_p']:.1e} | "
                f"{c['cliff_delta']:+.2f} | {verdict(c)} | {v['median']:.4f} | {v['null_median']:.4f} | "
                f"{v['mw_p']:.1e} | {v['cliff_delta']:+.2f} | {verdict(v)} |")
    lines += ["", "A/A (null halves of 200) Mann-Whitney p, per size:", "",
              "| B | Cmax | coverage |", "|--:|--:|--:|"]
    for sz in sizes:
        aa = sz["aa_null_halves"]
        lines.append(f"| {sz['B']} | {aa['cmax']['mw_p']:.3f} | {aa['coverage']['mw_p']:.3f} |")
    if m == 3:
        lines += ["", "Cmax with the 3B-2 trivial solutions removed (medians; secondary, descriptive):", "",
                  "| B | " + " | ".join(FAMS + ["null", "smallmult"]) + " |",
                  "|--:|" + "--:|" * (len(FAMS) + 2)]
        for sz in sizes:
            cells = [f"{fam(sz, n)['cmax_nt']['median']:g}" for n in FAMS + ["null", "smallmult"]]
            lines.append(f"| {sz['B']} | " + " | ".join(cells) + " |")
    lines.append("")
open(os.path.join(root, "tables.md"), "w").write("\n".join(lines))

# ---- figure ---------------------------------------------------------------
ms = sorted(summaries)
fig, axes = plt.subplots(2, len(ms), figsize=(5.6 * len(ms), 7.4), squeeze=False)
for col, m in enumerate(ms):
    sizes = summaries[m]
    xs = [s["B"] for s in sizes]
    top, bot = axes[0][col], axes[1][col]
    thr = [s["threshold"] for s in sizes]
    nul = [fam(s, "null")["cmax"] for s in sizes]
    top.fill_between(xs, [c["min"] / t for c, t in zip(nul, thr)], [c["max"] / t for c, t in zip(nul, thr)],
                     color="#bbbbbb", alpha=0.55, lw=0, label="null (400 random bases): min to max")
    top.plot(xs, [c["median"] / t for c, t in zip(nul, thr)], color="#555555", lw=1.2, label="null median")
    for name in FAMS:
        st = STYLE[name]
        med = [fam(s, name)["cmax"]["median"] / t for s, t in zip(sizes, thr)]
        top.plot(xs, med, lw=1.1, ms=5, label=name, **st)
    ctl = [fam(s, "smallmult")["cmax"]["median"] / t for s, t in zip(sizes, thr)]
    top.plot(xs, ctl, color="#d62728", lw=1.6, marker="*", ms=9, label="x(kP) control (fires)")
    top.axhline(1.0, color="#222222", lw=1.0, ls="--")
    top.text(xs[0], 1.18, "threshold B^(m-1): the lever is 'dead' at or above", fontsize=8, color="#222222")
    top.set_xscale("log", base=2)
    top.set_yscale("log")
    top.set_xlabel("base size B")
    top.set_ylabel("Cmax / B^(m-1)")
    top.set_title(f"m = {m}: largest closure count over all targets", loc="left", fontsize=10)
    if col == 0:
        top.legend(frameon=False, fontsize=7.5, loc="center right")
    for name in FAMS:
        st = STYLE[name]
        bot.plot(xs, [fam(s, name)["coverage"]["median"] for s in sizes], lw=1.1, ms=5, label=name, **st)
    bot.plot(xs, [fam(s, "null")["coverage"]["median"] for s in sizes], color="#555555", lw=2.2, label="null")
    bot.plot(xs, [fam(s, "smallmult")["coverage"]["median"] for s in sizes], color="#d62728", lw=1.6,
             marker="*", ms=9, label="x(kP) control")
    bot.set_xscale("log", base=2)
    bot.set_yscale("log")
    bot.set_xlabel("base size B")
    bot.set_ylabel("coverage: fraction of targets with a solution")
    bot.set_title(f"m = {m}: coverage", loc="left", fontsize=10)
fig.text(0.01, 0.005,
         "Exact counts over every ordered tuple of the base and every target in F_p (runs/m*/summary.json); "
         "40 instances per family, 400 for the null, p the smallest prime 2Bk+1 >= B^m.",
         fontsize=7.5, color="#555")
fig.tight_layout(rect=(0, 0.02, 1, 1))
fig.savefig(os.path.join(root, "counts.svg"))
fig.savefig(os.path.join(root, "counts.png"), dpi=160)
print("wrote counts.svg counts.png tables.md for m =", ms)
