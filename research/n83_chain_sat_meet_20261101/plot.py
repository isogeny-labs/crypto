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
CONTROL = "all-summands+t2.2"
cell = {(c["m"], c["arm"], c["kind"]): c for c in a["cells"]}
meet_arm = {m: next(c["arm"] for c in a["cells"] if c["m"] == m and "meet" in c["arm"]) for m in MS}
arm_of = {"control": lambda m: CONTROL, "lookup": lambda m: meet_arm[m]}
LABEL = {"control": "A2 order + depth-2 tables (control)", "lookup": "A4 A2 + meet lookup (d per m)"}
COLOR = {"control": "#888888", "lookup": "#1f77b4"}

fig, axes = plt.subplots(1, 3, figsize=(15, 4.6))
w = 0.35
for i, key in enumerate(["control", "lookup"]):
    for ax, kind, field in ((axes[0], "public", "refuted"), (axes[1], "planted", "verified")):
        ys = [cell.get((m, arm_of[key](m), kind), {}).get(field, 0) for m in MS]
        ax.bar([x + (i - 0.5) * w for x in MS], ys, w, label=LABEL[key], color=COLOR[key])
axes[0].set_title("public [a]G refuted (of 3)", fontsize=9)
axes[1].set_title("planted: verified witnesses (of 2)", fontsize=9)
for ax in axes[:2]:
    ax.set_xticks(MS, [f"m={m}\nd={meet_arm[m].split('meet')[1]}" for m in MS])
    ax.set_ylim(0, 3.3)
    ax.grid(alpha=0.3, axis="y")
axes[0].legend(fontsize=7, loc="upper right")
ax = axes[2]
for kind, marker, color in (("public", "o", "#d62728"), ("planted", "^", "#2ca02c")):
    xs, ys = [], []
    for m in MS:
        for d in cell.get((m, meet_arm[m], kind), {}).get("meet_domains", []):
            if d is not None:
                xs.append(m + (0.08 if kind == "planted" else -0.08))
                ys.append(d + 0.5)
    ax.scatter(xs, ys, marker=marker, color=color, label=f"{kind} targets", alpha=0.8)
ax.set_yscale("log")
ax.set_xticks(MS)
ax.set_xlabel("summands m")
ax.set_ylabel("lookup domain size of y_(d-1) (0 drawn at 0.5)")
ax.set_title("lookup domain: empty on every public, non-empty on every planted", fontsize=9)
ax.legend(fontsize=7)
ax.grid(alpha=0.3)
fig.suptitle("Chained-S3 SAT on N83 (icv1-f2m83-t6151469093347-cdcc5432), m = 5…9, meet-in-the-middle lookup — run n83_chain_sat_meet_20261101", fontsize=10)
fig.tight_layout()
fig.savefig(out / "n83_chain_sat_meet_m5_9.svg")
fig.savefig(out / "n83_chain_sat_meet_m5_9.png", dpi=150)

fig, ax = plt.subplots(figsize=(7.5, 4.6))
for key, ls in (("control", ":"), ("lookup", "-")):
    ys = [cell.get((m, arm_of[key](m), "public"), {}).get("median_seconds_all") for m in MS]
    ax.plot(MS, ys, ls=ls, marker="o", color=COLOR[key], label=f"{LABEL[key]}: median public query s")
warm = {}
for w_ in a.get("warmups", []):
    warm.setdefault(w_["meet_depth"], w_["seconds"])
ax.plot(MS, [warm.get(int(meet_arm[m].split("meet")[1])) for m in MS], ls="--", marker="s", color="#9467bd",
        label="lookup set build, once per base (warmup s)")
ax.axhline(120, color="k", ls=":", lw=1)
ax.text(MS[0], 130, "120 s per-query cap", fontsize=7)
ax.set_yscale("log")
ax.set_xticks(MS)
ax.set_xlabel("summands m")
ax.set_ylabel("seconds (L0 host)")
ax.set_title("Cost — run n83_chain_sat_meet_20261101", fontsize=9)
ax.legend(fontsize=7)
ax.grid(alpha=0.3)
fig.tight_layout()
fig.savefig(out / "n83_chain_sat_meet_cost.svg")
fig.savefig(out / "n83_chain_sat_meet_cost.png", dpi=150)
print("ok")
