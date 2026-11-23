#!/usr/bin/env python3
"""Plot recorded values from analysis.json (no new computation).

Usage: python3 plot.py analysis.json  -> fig_ladder.svg, fig_ladder.png
"""
import json
import sys

import matplotlib

matplotlib.use("Agg")
matplotlib.rcParams["svg.fonttype"] = "none"
matplotlib.rcParams["svg.hashsalt"] = "ladder"
import matplotlib.pyplot as plt

a = json.load(open(sys.argv[1]))
t = a["ic_over_rho"]
fig, axes = plt.subplots(1, 2, figsize=(9.0, 3.6))
cols = {"0.0": "#1b9e77", "0.1": "#d95f02", "1.0": "#7570b3"}
mk = {"Q_diem_2lp_new": ("o", "--", "2lp-new"), "Q_diem_2lp_sfb": ("s", "-", "2lp-sfb")}
ax = axes[0]
for arm, (m, ls, lab) in mk.items():
    for w, c in cols.items():
        pp = t[arm][f"all_{w}"]["per_prime"]
        ps = sorted(int(p) for p in pp)
        y = [pp[str(p)]["gm"] for p in ps]
        lo = [pp[str(p)]["gm"] - pp[str(p)]["ci95"][0] for p in ps]
        hi = [pp[str(p)]["ci95"][1] - pp[str(p)]["gm"] for p in ps]
        ax.errorbar(ps, y, yerr=[lo, hi], marker=m, ls=ls, color=c, ms=4, lw=1, capsize=0,
                    label=f"{lab}, w={w}")
ax.axhline(1, color="k", lw=0.8)
ax.set_xscale("log", base=2)
ax.set_yscale("log")
ax.set_xlabel("p")
ax.set_ylabel("IC / ideal neg. rho (GAE_w, all native)")
ax.set_title("IC/rho, 5 curves x 2 seeds, boot95", fontsize=9)
ax.legend(fontsize=6, ncol=2)
ax = axes[1]
for arm, (m, ls, lab) in mk.items():
    pp = a["descriptors"][arm]
    ps = sorted(int(p) for p in pp)
    ax.plot(ps, [pp[str(p)]["fpm_mid"] for p in ps], marker=m, ls=ls, color="#444", ms=4,
            label=f"{lab} FPM mid")
    ax.plot(ps, [pp[str(p)]["rel_enum_mul"] for p in ps], marker=m, ls=ls, color="#e7298a", ms=4,
            label=f"{lab} rel_enum_mul")
ref = a["reference"]
byp = {}
for k, v in ref.items():
    byp.setdefault(int(k.split("/")[0]), []).append(v["ideal_fpm"])
ps = sorted(byp)
ax.plot(ps, [sum(byp[p]) / len(byp[p]) for p in ps], color="#66a61e", lw=1.5,
        label="ideal neg. rho FPM")
ax.set_xscale("log", base=2)
ax.set_yscale("log")
ax.set_xlabel("p")
ax.set_ylabel("counted F_p multiplications")
ax.set_title("cost per solve (geo-mean)", fontsize=9)
ax.legend(fontsize=6)
for ax in axes:
    ax.minorticks_off()
fig.suptitle("research/quartic_diem_ladder_20261121  [pcc:unverified]", fontsize=8)
fig.tight_layout()
fig.savefig("fig_ladder.svg")
fig.savefig("fig_ladder.png", dpi=72)

# size: round SVG coordinates to 1 decimal, palette PNG (layout only)
import re

svg = open("fig_ladder.svg").read()
svg = re.sub(r"(\d+\.\d)\d+", r"\1", svg)
svg = re.sub(r"<metadata>.*?</metadata>", "", svg, flags=re.S)
svg = re.sub(r"<!--.*?-->", "", svg, flags=re.S)
svg = "\n".join(x.strip() for x in svg.splitlines() if x.strip())
open("fig_ladder.svg", "w").write(svg)
try:
    from PIL import Image

    Image.open("fig_ladder.png").convert("RGB").quantize(64).save("fig_ladder.png", optimize=True)
except ImportError:
    pass
