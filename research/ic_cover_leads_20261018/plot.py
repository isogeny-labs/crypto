"""Graphs of the recorded cost breakdown (aggregate.sh output); layout only.

    python3 plot.py reports/run-20261018T1900Z
"""
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import matplotlib.ticker

RUN = Path(sys.argv[1])
RUN_ID = RUN.name
OUT = Path(__file__).parent
BLUE, ORANGE, GREEN, GREY, INK = "#2a78d6", "#eb6834", "#1baf7a", "#c3c2b7", "#1a1a19"
plt.rcParams.update({"font.size": 9, "axes.edgecolor": "#898781", "axes.labelcolor": INK,
                     "xtick.color": INK, "ytick.color": INK, "svg.fonttype": "none"})


def kv(name):
    return {l.split()[0]: float(l.split()[1]) for l in (RUN / name).read_text().splitlines()}


def table(name):
    rows = [l.split() for l in (RUN / name).read_text().splitlines()]
    return [dict(zip(rows[0], map(float, r))) for r in rows[1:]]


def save(fig, stem):
    fig.tight_layout()
    fig.savefig(OUT / f"{stem}.svg")
    fig.savefig(OUT / f"{stem}.png", dpi=150)
    plt.close(fig)


# 1. Where one decomposition test's counted cost goes (p = 101).
b = kv("breakdown_p101.txt")
parts = [
    ("F4 (Gröbner basis)", b["f4"], BLUE, "L2, L4"),
    ("characteristic polynomial (64×64 Hessenberg)", b["charpoly"], ORANGE, "L1"),
    ("random combination + eigenvector solves", b["combination_and_eigenvectors"], ORANGE, "L1"),
    ("normal forms of the 6 matrices", b["normal_forms"], ORANGE, "L1"),
    ("root finding in F_p", b["root_finding"], ORANGE, ""),
    ("Weil restriction + post-check", b["weil_and_post"], GREY, ""),
]
fig, ax = plt.subplots(figsize=(7.2, 3.0))
for i, (lab, v, c, lead) in enumerate(parts):
    ax.barh(i, v, color=c, height=0.62)
    tag = f"  {v/1e3:,.0f}k ({100*v/b['c_cov']:.0f} %)" + (f"   → {lead}" if lead else "")
    ax.text(v, i, tag, va="center", fontsize=8, color=INK)
ax.set_yticks(range(len(parts)), [p[0] for p in parts])
ax.invert_yaxis()
ax.set_xlim(0, b["f4"] * 1.55)
ax.set_xlabel("field multiplications per decomposition test (mean of 120 tests)")
ax.set_title(f"C_cov = {b['c_cov']/1e6:.3f}M per test at p = 101: F4 (blue) and the linear stage (orange)\n"
             f"{RUN_ID}, jv_cover --exp ccov, build 536aba4ce", fontsize=8.5)
ax.xaxis.set_major_formatter(matplotlib.ticker.FuncFormatter(lambda x, _: f"{x/1e3:.0f}k"))
for s in ("top", "right"):
    ax.spines[s].set_visible(False)
save(fig, "breakdown")

# 2. F4 rows per degree: pivots against reductions to zero.
f = table("f4_by_degree.txt")
fig, ax = plt.subplots(figsize=(6.0, 2.9))
d = [r["degree"] for r in f]
ax.bar(d, [r["pivots"] for r in f], color=BLUE, width=0.6, label="rows that give a pivot")
ax.bar(d, [r["zero"] for r in f], bottom=[r["pivots"] + 2 for r in f], color=ORANGE, width=0.6,
       label="rows reducing to zero (F5 removes these)")
for r in f:
    if r["zero"]:
        ax.text(r["degree"], r["rows"] + 6, f"{r['zero']:.0f}", ha="center", fontsize=8, color=INK)
tot, zero = sum(r["rows"] for r in f), sum(r["zero"] for r in f)
ax.set_xlabel("F4 step degree")
ax.set_ylabel("matrix rows (mean per system)")
ax.set_title(f"F4 on the six Nagao quadrics, p = 101, 100 systems: {zero:.0f} of {tot:.0f} rows "
             f"({100*zero/tot:.0f} %) reduce to zero\n{RUN_ID}, F4_DEBUG=1", fontsize=8.5)
ax.legend(frameon=False, fontsize=8, loc="upper left")
ax.set_ylim(0, max(r["rows"] for r in f) * 1.3)
for s in ("top", "right"):
    ax.spines[s].set_visible(False)
save(fig, "f4_by_degree")

# 3. Non-unit columns of each multiplication matrix.
lb = kv("lin_breakdown.txt")
fig, ax = plt.subplots(figsize=(6.0, 2.6))
xs = [f"x{k}" for k in range(1, 7)]
vals = [lb[f"dense_cols_x{k}"] for k in range(1, 7)]
ax.bar(xs, vals, color=[GREY] * 5 + [GREEN], width=0.6)
for x, v in zip(xs, vals):
    ax.text(x, v + 1, f"{v:.1f}", ha="center", fontsize=8, color=INK)
ax.axhline(64, color="#898781", lw=1, ls="--")
ax.text(5.45, 64, "δ = 64", va="bottom", ha="right", fontsize=8, color=INK)
ax.set_ylim(0, 75)
ax.set_ylabel("non-unit columns of M_xi (of δ)")
ax.set_title(f"Multiplication matrices: M_x6 (green, last grevlex variable) is the sparse one\n"
             f"p = 101, {lb['systems']:.0f} systems, {RUN_ID}", fontsize=8.5)
for s in ("top", "right"):
    ax.spines[s].set_visible(False)
save(fig, "dense_cols")

# 4. F4 with the recorded trace replayed, against the stock run.
st, tr = table("ccov_stock.txt"), table("ccov_trace.txt")
fig, ax = plt.subplots(figsize=(6.0, 2.6))
for i, (a, t) in enumerate(zip(st, tr)):
    ax.bar(i - 0.17, a["f4"], 0.3, color=BLUE, label="stock F4" if i == 0 else None)
    ax.bar(i + 0.17, t["f4"], 0.3, color=ORANGE, label="--trace (replay)" if i == 0 else None)
    ax.text(i - 0.17, a["f4"], f"{a['f4']/1e3:.0f}k", ha="center", va="bottom", fontsize=8)
    ax.text(i + 0.17, t["f4"], f"{t['f4']/1e3:.0f}k\n{t['replayed']:.0f} replayed,\n"
            f"{t['trace_mismatches']:.0f} diverged", ha="center", va="bottom", fontsize=7.5)
ax.set_xticks(range(len(st)), [f"p = {a['p']:.0f}" for a in st])
ax.set_ylim(0, max(t["f4"] for t in tr) * 1.45)
ax.set_ylabel("F4 multiplications per test")
ax.yaxis.set_major_formatter(matplotlib.ticker.FuncFormatter(lambda x, _: f"{x/1e3:.0f}k"))
ax.set_title(f"Replaying a recorded F4 trace costs more than the current engine\n"
             f"120 tests per p, {RUN_ID}", fontsize=8.5)
ax.legend(frameon=False, fontsize=8, loc="upper left")
for s in ("top", "right"):
    ax.spines[s].set_visible(False)
save(fig, "trace_replay")
