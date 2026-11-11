"""Plots only: reads results/run.json and draws recorded medians, censored counts and
fitted exponents with their recorded bootstrap CIs. Computes nothing new.
Run: python3 figures/make_figure.py  (needs matplotlib)."""
import json, os, csv
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.lines import Line2D
here = os.path.dirname(os.path.abspath(__file__))
R = json.load(open(os.path.join(here, "..", "results", "run.json")))
SOLV = ["uniform", "greedy", "sa_B", "sa_4B", "sa_16B", "pt", "root_enum"]
COL = {"uniform": "#555555", "greedy": "#1f5fa8", "sa_B": "#c4501b", "sa_4B": "#d98a1c",
       "sa_16B": "#8a4fa8", "pt": "#2a8a3a", "root_enum": "#000000"}
MK = {"uniform": "o", "greedy": "s", "sa_B": "^", "sa_4B": "v", "sa_16B": "D", "pt": "P", "root_enum": "*"}
pts = {}  # (solver) -> list of (B, median, censored)
for f in R["fits"]:
    if f["set"] == "all":
        pts[f["solver"]] = [(p["B"], p["median_evals"], p["censored"]) for p in f["points"]]
fig, (a1, a2) = plt.subplots(1, 2, figsize=(13, 5.4), gridspec_kw={"width_ratios": [1.25, 1]})
Bs = [p[0] for p in pts["uniform"]]
a1.plot(Bs, [b * b for b in Bs], "-", color="#999999", lw=1.2, label="B² (box enumeration)")
a1.plot(Bs, Bs, "--", color="#999999", lw=1.2, label="B (root-solving, exact count)")
for s in SOLV:
    xs = [p[0] for p in pts[s]]; ys = [p[1] for p in pts[s]]
    a1.plot(xs, ys, "-", color=COL[s], lw=1, alpha=0.8)
    for B, m, c in pts[s]:
        a1.plot(B, m, MK[s], color=COL[s], ms=9 if s == "root_enum" else 6.5,
                mfc="white" if c > 0 else COL[s], mew=1.6, zorder=3)
cn = [f"{s} {c}/20" for s in SOLV for B, m, c in pts[s] if c > 0]
a1.text(0.98, 0.04, "censored at n = 28 (hollow): " + ", ".join(cn) + "\n(medians exact: fewer than half censored)", transform=a1.transAxes, ha="right", fontsize=7.5)
a1.set_xscale("log", base=2); a1.set_yscale("log")
a1.set_xticks(Bs); a1.set_xticklabels([str(int(b)) for b in Bs])
a1.set_xlabel("B = ceil(sqrt p)  (n = 12, 16, 20, 24, 28)")
a1.set_ylabel("median cost evaluations to first c = 0 (20 instances)")
a1.set_title("Median evaluations vs B (hollow marker: >= 1 instance censored at the cap)", fontsize=9)
h = [Line2D([], [], marker=MK[s], color=COL[s], ls="-", label=s + (" (measured baseline)" if s == "root_enum" else "")) for s in SOLV]
h += [Line2D([], [], color="#999999", ls="-", label="B²"), Line2D([], [], color="#999999", ls="--", label="B")]
a1.legend(handles=h, fontsize=7.5, loc="upper left")
# panel 2: exponents with CI
fits = {(f["solver"], f["set"]): f for f in R["fits"]}
ys = []; y = 0
for s in SOLV:
    for st, dy, mfc in (("all", 0.17, COL[s]), ("n_ge_20", -0.17, "white")):
        f = fits[(s, st)]
        yy = len(SOLV) - 1 - SOLV.index(s) + dy
        a2.errorbar(f["slope"], yy, xerr=[[f["slope"] - f["ci95_lo"]], [f["ci95_hi"] - f["slope"]]],
                    fmt=MK[s], color=COL[s], mfc=mfc, ms=6, capsize=3, lw=1.2)
a2.axvline(1.0, color="#2a8a3a", ls="--", lw=1.2); a2.axvline(2.0, color="#c4501b", ls="--", lw=1.2)
a2.axvline(1.8, color="#999999", ls=":", lw=1)
a2.text(1.0, len(SOLV) - 0.35, "e = 1\n(root-solving)", ha="center", fontsize=7.5, color="#2a8a3a")
a2.text(2.0, len(SOLV) - 0.35, "e = 2\n(box)", ha="center", fontsize=7.5, color="#c4501b")
a2.text(1.8, -0.75, "P1: 1.8", ha="center", fontsize=7.5, color="#666666")
a2.set_yticks([len(SOLV) - 1 - i for i in range(len(SOLV))]); a2.set_yticklabels(SOLV)
a2.set_ylim(-0.9, len(SOLV) + 0.4)
a2.set_xlabel("fitted exponent e, median ~ B^e (ln-ln OLS), bootstrap 95% CI over instances")
a2.set_title("Exponent per solver: filled = all n (5 sizes), hollow = n >= 20 (3 sizes)", fontsize=9)
fig.suptitle("F6 physics-style solvers on the m = 2 decomposition landscape, run F6-RUN-01 (counts, local host)", fontsize=10)
fig.tight_layout()
fig.savefig(os.path.join(here, "evals_vs_B.svg"))
fig.savefig(os.path.join(here, "evals_vs_B.png"), dpi=150)
with open(os.path.join(here, "evals_vs_B.data.csv"), "w", newline="") as f:
    w = csv.writer(f); w.writerow(["solver", "n_set", "B", "median_evals", "censored_of_20"])
    for s in SOLV:
        for B, m, c in pts[s]:
            w.writerow([s, "all", B, m, c])
    w.writerow([]); w.writerow(["solver", "set", "slope", "ci95_lo", "ci95_hi"])
    for (s, st), ft in fits.items():
        w.writerow([s, st, ft["slope"], ft["ci95_lo"], ft["ci95_hi"]])
