"""Plot recorded R7.7 values only (no result is computed here).

Reads exhaustive.json, exhaustive_alla.json, random.json, control*.json,
cost.json in this directory; writes fig1_labels.svg, fig2_evidence.svg,
fig3_cost.svg and plot_data.json (the numbers that were plotted).
"""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch

D = os.path.dirname(os.path.abspath(__file__))


def ld(f):
    return json.load(open(os.path.join(D, f)))


ex, ea, rn = ld("exhaustive.json"), ld("exhaustive_alla.json"), ld("random.json")
c1, c2, c1r = ld("control1_wrong_a.json"), ld("control2_drop_term.json"), ld("control1_random.json")
cost = ld("cost.json")

TXT, SEC, GRID, SURF = "#0b0b0b", "#52514e", "#e4e3df", "#fcfcfb"
BLUE, ORANGE, GREEN, AMBER, GREY = "#2a78d6", "#eb6834", "#1baf7a", "#eda100", "#9a9994"
plt.rcParams.update({
    "font.size": 9, "text.color": TXT, "axes.labelcolor": SEC, "xtick.color": SEC,
    "ytick.color": SEC, "axes.edgecolor": GRID, "svg.fonttype": "none",
})
plot_data = {}


def tot(runs, key, idx=None, vfilter=None):
    """Sum an aggregate field over the groups of the given runs."""
    s = 0
    for r in runs:
        for g in r["groups"]:
            if vfilter and not vfilter(g):
                continue
            v = g["agg"][key]
            s += v[idx] if idx is not None else v
    return s


# ---------------------------------------------------------------- fig 1 ----
fig, ax = plt.subplots(figsize=(9.6, 5.6), facecolor=SURF)
ax.set_facecolor(SURF)
ax.set_xlim(0, 100)
ax.set_ylim(0, 60)
ax.axis("off")


def box(x, y, w, h, text, fc, tc="#ffffff", fs=8.5):
    ax.add_patch(FancyBboxPatch((x, y), w, h, boxstyle="round,pad=0.0,rounding_size=0.8",
                                fc=fc, ec="none"))
    ax.text(x + w / 2, y + h / 2, text, ha="center", va="center", fontsize=fs, color=tc)


b2h = [[0, 0], [0, 0]]
for r in (ex, ea, rn):
    for g in r["groups"]:
        if g["vcat"] == 1:
            for k in range(2):
                for b in range(2):
                    b2h[k][b] += g["agg"]["b2_hist"][k][b]
frac = [b2h[k][1] / (b2h[k][0] + b2h[k][1]) for k in range(2)]
plot_data["v1_b2_hist"] = b2h
plot_data["v1_b2_frac_one"] = frac

ax.text(0, 58, "Label of E/2^J E on  y² + xy = x³ + a x² + b,  n odd  (k = coset, relative to a reference odd point Q0)",
        fontsize=10, weight="bold")
# row 1: v = 1
ax.text(0, 52, "Tr(a) = 1  (v = 1, #E ≡ 2 mod 4):  E/4E = E/8E = Z/2", fontsize=9, color=SEC)
box(2, 44, 20, 5.5, "k = 0 (P ∈ 2E)\nlabel = Tr(x)+Tr(a) = 0", BLUE)
box(24, 44, 20, 5.5, "k = 1 (P ∉ 2E)\nlabel = Tr(x)+Tr(a) = 1", ORANGE)
ax.text(47, 48.5, f"second trace bit is NOT a coset invariant:\nit equals 1 on {100*frac[0]:.2f}% (k=0) and {100*frac[1]:.2f}% (k=1) of points",
        fontsize=8.5, va="center")
ax.text(47, 44.6, "on 2E it is the root selector of the halving inside the odd-order subgroup", fontsize=8.5, va="center", color=SEC)
# row 2: v = 2
ax.text(0, 38, "Tr(a) = 0, Tr(b) = 1  (v = 2, Z/4):  pair (Tr(x), b₂),  b₂ = Tr(y) + Tr(x·H(x+a)),  β = Tr(b·H(b))", fontsize=9, color=SEC)
for i, (lab, col) in enumerate([("k=0\n00", BLUE), ("k=1 (P4)\n1β", ORANGE), ("k=2 (T)\n01", GREEN), ("k=3\n1(1+β)", AMBER)]):
    box(2 + i * 12, 30, 11, 6, lab, col, tc="#ffffff" if i in (0, 1) else TXT)
ax.text(51, 33, "P4 = (b^¼, b^¼ (H(a)+b^¼+1)),  2·P4 = T", fontsize=8.5, va="center", color=SEC)
# row 3: v >= 3
ax.text(0, 24, "Tr(a) = Tr(b) = 0  (v ≥ 3, Z/2^v):  triple (Tr(x), b₂, b₃),  Q0 odd with b₂(Q0) = 0", fontsize=9, color=SEC)
labs8 = ["000", "100", "010", "111", "001", "101", "011", "110"]
cols = [BLUE, ORANGE, GREEN, AMBER, "#7b5ec9", "#b13a8a", "#2f8f9d", "#6b6b6b"]
for k in range(8):
    box(2 + k * 11.2, 16, 10.2, 6, f"k={k}\n{labs8[k]}", cols[k], tc="#ffffff" if k in (0, 1, 4, 5, 6, 7) else TXT)
ax.text(2, 12.3, "b₃(R) = b₂(R/2) for even R (either half);  b₃(P) = b₃(P+Q0) for odd P.   Level J: (bit1, b₂(S₁), …, b₂(S_{J-1})), S_{i+1} = half(S_i or S_i+Q0).",
        fontsize=8.2, color=SEC)
ax.text(2, 8.5, "Measured: all four of E/4E, E/8E, E/16E, E/32E labels match this table (v ≥ 2, 3, 4, 5); see evidence figure.", fontsize=8.5)
ax.text(2, 4.5, "L is a set bijection E/2^J E → F₂^J, not a group map (Z/2^J has no injective additive map into F₂^J, J ≥ 2).", fontsize=8.5, color=SEC)
fig.savefig(os.path.join(D, "fig1_labels.svg"), facecolor=SURF)
plt.close(fig)

# ---------------------------------------------------------------- fig 2 ----
runs = (ex, ea, rn)
rows = []
v1pts = tot(runs, "points", vfilter=lambda g: g["vcat"] == 1)
v1mis = tot(runs, "mis_coset1", vfilter=lambda g: g["vcat"] == 1)
v1nc = tot(runs, "pair_nonconst_cosets")
v1cos = tot(runs, "pair_cosets")
plot_data["v1_pair_nonconst"] = {"nonconstant_cosets": v1nc, "cosets": v1cos}
rows.append(("v=1: coset = Tr(x)+Tr(a)", v1pts, v1mis, 0, 0))
for J, name in [(2, "v≥2: E/4E pair"), (3, "v≥3: E/8E triple"), (4, "v≥4: E/16E"), (5, "v≥5: E/32E")]:
    rows.append((name, tot(runs, "points_j", J), tot(runs, "mis", J), tot(runs, "nonconst", J), tot(runs, "coll", J)))
plot_data["evidence_rows"] = [dict(zip(["case", "points", "mispredicted", "nonconstant_cosets", "collisions"], r)) for r in rows]

ctl = {}
for nm, c in (("control 1: forgets a", c1), ("control 2: drops Tr(xH)", c2)):
    ctl[nm] = [(tot([c], "mis", J), tot([c], "points_j", J)) for J in (2, 3, 4, 5)]
ctl["control 1 (random, a≠0)"] = [(tot([c1r], "mis", J), tot([c1r], "points_j", J)) for J in (2, 3, 4, 5)]
plot_data["controls"] = ctl

fig, (a1, a2) = plt.subplots(1, 2, figsize=(10.4, 3.9), facecolor=SURF, gridspec_kw={"width_ratios": [1.15, 1]})
for a in (a1, a2):
    a.set_facecolor(SURF)
    for sp in ("top", "right"):
        a.spines[sp].set_visible(False)
ys = list(range(len(rows)))[::-1]
a1.barh(ys, [r[1] for r in rows], color=BLUE, height=0.55)
a1.set_xscale("log")
a1.set_yticks(ys)
a1.set_yticklabels([r[0] for r in rows])
for y, r in zip(ys, rows):
    if r[0].startswith("v=1"):
        txt = (f"{r[1]:,} pts; coset rule mispredicted {r[2]}, collisions {r[4]}\n"
               f"(the pair itself: non-constant on {v1nc:,} of {v1cos:,} cosets)")
    else:
        txt = f"{r[1]:,} pts;  mispredicted {r[2]}, non-const {r[3]}, collisions {r[4]}"
    a1.text(r[1] * 1.15, y, txt, va="center", fontsize=7.8)
a1.set_xlim(1e4, 3e12)
a1.set_xlabel("points tested (exhaustive + all-a + random; log)")
a1.set_title("Real label: failures per case", fontsize=9, loc="left")
labs = ["J=2", "J=3", "J=4", "J=5"]
w = 0.26
for i, (nm, vals) in enumerate(ctl.items()):
    fr = [100.0 * m / max(p, 1) for m, p in vals]
    a2.bar([j + (i - 1) * w for j in range(4)], fr, width=w, label=nm)
a2.set_xticks(range(4))
a2.set_xticklabels(labs)
a2.set_ylabel("mispredicted points (%)")
a2.set_title("Negative controls (broken labels): the test fires", fontsize=9, loc="left")
a2.set_ylim(0, 70)
a2.legend(fontsize=7.5, frameon=False, loc="upper left")
fig.tight_layout()
fig.savefig(os.path.join(D, "fig2_evidence.svg"), facecolor=SURF)
plt.close(fig)

# ---------------------------------------------------------------- fig 3 ----
NMAX = max(r["n"] for r in cost["rows"])


def ops(vcat, method, parity, n=NMAX):
    for r in cost["rows"]:
        if r["n"] == n and r["vcat"] == vcat and r["method"] == method and r["parity"] == parity:
            return r["mean_ops"]
    raise KeyError((vcat, method, parity))


OPC = {"M": BLUE, "S": GREEN, "I": ORANGE, "H": AMBER, "T": GREY}
groups = [
    ("J=2 label\neven", ops(2, "label2", "even")), ("J=2 halving\neven", ops(2, "halving2", "even")),
    ("J=2 label\nodd", ops(2, "label2", "odd")), ("J=2 halving\nodd", ops(2, "halving2", "odd")),
    ("J=3 label\neven", ops(3, "label3", "even")), ("J=3 halving\neven", ops(3, "halving3", "even")),
    ("J=3 label\nodd", ops(3, "label3", "odd")), ("J=3 halving\nodd", ops(3, "halving3", "odd")),
]
sc2 = ops(2, "scalar", "even")
plot_data["cost_n"] = NMAX
plot_data["cost_groups"] = [{"name": g[0].replace("\n", " "), "ops": {k: g[1][k] for k in "MSIHT"}} for g in groups]
plot_data["cost_scalar_v2_even"] = {k: sc2[k] for k in "MSIHT"}
fig, ax = plt.subplots(figsize=(9.6, 3.9), facecolor=SURF)
ax.set_facecolor(SURF)
for sp in ("top", "right"):
    ax.spines[sp].set_visible(False)
for i, (nm, o) in enumerate(groups):
    bot = 0
    for k in "MSIHT":
        ax.bar(i, o[k], bottom=bot, color=OPC[k], width=0.62, label=k if i == 0 else None)
        bot += o[k]
    ax.text(i, bot + 0.3, f"{bot:.1f}", ha="center", fontsize=7.5)
ax.set_xticks(range(len(groups)))
ax.set_xticklabels([g[0] for g in groups], fontsize=7.8)
ax.set_ylabel(f"counted F_(2^n) operations per point, n = {NMAX}")
ax.set_ylim(0, 22)
ax.legend(title="op", fontsize=7.5, frameon=False, ncol=5, loc="upper left")
ax.set_title(f"Counted ops of the trace label vs halving (mean per point). Scalar-mult truth, J=2: "
             f"M={sc2['M']:.0f}, S={sc2['S']:.0f}, I={sc2['I']:.0f}", fontsize=9, loc="left")
fig.tight_layout()
fig.savefig(os.path.join(D, "fig3_cost.svg"), facecolor=SURF)
plt.close(fig)

json.dump(plot_data, open(os.path.join(D, "plot_data.json"), "w"), indent=1)
print("ok")
