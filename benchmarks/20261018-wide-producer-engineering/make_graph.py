#!/usr/bin/env python3
"""Graph and markdown tables from the run_ab.py JSON files in this directory."""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
load = lambda n: json.load(open(os.path.join(here, n))) if os.path.exists(os.path.join(here, n)) else None
scan = {"K=40": load("scan_k40.json"), "K=300": load("scan_k300.json"), "K=600": load("scan_k600.json")}
pairs = load("scan_k300_pairs.json")
idx = {"K=300": load("index_k300.json"), "K=600": load("index_k600.json")}

lines = []
def table(title, doc, unit, key):
    lines.append(f"\n**{title}**\n")
    lines.append(f"| arm | median {unit} | min-max | ratio to first arm | repeats |")
    lines.append("|:--|--:|:--|--:|--:|")
    arms = list(doc["summary"])
    base = doc["summary"][arms[0]]["median"]
    for a in arms:
        s = doc["summary"][a]
        lines.append(f"| {a} | {s['median']:.2f} | {s['min']:.2f}-{s['max']:.2f} | {s['median']/base:.3f} | {s['n']} |")

for k, d in scan.items():
    if d:
        table(f"Scan, n = 73, {k}, {d['states_cap']} states, ns per probe", d, "ns/probe", "ns_per_probe")
if pairs:
    table("Scan, n = 73, K=300 (second run: pair arms), ns per probe", pairs, "ns/probe", "ns_per_probe")
for k, d in idx.items():
    if d:
        table(f"Index build, n = 73, {k}, seconds", d, "s", "total_s")
open(os.path.join(here, "tables.md"), "w").write("\n".join(lines) + "\n")

fig, axes = plt.subplots(1, 3, figsize=(14, 4.2))
colors = {"control": "#7f7f7f", "lazy_canon": "#1f77b4", "blocked_bloom": "#ff7f0e", "prefetch": "#2ca02c",
          "all_three": "#d62728", "serial": "#7f7f7f", "batch_inv_1thread": "#1f77b4",
          "threads_only": "#2ca02c", "threads_batch_inv": "#d62728"}
for ax, (k, d) in zip(axes[:2], [(k, d) for k, d in scan.items() if d and k != "K=40"]):
    arms = list(d["summary"])
    for i, a in enumerate(arms):
        s = d["summary"][a]
        ax.bar(i, s["median"], color=colors.get(a, "#555"), yerr=[[s["median"]-s["min"]], [s["max"]-s["median"]]], capsize=3)
    ax.set_xticks(range(len(arms))); ax.set_xticklabels(arms, rotation=30, ha="right")
    ax.set_ylabel("ns per probe (median, min-max)"); ax.set_title(f"Scan, n = 73, {k}")
ax = axes[2]
width = 0.35
ks = [k for k, d in idx.items() if d]
for j, k in enumerate(ks):
    d = idx[k]
    arms = list(d["summary"])
    for i, a in enumerate(arms):
        s = d["summary"][a]
        ax.bar(j * 6 + i, s["median"], color=colors.get(a, "#555"), yerr=[[s["median"]-s["min"]], [s["max"]-s["median"]]], capsize=3,
               label=a if j == 0 else None)
ax.set_xticks([j * 6 + 1 for j in range(len(ks))]); ax.set_xticklabels(ks)
ax.set_ylabel("index build seconds (median, min-max)"); ax.set_title("build_index128, n = 73"); ax.legend(fontsize=8)
fig.suptitle("Wide producer engineering, unisolated 4-vCPU guest, same binary, interleaved A/B", fontsize=10)
fig.tight_layout()
for ext in ("png", "svg", "pdf"):
    fig.savefig(os.path.join(here, f"wide_producer_engineering.{ext}"), dpi=140)
print("\n".join(lines))
