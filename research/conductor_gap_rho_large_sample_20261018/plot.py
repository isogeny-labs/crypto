#!/usr/bin/env python3
"""Graphs of recorded values for research/conductor_gap_rho_large_sample_20261018.

Reads sessions/*/records.jsonl (cost.s per verified record) and results/summary.json
(ratios computed by analyze.py). Writes figures/*.svg and 150-dpi PNG from the same
source. Plotting only; no new computation beyond means and the ratios already saved.
Run with /tmp/report-venv/bin/python (matplotlib).
"""
import json, os, re, statistics as st
from collections import defaultdict
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = os.path.dirname(os.path.abspath(__file__))
SESS = os.path.join(HERE, "sessions"); FIG = os.path.join(HERE, "figures"); os.makedirs(FIG, exist_ok=True)
summary = json.load(open(os.path.join(HERE, "results", "summary.json")))

def roles_of(spec):
    return [r.strip() for r in spec["description"].split("Curve roles, in order:")[1].rstrip(".").split(",")]

def save(fig, name):
    fig.savefig(os.path.join(FIG, name + ".svg")); fig.savefig(os.path.join(FIG, name + ".png"), dpi=150); fig.savefig(os.path.join(FIG, name + ".pdf")); plt.close(fig)

# ---- Figure 1: every floor/crater and twin/original ratio with 95% bootstrap interval, arm rho-neg
rows = []
for name, ses in sorted(summary["sessions"].items()):
    for k, v in ses["arms"].get("rho-neg", {}).items():
        kind = "twin / original" if "twin" in k else "floor / crater"
        rows.append((name, k, kind, v["ratio"], v["ci95"], v["n_other"]))
fig, ax = plt.subplots(figsize=(9, max(4, 0.28 * len(rows) + 1.5)))
ys = range(len(rows))
for y, (name, k, kind, r, (lo, hi), n) in zip(ys, rows):
    c = "#1f77b4" if kind == "floor / crater" else "#7f7f7f"
    ax.errorbar(r, y, xerr=[[r - lo], [hi - r]], fmt="o", color=c, capsize=2, ms=4)
ax.set_yticks(list(ys)); ax.set_yticklabels([f"{n}: {k.split(' / ')[0]} (n={nn})" for n, k, _, _, _, nn in rows], fontsize=7)
ax.axvline(1, color="k", lw=0.8); ax.invert_yaxis()
ax.set_xlabel("ratio of mean S (charged group ops / sqrt r), 95% bootstrap over targets")
ax.set_title("rho.negation: floor/crater (blue) and isomorphic twin/original (grey), every session", fontsize=10)
fig.tight_layout(); save(fig, "ratios_rho_neg")

# ---- Figure 2: pooled ratios per family / tier
pooled = summary["pooled"]
keys = sorted(pooled)
fig, ax = plt.subplots(figsize=(9, 0.32 * len(keys) + 1.6))
for i, k in enumerate(keys):
    v = pooled[k]
    ax.errorbar(v["ratio"], i, xerr=[[v["ratio"] - v["ci95"][0]], [v["ci95"][1] - v["ratio"]]], fmt="s", color="#1f77b4", capsize=3)
    ax.errorbar(v["paired_geomean_ratio"], i + 0.3, xerr=[[v["paired_geomean_ratio"] - v["paired_ci95"][0]], [v["paired_ci95"][1] - v["paired_geomean_ratio"]]], fmt="^", color="#ff7f0e", capsize=3)
ax.set_yticks([i + 0.15 for i in range(len(keys))]); ax.set_yticklabels([f"{k} (n={pooled[k]['n_floor']})" for k in keys], fontsize=7)
ax.axvline(1, color="k", lw=0.8); ax.invert_yaxis()
ax.set_xlabel("pooled floor/crater: ratio of means (squares) and paired geometric mean (triangles), 95%")
ax.set_title("Pooled ratios by tier/family (floor/crater unless the key names a twin comparison), arm rho.negation", fontsize=9)
fig.tight_layout(); save(fig, "pooled_ratios")

# ---- Figure 3: distribution of S on crater vs floors, s1 shards pooled and l pooled (histograms of recorded values)
def collect(prefix):
    out = defaultdict(list)
    for name in sorted(os.listdir(SESS)):
        d = os.path.join(SESS, name)
        if not name.startswith(prefix) or not os.path.isfile(f"{d}/records.jsonl"):
            continue
        spec = json.load(open(f"{d}/spec.json")); roles = roles_of(spec)
        key = lambda c: json.dumps(c, sort_keys=True); role_of = {key(c): roles[i] for i, c in enumerate(spec["workloads"]["curves"])}
        for line in open(f"{d}/records.jsonl"):
            r = json.loads(line)
            if r["warmup"] or r["outcome"]["status"] != "verified" or r["arm"] != "rho-neg":
                continue
            role = role_of[key(r["workload"]["curve_spec"])]
            lvl = "twin" if "twin" in role else ("crater" if role.endswith("crater") else "floor")
            out[lvl].append(r["cost"]["s"])
    return out
fig, axes = plt.subplots(1, 4, figsize=(15, 3.6))
for ax, (prefix, title) in zip(axes, [("s1", "S1: ell=8209, q 29 bits (hard gap)"), ("m", "M: ell~2^20.2, q 45 bits (hard gap)"), ("l", "L + replication: ell~2^25, 53 bits"), ("t", "T: twin round, 53 bits")]):
    data = collect(prefix)
    bins = 40
    for lvl, c in [("crater", "#d62728"), ("floor", "#1f77b4"), ("twin", "#7f7f7f")]:
        if data.get(lvl):
            ax.hist(data[lvl], bins=bins, range=(0, 4), histtype="step", density=True, color=c, lw=1.4,
                    label=f"{lvl} (n={len(data[lvl])}, mean {st.mean(data[lvl]):.3f})")
    ax.set_title(title, fontsize=9); ax.set_xlabel("S per solve (rho.negation)"); ax.legend(fontsize=7)
axes[0].set_ylabel("density of recorded values")
fig.suptitle("Distribution of recorded S: crater vs floor vs isomorphic twin", fontsize=10)
fig.tight_layout(); save(fig, "s_distributions")

# ---- Figure 4: controls (positive sqrt2 and A/A) in S1 shards and S2
fig, ax = plt.subplots(figsize=(7, 3))
items = []
for name, ses in sorted(summary["sessions"].items()):
    for arm, lab, c in [("rho-plain", "rho-plain / rho-neg (expect 1.414)", "#2ca02c"), ("rho-neg-aa", "rho-neg-aa / rho-neg (expect 1)", "#9467bd")]:
        for k, v in ses["arms"].get(arm, {}).items():
            if k.startswith(arm + " /"):
                items.append((f"{name}: {lab}", v["ratio"], v["ci95"], c))
for i, (lab, r, (lo, hi), c) in enumerate(items):
    ax.errorbar(r, i, xerr=[[r - lo], [hi - r]], fmt="o", color=c, capsize=2, ms=4)
ax.set_yticks(range(len(items))); ax.set_yticklabels([x[0] for x in items], fontsize=7)
ax.axvline(1, color="k", lw=0.8); ax.axvline(2 ** 0.5, color="#2ca02c", lw=0.8, ls="--"); ax.invert_yaxis()
ax.set_xlabel("ratio of mean S on the crater, 95% bootstrap"); ax.set_title("Controls", fontsize=10)
fig.tight_layout(); save(fig, "controls")
print("figures written:", sorted(os.listdir(FIG)))
