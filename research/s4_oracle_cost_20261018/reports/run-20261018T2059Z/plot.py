"""Plots recorded values only (no computation of results). usage: plot.py RUNS OUT RUN_ID"""
import glob, json, sys
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
runs, out, rid = sys.argv[1:4]
recs = [json.loads(l) for f in sorted(glob.glob(runs + "/cell-*.jsonl")) for l in open(f) if l.strip()]
cells = [(12,3),(13,4),(16,5),(18,6)]
col = {"chained": "#c0392b", "eliminated": "#1f6fb4"}
fig, ax = plt.subplots(1, 2, figsize=(10, 4))
for arm, dx in (("chained", -0.08), ("eliminated", 0.08)):
    for i, (n, l) in enumerate(cells):
        rs = [r for r in recs if r["arm"] == arm and r.get("rep") == 1 and (r.get("n"), r.get("ell")) == (n, l) or (r["arm"] == arm and r.get("rep") == 1 and r.get("cell") == f"{n}:{l}")]
        ok = [r for r in rs if r.get("verdict") == "verified"]
        bad = [r for r in rs if r.get("verdict") != "verified"]
        ax[0].scatter([i+dx]*len(ok), [r["word_xors"] for r in ok], color=col[arm], label=arm if i == 0 else None, s=22)
        ax[1].scatter([i+dx]*len(ok), [r["solving_degree"] for r in ok], color=col[arm], s=22)
        if bad:
            ax[0].scatter([i+dx]*len(bad), [1e11]*len(bad), color=col[arm], marker="x", s=30)
            ax[0].annotate(f"{len(bad)} unmeasured", (i+dx, 2.5e11 if arm == "chained" else 6e11), fontsize=7, ha="center", color=col[arm])
ax[0].set_yscale("log"); ax[0].set_ylabel("F4 word_xors (64-bit XORs)"); ax[0].set_ylim(1e3, 1e13)
ax[1].set_ylabel("F4 solving degree")
for a in ax:
    a.set_xticks(range(len(cells))); a.set_xticklabels([f"({n},{l})" for n, l in cells]); a.set_xlabel("cell (n, l)")
ax[0].legend()
ax[0].set_title("cost per draw (x = unmeasured)"); ax[1].set_title("solving degree per draw")
fig.suptitle(f"m=3 oracle, chained S3 vs eliminated S4, n=4 draws/cell, {rid}", fontsize=9)
fig.tight_layout(); fig.savefig(out + "/oracle_cost.svg"); fig.savefig(out + "/oracle_cost.png", dpi=150)
