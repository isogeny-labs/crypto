#!/usr/bin/env python3
"""Plots recorded values of item 20 from raw/*.jsonl only (no computation of results).
Writes graph.svg and graph.png (150 dpi)."""
import json, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
L = lambda f: [json.loads(l) for l in open("raw/" + f)]
fig, ax = plt.subplots(1, 3, figsize=(15, 4.8))
# (a) product ratio vs A0 at n=256, band min..max
arms = [r for r in L("model.jsonl") if r["kind"] == "arm" and r["n"] == 256 and r["arm"] not in ("A0", "A0p") and not r["arm"].startswith("EH")]
names = [a["arm"] for a in arms][::-1]
for i, a in enumerate(arms[::-1]):
    lo, hi = a["ratio_product_min"], a["ratio_product_max"]
    col = {"ADVANCES": "#1b7837", "TRADES": "#b8860b", "MATCHES": "#555555", "REGRESSES": "#b2182b"}.get(a["verdict"], "#999")
    ax[0].plot([lo, hi], [i, i], color=col, lw=4, solid_capstyle="butt")
    ax[0].plot([lo, hi], [i, i], "|", color="k", ms=8)
ax[0].set_yticks(range(len(names))); ax[0].set_yticklabels(names, fontsize=8)
ax[0].set_xscale("log"); ax[0].axvline(1, color="k", lw=.8); ax[0].axvline(.95, color="k", ls=":", lw=.8); ax[0].axvline(1.05, color="k", ls=":", lw=.8)
ax[0].set_xlabel("qubits x expected Toffoli, ratio to A0 (log); bar = model band")
ax[0].set_title("(a) model rows, n=256 (P-256, secp256k1)\ngreen ADVANCES, amber TRADES, grey MATCHES, red REGRESSES\n(rule-level; overall verdict INCONCLUSIVE, see text)", fontsize=8)
# (b) positive control: S16 nu=0, b_eta=0, success vs b_delta
cells = [r for r in L("s16-nu0.jsonl") if r["kind"] == "single_cell" and r["b_eta"] == 0]
for sg, c in zip((0, 2, 4, 7), ("#d73027", "#fc8d59", "#91bfdb", "#4575b4")):
    rs = sorted([r for r in cells if r["sigma"] == sg], key=lambda r: r["b_delta"])
    x = [max(r["b_delta"], .5) for r in rs]
    ax[1].errorbar(x, [r["rate"] for r in rs], yerr=[[r["rate"] - r["ci_lo"] for r in rs], [r["ci_hi"] - r["rate"] for r in rs]], color=c, marker="o", ms=3, capsize=2, label=f"sigma={sg}")
ax[1].set_xscale("log"); ax[1].set_xlabel("B_Delta (0 plotted at 0.5)"); ax[1].set_ylabel("single-run success, Wilson 95%")
ax[1].set_title("(b) S16 nu=0, B_eta=0, m=16, 4000 runs/cell\npositive control: sigma=0,Bt=0 -> 0.601; sigma=7,B_D=100 -> 0.998", fontsize=8); ax[1].legend(fontsize=7); ax[1].set_ylim(.55, 1.01)
# (c) lattice cumulative success m=16
for f in ("lattice-m12.jsonl", "lattice-m16.jsonl", "lattice-m20.jsonl"):
    for s, ls in ((2, "-"), (3, "--")):
        rs = [r for r in L(f) if r["s"] == s and r["sigma"] == 4]
        if not rs: continue
        rs.sort(key=lambda r: r["runs"]); m = rs[0]["m"]
        ax[2].errorbar([r["runs"] for r in rs], [r["rate"] for r in rs], yerr=[[r["rate"] - r["ci_lo"] for r in rs], [r["ci_hi"] - r["rate"] for r in rs]], ls=ls, marker="o", ms=3, capsize=2, label=f"m={m}, s={s}")
ax[2].set_xlabel("runs used (lattice over subsets of size >= s)"); ax[2].set_ylabel("recovery rate, Wilson 95%")
ax[2].set_title("(c) lattice cells, sigma=4, 400 trials/cell", fontsize=8); ax[2].legend(fontsize=7)
fig.suptitle("Item 20, blind-spots run 20261018: classical processing of quantum ECDLP; toy groups (m<=20) and a resource model, no hardware claim", fontsize=9)
fig.tight_layout()
fig.savefig("graph.svg"); fig.savefig("graph.png", dpi=150)
