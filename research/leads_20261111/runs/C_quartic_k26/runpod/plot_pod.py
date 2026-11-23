#!/usr/bin/env python3
"""Pod run: DL / DL1 unit costs (M-eq w=0) against q, the rho-neg model, and the k = 22..26 DL fit;
second panel: per-size calibration constants across the 4 rounds. Writes fig_pod.{png,svg}."""
import json
import math
import os
import statistics as stt

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

D = os.path.dirname(os.path.abspath(__file__))
res = json.load(open(os.path.join(D, "results_pod.json")))
fig, (ax, bx) = plt.subplots(1, 2, figsize=(11, 4.4))
for arm, col in [("DL", "#1f5fa8"), ("DL1", "#2f9e6e")]:
    xs, ys = [], []
    for key, v in res["per_size"].items():
        a, q = key.split(":")
        if a == arm:
            xs += [int(q)] * len(v["w0"]); ys += v["w0"]
    ax.plot(xs, ys, ".", color=col, ms=7, label=f"{arm} units (pod, one binary)")
f = [x for x in res["fits"] if x["arm"] == "DL" and x["range"] == "k22-26" and x["measure"] == "w0"][0]
qs = [4194271, 67108819]
m22 = stt.mean(res["per_size"]["DL:4194271"]["w0"])
m26 = stt.mean(res["per_size"]["DL:67108819"]["w0"])
gm = math.exp((math.log(m22) + math.log(m26)) / 2)
ax.plot(qs, [m22, m26], ":", color="#1f5fa8", lw=0.8)
rows = res["rho_model"]["rows"]
rq = [r["q"] for r in rows]
rm = [stt.mean(res["per_size"][f"DL:{r['q']}"]["w0"]) / r["w0"][0] for r in rows]
ax.plot(rq, rm, "-", color="#b5462f", label="rho-neg model (c = 0.9175)")
ax.axvline(2**22.5, color="#cccccc", lw=0.8, ls="--")
ax.text(2**22.6, 3e7, "ell > 2^64", fontsize=8, color="#555555")
ax.set_xscale("log", base=2); ax.set_yscale("log")
ax.set_xlabel("q"); ax.set_ylabel("M-eq, w = 0")
ax.set_title(f"DL k=22..26: e = {f['e']:.3f} [{f['lo']:.3f}, {f['hi']:.3f}] [pcc:unverified]", fontsize=9)
ax.legend(fontsize=7.5, frameon=False, loc="upper left"); ax.grid(True, color="#eeeeee")
for key, mk, col in [(0, "o", "#7a4fa8"), (1, "s", "#c07a1a")]:
    for q, v in res["calibration"].items():
        rr = v["rounds"] or {}
        k = round(math.log2(int(q)))
        bx.plot([k] * len(rr), [x[key] / v["pooled"][key] for x in rr.values()], mk, mfc="none", color=col, ms=5)
bx.plot([], [], "o", mfc="none", color="#7a4fa8", label="r_inv, round median / pooled")
bx.plot([], [], "s", mfc="none", color="#c07a1a", label="r_lmul, round median / pooled")
bx.set_xlabel("k"); bx.set_ylabel("ratio"); bx.set_title("Calibration rounds on the pod (4 rounds)", fontsize=9)
bx.legend(fontsize=7.5, frameon=False); bx.grid(True, color="#eeeeee")
fig.tight_layout()
for ext in ("png", "svg"):
    fig.savefig(os.path.join(D, f"fig_pod.{ext}"), dpi=140)
print("wrote fig_pod")
