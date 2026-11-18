"""Figures for the f4-model follow-up.  [pcc:unverified]"""
import json, os, statistics
from collections import defaultdict
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
HERE = os.path.dirname(os.path.abspath(__file__)); OUT = os.path.join(HERE, "..", "out")
rs = [json.loads(l) for l in open(os.path.join(OUT, "cells.jsonl"))]
rs = [r for r in rs if r["ms"]["status"] == "ok"]
C = {2: "#1f77b4", 3: "#d62728", 4: "#2ca02c"}
fig, ax = plt.subplots(1, 3, figsize=(16, 4.8))
g = defaultdict(lambda: defaultdict(list))
for r in rs:
    if r["family"] in ("uni", "interval"): g[r["m"]][r["size"]].append(r)
for m in (2, 3, 4):
    ds = sorted(g[m])
    med = lambda f: [statistics.median([f(c) for c in g[m][d] if f(c) is not None] or [float("nan")]) for d in ds]
    ax[0].plot(ds, med(lambda c: c["ms"]["D_F4"]), "o-", color=C[m], label=f"F4 max degree, m={m}")
    ax[0].plot(ds, med(lambda c: c["xl"].get("D_XL")), "s--", color=C[m], alpha=.6, label=f"exact XL degree, m={m}")
    ax[0].plot(ds, [m * 2 ** (m - 1) + max(0, d - 2 ** (m - 1)) for d in ds], ":", color="k", lw=.8)
    ax[1].loglog(ds, med(lambda c: c["ms"]["sum_nnz"]), "o-", color=C[m], label=f"F4 sum nnz, m={m}")
    ax[1].loglog(ds, med(lambda c: c["ms"]["rows_reduced"]), "^:", color=C[m], label=f"F4 rows reduced, m={m}")
    ax[1].loglog(ds, med(lambda c: c["xl"].get("XL_cols")), "s--", color=C[m], alpha=.5, label=f"XL columns, m={m}")
    ax[1].loglog(ds, [d ** (m - 1) for d in ds], "-", color=C[m], lw=.6, alpha=.4)
ax[0].set_xlabel("base degree d (uni + interval)"); ax[0].set_ylabel("degree")
ax[0].set_title("F4 degree = first fall degree (dotted) vs exact XL degree"); ax[0].legend(fontsize=7)
ax[1].set_xlabel("d"); ax[1].set_title("per-attempt cost proxies (thin line: d^(m-1))"); ax[1].legend(fontsize=6)
gg = defaultdict(lambda: defaultdict(list))
for r in rs:
    if r["family"] in ("graph2", "graphr", "bits"):
        s = 2 ** r["size"] if r["family"] == "bits" else r["size"]
        gg[(r["family"], r["m"])][s].append(r)
for (fam, m), dd in sorted(gg.items()):
    ss = sorted(dd)
    ax[2].loglog(ss, [statistics.median([c["ms"]["sum_nnz"] for c in dd[s]]) for s in ss], "o-", label=f"{fam} m={m} nnz")
    ax[2].loglog(ss, [statistics.median([c["ms"]["rows_reduced"] for c in dd[s]]) for s in ss], "^:", label=f"{fam} m={m} rows red.")
ax[2].set_xlabel("|T| (graph) or 2^s (bits)"); ax[2].set_title("aux-variable presentations"); ax[2].legend(fontsize=6)
fig.text(0.5, 0.005, "f4-model follow-up 2026-11-15, msolve 0.6.5 F4, p in {1021,4093,65521,1048573}, prime-order curves; all [pcc:unverified]", ha="center", fontsize=8)
plt.tight_layout(rect=(0, 0.03, 1, 1))
for ext in ("svg", "png", "pdf"):
    plt.savefig(os.path.join(OUT, f"f4_vs_xl.{ext}"), dpi=130)
print("ok")
