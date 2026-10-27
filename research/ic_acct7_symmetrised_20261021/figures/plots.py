#!/usr/bin/env python3
"""Figures for research/ic_acct7_symmetrised_20261021. Plots recorded values only.

Reads audit/n17-work-counted.jsonl (`ecbench work --counted`), the session's
records.jsonl (for seq, round and recovered scalar, joined on record_id),
audit/n17-work-medians.json (the per-arm medians, taken by jq from the work
lines; see the README) and audit/n17-audit-ir-repriced.jsonl (`ecbench
audit-ir`'s Callgrind Ir joined by jq with the repriced gate counts). Computes
no result: Ir per gate is the recorded `ir_per_kilo_bitop_repriced` / 1000, and
the drawn median lines are the recorded medians.

    /tmp/report-venv/bin/python research/ic_acct7_symmetrised_20261021/figures/plots.py
"""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402
from matplotlib.lines import Line2D  # noqa: E402

HERE = Path(__file__).resolve().parent
D = HERE.parent
BLUE, ORANGE, INK, MUTED = "#2a78d6", "#eb6834", "#333333", "#6b6b6b"
plt.rcParams.update({"font.size": 9, "axes.spines.top": False, "axes.spines.right": False,
                     "axes.grid": True, "grid.color": "#e4e4e4", "grid.linewidth": 0.6,
                     "svg.fonttype": "none"})
ARMS = {"ic-symmetrised-d9": ("IC  ic-symmetrised-d9", BLUE), "rho-frob": ("rho  rho-frob (reference)", ORANGE)}


def jl(p):
    return [json.loads(x) for x in p.read_text().splitlines() if x.strip()]


def save(fig, name):
    fig.tight_layout()
    fig.savefig(HERE / f"{name}.svg", bbox_inches="tight")
    fig.savefig(HERE / f"{name}.png", dpi=150, bbox_inches="tight")
    plt.close(fig)


rec = {r["record_id"]: r for r in jl(D / "sessions" / "n17-symmetrised" / "records.jsonl")}
work = jl(D / "audit" / "n17-work-counted.jsonl")
medians = {m["arm"]: m for m in json.loads((D / "audit" / "n17-work-medians.json").read_text())["arms"]}
assert len(work) == 16
rows = []
for w in work:
    r = rec[w["record_id"]]
    ok = w["work"]["Ok"]
    assert not r["warmup"] and w["status"] == "verified" and ok["unpriced"] == [] and ok["known_uncounted"] == []
    rows.append((r["round"], int(r["outcome"]["recovered"]), w["arm"], int(ok["bitops"]), r["seq"]))
targets = sorted({k for _, k, _, _, _ in rows})
assert len(targets) == 4

# 1. Per-record bitop/v1, IC vs rho, by round and target.
slots = [(rd, k) for rd in (1, 2) for k in targets]
fig, ax = plt.subplots(figsize=(7.0, 3.9))
med = {}
for arm, (label, col) in ARMS.items():
    pts = sorted((slots.index((rd, k)), b, s) for rd, k, a, b, s in rows if a == arm)
    assert len(pts) == 8
    ax.plot([p[0] for p in pts], [p[1] for p in pts], "o", ms=6, color=col, mec="white", mew=0.8,
            label=f"{label}, 8 counted records")
    assert medians[arm]["records"] == 8
    med[arm] = medians[arm]["median_bitops"]
    ax.axhline(med[arm], color=col, lw=1, ls="--")
    ax.annotate(f"median {med[arm]:.3e}", (0.43, med[arm]), xycoords=ax.get_yaxis_transform(), xytext=(0, 4),
                textcoords="offset points", ha="right", fontsize=7.5, color=INK)
ax.set_yscale("log")
ax.set_ylim(5e5, 1e10)
ax.set_xticks(range(len(slots)), [f"r{rd}\nk={k}" for rd, k in slots])
ax.axvline(3.5, color=MUTED, lw=0.6)
ax.grid(axis="x", visible=False)
ax.set_xlabel("round and recovered target scalar (4 public targets, K1 / F_2^17)")
ax.set_ylabel("bitop/v1 per record (log scale)")
ax.set_title("Symmetrised IC vs rho-frob, every counted record priced complete (ECBS1h6fd757ff9d9f)",
             fontsize=9)
ax.legend(fontsize=7, frameon=False, loc="upper center", bbox_to_anchor=(0.5, -0.3), ncol=2)
save(fig, "bitops_per_record")

# 2. Instructions per counted gate, the four audit-ir re-executions.
ir = sorted(jl(D / "audit" / "n17-audit-ir-repriced.jsonl"), key=lambda r: r["seq"])
assert [r["seq"] for r in ir] == [8, 9, 10, 11]
arm_of = {r["seq"]: r["arm"] for r in rec.values()}
fig, ax = plt.subplots(figsize=(6.4, 2.6))
order = [r for r in ir if arm_of[r["seq"]] == "ic-symmetrised-d9"] + \
        [r for r in ir if arm_of[r["seq"]] == "rho-frob"]
for i, r in enumerate(order):
    y = len(order) - 1 - i
    v = r["ir_per_kilo_bitop_repriced"] / 1000
    col = ARMS[arm_of[r["seq"]]][1]
    ax.plot([0, v], [y, y], color=col, lw=1.2)
    ax.plot(v, y, "o", ms=6, color=col)
    ax.annotate(f"{v:.3f}  (Ir {r['solve_ir']:,} / {int(r['bitops_repriced']):,} gates)", (v, y), xytext=(6, 0),
                textcoords="offset points", va="center", fontsize=7.5, color=INK)
ax.set_yticks(range(len(order)), [f"seq {r['seq']} · {arm_of[r['seq']]}" for r in order][::-1])
ax.set_xlim(0, 3.2)
ax.grid(axis="y", visible=False)
ax.set_xlabel("Callgrind instructions (Ir) per counted bitop/v1 gate")
ax.set_title("ecbench audit-ir: instructions per counted gate, 2 records per arm", fontsize=9)
ax.legend(handles=[Line2D([], [], ls="", marker="o", ms=6, color=c, label=l) for l, c in ARMS.values()],
          fontsize=7, frameon=False, loc="lower right")
save(fig, "ir_per_gate")
print("figures written")
