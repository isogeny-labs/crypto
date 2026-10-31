"""Left: K1, each cell's max_cols on a special curve family divided by the
generic cell's, against the generic cell's size. Right: T4a, the stop-bounded D
minus the committed D, per system, by m. Reads committed rows only; a
rendering aid (AGENTS.md "no Python")."""
import json, os
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
runs = os.path.join(here, "..", "..", "runs")


def load(name):
    path = os.path.join(runs, name)
    return [r for r in (json.loads(l) for l in open(path) if l.strip()) if "summary" not in r]


def cells(rows):
    c = {}
    for r in rows:
        if r["status"] == "ok":
            c.setdefault(r["t"], []).append(r["max_cols"])
    return {t: sum(v) / len(v) for t, v in c.items()}


COL = {"a0": "#eb6834", "am3": "#2a78d6"}
MK = {"A0": "o", "A11": "P"}
plt.rcParams.update({"font.size": 9, "axes.edgecolor": "#888"})
fig, (ax, bx) = plt.subplots(1, 2, figsize=(9.5, 4.0))
for fam in ["a0", "am3"]:
    for arm in ["A0", "A11"]:
        xs, ys = [], []
        for m in [2, 3]:
            k = cells(load(f"K1-{arm}-{fam}-m{m}.jsonl"))
            g = cells(load(f"{arm}-m{m}.jsonl"))
            for t in sorted(k):
                if t in g:
                    # x is jittered (family and arm) so coincident points show; y is exact
                    jit = (1.06 if fam == "a0" else 0.94) * (1.02 if arm == "A11" else 0.98)
                    xs.append(g[t] * jit)
                    ys.append(100.0 * (k[t] - g[t]) / g[t])
        ax.plot(xs, ys, ls="none", marker=MK[arm], ms=7, mfc=COL[fam], mec="white", color=COL[fam],
                label=f"{fam}, {arm}")
ax.axhline(0.0, color="#888", lw=0.8)
ax.set_xscale("log", base=2)
ax.set_xlabel("generic cell's max_cols (columns, log2 scale)")
ax.set_ylabel("max_cols on the family vs generic (percent)")
ax.set_title("K1: matrix width by curve family")
ax.set_ylim(-2, 1)
ax.grid(color="#e5e5e5", lw=0.6)
ax.set_axisbelow(True)
ax.legend(frameon=False, fontsize=7.5, loc="lower left", ncol=2)
adm = load("T4-admission.jsonl")
for m, xo in [(2, 0), (3, 0), (4, 0)]:
    ds = [(r["D"] - r["committed"]["D"], r["t"]) for r in adm if r["m"] == m]
    for j, (d, t) in enumerate(sorted(ds, key=lambda x: (x[1], x[0]))):
        bx.plot([m + (j % 8 - 3.5) * 0.06], [d], marker="o", ms=7, color="#2a78d6" if m == 4 else "#888",
                mec="white", ls="none")
bx.axhline(0, color="#888", lw=0.8)
bx.set_xticks([2, 3, 4])
bx.set_xticklabels(["m = 2\n(16 systems)", "m = 3\n(6)", "m = 4, gating\n(4)"])
bx.set_ylim(-2, 1)
bx.set_yticks([-2, -1, 0, 1])
bx.set_ylabel("stop-bounded D − committed D")
bx.set_title("T4a: tower engine with the k = 64 stop")
bx.grid(color="#e5e5e5", lw=0.6)
bx.set_axisbelow(True)
fig.suptitle("Amendment 8: K1 curve families and T4a, run-20261020T0550Z", fontsize=9.5)
fig.text(0.5, 0.005, "Left: points are spread sideways by family and arm, not by value; both targets per cell enter each cell mean. Right: each point is one system.",
         ha="center", fontsize=7.5, color="#555")
fig.tight_layout(rect=(0, 0.03, 1, 0.95))
fig.savefig(os.path.join(here, "k1_t4a.svg"))
fig.savefig(os.path.join(here, "k1_t4a.png"), dpi=150)
print("wrote k1_t4a.svg/png")
