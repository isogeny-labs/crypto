"""Plot, per (arm, m, t) cell, the cell D and max_cols on a new instance
(P4: another prime; Z1: x_R = 0 targets) against the committed generic p1 cell.
Reads committed rows only; a rendering aid (AGENTS.md "no Python")."""
import json, os, glob
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
runs = os.path.join(here, "..", "..", "runs")


def load(name):
    path = os.path.join(runs, name)
    if not os.path.exists(path):
        return []
    return [r for r in (json.loads(l) for l in open(path) if l.strip()) if "summary" not in r]


def cells(rows, key):
    c = {}
    for r in rows:
        if r["status"] == "ok":
            c.setdefault(r["t"], []).append(r[key])
    return {t: sum(v) / len(v) for t, v in c.items()}


GROUPS = {"P4": ("#2a78d6", "o", "p4 = 10007297 (2-adic valuation 8)"),
          "Z1": ("#eb6834", "s", "x_R = 0 targets (p1)")}
plt.rcParams.update({"font.size": 9, "axes.edgecolor": "#888"})
fig, (ax, bx) = plt.subplots(1, 2, figsize=(9.5, 4.0))
for grp, (col, mk, label) in GROUPS.items():
    dd, ww, xs = [], [], []
    for arm in ["A0", "A11"]:
        for m in [2, 3]:
            new = load(f"{grp}-{arm}-m{m}.jsonl")
            gen = load(f"{arm}-m{m}.jsonl")
            nd, gd = cells(new, "D"), cells(gen, "D")
            nw, gw = cells(new, "max_cols"), cells(gen, "max_cols")
            for t in sorted(nd):
                if t in gd:
                    xs.append(gw[t] * (1.04 if grp == "Z1" else 0.96))
                    dd.append(nd[t] - gd[t])
                    ww.append(100.0 * (nw[t] - gw[t]) / gw[t])
    ax.plot(xs, dd, ls="none", marker=mk, ms=7, color=col, mec="white", label=label)
    bx.plot(xs, ww, ls="none", marker=mk, ms=7, color=col, mec="white", label=label)
for a_ in (ax, bx):
    a_.axhline(0, color="#888", lw=0.8)
    a_.set_xscale("log", base=2)
    a_.set_xlabel("generic p1 cell's max_cols (columns, log2 scale)")
    a_.grid(color="#e5e5e5", lw=0.6)
    a_.set_axisbelow(True)
ax.set_ylabel("cell D − generic p1 cell D")
ax.set_ylim(-1.5, 1.5)
ax.set_title("Solving degree against the generic cell")
bx.set_ylabel("cell max_cols vs generic p1 cell (percent)")
bx.set_title("Matrix width against the generic cell")
ax.legend(frameon=False, fontsize=8, loc="upper left")
fig.suptitle("Amendment 9: another prime (P4) and structured targets (Z1), run-20261020T1555Z", fontsize=9.5)
fig.text(0.5, 0.005, "One point per (arm, m, t) cell shared with the committed first run; both targets enter each cell mean. "
         "Points are spread sideways by group, not by value.", ha="center", fontsize=7.5, color="#555")
fig.tight_layout(rect=(0, 0.03, 1, 0.95))
fig.savefig(os.path.join(here, "p4_z1.svg"))
fig.savefig(os.path.join(here, "p4_z1.png"), dpi=150)
print("wrote p4_z1.svg/png")
