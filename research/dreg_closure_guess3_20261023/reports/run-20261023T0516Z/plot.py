"""Layout only.  Left: outcomes recorded in ../../runs/m3-22-8.d<K>.<arm>.jsonl
(rows = draws, columns = G6 and the 8 G3 branches): filled = the degree-4
closure refutes, hollow = degree 4 saturates without refuting, grey cross =
not run (the draw's early stop after a non-refuting branch).  Right: guessed
variables k against l.  l = 6 and l = 7 are read from the committed records of
the closure-driver and specialisation studies, l = 8 (k = 4) from the previous
study; nothing here is recomputed."""
import glob, json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
runs = os.path.join(here, "../../runs")
arms = ["G6"] + [f"G3b{b}" for b in range(8)]
rows = {}
for p in sorted(glob.glob(os.path.join(runs, "m3-22-8.*.jsonl"))):
    _, d, arm = os.path.basename(p)[:-6].split(".")
    v = json.loads(open(p).read().splitlines()[0])
    rows.setdefault(int(d[1:]), {})[arm] = v["closure_degree"] is not None
draws = sorted(rows)

fig, (ax, bx) = plt.subplots(1, 2, figsize=(12, 4.6), gridspec_kw={"width_ratios": [1.5, 1]})
for i, k in enumerate(draws):
    for j, a in enumerate(arms):
        ref = rows[k].get(a)
        c = "tab:green" if a == "G6" else "tab:blue"
        if ref is None:
            ax.scatter(j, i, s=60, marker="x", color="0.6", linewidths=1.5)
            continue
        ax.scatter(j, i, s=150, marker="o", facecolors=c if ref else "white", edgecolors=c, linewidths=1.8)
ax.set_xticks(range(len(arms)))
ax.set_xticklabels(["G6\n(6,6,6)"] + [f"{b & 1}{b >> 1 & 1}{b >> 2 & 1}" for b in range(8)], fontsize=8)
ax.set_xlabel("G3 branch: values of variables 7, 15, 23", fontsize=8)
ax.set_yticks(range(len(draws)))
ax.set_yticklabels([f"(22,8) draw {d}" for d in draws], fontsize=8)
ax.invert_yaxis(); ax.set_xlim(-0.7, len(arms) - 0.3); ax.set_ylim(len(draws) - 0.4, -0.6)
n_ref = sum(rows[k].get(a, False) for k in draws for a in arms[1:])
n_run = sum(rows[k].get(a) is not None for k in draws for a in arms[1:])
n_suff = sum(all(rows[k].get(f"G3b{b}") for b in range(8)) for k in draws)
ax.set_title(f"m = 3, l = 8, N = 43: G3 refutes on {n_ref} of {n_run} measured branches; "
             f"suffices on {n_suff} of {len(draws)} draws\n"
             "filled = degree-4 closure refutes; hollow = saturates without 1; grey x = not run (early stop)",
             fontsize=8)

ells = [6, 7, 8, 9, 10]
bx.plot(ells, [3 * (l - 6) for l in ells], "--", color="tab:red", label="K_excess: m*(l-6)")
bx.plot(ells, [max(0, 3 * (l - 7) + 1) for l in ells], ":", color="tab:purple", label="K_marginal: m*(l-7)+1")
bx.scatter([6], [0], color="black", zorder=3, label="measured sufficient")
bx.scatter([7], [1], color="black", zorder=3)
bx.annotate("5 of 8 draws", (7, 1), textcoords="offset points", xytext=(6, -12), fontsize=7)
bx.scatter([8], [4], color="black", marker="v", zorder=3)
bx.annotate("4 suffices (previous study)", (8, 4), textcoords="offset points", xytext=(8, 4), fontsize=7)
bx.scatter([8], [3], color="tab:blue", marker="X", s=70, zorder=3, label="3 (this set): fails on 3 of 4 draws")
bx.annotate("3 fails on draws 0, 3, 5;\nsuffices on draw 6", (8, 3), textcoords="offset points",
            xytext=(8, -22), fontsize=7)
bx.set_xlabel("l (m = 3)", fontsize=8); bx.set_ylabel("guessed variables k for degree 4", fontsize=8)
bx.set_xticks(ells); bx.set_xlim(5.7, 10.6); bx.set_ylim(-0.5, 7); bx.legend(fontsize=7, loc="upper left"); bx.grid(alpha=0.3)
bx.set_title("Guesses for degree 4: bounds at l = 8", fontsize=9)
fig.suptitle("research/dreg_closure_guess3_20261023, run 2026-10-23T05:16Z", fontsize=9)
fig.tight_layout()
fig.savefig(os.path.join(here, "guess3.svg")); fig.savefig(os.path.join(here, "guess3.png"), dpi=150)
