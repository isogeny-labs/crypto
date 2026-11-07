"""Layout only.  Left: the outcomes recorded in ../../runs/m3-22-8.d<K>.<arm>.jsonl
(rows = draws, columns = G6 and the 16 G4 branches): filled = the degree-4
closure refutes, hollow = degree 4 saturates without refuting.  Right: guesses
needed for degree 4 against l, with the two registered extrapolation rules.
The l = 6 and l = 7 points are read from the committed records of the
closure-driver and specialisation studies; nothing here is recomputed."""
import glob, json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
runs = os.path.join(here, "../../runs")
arms = ["G6"] + [f"G4b{b}" for b in range(16)]
rows = {}
for p in sorted(glob.glob(os.path.join(runs, "m3-22-8.*.jsonl"))):
    _, d, arm = os.path.basename(p)[:-6].split(".")
    v = json.loads(open(p).read().splitlines()[0])
    rows.setdefault(int(d[1:]), {})[arm] = v["closure_degree"] is not None
draws = sorted(rows)

fig, (ax, bx) = plt.subplots(1, 2, figsize=(12, 4.6), gridspec_kw={"width_ratios": [1.7, 1]})
for i, k in enumerate(draws):
    for j, a in enumerate(arms):
        ref = rows[k].get(a)
        if ref is None:
            continue
        c = "tab:green" if a == "G6" else "tab:blue"
        ax.scatter(j, i, s=150, marker="o", facecolors=c if ref else "white", edgecolors=c, linewidths=1.8)
ax.set_xticks(range(len(arms)))
ax.set_xticklabels(["G6\n(6,6,6)"] + [f"{b:04b}"[::-1] for b in range(16)], fontsize=7, rotation=0)
ax.set_xlabel("G4 branch: values of variables 7, 15, 23, 24", fontsize=8)
ax.set_yticks(range(len(draws)))
ax.set_yticklabels([f"(22,8) draw {d}" for d in draws], fontsize=8)
ax.invert_yaxis(); ax.set_xlim(-0.7, len(arms) - 0.3)
n_ref = sum(rows[k].get(a, False) for k in draws for a in arms[1:])
ax.set_title(f"m = 3, ℓ = 8, N = 46: G4 refutes at degree 4 on {n_ref} of {16*len(draws)} branches\n"
             "filled = degree-4 closure refutes; hollow = saturates without 1", fontsize=9)

# Right: guesses for degree 4 vs l (m = 3).  l = 6: 0 (closure-driver study);
# l = 7: 1 guess (both branches) on 5 of 8 draws (specialisation study);
# l = 8: 4 guesses suffice on 4 of 4 draws, an upper bound (this study).
ells = [6, 7, 8, 9, 10]
bx.plot(ells, [3 * (l - 6) for l in ells], "--", color="tab:red", label="K_excess: m·(ℓ−6)")
bx.plot(ells, [max(0, 3 * (l - 7) + 1) if l >= 7 else 0 for l in ells], ":", color="tab:purple",
        label="K_marginal: m·(ℓ−7)+1")
bx.scatter([6], [0], color="black", zorder=3, label="measured")
bx.scatter([7], [1], color="black", zorder=3)
bx.annotate("5 of 8 draws", (7, 1), textcoords="offset points", xytext=(6, -12), fontsize=7)
bx.errorbar([8], [4], yerr=[[4], [0]], fmt="v", color="black", capsize=3, zorder=3)
bx.annotate("≤ 4 on 4 of 4\n(k < 4 not measurable here)", (8, 4), textcoords="offset points",
            xytext=(10, -40), fontsize=7)
bx.set_xlabel("ℓ (m = 3)", fontsize=8); bx.set_ylabel("guessed variables k for degree 4", fontsize=8)
bx.set_xticks(ells); bx.legend(fontsize=7, loc="upper left"); bx.grid(alpha=0.3)
bx.set_title("Guesses needed for degree 4", fontsize=9)
fig.suptitle("research/dreg_closure_guess_20261022, run 2026-10-22T05:55Z", fontsize=9)
fig.tight_layout()
fig.savefig(os.path.join(here, "guess.svg")); fig.savefig(os.path.join(here, "guess.png"), dpi=150)
