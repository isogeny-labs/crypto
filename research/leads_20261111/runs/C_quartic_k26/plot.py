#!/usr/bin/env python3
"""DL cost against q: committed k = 12..22 means (summary.json, M4 host), this host's k = 17..24 units,
the calibrated rho-neg model, and the k = 25, 26 projections. Writes fig_dl_scaling.{png,svg}."""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

D = os.path.dirname(os.path.abspath(__file__))
res = json.load(open(os.path.join(D, "results.json")))
summ = [json.loads(l) for l in open(os.path.join(D, "../../../quartic_n4v2_20261020/results/summary.json")) if l.strip()]

fig, ax = plt.subplots(figsize=(7.2, 4.6))
cq = [r["q"] for r in summ if r.get("kind") == "ic_size" and r["arm"] == "DL"]
cm = [r["meq"][0] for r in summ if r.get("kind") == "ic_size" and r["arm"] == "DL"]
ax.plot(cq, cm, "o", mfc="none", color="#7a7a7a", label="DL, committed means (k=8..22, M4 host)")
for arm, col in [("DL", "#1f5fa8"), ("DL1", "#2f9e6e")]:
    xs, ys = [], []
    for key, v in res["per_size"].items():
        a, q = key.split(":")
        if a == arm:
            for m in v["w0"]:
                xs.append(int(q))
                ys.append(m)
    ax.plot(xs, ys, ".", color=col, ms=7, label=f"{arm}, this run (units, k=17..25)")
rq = [r["q"] for r in res["rho_ratio"]]
rm = [r["rho_model"]["w0"] for r in res["rho_ratio"]]
pj = [p for p in res["projections"] if p["from"] == "k22-25"]
ax.plot(rq + [p["q"] for p in pj], rm + [p["rho_model_w0"] for p in pj], "-", color="#b5462f", label="rho-neg, calibrated model (this host GAE)")
ax.plot([p["q"] for p in pj], [p["dl_meq_w0"] for p in pj], "x", color="#1f5fa8", ms=9, label="DL projection k=26 (k=22..25 fit)")
ax.axvline(2**22, color="#cccccc", lw=0.8, ls="--")
ax.text(2**22 * 1.05, ax.get_ylim()[0] * 3 if ax.get_ylim()[0] > 0 else 1e7, "ell > 2^64\n(Barrett)", fontsize=8, color="#555555")
ax.set_xscale("log", base=2)
ax.set_yscale("log")
ax.set_xlabel("q")
ax.set_ylabel("M-eq (F_q multiplications), w = 0")
ax.set_title("N4v2 DL past the 64-bit group-order wall [pcc:unverified]", fontsize=10)
ax.legend(fontsize=7.5, loc="upper left", frameon=False)
ax.grid(True, which="major", color="#eeeeee")
fig.tight_layout()
for ext in ("png", "svg"):
    fig.savefig(os.path.join(D, f"fig_dl_scaling.{ext}"), dpi=150)
print("wrote fig_dl_scaling.png/.svg")
