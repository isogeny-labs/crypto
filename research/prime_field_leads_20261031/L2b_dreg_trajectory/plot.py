# Plots values already recorded in results/summary.csv and results/cells.csv. No computation of results.
import csv, os, math
os.environ.setdefault("MPLCONFIGDIR", os.path.join(os.path.dirname(os.path.abspath(__file__)), "work", "mpl"))
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
D = os.path.dirname(os.path.abspath(__file__))
RES = os.environ.get("L2B_RESULTS", f"{D}/results")
OUT = os.environ.get("L2B_PLOT_OUT", f"{D}/dreg_vs_log2p.svg")
S = list(csv.DictReader(open(f"{RES}/summary.csv")))
# cells.csv is written without a header (run_cell.sh appends rows); its columns are in results_header.txt.
_hdr = open(f"{D}/results_header.txt").read().strip().split(",")
_lines = open(f"{RES}/cells.csv").read().splitlines()
C = list(csv.DictReader(_lines[1:] if _lines and _lines[0].startswith("p,") else _lines, fieldnames=_hdr))
col = {"real": "#1f77b4", "ctrl": "#d95f02"}
lab = {"real": "S_4 + interval membership", "ctrl": "random f + random membership"}
fig, ax = plt.subplots(1, 2, figsize=(11, 4.2))
for arm in ("real", "ctrl"):
    rows = [r for r in S if r["arm"] == arm and r["n_ok"] != "0"]
    x = [float(r["log2p"]) for r in rows]
    y = [float(r["maxdeg_med"]) for r in rows]
    lo = [float(r["maxdeg_med"]) - float(r["maxdeg_min"]) for r in rows]
    hi = [float(r["maxdeg_max"]) - float(r["maxdeg_med"]) for r in rows]
    ax[0].errorbar(x, y, yerr=[lo, hi], marker="o", color=col[arm], label=lab[arm], capsize=3)
    ym = [float(r["maxcols_med"]) for r in rows]
    ax[1].plot(x, ym, marker="o", color=col[arm], label=lab[arm])
    # unfinished cells: partial value reached at kill, drawn as hollow markers
    for c in C:
        if c["arm"] == arm and c["status"] != "ok":
            lp = math.log2(int(c["p"]))
            ax[0].plot([lp], [int(c["maxdeg"])], marker="v", mfc="none", color=col[arm], ls="")
            ax[1].plot([lp], [int(c["maxcols"])], marker="v", mfc="none", color=col[arm], ls="")
Ns = sorted({(float(r["log2p"]), int(r["N"])) for r in S})
ax[0].plot([a for a, b in Ns], [b for a, b in Ns], "k:", label="|V| = round(p^(1/3)) (membership degree)")
ax[0].set_xlabel("log2 p"); ax[0].set_ylabel("maximal F4 step degree (msolve -v 2)")
ax[0].set_title("Step degree vs log2 p (median, min-max; hollow v = killed cell, partial value)", fontsize=8)
ax[1].set_xlabel("log2 p"); ax[1].set_ylabel("largest Macaulay matrix columns"); ax[1].set_yscale("log")
ax[1].set_title("Largest matrix width (median of finished runs)", fontsize=9)
ax[0].legend(fontsize=7); ax[0].grid(alpha=.3); ax[1].grid(alpha=.3)
fig.suptitle("L2b: constrained prime-field Semaev system, m = 3, msolve", fontsize=10)
fig.tight_layout(); fig.savefig(OUT)
