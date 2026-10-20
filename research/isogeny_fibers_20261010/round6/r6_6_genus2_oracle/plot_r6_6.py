"""Plot recorded R6.6 census summaries (reads raw/*.json written by
examples/r6_6_genus2_oracle.rs; no computation beyond reading them).

Writes plot_data.json (the plotted rows) and
  measured_vs_predicted.svg : measured/predicted ratio per arm, curve, p
                               (unknowns gain, class factors, costs)
  oracle_cost.svg           : counted F_p multiplications per Nagao m = 2 call,
                               and oracle calls per relation, per arm
"""
import glob
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
rows = []
for f in sorted(glob.glob(os.path.join(HERE, "raw", "census_*.json"))):
    d = json.load(open(f))
    for key, group in (("summary_j", "J"), ("summary_jprime", "J'")):
        for a in d[key]:
            rows.append(dict(file=os.path.basename(f), p=d["p"], b=d["b"], family=d["family"],
                             h=d["validation"]["h"], r=d["validation"]["r"], size=d["size"],
                             group=group, arm=a["arm"], reference=a["reference"],
                             n_reps=a["n_reps"], predicted=a["predicted"],
                             **{k: dict(value=a[k]["value"], ci95=a[k]["ci95"]) for k in
                                ("unknowns_gain", "columns_ratio", "c2", "c2_strict", "c3", "c3_strict",
                                 "cost2_nominal", "cost3_nominal", "hit_rate2", "hit_rate3",
                                 "oracle2_muls_per_call", "oracle2_calls_per_relation",
                                 "oracle2_sampled_hit_rate", "oracle_cost2_ratio")},
                             oracle2_disagreements=a["oracle2_disagreements"],
                             oracle3_disagreements=a["oracle3_disagreements"],
                             oracle3_targets=a["oracle3_targets"]))
json.dump(rows, open(os.path.join(HERE, "plot_data.json"), "w"), indent=1)

colors = {1051: "#4C78A8", 2131: "#F58518", 4111: "#54A24B"}
markers = {80: "o", 100: "s", 160: "D"}
panels = [("unknowns_gain", 5, "unknowns gain |F| / U  (A: 10)"),
          ("c2_strict", 1, "class factor c, m = 2 (distinct summands)"),
          ("c3", 2, "class factor c, m = 3"),
          ("cost3_nominal", 4, "derived cost, m = 3 (nominal reference columns)")]
labels = []
for r in rows:
    lab = f"{r['arm']}\n{r['family']} {r['group']} |F|={r['size']}"
    if lab not in labels:
        labels.append(lab)
fig, axes = plt.subplots(2, 2, figsize=(12, 9), sharey=True)
for ax, (key, idx, title) in zip(axes.flat, panels):
    seen = set()
    for r in rows:
        lab = f"{r['arm']}\n{r['family']} {r['group']} |F|={r['size']}"
        x = labels.index(lab)
        pred = r["predicted"][idx]
        m = r[key]["value"] / pred
        lo, hi = (v / pred for v in r[key]["ci95"])
        off = {1051: -0.22, 2131: 0.0, 4111: 0.22}[r["p"]]
        lbl = f"p = {r['p']}" if r["p"] not in seen else None
        seen.add(r["p"])
        ax.errorbar(x + off, m, yerr=[[max(m - lo, 0)], [max(hi - m, 0)]], fmt=markers.get(r["size"], "o"),
                    color=colors[r["p"]], ms=5, capsize=2, label=lbl)
    ax.axhline(1.0, color="black", lw=0.8)
    ax.set_yscale("log")
    ax.set_ylim(0.5, 2.0)
    ax.set_xticks(range(len(labels)))
    ax.set_xticklabels(labels, fontsize=6.5)
    ax.set_title(title, fontsize=10)
    ax.set_ylabel("measured / predicted (95% bootstrap over bases)")
    ax.grid(axis="y", alpha=0.3)
axes[0, 0].legend(fontsize=8)
fig.suptitle("R6.6: the affine law under the Nagao-style degree-1 oracle in genus 2 — measured / predicted\n"
             f"y² = x⁵ + b over F_p, {rows[0]['n_reps'] if rows else '?'} random bases per cell; "
             "A: ζ₅-closed base; R: degree-1 base in H₄ = φ̂(J′(F_p)) (Richelot, index 4); G: base in the order-r subgroup (index h); "
             "φ_plain/φ_A: Richelot image bases on J′ against a plain degree-1 base on C′", fontsize=9)
fig.tight_layout()
fig.savefig(os.path.join(HERE, "measured_vs_predicted.svg"))
fig.savefig(os.path.join(HERE, "measured_vs_predicted.png"), dpi=150)

# oracle cost figure
orows = [r for r in rows if r["group"] == "J"]
fig, axes = plt.subplots(1, 2, figsize=(12, 4.5))
olabels = []
for r in orows:
    lab = f"{r['arm']}\n{r['family']} |F|={r['size']}"
    if lab not in olabels:
        olabels.append(lab)
for ax, key, title in zip(axes, ("oracle2_muls_per_call", "oracle2_calls_per_relation"),
                          ("counted F_p multiplications per Nagao m = 2 oracle call", "oracle calls per relation (r / D₂, exact)")):
    seen = set()
    for r in orows:
        lab = f"{r['arm']}\n{r['family']} |F|={r['size']}"
        x = olabels.index(lab)
        off = {1051: -0.22, 2131: 0.0, 4111: 0.22}[r["p"]]
        v = r[key]["value"]
        lo, hi = r[key]["ci95"]
        lbl = f"p = {r['p']}" if r["p"] not in seen else None
        seen.add(r["p"])
        ax.errorbar(x + off, v, yerr=[[max(v - lo, 0)], [max(hi - v, 0)]], fmt=markers.get(r["size"], "o"),
                    color=colors[r["p"]], ms=5, capsize=2, label=lbl)
    ax.set_xticks(range(len(olabels)))
    ax.set_xticklabels(olabels, fontsize=7)
    ax.set_title(title, fontsize=10)
    ax.grid(axis="y", alpha=0.3)
axes[1].set_yscale("log")
axes[0].legend(fontsize=8)
fig.suptitle("R6.6: Nagao m = 2 oracle, counted field operations (means over bases, 95% bootstrap); "
             "the plain reference is included at its own value", fontsize=9)
fig.tight_layout()
fig.savefig(os.path.join(HERE, "oracle_cost.svg"))
fig.savefig(os.path.join(HERE, "oracle_cost.png"), dpi=150)
print(f"{len(rows)} summary rows plotted")
