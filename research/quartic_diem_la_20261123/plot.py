#!/usr/bin/env python3
"""Plot recorded values from analysis.json (no new computation).
Usage: python3 plot.py analysis.json -> fig_la.svg, fig_la.png"""
import json, re, sys
import matplotlib
matplotlib.use("Agg")
matplotlib.rcParams["svg.fonttype"] = "none"
matplotlib.rcParams["svg.hashsalt"] = "qla"
import matplotlib.pyplot as plt

a = json.load(open(sys.argv[1]))
fig, axes = plt.subplots(1, 2, figsize=(9.0, 3.6))
cols = {"0.0": "#1b9e77", "0.1": "#d95f02", "1.0": "#7570b3"}
mk = {"Q_diem_2lp_sfb": ("s", "--", "2lp-sfb"), "Q_diem_2lp_sfb_sge": ("o", "-", "2lp-sfb-sge")}
ax = axes[0]
for arm, (m, ls, lab) in mk.items():
    for w, c in cols.items():
        pp = a["ic_over_rho"][arm][f"all_{w}"]["per_prime"]
        ps = sorted(int(p) for p in pp)
        y = [pp[str(p)]["gm"] for p in ps]
        lo = [pp[str(p)]["gm"] - pp[str(p)]["ci95"][0] for p in ps]
        hi = [pp[str(p)]["ci95"][1] - pp[str(p)]["gm"] for p in ps]
        ax.errorbar(ps, y, yerr=[lo, hi], marker=m, ls=ls, color=c, ms=4, lw=1, label=f"{lab}, w={w}")
ax.axhline(1, color="k", lw=0.8)
ax.set_xscale("log", base=2); ax.set_yscale("log")
ax.set_xlabel("p"); ax.set_ylabel("IC / ideal neg. rho (GAE_w)")
ax.set_title("IC/rho, 5 curves x 2 seeds, boot95", fontsize=9)
ax.legend(fontsize=6, ncol=2)
ax = axes[1]
for w, c in cols.items():
    pp = a["paired_sge_over_sfb"][f"gae_{w}"]["per_prime"]
    ps = sorted(int(p) for p in pp)
    y = [pp[str(p)]["gm"] for p in ps]
    lo = [pp[str(p)]["gm"] - pp[str(p)]["ci95"][0] for p in ps]
    hi = [pp[str(p)]["ci95"][1] - pp[str(p)]["gm"] for p in ps]
    ax.errorbar(ps, y, yerr=[lo, hi], marker="o", color=c, ms=4, lw=1, label=f"GAE w={w}")
ax.axhline(1, color="k", lw=0.8)
ax.set_xscale("log", base=2); ax.set_ylim(0, 1.1)
ax.set_xlabel("p"); ax.set_ylabel("paired ratio sge / sfb")
ax.set_title("paired cost ratio (same instances, seeds)", fontsize=9)
ax.legend(fontsize=6)
for ax in axes:
    ax.minorticks_off()
fig.suptitle("research/quartic_diem_la_20261123  [pcc:unverified]", fontsize=8)
fig.tight_layout()
fig.savefig("fig_la.svg"); fig.savefig("fig_la.png", dpi=72)
svg = open("fig_la.svg").read()
svg = re.sub(r"(\d+\.\d)\d+", r"\1", svg)
svg = re.sub(r"<metadata>.*?</metadata>", "", svg, flags=re.S)
svg = re.sub(r"<!--.*?-->", "", svg, flags=re.S)
open("fig_la.svg", "w").write("\n".join(x.strip() for x in svg.splitlines() if x.strip()))
from PIL import Image
Image.open("fig_la.png").convert("RGB").quantize(64).save("fig_la.png", optimize=True)
