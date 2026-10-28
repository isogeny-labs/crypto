"""Layout only.  Reads committed records and counts, per cell, on how many
draws the degree-4 MutantXL closure refutes.  Nothing is recomputed: each bar
is the number of draws whose record shows closure_refuted at degree 4."""
import glob, json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
repo = os.path.abspath(os.path.join(here, "../../../.."))
rel = lambda p: os.path.join(repo, p)

def refutes(path):
    r = json.loads(open(path).readline())
    return r.get("closure_degree") is not None and r["closure_degree"] <= 4

def count(paths):
    return sum(refutes(p) for p in paths), len(paths)

S = "research/dreg_closure_null22_7_20261028/runs/"
cells = []
# unspecialised l = 7 draws of the closure-at-scale study (N = 40, 42) and this study (N = 43)
for n, N in ((19, 40), (21, 42)):
    ps = sorted(glob.glob(rel(f"research/dreg_closure_scale_20261016/runs/fresh-m3-{n}-7.d*.jsonl")))
    cells.append((f"unspecialised ℓ=7, n={n}, N={N}", count(ps), "null"))
ps = [rel(S + f"m3-22-7.d{k}.N0.jsonl") for k in (1, 2, 3, 4)]
cells.append(("unspecialised ℓ=7, n=22, N=43  (this study, N0)", count(ps), "this"))
# the three-guess study at l = 8: a draw counts when every run of the set refutes
def all_branches(prefix, glob_pat, draws):
    n = 0
    for d in draws:
        ps = sorted(glob.glob(rel(glob_pat.format(d=d))))
        n += bool(ps) and all(refutes(p) for p in ps)
    return n, len(draws)
D = (0, 3, 5, 6)
cells.append(("ℓ=8, 3 guesses (7,15,23), N=43", all_branches("g3", "research/dreg_closure_guess3_20261023/runs/m3-22-8.d{d}.G3b*.jsonl", D), "g3"))
cells.append(("ℓ=8, 4 guesses (7,15,23,24), N=42", all_branches("g4", "research/dreg_closure_guess_20261022/runs/m3-22-8.d{d}.G4b*.jsonl", D), "pos"))
cells.append(("ℓ=8, dimension-6 control G6, N=40", all_branches("g6", "research/dreg_closure_guess3_20261023/runs/m3-22-8.d{d}.G6.jsonl", D), "pos"))
ps = [rel(S + f"m3-22-7.d{k}.B.jsonl") for k in (1, 2, 3, 4)]
cells.append(("ℓ=7, dimension-6 control B, N=40  (this study)", count(ps), "pos"))

colour = {"null": "#8a8a8a", "this": "#d1495b", "g3": "#e0a030", "pos": "#2e86ab"}
fig, ax = plt.subplots(figsize=(9.5, 4.6))
for i, (label, (k, n), kind) in enumerate(cells):
    ax.barh(i, k / n, color=colour[kind], edgecolor="black", linewidth=0.8)
    ax.text(k / n + 0.015, i, f"{k} of {n}", va="center", fontsize=9)
ax.set_yticks(range(len(cells)))
ax.set_yticklabels([c[0] for c in cells], fontsize=8)
ax.invert_yaxis()
ax.set_xlim(0, 1.15)
ax.set_xlabel("fraction of draws on which the degree-4 closure refutes", fontsize=9)
ax.axvline(0.5, color="black", lw=0.5, ls=":")
ax.set_title("Draws on which the degree-4 closure refutes, by cell\n"
             "research/dreg_closure_null22_7_20261028, run 2026-10-28T03:35Z", fontsize=9)
fig.tight_layout()
fig.savefig(os.path.join(here, "null.svg")); fig.savefig(os.path.join(here, "null.png"), dpi=150)
for c in cells:
    print(c)
