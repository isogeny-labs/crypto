"""Lay out recorded values from ANALYSIS.json (written by the native
examples/n83_msweep_analyze.rs). Plotting only: no value is computed here
beyond log scaling for display."""
import json
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import Patch

# usage: plot.py <ANALYSIS.json> <out_dir>
a = json.loads(Path(sys.argv[1]).read_text())
out = Path(sys.argv[2])
cells = a["cells"]
MS = list(range(2, 10))
ALG = ["f4-matrix", "f5-matrix", "f4-inherited", "f6-ic", "f6-ic-pair", "crossbred",
       "sat-native", "sat-cnf", "sat-restrict", "mq-fes"]
CHAIN = [("chain-s3", "union:2"), ("chain-s3", "union:3"), ("chain-s3", "union:4"),
         ("chain-s3-orbit", "union:2"), ("chain-s3-orbit-ordered", "union:2")]
COL = {"decided": "#2b8a3e", "witness": "#1c7ed6", "incomplete": "#f08c00",
       "unsupported": "#adb5bd", "process_cap": "#c92a2a", "missing": "#ffffff"}


def cat(c):
    if c is None:
        return "missing"
    if c["status"] == "process_cap":
        return "process_cap"
    o = c.get("outcomes") or {}
    if o.get("witness") and c["probes"] == "planted":
        return "witness"
    if (c.get("decided_fraction") or 0) >= 0.9:
        return "decided"
    if o.get("unsupported"):
        return "unsupported"
    return "incomplete"


def find(curve, m, fb, engine, probes):
    for c in cells:
        if (c["curve"], c["m"], c["factor_base"], c["engine"], c["probes"]) == (curve, m, fb, engine, probes):
            return c
    return None


def save(fig, name):
    fig.savefig(out / f"{name}.svg")
    fig.savefig(out / f"{name}.png", dpi=150)
    plt.close(fig)


out.mkdir(parents=True, exist_ok=True)

# 1. Status map at n = 83: what each engine did at each m (public probes; algebraic rows are planted).
rows = [("n83k1", "subspace:8", e, "planted") for e in ALG] + [("n83k1", fb, e, p) for e, fb in CHAIN for p in ("public", "planted")]
fig, ax = plt.subplots(figsize=(10, 9))
for i, (cv, fb, e, p) in enumerate(rows):
    for j, m in enumerate(MS):
        c = find(cv, m, fb, e, p)
        k = cat(c)
        ax.add_patch(plt.Rectangle((j, i), 1, 1, color=COL[k], ec="white"))
        txt = ""
        if c and c["status"] == "process_cap":
            txt = ">180s"
        elif c and c.get("seconds_per_trial") is not None:
            s = c["seconds_per_trial"]
            txt = f"{s:.2g}s" if s >= 0.01 else f"{s*1e3:.1g}ms"
            if c["probes"] == "planted" and c["trials"]:
                txt += f"\n{c['verified_relations']}/{c['trials']}"
        ax.text(j + 0.5, i + 0.5, txt, ha="center", va="center", fontsize=6.5,
                color="white" if k in ("decided", "witness", "process_cap") else "black")
ax.set_xlim(0, len(MS)); ax.set_ylim(len(rows), 0)
ax.set_xticks([j + 0.5 for j in range(len(MS))], [f"m={m}" for m in MS])
ax.set_yticks([i + 0.5 for i in range(len(rows))], [f"{e} · {fb} · {p}" for _, fb, e, p in rows], fontsize=7)
ax.legend(handles=[Patch(color=COL[k], label=l) for k, l in [
    ("decided", "≥90% of queries decided (UNSAT/witness)"), ("witness", "planted witnesses recovered (verified/planted)"),
    ("incomplete", "incomplete (resource preflight / budget)"), ("unsupported", "refused: field > 64 bits (fail-closed)"),
    ("process_cap", "process killed at 180 s cap")]], loc="upper center", bbox_to_anchor=(0.5, -0.04), ncol=2, fontsize=7)
ax.set_title("N83 K1 (icv1-f2m83-t6151469093347-cdcc5432): decomposition engines × m\n"
             "cell text = mean seconds per query (planted: verified/planted); run n83_msweep_20261024", fontsize=9)
fig.tight_layout(); save(fig, "n83_status_map")

# 2. Projected relation-collection time at n = 83 against m, with rho and the m=4 rootfinder.
fig, ax = plt.subplots(figsize=(8, 5))
mk = {"chain-s3": "o", "chain-s3-orbit": "s", "chain-s3-orbit-ordered": "^"}
plotted = False
for e, fb in CHAIN:
    xs, ys = [], []
    for m in MS:
        c = find("n83k1", m, fb, e, "public")
        if c and c.get("log2_projected_collection_seconds") is not None:
            xs.append(m); ys.append(c["log2_projected_collection_seconds"])
    if xs:
        plotted = True
        ax.plot(xs, ys, marker=mk[e], label=f"{e} · {fb} (projected, heuristic p)")
rho_s = a["rho"]["mean_online_seconds"]
import math
if rho_s:
    ax.axhline(math.log2(rho_s), color="k", ls="--", label=f"strong rho, mean of {a['rho']['verified_runs']} verified runs ({rho_s:.2f} s)")
for r in a["rootfinder_reference"]:
    if r["process_wall_seconds"]:
        ax.scatter([4], [math.log2(r["process_wall_seconds"])], marker="*", s=160, zorder=5,
                   label=f"{r['tag']}: m=4 rootfinder, complete DLP process ({r['process_wall_seconds']:.0f} s)")
ax.set_xticks(MS); ax.set_xlabel("summands m"); ax.set_ylabel("log2 seconds")
ax.set_title("N83 K1: projected collection time of decided cells vs matched rho\n"
             "(undecided cells have no price; see status map) — run n83_msweep_20261024", fontsize=9)
ax.legend(fontsize=7); ax.grid(alpha=0.3)
fig.tight_layout(); save(fig, "n83_projected_vs_m")

# 3. Companion n = 61 (exact one-word field): seconds per query, m = 2, by engine and subspace dimension.
fig, ax = plt.subplots(figsize=(10, 4.8))
dims = ["subspace:8", "subspace:12", "subspace:16", "subspace:20"]
w = 0.2
for k, fb in enumerate(dims):
    ys = []
    for e in ALG:
        c = find("k1n61", 2, fb, e, "public")
        ys.append(c["seconds_per_trial"] if c and c.get("seconds_per_trial") and cat(c) == "decided" else float("nan"))
    ax.bar([i + (k - 1.5) * w for i in range(len(ALG))], ys, w, label=fb)
ax.set_yscale("log"); ax.set_xticks(range(len(ALG)), ALG, rotation=30, fontsize=8)
ax.set_ylabel("mean seconds per decided query (16 public probes)")
ax.set_title("Companion K1/F_2^61, m = 2: engine × factor-base dimension (all queries proved UNSAT;\n"
             "m ≥ 3 refused by the 64-variable descent mask) — run n83_msweep_20261024", fontsize=9)
ax.legend(fontsize=8); ax.grid(alpha=0.3, axis="y")
fig.tight_layout(); save(fig, "k1n61_engines_m2")
print("ok", plotted)
