#!/usr/bin/env python3
"""Plot recorded values from analysis.json (no new computation).
Usage: python3 plot.py analysis.json -> fig_p0lines.svg, fig_p0lines.png"""
import json, re, sys
import matplotlib
matplotlib.use("Agg")
matplotlib.rcParams["svg.fonttype"] = "none"
matplotlib.rcParams["svg.hashsalt"] = "p0lines"
import matplotlib.pyplot as plt

a = json.load(open(sys.argv[1]))
fig, axes = plt.subplots(1, 2, figsize=(9.0, 3.6))
cols = {"0.0": "#1b9e77", "0.1": "#d95f02", "1.0": "#7570b3"}
ax = axes[0]
for (cmp, panel), ls, mk, lab in [(("p0_over_nofb", "primary_seeds_5_6"), "-", "o", "p0/nofb, held-out 5-6"),
                                  (("p0i3_over_nofb", "primary_seeds_5_6"), ":", "^", "p0i3/nofb, held-out 5-6")]:
    for w, c in cols.items():
        pp = a["paired"][cmp][panel][f"gae_{w}"]["per_prime"]
        ps = sorted(int(p) for p in pp)
        y = [pp[str(p)]["gm"] for p in ps]
        lo = [pp[str(p)]["gm"] - pp[str(p)]["ci95"][0] for p in ps]
        hi = [pp[str(p)]["ci95"][1] - pp[str(p)]["gm"] for p in ps]
        ax.errorbar(ps, y, yerr=[lo, hi], marker=mk, ls=ls, color=c, ms=4, lw=1,
                    label=f"w={w}, {lab}")
ax.axhline(1, color="k", lw=0.8)
ax.set_xscale("log", base=2)
ax.set_ylim(0.5, 1.15)
ax.set_xlabel("p")
ax.set_ylabel("paired GAE_w ratio arm / nofb")
ax.set_title("paired cost ratio (same instances, seeds)", fontsize=9)
ax.legend(fontsize=6, ncol=2)
ax = axes[1]
for arm, ls, mk, lab in [("Q_diem_2lp_sfb_sge_nofb", "--", "s", "nofb"),
                         ("Q_diem_2lp_sfb_sge_p0", "-", "o", "p0")]:
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
ax.set_title("IC/rho, 5 curves x 6 seeds, boot95", fontsize=9)
ax.legend(fontsize=6, ncol=2)
for ax in axes:
    ax.minorticks_off()
fig.suptitle("research/quartic_diem_p0lines_20261128  [pcc:unverified]", fontsize=8)
fig.tight_layout()
fig.savefig("fig_p0lines.svg")
fig.savefig("fig_p0lines.png", dpi=72)
svg = open("fig_p0lines.svg").read()
svg = re.sub(r"(\d+\.\d)\d+", r"\1", svg)
svg = re.sub(r"<metadata>.*?</metadata>", "", svg, flags=re.S)
svg = re.sub(r"<!--.*?-->", "", svg, flags=re.S)
open("fig_p0lines.svg", "w").write("\n".join(x.strip() for x in svg.splitlines() if x.strip()))
from PIL import Image
Image.open("fig_p0lines.png").convert("RGB").quantize(64).save("fig_p0lines.png", optimize=True)
