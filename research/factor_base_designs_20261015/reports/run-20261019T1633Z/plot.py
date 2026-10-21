"""Plot recorded values for Amendment 5 (L1, L2) from runs/*.jsonl only: log2 field_ops
and max_cols against b at m = 4 for A0 and A11. Completed cells are F4's (through their
C1 replays), C2's A11 cell; L1 and L2 have no values and are drawn as limit markers.
F4's t = 4 timeouts recorded a partial max_cols, drawn as an open marker (a value
reached, not the cell's maximum). No fit is computed here.
Rendering aid only (AGENTS.md "no Python")."""
import json, math, os
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
runs = os.path.join(here, "..", "..", "runs")


def load(name):
    return [json.loads(l) for l in open(os.path.join(runs, name)) if l.strip()]


c1 = [r for r in load("C1-rescore.jsonl") if r["m"] == 4 and r.get("replay_of", "").startswith("F4-")]
c2 = load("C2-A0-m4-t4.jsonl") + load("C2-A11-m4-t4.jsonl")
f4 = load("F4-A0-m4.jsonl") + load("F4-A11-m4.jsonl")
l1, l2 = load("L1-A0-m4-t4.jsonl"), load("L2-A11-m4-t5.jsonl")
COL = {"A0": "#2a78d6", "A11": "#008300"}
MK = {"A0": "o", "A11": "P"}

plt.rcParams.update({"font.size": 8.5, "axes.edgecolor": "#888", "xtick.color": "#444", "ytick.color": "#444"})
fig, axes = plt.subplots(1, 2, figsize=(10.5, 4.2))
for ax, key, ylab in [(axes[0], "field_ops", "log2 field_ops"), (axes[1], "max_cols", "max_cols")]:
    for a in ["A0", "A11"]:
        rows = [r for r in c1 + c2 if r["arm"] == a and r["status"] == "ok" and r.get("trace_matches", True)]
        by = {}
        for r in rows:
            by.setdefault(r["b"], []).append(r[key])
        bs = sorted(by)
        val = (lambda v: math.log2(v)) if key == "field_ops" else (lambda v: v)
        # each system's recorded value
        ax.scatter([b for b in bs for _ in by[b]], [val(v) for b in bs for v in by[b]],
                   color=COL[a], marker=MK[a], s=26, zorder=3, edgecolors="white", linewidths=0.6)
        ax.plot(bs, [val(sum(by[b]) / len(by[b])) for b in bs], color=COL[a], lw=1.2,
                label=f"{a}: completed cells (2 systems each)")
        if key == "max_cols":
            part = [r for r in f4 if r["arm"] == a and r["status"] == "timeout" and r.get("max_cols")]
            if part and a == "A0":
                ax.scatter([r["b"] for r in part], [r["max_cols"] for r in part], facecolors="none",
                           edgecolors=COL[a], marker="o", s=40, zorder=3,
                           label="A0 t = 4: max_cols reached before F4's 300 s timeout (F4, partial)")
    ax.set_xlabel("b = log2|V| per summand (bits)")
    ax.set_ylabel(ylab + (" (per system; line = cell mean)" if key == "field_ops" else " (per system)"))
    if key == "max_cols":
        ax.set_yscale("log", base=2)
    ax.set_xlim(1.6, 5.6)
    ax.grid(color="#e5e5e5", lw=0.6)
    lo, hi = ax.get_ylim()
    if key == "max_cols":
        ax.set_ylim(lo, hi * 2 ** 5)
        top = hi * 2 ** 4.3
    else:
        ax.set_ylim(lo, hi + 12)
        top = hi + 10
    # limit markers: no recorded value, drawn at the top edge
    t_l1 = ", ".join(f"{r['time']:.0f}" for r in l1)
    t_l2 = ", ".join(f"{r['time']:.0f}" for r in l2)
    ax.scatter([4.0], [top], marker="x", color=COL["A0"], s=50, zorder=4)
    ax.scatter([5.0], [top], marker="x", color=COL["A11"], s=50, zorder=4)
    ax.annotate(f"L1 A0 t=4: no value\nulimit -v 14.5 GB abort\nat {t_l1} s", (4.0, top), xytext=(-8, -4), va="top",
                textcoords="offset points", ha="right", fontsize=6.5, color=COL["A0"])
    ax.annotate(f"L2 A11 t=5: no value\ncgroup OOM (14.0 GB)\nat {t_l2} s", (5.0, top), xytext=(-8, -4), va="top",
                textcoords="offset points", ha="right", fontsize=6.5, color=COL["A11"])
    ax.legend(frameon=False, fontsize=6.5, loc="lower right")
axes[0].set_title("F4 row-reduction field multiplications, m = 4, p1", fontsize=9)
axes[1].set_title("widest Macaulay matrix (columns), m = 4, p1", fontsize=9)
fig.suptitle("Amendment 5 (L1, L2) with every m = 4 cell, A0 and A11 (run-20261019T1633Z)", fontsize=9.5)
fig.text(0.5, 0.01, "Rows: runs/C1-rescore.jsonl (F4 replays, trace matched), runs/C2-*, runs/F4-* (partial max_cols), "
         "runs/L1-A0-m4-t4.jsonl, runs/L2-A11-m4-t5.jsonl. x = cell hit a limit; no value recorded.",
         ha="center", fontsize=7, color="#555")
fig.tight_layout(rect=(0, 0.04, 1, 0.95))
fig.savefig(os.path.join(here, "m4_vs_b.svg"))
fig.savefig(os.path.join(here, "m4_vs_b.png"), dpi=150)
