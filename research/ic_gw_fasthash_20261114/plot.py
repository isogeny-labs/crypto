#!/usr/bin/env python3
"""Plots analysis.json -> fasthash.svg / fasthash.png. usage: plot.py"""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
H = os.path.dirname(os.path.abspath(__file__))
a = json.load(open(os.path.join(H, "analysis.json")))
QS = [262111, 524287, 1048507, 2097091, 4194271]
ks = [18, 19, 20, 21, 22]
col = {"HASH": "#1f77b4", "COMBO": "#d95f02"}
fig, ax = plt.subplots(1, 3, figsize=(13, 4))
for X in ("HASH", "COMBO"):
    for s, mk in zip((1, 2, 3), "ovs"):
        ax[0].plot(ks, [a["primary_relations_cpu_ratio"][f"{X}/{q}/{s}"]["mean"] for q in QS], mk, color=col[X], alpha=.7,
                   label=f"{X} seed {s}" if s == 1 else None)
    ax[0].plot(ks, [a["relations_ratio_geomean_over_seeds"][f"{X}/{q}"] for q in QS], "-", color=col[X])
    ax[1].plot(ks, [a["total_cpu_ratio"][f"{X}/{q}"]["geomean"] for q in QS], "o-", color=col[X], label=X)
    ax[2].plot(ks, [a["rss_ratio"][f"{X}/{q}"]["geomean"] for q in QS], "o-", color=col[X], label=X)
fl = a["aa_floor"]
ax[0].axhspan(1 - fl, 1 + fl, color="grey", alpha=.25, label=f"A/A floor ±{fl*100:.1f}%")
for i, t in enumerate(["relations.cpu / REF (mean of 2 passes)", "total CPU / REF (geomean)", "peak RSS / REF (geomean)"]):
    ax[i].axhline(1, color="k", lw=.7); ax[i].set_title(t, fontsize=10); ax[i].set_xlabel("k = log2 q"); ax[i].set_xticks(ks)
    ax[i].legend(fontsize=7); ax[i].grid(alpha=.3)
fig.suptitle("Plane-quartic IC, c_D = 1.7: open-addressing hash (HASH) and hash + compiled sqrt (COMBO) vs v3  [pcc:unverified, L0]", fontsize=10)
fig.tight_layout()
for e in ("svg", "png"):
    fig.savefig(os.path.join(H, f"fasthash.{e}"), dpi=130)
