"""Plot recorded D against b for m = 3 on p1 and p2: A0 (f4_fp cells, and the
tower-engine t = 5 cell) and A11 (f4_fp), with the t = 5 cells of T3 on p2.
Reads committed rows only; a rendering aid (AGENTS.md "no Python")."""
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


def ok(rows, tmax=99, tmin=0):
    return sorted((r["b"], r["D"]) for r in rows if r["status"] == "ok" and tmin <= r["t"] <= tmax)


p1 = {
    "A0": ok(load("A0-m3.jsonl")),
    "A0 t=5 (tower engine)": ok(load("T2-A0-m3.jsonl")),
    "A11": ok(load("A11-m3.jsonl")) + ok(load("F3-A11-m3-t5.jsonl")),
}
p2 = {
    "A0": ok(load("F1-A0-m3-p2.jsonl"), 4),
    "A0 t=5 (tower engine)": ok(load("T3-A0-m3-p2.jsonl")),
    "A11": ok(load("F1-A11-m3-p2.jsonl"), 4) + ok(load("T3-A11-m3-p2.jsonl")),
}
stopped = [r for n in ("T3-A0-m3-p2.jsonl", "T3-A11-m3-p2.jsonl") for r in load(n) if r["status"] != "ok"]
style = {
    "A0": dict(color=BLUE, marker="o", ls="-", mfc=BLUE),
    "A0 t=5 (tower engine)": dict(color=BLUE, marker="s", ls="none", mfc="white", mew=2),
    "A11": dict(color=GREEN, marker="P", ls="-", mfc=GREEN),
}
plt.rcParams.update({"font.size": 9, "axes.edgecolor": "#888"})
fig, axes = plt.subplots(1, 2, figsize=(9.5, 4.0), sharey=True)
for ax, (title, data) in zip(axes, [("p1 = 786433", p1), ("p2 = 2013265921", p2)]):
    for name, pts in data.items():
        if pts:
            ax.plot([b for b, _ in pts], [d for _, d in pts], ms=8, lw=1.5, label=name, **style[name])
    ax.set_title(f"m = 3, {title}")
    ax.set_xlabel("b = log2|V| per summand (bits)")
    ax.set_ylim(3, 9.5)
    ax.grid(color="#e5e5e5", lw=0.6)
    ax.set_axisbelow(True)
for r in stopped:
    col = BLUE if r["arm"] == "A0" else GREEN
    y = r["D"] if r.get("D") is not None else 9.0
    axes[1].plot([r["b"]], [y], marker="x", ms=9, color=col, mew=2, ls="none")
    axes[1].annotate(f"{r['arm']} {r['status']}", (r["b"], y), xytext=(-8, 0), textcoords="offset points",
                     ha="right", va="center", fontsize=7, color="#555")
axes[0].set_ylabel("D = solving_degree_max")
axes[0].legend(frameon=False, fontsize=8, loc="upper left")
fig.suptitle("m = 3, t = 5 on a second prime (Amendment 7): D against b, run-20261019T1907Z", fontsize=9.5)
fig.text(0.5, 0.005, "Both planted targets per cell plotted (they coincide). Open squares: A0 cells on f4_fp_tower (admitted on each prime). "
         "Crosses: a system that hit a limit.", ha="center", fontsize=7.2, color="#555")
fig.tight_layout(rect=(0, 0.035, 1, 0.95))
fig.savefig(os.path.join(here, "m3_t5_p1_p2.svg"))
fig.savefig(os.path.join(here, "m3_t5_p1_p2.png"), dpi=150)
print("wrote m3_t5_p1_p2.svg/png")
