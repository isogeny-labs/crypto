"""Plot, per (m, t) cell, the A0 cell D and max_cols on twisted Edwards curves
(E1) against the committed A0 cell on short Weierstrass curves (first run).
Reads committed rows only; a rendering aid (AGENTS.md "no Python")."""
import json, os
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
runs = os.path.join(here, "..", "..", "runs")


def load(name):
    path = os.path.join(runs, name)
    return [r for r in (json.loads(l) for l in open(path) if l.strip()) if "summary" not in r]


def cells(rows, key):
    c = {}
    for r in rows:
        if r["status"] == "ok":
            c.setdefault(r["t"], []).append(r[key])
    return {t: sum(v) / len(v) for t, v in c.items()}


plt.rcParams.update({"font.size": 9, "axes.edgecolor": "#888"})
fig, (ax, bx) = plt.subplots(1, 2, figsize=(9.5, 4.0))
cols = {2: "#2a78d6", 3: "#eb6834"}
for m in (2, 3):
    ed, we = load(f"E1-A0-m{m}.jsonl"), load(f"A0-m{m}.jsonl")
    ed_d, we_d = cells(ed, "D"), cells(we, "D")
    ed_w, we_w = cells(ed, "max_cols"), cells(we, "max_cols")
    ts = sorted(t for t in ed_d if t in we_d)
    ax.plot([we_w[t] for t in ts], [ed_d[t] - we_d[t] for t in ts], ls="none", marker="o", ms=7,
            color=cols[m], mec="white", label=f"A0, m = {m}")
    bx.plot([we_w[t] for t in ts], [100.0 * (ed_w[t] - we_w[t]) / we_w[t] for t in ts], ls="none",
            marker="o", ms=7, color=cols[m], mec="white", label=f"A0, m = {m}")
for a_ in (ax, bx):
    a_.axhline(0, color="#888", lw=0.8)
    a_.set_xscale("log", base=2)
    a_.set_xlabel("Weierstrass cell's max_cols (columns, log2 scale)")
    a_.grid(color="#e5e5e5", lw=0.6)
    a_.set_axisbelow(True)
ax.set_ylabel("Edwards cell D − Weierstrass cell D")
ax.set_ylim(-1.5, 1.5)
ax.set_title("Solving degree against the Weierstrass cell")
bx.set_ylabel("Edwards max_cols vs Weierstrass (percent)")
bx.set_title("Matrix width against the Weierstrass cell")
ax.legend(frameon=False, fontsize=8, loc="upper left")
fig.suptitle("Amendment 11: A0 on twisted Edwards curves (E1), run-20261020T1636Z", fontsize=9.5)
fig.text(0.5, 0.005, "One point per (m, t) cell shared with the committed first run; both targets enter each cell mean.",
         ha="center", fontsize=7.5, color="#555")
fig.tight_layout(rect=(0, 0.03, 1, 0.95))
fig.savefig(os.path.join(here, "e1.svg"))
fig.savefig(os.path.join(here, "e1.png"), dpi=150)
print("wrote e1.svg/png")
