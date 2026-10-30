"""Plots the recorded per-cell D (min/median/max over random targets, draw 0) from raw/summary-r2.tsv's
source rows; reads raw/*.jsonl minus the two unreceipted first-run files. Layout only, no new result."""
import json, glob, statistics as st, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
fs = [f for f in sorted(glob.glob("raw/*.jsonl")) if not f.split("/")[-1].startswith(("card-02-aa-", "card-04-e-cell3-second"))]
rows = [json.loads(l) for f in fs for l in open(f) if l.startswith("{")]
cells = [(17,4,3),(19,4,3),(13,3,4),(15,3,4),(17,4,4)]
arms = ["a","c","d0","d","e","b"]
col = {"a":"#6b7280","c":"#9ca3af","d0":"#2563eb","d":"#1d4ed8","e":"#d97706","b":"#059669"}
data = {}
fig, ax = plt.subplots(figsize=(11,5.2))
w = 0.13
for ci, c in enumerate(cells):
    for ai, arm in enumerate(arms):
        rs = [r for r in rows if (r["n"],r["l"],r["m"])==c and r["arm"]==arm and r["kind"]=="random" and r["draw"]==0]
        ex = [r["D"] for r in rs if r["D"] is not None]
        lb = [r["D_lb"] for r in rs if r["D"] is None]
        x = ci + (ai-2.5)*w
        key = f"{c}|{arm}"
        if len(ex) == len(rs) and rs:
            md = st.median(ex); ax.bar(x, md, w, color=col[arm], label=arm if ci==0 else None)
            ax.plot([x,x],[min(ex),max(ex)], color="k", lw=1)
            data[key] = {"n":len(rs),"exact":len(ex),"min":min(ex),"median":md,"max":max(ex)}
        elif rs:
            v = max(lb) if lb else None
            if ex: v = max(v or 0, max(ex))
            ax.bar(x, v, w, color="white", edgecolor=col[arm], hatch="///", label=arm if ci==0 else None)
            data[key] = {"n":len(rs),"exact":len(ex),"lower_bound":v}
        else:
            ax.text(x, 0.2, "x", ha="center", color=col[arm]); data[key] = {"n":0}
ax.set_xticks(range(5)); ax.set_xticklabels([f"cell {i+1}\n(n,l,m)={c}" for i,c in enumerate(cells)])
ax.set_ylabel("closure degree D (random targets, draw 0)")
ax.set_title("Blind spot 04, revised run: median D per cell and arm (bars; whisker = min-max over targets)\n"
             "hatched = certified lower bound or partial targets (not an exact median); x = unmeasured. Frozen verdict: ADVANCES (primitive), arm d", fontsize=10)
ax.legend(ncol=6, loc="upper left", title="arm")
ax.set_ylim(0, 15)
fig.text(0.01, 0.005, "Source: raw/*.jsonl of the receipted cards (first run + -r2), summary in raw/summary-r2.txt; instrument sha256 9648dce3...; toy cells, host L0.", fontsize=7)
fig.tight_layout(rect=(0,0.02,1,1))
fig.savefig("median_D_r2.svg"); fig.savefig("median_D_r2.png", dpi=150)
json.dump(data, open("median_D_r2.data.json","w"), indent=1)
