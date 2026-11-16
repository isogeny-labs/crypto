"""Lay out recorded and jq-derived values (analysis/*.json) as SVG + 150 dpi PNG.

Plotting only: every number comes from analysis/A-1k-8k.json and
analysis/B-16k.json, which scripts/analyze.jq derives from the session records.
"""
import json
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

here = Path(__file__).resolve().parent.parent
an = [json.loads((here / "analysis" / f).read_text()) for f in ("A-1k-8k.json", "B-16k.json")]
out = here / "figures"
out.mkdir(exist_ok=True)
sids = " / ".join(a["session_id"] for a in an)

arms = {}
for a in an:
    for s in a["arms"]:
        arms[s["arm"]] = s
sizes = sorted({int(k.split("-")[1]) for k in arms if k.startswith("dense-")})
COL = {"dense": "#c0504d", "cado": "#4f81bd"}
LAB = {"dense": "incremental-gauss-full-rank (dense)", "cado": "sparse-cado (filter + block Wiedemann)"}


def save(fig, name):
    fig.tight_layout()
    fig.savefig(out / f"{name}.svg")
    fig.savefig(out / f"{name}.png", dpi=150)
    plt.close(fig)


# 1. Linear-algebra wall time per run against N.
fig, ax = plt.subplots(figsize=(6.4, 4.2))
for kind in ("dense", "cado"):
    xs = [n for n in sizes if f"{kind}-{n}" in arms]
    ys = [arms[f"{kind}-{n}"]["wall_ms_mean"]["linear_algebra"] / 1e3 for n in xs]
    ax.plot(xs, ys, "o-", color=COL[kind], label=LAB[kind])
    for n, y in zip(xs, ys):
        ax.annotate(f"{y:.2g} s", (n, y), textcoords="offset points", xytext=(4, -10 if kind == "cado" else 5), fontsize=7)
ax.set_xscale("log", base=2)
ax.set_yscale("log")
ax.set_xlabel("factor-base columns N")
ax.set_ylabel("linear-algebra wall time per run (s, mean)")
ax.set_title("Relation matrix inside ic.pipeline, icv1-fp32-t7319-90da43e9", fontsize=9)
ax.legend(fontsize=8)
ax.grid(alpha=0.3, which="both")
fig.text(0.01, 0.005, f"Sessions {sids}. Means over 40 runs (N ≤ 8192) and 8 runs (N = 16384). Isolation L1 (hypervisor steal): descriptive.", fontsize=6)
save(fig, "la_wall_vs_n")

# 2. row_ops (counted multiply-subtracts) against N.
fig, ax = plt.subplots(figsize=(6.4, 4.2))
for kind in ("dense", "cado"):
    xs = [n for n in sizes if f"{kind}-{n}" in arms]
    ys = [arms[f"{kind}-{n}"]["row_ops"]["median"] for n in xs]
    ax.plot(xs, ys, "o-", color=COL[kind], label=LAB[kind])
ax.set_xscale("log", base=2)
ax.set_yscale("log")
ax.set_xlabel("factor-base columns N")
ax.set_ylabel("row_ops per run (median; unpriced in S)")
ax.set_title("Counted linear-algebra work, same relation stream per N", fontsize=9)
ax.legend(fontsize=8)
ax.grid(alpha=0.3, which="both")
fig.text(0.01, 0.005, f"Sessions {sids}. row_ops = entry multiply-subtracts; the dense method's column scans are not counted.", fontsize=6)
save(fig, "row_ops_vs_n")

# 3. Where each run's wall time goes.
fig, ax = plt.subplots(figsize=(7.2, 4.2))
labels, bottoms = [], []
phases = [("oracle_setup", "oracle set-up (|F|² pair table)", "#9bbb59"), ("relations", "relations", "#f79646"), ("linear_algebra", "linear algebra", "#8064a2")]
names = [f"{k}-{n}" for n in sizes for k in ("dense", "cado") if f"{k}-{n}" in arms]
x = range(len(names))
base = [0.0] * len(names)
for key, lab, col in phases:
    vals = [arms[nm]["wall_ms_mean"][key] / 1e3 for nm in names]
    ax.bar(x, vals, bottom=base, color=col, label=lab)
    base = [b + v for b, v in zip(base, vals)]
ax.set_xticks(list(x))
ax.set_xticklabels(names, rotation=45, ha="right", fontsize=7)
ax.set_ylabel("wall time per run (s, mean)")
ax.set_title("Phase wall time per arm (stacked)", fontsize=9)
ax.legend(fontsize=7)
fig.text(0.01, 0.005, f"Sessions {sids}. Isolation L1: descriptive.", fontsize=6)
save(fig, "phase_wall")
print("wrote", sorted(p.name for p in out.iterdir()), file=sys.stderr)
