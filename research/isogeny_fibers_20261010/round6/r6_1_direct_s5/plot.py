"""Plot recorded R6.1 values (no computation): S by arm and n from the
end-to-end runs, and word XORs per relation by arm and n from the m = 4 panel."""
import csv, glob, json, os, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
res = os.path.join(here, "results")
rows = []
for f in sorted(glob.glob(os.path.join(res, "e2e_*.json"))):
    d = json.load(open(f))
    for r in d["rows"]:
        if "S" in r:
            rows.append(dict(kind="e2e", n=d["n"], a=d["a"], arm=r["arm"], value=r["S"],
                             relations=r["relations_found"], trials=r["targets_tried"],
                             verified=r["verified"]))
for f in sorted(glob.glob(os.path.join(res, "panel_m4_*.json"))):
    d = json.load(open(f))
    for c in d["cells"]:
        if c.get("refused") is None and c["found"] > 0:
            rows.append(dict(kind="panel_xor_per_rel", n=d["n"], a=d["a"], arm=c["arm"],
                             value=c["xor_per_relation"], relations=c["found"],
                             trials=c["attempts"], verified=c["gate_failures"] == 0))
with open(os.path.join(here, "plot_data.csv"), "w", newline="") as fh:
    w = csv.DictWriter(fh, fieldnames=list(rows[0].keys()))
    w.writeheader(); w.writerows(rows)

fig, axes = plt.subplots(1, 2, figsize=(11, 4.2))
styles = {"mitm-m2": ("#1f77b4", "o", "mitm m=2 (control)"),
          "chained-m3": ("#2ca02c", "s", "chained-x m=3 (plug-in)"),
          "chained-m4": ("#9467bd", "D", "chained-x m=4 (plug-in)"),
          "sym-s5": ("#d62728", "^", "direct sym S5 m=4"),
          "x-chained": ("#9467bd", "D", "chained-x m=4 (plug-in)"),
          "sym-chained": ("#8c564b", "v", "chained sym m=4 (control)"),
          "x-s5": ("#ff7f0e", "P", "direct x S5 m=4")}
for ax, kind, ylabel, title in [
        (axes[0], "e2e", "S = total GAE / sqrt(r)", "End to end (run_pipeline), log recovered"),
        (axes[1], "panel_xor_per_rel", "word XORs per relation", "m = 4 panel, 24 targets, budget 4096")]:
    for arm, (col, mk, lab) in styles.items():
        pts = sorted((r["n"], r["value"]) for r in rows if r["kind"] == kind and r["arm"] == arm)
        if pts:
            ax.plot([p[0] for p in pts], [p[1] for p in pts], marker=mk, color=col, label=lab)
    ax.set_yscale("log"); ax.set_xlabel("n (toy Koblitz curve over F_2^n)")
    ax.set_ylabel(ylabel); ax.set_title(title, fontsize=10); ax.grid(alpha=0.3)
    ax.set_xticks([13, 17, 19, 23]); ax.legend(fontsize=7)
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(os.path.join(here, f"s_by_arm.{ext}"), dpi=150)
print(len(rows), "rows")
