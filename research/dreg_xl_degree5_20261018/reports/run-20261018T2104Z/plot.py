# Lays out recorded values only (runs/registered-*.jsonl, runs/soundness-*.jsonl).
import json, glob, sys
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt
rows = []
for f in sorted(glob.glob("runs/registered-*.jsonl") + glob.glob("runs/soundness-*.jsonl")):
    for l in open(f):
        r = json.loads(l)
        rows.append(r)
fig, ax = plt.subplots(figsize=(8, 4.2))
labels, vals, cols = [], [], []
for r in rows:
    reg = r["role"] == "registered"
    labels.append(f'{r["cell"][1:3]},{r["ell"]} d{r["draw"]}' + ("" if reg else "*"))
    vals.append(100 * r["rows_done"] / r["rows_dense"])
    cols.append("#1f77b4" if r["n_vars"] == 40 else "#d62728")
    if not reg:
        cols[-1] = "#aec7e8" if r["n_vars"] == 40 else "#ff9896"
ax.bar(range(len(vals)), vals, color=cols)
ax.set_xticks(range(len(vals)))
ax.set_xticklabels(labels, rotation=70, ha="right", fontsize=8)
ax.set_ylabel("rows inserted at the cap (%)")
ax.set_ylim(0, 100)
ax.set_title("Plain XL, degree 5: every run stopped at the pivot cap before a verdict")
fig.text(0.01, 0.005, "blue N=40, red N=42; light bars and * = satisfiable draws (soundness). Source: runs/*.jsonl, run-20261018T2104Z. n=15 runs.", fontsize=7)
fig.tight_layout(rect=(0, 0.03, 1, 1))
fig.savefig(sys.argv[1] + "/xl_degree5_cap.svg")
fig.savefig(sys.argv[1] + "/xl_degree5_cap.png", dpi=130)
