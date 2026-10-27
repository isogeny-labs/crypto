"""Lay out the recorded counts of research/s5_plain_c4_options_20261019/raw/.

Plots recorded values only (summary.json from the Rust summary, and the raw
per-solve JSON); computes nothing that the report quotes.
"""

import json
import os
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
RAW = os.path.join(HERE, "..", "raw")

main = json.load(open(os.path.join(RAW, "summary_r13.json")))
abl = json.load(open(os.path.join(RAW, "summary.json")))
r13 = json.load(open(os.path.join(RAW, "s5_p269_seed1_r13.json")))
raw4 = json.load(open(os.path.join(RAW, "s5_p269_seed1.json")))

ARMS = ["committed", "sparsest", "sparsest+chains", "sparsest+chains+replay"]
LABEL = {
    "committed": "committed\n(§11.17)",
    "sparsest": "+ sparsest",
    "sparsest+chains": "+ sparsest\n+ chains",
    "sparsest+chains+replay": "+ sparsest + chains\n+ replay",
}
PHASES = [
    ("echelon_muls", "elimination", "#4e79a7"),
    ("nf_muls", "normal forms", "#f28e2b"),
    ("charpoly_muls", "char. poly.", "#59a14f"),
    ("eigenvector_muls", "eigenvectors", "#b07aa1"),
]


def arm_summary(name):
    for src in (main, abl):
        for a in src["arms"]:
            if a["arm"] == name:
                return a, src is main
    raise KeyError(name)


def per_residual(name):
    src = r13 if name != "sparsest" else raw4
    out = []
    for res in src["residuals"]:
        for a in res["arms"]:
            if a["arm"] == name:
                if name.endswith("+replay") and a["solve"]["replay"] == "Recorded":
                    continue
                out.append((a["solve"]["total_muls"], a["solve"]["replay"]))
    return out


fig, (ax, bx) = plt.subplots(1, 2, figsize=(12.5, 5.2), gridspec_kw={"width_ratios": [1.35, 1]})

for i, name in enumerate(ARMS):
    s, _ = arm_summary(name)
    bottom = 0.0
    for key, lab, col in PHASES:
        v = s["mean"][key]
        ax.bar(i, v, bottom=bottom, color=col, width=0.6, label=lab if i == 0 else None)
        bottom += v
    pts = per_residual(name)
    for j, (c, rep) in enumerate(pts):
        ax.plot(i + 0.36, c, marker="x" if rep == "FellBack" else "o", ms=5,
                color="crimson" if rep == "FellBack" else "black", ls="none")
    ax.text(i, s["mean"]["total_muls"] * 1.25,
            f"{s['mean']['total_muls']:.3g}\n{s['ratio_of_means']:.2f}× (n={s['n']})",
            ha="center", va="bottom", fontsize=8)
ax.axhline(2 ** 36, color="grey", ls="--", lw=1)
ax.text(-0.45, 2 ** 36 * 0.8, "$D^3 = 2^{36}$", ha="left", va="top", fontsize=8, color="grey")
ax.set_yscale("log")
ax.set_ylim(1e10, 6e12)
ax.set_xticks(range(len(ARMS)))
ax.set_xticklabels([LABEL[a] for a in ARMS], fontsize=8)
ax.set_ylabel("$C_4$, $F_p$ multiplications per $S_5$ solve")
ax.set_title("Plain $S_5$, $p = 269$, seed 1 (§11.17 instances)", fontsize=10)
ax.legend(fontsize=7, loc="upper right")
ax.plot([], [], "ko", ms=4, label="solve")
ax.plot([], [], "x", color="crimson", ms=5, label="replay fell back")
ax.legend(fontsize=7, loc="upper right", ncol=2)

ds = sorted({g["equation_degree"] for g in main.get("generic", []) or abl["generic"]})
gen = abl["generic"]
for name, col, mk in [("sparsest", "#4e79a7", "o"), ("sparsest+chains", "#e15759", "s")]:
    xs = [g["equation_degree"] for g in gen if g["arm"] == name]
    ys = [g["ratio_vs_committed"] for g in gen if g["arm"] == name]
    s, _ = arm_summary(name)
    bx.plot(xs + [8], ys + [s["ratio_of_means"]], marker=mk, color=col, label=name)
    bx.plot([8], [s["ratio_of_means"]], marker=mk, color=col, ms=10, mfc="none")
s, _ = arm_summary("sparsest+chains+replay")
bx.plot([8], [s["ratio_of_means"]], marker="D", color="#59a14f", ms=7, ls="none",
        label="sparsest+chains+replay (S5 only)")
bx.set_xlabel("equation degree $d$ (four equations, four unknowns, $D = d^4$)\n$d = 8$ is $S_5$ (open marker); $d \\leq 7$ random planted systems")
bx.set_ylabel("committed $C$ / option $C$")
bx.set_xticks([4, 5, 6, 7, 8])
bx.set_title("Saving by degree, same solver", fontsize=10)
bx.grid(alpha=0.3)
bx.legend(fontsize=7)

fig.suptitle("Plain $S_5$ $C_4$ with the $D_4$ oracle's solver options (counted; run s5-plain-c4-options-20261019)", fontsize=10)
fig.tight_layout()
for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
    fig.savefig(os.path.join(HERE, f"c4_options.{ext}"), **kw)
print("wrote c4_options.svg/png", file=sys.stderr)
