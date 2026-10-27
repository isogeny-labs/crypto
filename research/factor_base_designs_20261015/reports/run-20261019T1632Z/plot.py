"""Plot recorded D against b for A0 at m = 3 and m = 4: f4_fp cells (the first
run, F4) and the tower-engine cells of Amendment 6 (T2), with A11 beside
(first run, F3, F4, C2). Reads committed rows only; a rendering aid
(AGENTS.md "no Python")."""
import json, os
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
runs = os.path.join(here, "..", "..", "runs")
BLUE, GREEN = "#2a78d6", "#008300"


def load(name):
    path = os.path.join(runs, name)
    if not os.path.exists(path):
        return []
    return [r for r in (json.loads(l) for l in open(path) if l.strip()) if "summary" not in r]


def ok(rows, tmax=99):
    return sorted((r["b"], r["D"]) for r in rows if r["status"] == "ok" and r["t"] <= tmax)


series = {
    3: {
        "A0 f4_fp": ok(load("A0-m3.jsonl")),
        "A0 tower engine (T2)": ok(load("T2-A0-m3.jsonl")),
        "A11 f4_fp": ok(load("A11-m3.jsonl")) + ok(load("F3-A11-m3-t5.jsonl")),
    },
    4: {
        "A0 f4_fp": ok(load("F4-A0-m4.jsonl"), 3),
        "A0 tower engine (T2)": ok(load("T2-A0-m4.jsonl")),
        "A11 f4_fp": ok(load("F4-A11-m4.jsonl"), 3) + ok(load("C2-A11-m4-t4.jsonl")),
    },
}
stopped = {3: [r for r in load("T2-A0-m3.jsonl") if r["status"] != "ok"],
           4: [r for r in load("T2-A0-m4.jsonl") if r["status"] != "ok"]}
style = {
    "A0 f4_fp": dict(color=BLUE, marker="o", ls="-", mfc=BLUE),
    "A0 tower engine (T2)": dict(color=BLUE, marker="s", ls="none", mfc="white", mew=2),
    "A11 f4_fp": dict(color=GREEN, marker="P", ls="-", mfc=GREEN),
}
plt.rcParams.update({"font.size": 9, "axes.edgecolor": "#888"})
fig, axes = plt.subplots(1, 2, figsize=(9.5, 4.0), sharey=True)
for ax, m in zip(axes, [3, 4]):
    for name, pts in series[m].items():
        if not pts:
            continue
        ax.plot([b for b, _ in pts], [d for _, d in pts], ms=8, lw=1.5, label=name, **style[name])
    for r in stopped[m]:
        y = r.get("D")
        ax.plot([r["b"]], [y if y is not None else 9.3], marker="x", ms=9, color=BLUE, mew=2, ls="none")
        ax.annotate("A0 T2 stopped" + ("" if y is not None else " (no D)"), (r["b"], y if y is not None else 9.3),
                    xytext=(-8, 0), textcoords="offset points", ha="right", va="center", fontsize=7, color="#555")
    ax.set_title(f"m = {m}")
    ax.set_xlabel("b = log2|V| per summand (bits)")
    ax.set_ylim(2.5, 9.8)
    ax.grid(color="#e5e5e5", lw=0.6)
    ax.set_axisbelow(True)
axes[0].set_ylabel("D = solving_degree_max")
axes[0].legend(frameon=False, fontsize=8, loc="upper left")
fig.suptitle("A0 with its memory-bound cells on the admitted tower engine (Amendment 6), p1 = 786433, run-20261019T1632Z",
             fontsize=9.5)
fig.text(0.5, 0.005, "Both planted targets per cell plotted (they coincide). Open squares: A0 cells measured on f4_fp_tower "
         "(admitted: same D on all 26 committed A0 systems). Crosses: a T2 system that hit a limit.",
         ha="center", fontsize=7.2, color="#555")
fig.tight_layout(rect=(0, 0.035, 1, 0.95))
fig.savefig(os.path.join(here, "a0_tower_d_vs_b.svg"))
fig.savefig(os.path.join(here, "a0_tower_d_vs_b.png"), dpi=150)
print("wrote a0_tower_d_vs_b.svg/png")
