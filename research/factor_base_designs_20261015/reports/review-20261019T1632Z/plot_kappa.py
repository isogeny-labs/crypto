"""kappa against m - 1, per arm, from runs/analysis-amendment4.txt (committed).
Reads the analysis file only; computes nothing but the plotted positions."""
import os, re
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
src = os.path.join(here, "..", "..", "runs", "analysis-amendment4.txt")
head = re.compile(r"^── (\w+) m = (\d) (p\d)")
cell = re.compile(r"completed cells (\d+): κ = .*?= ([0-9.]+) against m − 1 = (\d)")
rows, cur = [], None
for line in open(src, encoding="utf-8"):
    h = head.match(line)
    if h:
        cur = (h.group(1), int(h.group(2)), h.group(3)); continue
    c = cell.search(line)
    if c and cur:
        rows.append(dict(arm=cur[0], m=cur[1], prime=cur[2], cells=int(c.group(1)),
                         kappa=float(c.group(2)), be=int(c.group(3))))
        cur = None
assert all(r["be"] == r["m"] - 1 for r in rows)
p1 = [r for r in rows if r["prime"] == "p1"]
arms = ["A0", "N", "A1z", "A1g", "A4", "A11", "N11", "H1", "H2"]
colors = dict(A0="#222222", N="#888888", A1z="#1f77b4", A1g="#6baed6", A4="#2ca02c",
              A11="#d62728", N11="#ff9896", H1="#9467bd", H2="#ff7f0e")
marks = dict(A0="o", N="o", A1z="s", A1g="s", A4="^", A11="D", N11="D", H1="v", H2="P")

fig, ax = plt.subplots(figsize=(8.2, 5.6))
xs = [0.5, 3.5]
ax.plot(xs, xs, "k--", lw=1.2, label="break-even κ = m − 1")
ax.fill_between(xs, xs, [0, 0], color="#2ca02c", alpha=0.06)
for i, a in enumerate(arms):
    off = (i - 4) * 0.045
    pts = sorted((r for r in p1 if r["arm"] == a), key=lambda r: r["m"])
    ax.plot([r["m"] - 1 + off for r in pts], [r["kappa"] for r in pts],
            "-", color=colors[a], lw=0.8, alpha=0.6)
    for r in pts:
        inc = r["cells"] < 3
        ax.plot(r["m"] - 1 + off, r["kappa"], marks[a], color=colors[a],
                mfc="white" if inc else colors[a], ms=7, mew=1.5)
        ax.annotate(str(r["cells"]), (r["m"] - 1 + off, r["kappa"]),
                    textcoords="offset points", xytext=(5, 4), fontsize=7)
for a, k in (("A0", "o"), ("A11", "D")):
    for r in rows:
        if r["arm"] == a and r["prime"] == "p2":
            ax.plot(r["m"] - 1 + 0.30, r["kappa"], k, color=colors[a], mfc="none", ms=4,
                    alpha=0.7)
handles = [plt.Line2D([], [], marker=marks[a], color=colors[a], ls="", label=a) for a in arms]
handles += [plt.Line2D([], [], color="k", ls="--", label="break-even κ = m − 1"),
            plt.Line2D([], [], marker="o", color="k", mfc="white", ls="", label="open: < 3 cells (inconclusive)"),
            plt.Line2D([], [], marker="o", color="k", mfc="none", ms=4, ls="", label="small open: same arm on p₂ (A0, A11)")]
ax.text(2.0, 0.5, "below the line: κ < m − 1 (cost-favourable region; no series with at least 3 cells lies here)", fontsize=7.5, ha="center")
ax.legend(handles=handles, fontsize=7.5, loc="upper left", ncol=2)
ax.set_xticks([1, 2, 3]); ax.set_xticklabels(["1 (m = 2)", "2 (m = 3)", "3 (m = 4)"])
ax.set_xlim(0.5, 3.5); ax.set_ylim(0, 16.5)
ax.set_xlabel("break-even m − 1 (F4 multiplications per factor-base bit)")
ax.set_ylabel("κ = Δlog₂ field_ops / Δb (least-squares slope)")
ax.set_title("κ against m − 1 per arm, p₁ (numbers beside points: completed cells)", fontsize=10)
ax.grid(alpha=0.25)
fig.text(0.01, 0.005, "Source: runs/analysis-amendment4.txt (Amendment 4, run-20261016T0616Z). Points at one m − 1 are offset horizontally for legibility only.",
         fontsize=6.5)
fig.tight_layout(rect=(0, 0.02, 1, 1))
fig.savefig(os.path.join(here, "kappa_vs_breakeven.svg"))
fig.savefig(os.path.join(here, "kappa_vs_breakeven.png"), dpi=150)
print(len(rows), "series parsed;", len(p1), "on p1")
