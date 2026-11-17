"""Summary graph for the twist-ECDLP study. Plots ONLY values recorded in the experiment
directories (copied below with their source file); nothing is computed or invented here.

Panel (a): counted-cost ratio  (twist-based arm) / (matched baseline arm), log axis, 1 = tie.
Panel (b): G3 observed / structurally predicted number of mixed (twist) Semaev solutions.
"""
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

OUT = "/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/summary"

# (label, low, high, kind, source)  kind: 'meas' measured counted ops, 'model' model-derived
ROWS = [
    # E2 RESULTS.md primary table: sav in [-0.062, 0] over 9 cells and w in {0,0.1,1};
    # ratio = C_both/(C_E+C_E') = 1 - sav
    ("E2  one shared table for E and E' vs\n      separate E-alone + E'-alone runs", 1.000, 1.062, "meas"),
    # E3 results.csv / RESULTS.md: rho_gen over 33 powered cells
    ("E3  F_{p^2} eigen-route vs direct Semaev,\n      cost per E-relation", 53.3, 2812.0, "meas"),
    # E4 RESULTS.md cost S/E median over 18 random-V cells with l>=6 (1.91-2.13); full table 1.9099-2.2143
    ("E4  Koblitz K0+K1 shared base vs E-only base,\n      counted solves per K0 relation", 1.9099, 2.2143, "meas"),
    # E5 summary.txt cost ratios b>=28, I/M = 100 .. 3 (S = 0.8 M)
    ("E5  Kummer walk, K-mix vs additive rho A0\n      (I/M = 100 ... 3)", 1.003, 4.625, "meas"),
    ("E5  Kummer walk, K-pure vs additive rho A0\n      (I/M = 100 ... 3)", 1.349, 6.221, "meas"),
    # G1 summary.txt / ratios.csv, w = 0, I/M = 100 .. 3
    ("G1  K-mix vs sound negation-map rho N1\n      (I/M = 100 ... 3)", 1.287, 5.907, "meas"),
    ("G1  K-pure vs sound negation-map rho N1\n      (I/M = 100 ... 3)", 1.785, 8.193, "meas"),
    # G2 RESULTS.md: closest decoupled arm PF(j=m,k=1) range over primary pooled cells
    ("G2  decoupled A/B projection, closest arm\n      (j=m, k=1) vs direct D_m", 1.0069, 1.1111, "meas"),
    ("G2  decoupled A/B projection, like-for-like\n      (j=m, k=m) vs direct D_m", 2.087, 8.333, "meas"),
    # E6 RESULTS.md operation-count table: model IC ops / rho ops (l = #E), p = 101 .. 20011
    ("E6  genus-2 index calculus (model, exponent 1)\n      vs rho on E, p = 101 ... 20011", 8.019, 112.7, "model"),
]

# Panel (b): G3 f4_table.tsv / RESULTS.md ratio c_hat/c_ref with 95% hierarchical interval
B = [
    ("12 bit\ngrid", 0.9965, 0.9458, 1.0476),
    ("16 bit\ngrid", 0.9935, 0.8302, 1.1412),
    ("20 bit\ngrid", 0.8465, 0.2934, 1.4702),
    ("20 bit\nR20", 1.1265, 0.3288, 1.8424),
]

C_MEAS = "#2a6f97"     # blue
C_MODEL = "#8d99ae"    # grey-blue (model, not measured)
C_REF = "#c1121f"      # red reference line
INK = "#1d1d1f"

plt.rcParams.update({
    "font.family": "DejaVu Sans", "font.size": 9, "axes.edgecolor": "#888888",
    "axes.labelcolor": INK, "xtick.color": INK, "ytick.color": INK, "text.color": INK,
})

fig, (ax, bx) = plt.subplots(
    1, 2, figsize=(14, 6.2), gridspec_kw={"width_ratios": [2.3, 1], "wspace": 0.55})

# ---- panel (a)
n = len(ROWS)
for i, (lab, lo, hi, kind) in enumerate(ROWS):
    y = n - 1 - i
    col = C_MEAS if kind == "meas" else C_MODEL
    ax.barh(y, hi - lo, left=lo, height=0.52, color=col,
            hatch="////" if kind == "model" else None,
            edgecolor="white" if kind == "meas" else "#5c677d", linewidth=0.8, zorder=3)
    ax.text(hi * 1.08, y, f"{lo:g} to {hi:g}", va="center", ha="left", fontsize=8.2, zorder=4)
ax.set_xscale("log")
ax.set_xlim(0.8, 9000)
ax.set_yticks(range(n))
ax.set_yticklabels([r[0] for r in ROWS][::-1], fontsize=8.2)
ax.axvline(1.0, color=C_REF, lw=1.6, zorder=2)
ax.text(1.0, n - 0.35, " tie (=1)", color=C_REF, fontsize=8.5, va="bottom", ha="left")
ax.axvspan(0.8, 1.0, color=C_REF, alpha=0.08, zorder=1)
ax.text(0.9, -0.62, "<1: no row", color=C_REF, fontsize=7.6,
        ha="right", va="top")
ax.set_ylim(-1.2, n - 0.1)
ax.set_xlabel("counted cost of the twist-based arm / counted cost of the matched baseline arm   "
              "(>1 = twist arm costs more; log scale)")
ax.set_title("(a) No tested twist-based arm is cheaper than its matched baseline", loc="left",
             fontsize=10.5, fontweight="bold")
ax.grid(axis="x", which="major", color="#dddddd", lw=0.6, zorder=0)
for s in ("top", "right"):
    ax.spines[s].set_visible(False)

# ---- panel (b)
m = len(B)
for i, (lab, pt, lo, hi) in enumerate(B):
    x = i
    bx.plot([x, x], [lo, hi], color=C_MEAS, lw=2.2, zorder=3, solid_capstyle="butt")
    bx.plot([x], [pt], "o", color=C_MEAS, ms=7, zorder=4)
    bx.text(x + 0.1, pt - 0.07, f"{pt:.3f}", fontsize=8, va="center", ha="left")
bx.axhspan(0.8, 1.25, color="#2a9d8f", alpha=0.13, zorder=1)
bx.axhline(1.0, color=C_REF, lw=1.2, zorder=2)
bx.text(-0.45, 1.27, "preregistered band [0.8, 1.25]", fontsize=7.6, color="#1b6b61", ha="left")
bx.text(m - 0.45, 0.04, "24 bit: unresolved (0 mixed in\n720,000 targets; expected 0.061)",
        fontsize=7.4, ha="right", va="bottom", color="#555555")
bx.set_xticks(range(m))
bx.set_xticklabels([b[0] for b in B], fontsize=8)
bx.set_xlim(-0.5, m - 0.2)
bx.set_ylim(0, 2.0)
bx.set_ylabel("observed / predicted mixed solutions\n(m=3, n_T>0; 95% interval)")
bx.set_title("(b) G3: mixed-solution count\nvs structural prediction", loc="left",
             fontsize=10.5, fontweight="bold")
for s in ("top", "right"):
    bx.spines[s].set_visible(False)

fig.text(0.012, 0.012,
         "Solid blue = measured counted operations (E2-E5, G1, G2); hatched grey = model-derived, not measured (E6). "
         "Rows are ranges over the tested cells / pricing band. Toy to 40-bit fields only.\n"
         "E0 (separation lemma, 0 violations in 8254 + 34498 classes) and E1+G3 (mixed solutions = predicted family exactly, "
         "0 target mismatches) are exact-structure results with no cost ratio; E6 row is a model ratio.",
         fontsize=7.8, color="#444444", ha="left", va="bottom")
fig.subplots_adjust(left=0.235, right=0.985, top=0.9, bottom=0.17)
fig.savefig(OUT + ".svg")
fig.savefig(OUT + ".png", dpi=170)
print("wrote", OUT + ".svg", OUT + ".png")
