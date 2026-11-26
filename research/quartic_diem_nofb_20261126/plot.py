#!/usr/bin/env python3
"""Plot recorded values from analysis.json (no new computation).
Usage: python3 plot.py analysis.json -> fig_nofb.svg, fig_nofb.png"""
import json, re, sys
import matplotlib
matplotlib.use("Agg")
matplotlib.rcParams["svg.fonttype"] = "none"
matplotlib.rcParams["svg.hashsalt"] = "nofb"
import matplotlib.pyplot as plt

a = json.load(open(sys.argv[1]))
fig, axes = plt.subplots(1, 2, figsize=(9.0, 3.6))
cols = {"0.0": "#1b9e77", "0.1": "#d95f02", "1.0": "#7570b3"}
ax = axes[0]
for panel, ls, mk, lab in [("primary_seeds_3_4", "-", "o", "held-out seeds 3-4"),
                           ("all_seeds_1_4", "--", "s", "all seeds 1-4")]:
    for w, c in cols.items():
        pp = a["paired_nofb_over_sge"][panel][f"gae_{w}"]["per_prime"]
        ps = sorted(int(p) for p in pp)
        y = [pp[str(p)]["gm"] for p in ps]
        lo = [pp[str(p)]["gm"] - pp[str(p)]["ci95"][0] for p in ps]
        hi = [pp[str(p)]["ci95"][1] - pp[str(p)]["gm"] for p in ps]
        ax.errorbar(ps, y, yerr=[lo, hi], marker=mk, ls=ls, color=c, ms=4, lw=1,
                    label=f"w={w}, {lab}")
ax.axhline(1, color="k", lw=0.8)
ax.set_xscale("log", base=2)
ax.set_ylim(0, 1.1)
ax.set_xlabel("p")
ax.set_ylabel("paired GAE_w ratio nofb / sge")
ax.set_title("paired cost ratio (same instances, seeds)", fontsize=9)
ax.legend(fontsize=6, ncol=2)
ax = axes[1]
for arm, ls, mk, lab in [("Q_diem_2lp_sfb_sge", "--", "s", "2lp-sfb-sge"),
                         ("Q_diem_2lp_sfb_sge_nofb", "-", "o", "2lp-sfb-sge-nofb")]:
    for w, c in cols.items():
        pp = a["ic_over_rho"][arm][f"all_{w}"]["per_prime"]
        ps = sorted(int(p) for p in pp)
        y = [pp[str(p)]["gm"] for p in ps]
        lo = [pp[str(p)]["gm"] - pp[str(p)]["ci95"][0] for p in ps]
        hi = [pp[str(p)]["ci95"][1] - pp[str(p)]["gm"] for p in ps]
        ax.errorbar(ps, y, yerr=[lo, hi], marker=mk, ls=ls, color=c, ms=4, lw=1,
                    label=f"{lab}, w={w}")
ax.axhline(1, color="k", lw=0.8)
ax.set_xscale("log", base=2)
ax.set_yscale("log")
ax.set_xlabel("p")
ax.set_ylabel("IC / ideal neg. rho (GAE_w)")
ax.set_title("IC/rho, 5 curves x 4 seeds, boot95", fontsize=9)
ax.legend(fontsize=6, ncol=2)
for ax in axes:
    ax.minorticks_off()
fig.suptitle("research/quartic_diem_nofb_20261126  [pcc:unverified]", fontsize=8)
fig.tight_layout()
fig.savefig("fig_nofb.svg")
fig.savefig("fig_nofb.png", dpi=72)
svg = open("fig_nofb.svg").read()
svg = re.sub(r"(\d+\.\d)\d+", r"\1", svg)
svg = re.sub(r"<metadata>.*?</metadata>", "", svg, flags=re.S)
svg = re.sub(r"<!--.*?-->", "", svg, flags=re.S)
open("fig_nofb.svg", "w").write("\n".join(x.strip() for x in svg.splitlines() if x.strip()))
from PIL import Image
Image.open("fig_nofb.png").convert("RGB").quantize(64).save("fig_nofb.png", optimize=True)
