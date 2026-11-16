"""Plots and table layout only: reads results/run.json, computes nothing new beyond
ratio-band arithmetic (rand_sd/rand_mean) and sums of recorded counters.
Run: python3 figures/make_figure.py  (needs matplotlib; see README)."""
import json, os, csv
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
here = os.path.dirname(os.path.abspath(__file__))
R = json.load(open(os.path.join(here, "..", "results", "run.json")))
ORD = ["msb", "lsb", "sift_msb", "sift_lsb"]
MK = {"msb": "o", "lsb": "s", "sift_msb": "^", "sift_lsb": "v"}
ns = R["header"]["ns"]
rows = {(r["n"], r["set"]): r for r in R["rows"]}
C = {"X2": "#1f5fa8", "XE": "#c4501b"}
fig, (a1, a2) = plt.subplots(1, 2, figsize=(12.5, 5.2))
a1.axhspan(0.9, 1.1, color="#dddddd", alpha=0.6, label="P1 band [0.9, 1.1]")
a1.axhline(1.0, color="k", lw=0.6)
for s, dx in (("X2", -0.12), ("XE", 0.12)):
    # RAND +-3*relsd band (per order, per n; drawn as vertical bars at that n)
    for i, n in enumerate(ns):
        for j, o in enumerate(ORD):
            v = rows[(n, s)]["vs_rand"][o]
            rel = v["rand_sd"] / v["rand_mean"]
            x = n + dx + (j - 1.5) * 0.045
            a1.vlines(x, 1 - 3 * rel, 1 + 3 * rel, color=C[s], alpha=0.35, lw=3)
            a1.plot(x, v["ratio_x_over_mean_rand"], MK[o], color=C[s], ms=5.5,
                    mfc="white" if o.startswith("sift") else C[s])
a1.set_xticks(ns); a1.set_xlabel("n (bits of p)")
a1.set_ylabel("nodes(X) / mean nodes(RAND, 10 sets)")
a1.set_ylim(0.86, 1.14)
a1.set_title("Node-count ratio to RAND mean (bars: RAND ±3·relsd)")
from matplotlib.lines import Line2D
h = [Line2D([], [], color=C["X2"], lw=3, label="X2"), Line2D([], [], color=C["XE"], lw=3, label="X_E")]
h += [Line2D([], [], marker=MK[o], color="gray", ls="", mfc="white" if o.startswith("sift") else "gray", label=o) for o in ORD]
h += [matplotlib.patches.Patch(color="#dddddd", label="P1 band [0.9, 1.1]")]
a1.legend(handles=h, fontsize=8, loc="lower left", ncol=2)
# panel 2: absolute counts, msb and sift_lsb? use msb order (solid) ; log scale
for s in ("X2", "XE"):
    xs = [rows[(n, s)]["orders"]["msb"]["nodes"] for n in ns]
    rm = [rows[(n, s)]["vs_rand"]["msb"]["rand_mean"] for n in ns]
    a2.plot(ns, xs, "o-", color=C[s], label=f"{s} (msb)")
    a2.plot(ns, rm, "x--", color=C[s], label=f"RAND mean for {s} (msb)")
a2.plot(ns, [rows[(n, "INT")]["orders"]["msb"]["nodes"] for n in ns], "d-", color="#2a8a3a", label="INT (msb)")
a2.plot(ns, [2 * n for n in ns], ":", color="#2a8a3a", label="2n (P2 bound)")
a2.set_yscale("log"); a2.set_xticks(ns); a2.set_xlabel("n (bits of p)")
a2.set_ylabel("ROBDD nodes (log scale)"); a2.set_title("Absolute node counts, MSB-first order")
a2.legend(fontsize=8)
fig.suptitle("F5 compiled-advice ROBDD, run F5-RUN-01 (exact counts; 10 RAND sets per row; local host)", fontsize=10)
fig.tight_layout()
fig.savefig(os.path.join(here, "ratio_vs_n.svg"))
fig.savefig(os.path.join(here, "ratio_vs_n.png"), dpi=150)
# data beside the figure
with open(os.path.join(here, "ratio_vs_n.data.csv"), "w", newline="") as f:
    w = csv.writer(f)
    w.writerow(["n", "set", "order", "nodes", "rand_mean", "rand_sd", "relsd", "ratio", "z"])
    for n in ns:
        for s in ("X2", "XE"):
            for o in ORD:
                v = rows[(n, s)]["vs_rand"][o]
                w.writerow([n, s, o, rows[(n, s)]["orders"][o]["nodes"], v["rand_mean"], v["rand_sd"],
                            v["rand_sd"] / v["rand_mean"], v["ratio_x_over_mean_rand"], v["z_score"]])
