"""Lay out recorded values from ANALYSIS.json (native n83_chain_sat_analyze).
Plotting only. usage: plot.py <ANALYSIS.json> <out_dir>"""
import json
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

a = json.loads(Path(sys.argv[1]).read_text())
out = Path(sys.argv[2])
out.mkdir(parents=True, exist_ok=True)
MS = sorted({c["m"] for c in a["cells"]})
cell = {(c["m"], c["arm"], c["kind"]): c for c in a["cells"]}
base = {m: next(c["arm"] for c in a["cells"] if c["m"] == m and not c["arm"].endswith("+rebuild")) for m in MS}
arm = {"control": lambda m: base[m], "rebuild": lambda m: base[m] + "+rebuild"}
LABEL = {"control": "A4 lookup + SAT (merged, control)", "rebuild": "A5 lookup + rebuild"}
COLOR = {"control": "#888888", "rebuild": "#1f77b4"}
ticks = [f"m={m}\nd={base[m].split('meet')[1]}" for m in MS]

fig, axes = plt.subplots(1, 2, figsize=(12.5, 4.6))
w = 0.35
for i, key in enumerate(["control", "rebuild"]):
    ys = [cell.get((m, arm[key](m), "planted"), {}).get("verified", 0) for m in MS]
    axes[0].bar([x + (i - 0.5) * w for x in MS], ys, w, label=LABEL[key], color=COLOR[key])
axes[0].set_xticks(MS, ticks)
axes[0].set_ylim(0, 6.9)
axes[0].set_title("planted targets: verified witnesses (of 5)", fontsize=9)
axes[0].legend(fontsize=7, loc="upper center", ncol=2)
axes[0].grid(alpha=0.3, axis="y")
ax = axes[1]
for key in ("control", "rebuild"):
    for kind, ls, mk in (("planted", "-", "^"), ("public", ":", "o")):
        ys = [cell.get((m, arm[key](m), kind), {}).get("median_seconds_all") for m in MS]
        ax.plot(MS, ys, ls=ls, marker=mk, color=COLOR[key], label=f"{LABEL[key]} · {kind}")
ax.axhline(120, color="k", ls=":", lw=1)
ax.text(MS[0], 128, "120 s per-query cap", fontsize=7)
ax.set_yscale("log")
ax.set_xticks(MS, ticks)
ax.set_ylabel("median seconds per query (L0 host, capped)")
ax.set_title("query cost (public verdicts identical: 3/3 refuted by both)", fontsize=9)
ax.legend(fontsize=6.5)
ax.grid(alpha=0.3)
fig.suptitle("Chained-S3 on N83 (icv1-f2m83-t6151469093347-cdcc5432): witnesses rebuilt from the lookup hit — run n83_chain_sat_rebuild_20261101", fontsize=10)
fig.tight_layout()
fig.savefig(out / "n83_chain_sat_rebuild_m5_8.svg")
fig.savefig(out / "n83_chain_sat_rebuild_m5_8.png", dpi=150)
print("ok")
