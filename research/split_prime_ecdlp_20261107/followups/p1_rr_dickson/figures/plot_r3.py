#!/usr/bin/env python3
"""Plots the recorded values of the T24 m5 t4 cell across round 2 card B and round 3, read from
analysis/derived_r3/decision_r3.json (p1r3analyze). Hatched = capped or timed-out job (lower bound).
Reads and draws only; computes nothing."""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
H = os.path.dirname(os.path.abspath(__file__))
d = json.load(open(os.path.join(H, "..", "analysis", "derived_r3", "decision_r3.json")))
fig, ax = plt.subplots(1, 2, figsize=(12, 4.8))
labels = {"rr": "RR (Riemann–Roch)", "t": "T (cherry tree)"}
for k, (col, lab) in enumerate((("D", "F4 solving degree D reached"), ("widest", "widest F4 matrix reached (columns)"))):
    x = 0
    ticks, names = [], []
    for i, s in enumerate(("rr", "t")):
        for a in d[s]["attempts"]:
            src = a["source"]
            rnd = "R2b (13 GiB, 3 h)" if "run-r2b" in src else "R3 (56–60 GiB, 24 h)"
            v = a[col]
            b = ax[k].bar(x, v, 0.8, color=f"C{i}", label=labels[s] if (k == 0 and rnd.startswith("R2b")) or (k == 1 and rnd.startswith("R2b")) else None)
            if a["status"] != "ok":
                b[0].set_hatch("//"); b[0].set_edgecolor("k")
            ticks.append(x); names.append(f"{s.upper()} {rnd}\n{a['status']}")
            x += 1
        x += 0.6
    ax[k].set_xticks(ticks); ax[k].set_xticklabels(names, fontsize=7); ax[k].set_ylabel(lab)
    if k == 1: ax[k].set_yscale("log")
    ax[k].legend(fontsize=8)
ax[0].set_title("D24 m5 t4, seed 2: every bar is a lower bound (no job completed)")
ax[1].set_title("Widest matrix: T's bound rose from 662,876 to 1,305,345 at 56 GiB")
fig.suptitle(f"P1 round 3 (PROTOCOL-R3): verdict under frozen rule 3 = {d['verdict']}. RR a1–a3 on EKS were lost to Spot interruptions and supply no bound.", fontsize=9)
fig.tight_layout()
for ext, kw in (("svg", {}), ("png", {"dpi": 150})): fig.savefig(os.path.join(H, "p1_r3_m5t4." + ext), **kw)
print("ok")
