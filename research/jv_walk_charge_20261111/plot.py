#!/usr/bin/env python3
"""Left: reach (walk + transport + model) as a share of the validated arm-U S, per instance, against M.
Right: pooled charged/rho at M = 512 vs the uncharged validated ratio across the §5a band. Data: analysis.json + runs."""
import json, math, glob
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
a = json.load(open("analysis.json"))
V = "../jv_sieve_line_vs_rho_20261103/results/cover"
S = {}
for p in (1009, 1511, 1823):
    for r in json.load(open(f"{V}/U_{p}.json")):
        S[(p, r["seed"])] = r["total_muls"] / (331 * math.sqrt(r["l"]))
fig, (ax, bx) = plt.subplots(1, 2, figsize=(10, 4.2), gridspec_kw={"width_ratios": [1.4, 1]})
col = {1009: "#2471a3", 1511: "#c0392b", 1823: "#27ae60"}
off = {1009: -0.08, 1511: 0, 1823: 0.08}
for p in col:
    for i, m in enumerate((8, 32, 128, 512)):
        for s in range(17, 25):
            d = json.load(open(f"runs/p{p}_s{s}_m{m}.json")); d = d[0] if isinstance(d, list) else d
            reach = (d["walk_muls"] + d["transport_muls"] + d["model_muls"]) / (331 * math.sqrt(d["l"]))
            ax.plot(i + off[p], reach / S[(p, s)], "o", color=col[p], alpha=0.6, ms=4,
                    label=f"p = {p}" if (i == 0 and s == 17) else None)
ax.set_yscale("log"); ax.set_xticks(range(4)); ax.set_xticklabels(["8", "32", "128", "512"])
ax.axhline(1, color="#555", ls="--", lw=0.8); ax.text(3.3, 1.1, "walk = route", fontsize=7, ha="right")
ax.set_xlabel("M: random isogeny moves from the weak curve to the start"); ax.set_ylabel("reach cost / validated route cost (arm U)")
ax.legend(fontsize=8); ax.grid(alpha=0.3, which="both"); ax.set_title("Walk into the weak class, per instance", fontsize=9)
pool = a["by_M"]["512"]["pooled"]
ws = ["0", "0.1", "1"]; x = range(3)
ax2 = bx
ax2.errorbar([i - 0.1 for i in x], [pool[w]["point"] for w in ws],
             yerr=[[pool[w]["point"] - pool[w]["lo"] for w in ws], [pool[w]["hi"] - pool[w]["point"] for w in ws]],
             fmt="o", color="#c0392b", capsize=3, label="walk charged (M = 512)")
ax2.plot([i + 0.1 for i in x], [pool[w]["uncharged_point"] for w in ws], "s", color="#7f8c8d", label="validated, walk not charged")
ax2.axhline(1, color="#555", ls="--", lw=0.8); ax2.set_yscale("log"); ax2.set_ylim(0.005, 2)
ax2.set_xticks(list(x)); ax2.set_xticklabels([f"w = {w}" for w in ws]); ax2.set_ylabel("pooled cover / rho (U2, layer A)")
ax2.legend(fontsize=7.5, loc="upper left"); ax2.grid(alpha=0.3, which="both"); ax2.set_title("Distance to rho, 24 instances", fontsize=9)
fig.tight_layout(); fig.savefig("walk.svg"); fig.savefig("walk.png", dpi=200)
