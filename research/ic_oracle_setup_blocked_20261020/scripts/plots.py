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
sizes = sorted({int(k.split("-")[1]) for k in arms if k.startswith("blocked-") and not k.endswith("-aa")})
KINDS = ("scalar", "batch", "blocked")
COL = {"scalar": "#c0504d", "batch": "#f79646", "blocked": "#4f81bd"}
LAB = {
    "scalar": "mitm:negation_folded=1 (scalar adds, one map)",
    "batch": "mitm-prime-batch (batched inversion, one map)",
    "blocked": "mitm-prime-blocked (batched, packed regions)",
}
NOTE = f"Sessions {sids}. Means over 40 runs (N ≤ 8192) and 8 runs (N = 16384). IC isolation L1 (hypervisor steal): descriptive."


def save(fig, name):
    fig.tight_layout()
    fig.savefig(out / f"{name}.svg")
    fig.savefig(out / f"{name}.png", dpi=150)
    plt.close(fig)


def series(kind, f):
    xs = [n for n in sizes if f"{kind}-{n}" in arms]
    return xs, [f(arms[f"{kind}-{n}"]) for n in xs]


# 1. Oracle set-up wall time per run against N.
fig, ax = plt.subplots(figsize=(6.4, 4.2))
for kind in KINDS:
    xs, ys = series(kind, lambda a: a["wall_ms_mean"]["oracle_setup"] / 1e3)
    ax.plot(xs, ys, "o-", color=COL[kind], label=LAB[kind])
    for n, y in zip(xs, ys):
        ax.annotate(f"{y:.2g} s", (n, y), textcoords="offset points", xytext=(4, -10 if kind == "blocked" else 5), fontsize=7)
ax.set_xscale("log", base=2)
ax.set_yscale("log")
ax.set_xlabel("factor-base abscissae N")
ax.set_ylabel("oracle set-up wall time per run (s, mean)")
ax.set_title("Pair-table set-up inside ic.pipeline, icv1-fp32-t7319-90da43e9", fontsize=9)
ax.legend(fontsize=7)
ax.grid(alpha=0.3, which="both")
fig.text(0.01, 0.005, NOTE, fontsize=5.5)
save(fig, "setup_wall_vs_n")

# 2. Set-up wall per counted table addition (one per folded pair, about N²).
fig, ax = plt.subplots(figsize=(6.4, 4.2))
for kind in KINDS:
    xs, ys = series(kind, lambda a: a["wall_ms_mean"]["oracle_setup"] * 1e6 / a["setup_gae_mean"])
    ax.plot(xs, ys, "o-", color=COL[kind], label=LAB[kind])
ax.set_xscale("log", base=2)
ax.set_xlabel("factor-base abscissae N")
ax.set_ylabel("set-up wall per table addition (ns)")
ax.set_title("Cost of one table entry; every arm counts the same additions", fontsize=9)
ax.legend(fontsize=7)
ax.grid(alpha=0.3, which="both")
ax.set_ylim(bottom=0)
fig.text(0.01, 0.005, NOTE, fontsize=5.5)
save(fig, "setup_ns_per_entry")

# 3. Where each run's wall time goes.
fig, ax = plt.subplots(figsize=(7.6, 4.2))
phases = [("oracle_setup", "oracle set-up (pair table)", "#9bbb59"), ("relations", "relations", "#f79646"), ("linear_algebra", "linear algebra", "#8064a2")]
names = [f"{k}-{n}" for n in sizes for k in KINDS if f"{k}-{n}" in arms]
x = range(len(names))
base = [0.0] * len(names)
for key, lab, col in phases:
    vals = [arms[nm]["wall_ms_mean"][key] / 1e3 for nm in names]
    ax.bar(x, vals, bottom=base, color=col, label=lab)
    base = [b + v for b, v in zip(base, vals)]
ax.set_xticks(list(x))
ax.set_xticklabels(names, rotation=45, ha="right", fontsize=7)
ax.set_yscale("log")
ax.set_ylabel("wall time per run (s, mean; log scale)")
ax.set_title("Phase wall time per arm (stacked)", fontsize=9)
ax.legend(fontsize=7)
fig.text(0.01, 0.005, NOTE, fontsize=5.5)
save(fig, "phase_wall")

# 4. Peak resident memory per run.
fig, ax = plt.subplots(figsize=(6.4, 4.2))
for kind in KINDS:
    xs, ys = series(kind, lambda a: a["max_rss_kib"]["max"] / 2**20)
    ax.plot(xs, ys, "o-", color=COL[kind], label=LAB[kind])
ax.set_xscale("log", base=2)
ax.set_yscale("log")
ax.set_xlabel("factor-base abscissae N")
ax.set_ylabel("peak resident memory per process (GiB, max over runs)")
ax.set_title("Whole-process peak RSS (not isolated table memory)", fontsize=9)
ax.legend(fontsize=7)
ax.grid(alpha=0.3, which="both")
fig.text(0.01, 0.005, f"Sessions {sids}.", fontsize=5.5)
save(fig, "peak_rss")
print("wrote", sorted(p.name for p in out.iterdir()), file=sys.stderr)
