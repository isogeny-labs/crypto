#!/usr/bin/env python3
"""Plot recorded native audit fields only; no research arithmetic or solving."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
data = json.loads((root / "analysis.json").read_text())
plt.rcParams.update({"font.size": 9, "svg.fonttype": "none",
                     "svg.hashsalt": "factor-base-oracle-leads-20261019"})
colors = ["#314e69", "#b05a35", "#41816a"]
labels = {"product-membership":"Product", "asymmetric-branches":"Asymmetric",
          "selector-membership":"Selector", "direct-lift-base":"Direct lift",
          "torsion-quotient":"Quotient"}

for metric, title, ylabel in [
    ("verified", "Verified positive controls", "Returned, checked decompositions"),
    ("field_reduction_multiplications_completed_calls", "F4 reduction work on complete positive panels", "F2 reduction multiplications (solver counter)")
]:
    fig, axes = plt.subplots(2, 3, figsize=(11.2, 6.2), constrained_layout=True)
    for ax, case in zip(axes.flat, data["cases"]):
        rows = [(key.split("/",1)[1], value) for key,value in case["aggregates"].items()
                if key.startswith("positive-control/") and not key.endswith("reference-repeat")]
        for j, (arm, row) in enumerate(rows):
            complete = row["verified"] == row["trials"] and not row["partial_counters"]
            if metric == "verified" or complete:
                ax.bar(j, row[metric], color=colors[j], width=.65)
                text = f'{row[metric]:,}'
                if metric == "verified": text += f'/{row["trials"]}'
                ax.annotate(text, (j,row[metric]), xytext=(0,4), textcoords="offset points", ha="center", fontsize=8)
            else:
                ax.text(j,.12,"incomplete\npanel",transform=ax.get_xaxis_transform(),ha="center",fontsize=8,color="#8d3d32")
        ax.set_xticks(range(len(rows)), [labels[arm] for arm,_ in rows], fontsize=8)
        ax.set_xlim(-.6, len(rows)-.4)
        ax.set_title(case["id"] + "\n" + case["curve"], fontsize=8)
        ax.grid(axis="y",alpha=.2)
        ax.set_axisbelow(True)
        if metric == "verified": ax.set_ylim(0,5)
        else: ax.set_yscale("symlog",linthresh=1); ax.margins(y=.25)
        if ax.get_subplotspec().is_first_col(): ax.set_ylabel(ylabel)
    fig.suptitle(title+" — frozen oracle-leads panel, 2026-10-19",fontsize=13)
    fig.supxlabel("Conditioned positive controls; no total-cost or speed ranking. Missing panels are not zero work.",fontsize=9)
    stem = "positive-controls" if metric=="verified" else "f4-component-work"
    svg = root/f"{stem}.svg"
    fig.savefig(svg, metadata={"Date": None})
    svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines())+"\n")
    fig.savefig(root/f"{stem}.png",dpi=150)
    plt.close(fig)
