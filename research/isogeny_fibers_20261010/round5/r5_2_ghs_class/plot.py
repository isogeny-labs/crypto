"""Plot recorded R5.2 values (census.json, walk.json, walk_20.json). No new computation:
only ratios of recorded counts. Writes figure_data.tsv and three SVGs beside this file."""
import json, os, sys
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt

HERE = os.path.dirname(os.path.abspath(__file__))
C = ["#2a78d6", "#eb6834", "#1baf7a", "#eda100", "#e87ba4", "#008300"]
INK, INK2, GRID, SURF = "#0b0b0b", "#52514e", "#e4e3df", "#fcfcfb"
plt.rcParams.update({"font.size": 9, "axes.edgecolor": INK2, "axes.labelcolor": INK,
                     "xtick.color": INK2, "ytick.color": INK2, "figure.facecolor": SURF,
                     "axes.facecolor": SURF, "svg.fonttype": "none"})
census = json.load(open(os.path.join(HERE, "census.json")))
walk = json.load(open(os.path.join(HERE, "walk.json"))) + json.load(open(os.path.join(HERE, "walk_20.json")))
rows = []

# F1: fraction of curves with low-genus descent (LG: g <= 2n, n >= 3) per (N, l, n)
labels, fr, fcls = [], [], []
for r in census:
    for p in r["per_factorisation"]:
        if p["n"] < 3:
            continue
        c = p["counts"]
        f = c.get("weak_LG", 0) / c["curves"]
        labels.append(f"N={r['N']}  l={p['l']}, n={p['n']}")
        fr.append(f)
        fcls.append(p["models"]["LG"]["classes_with_weak"] / r["classes"])
        rows.append(("F1", r["N"], p["l"], p["n"], c.get("weak_LG", 0), c["curves"], f,
                     p["models"]["LG"]["classes_with_weak"], r["classes"]))
fig, ax = plt.subplots(figsize=(6.4, 6.2))
y = range(len(labels))
floor = 1e-6
ax.barh([i + 0.2 for i in y], [max(v, floor) for v in fr], height=0.38, color=C[0], label="share of curves")
ax.barh([i - 0.2 for i in y], [max(v, floor) for v in fcls], height=0.38, color=C[1], label="share of isogeny classes containing one")
ax.set_xscale("log"); ax.set_xlim(floor, 1.5)
ax.set_yticks(list(y)); ax.set_yticklabels(labels); ax.invert_yaxis()
for i, v in enumerate(fr):
    if v == 0:
        ax.text(1.4e-6, i + 0.2, "0 curves", va="center", fontsize=7, color=INK2)
ax.grid(axis="x", color=GRID, lw=0.6); ax.set_axisbelow(True)
ax.set_xlabel("share (log scale; bars at 1e-6 mean zero)", labelpad=2)
ax.set_title("Curves with a low-genus GHS descent (g ≤ 2n), exhaustive census", loc="left", fontsize=10)
ax.legend(loc="upper left", bbox_to_anchor=(0, -0.09), ncol=2, frameon=False, fontsize=8)
fig.text(0.01, 0.005, "R5.2 census.json, all b ∈ K*, both twists; measured counts, derived ratios. n = 2 rows omitted (g ≤ 2 never beats rho).", fontsize=7, color=INK2)
fig.tight_layout(rect=(0, 0.02, 1, 1)); fig.savefig(os.path.join(HERE, "fig1_low_genus_share.svg")); plt.close(fig)

# F2: class-best P-model margin (bits above rho) per N
fig, ax = plt.subplots(figsize=(6.4, 3.2))
Ns, mins, meds, maxs = [], [], [], []
for r in census:
    bm = sorted(c[7] for c in r["class_rows"] if c[7] is not None)
    Ns.append(r["N"]); mins.append(bm[0]); meds.append(bm[len(bm) // 2]); maxs.append(bm[-1])
    rows.append(("F2", r["N"], "", "", len(bm), r["classes"], bm[0], bm[len(bm) // 2], bm[-1]))
ax.vlines(Ns, mins, maxs, color=C[0], lw=2, label="range over classes")
ax.plot(Ns, meds, "o", color=C[0], ms=6, label="median class")
ax.plot(Ns, mins, "v", color=C[1], ms=7, label="best class")
ax.axhline(0, color=INK, lw=1); ax.text(19.0, -1.6, "rho parity (margin 0)", fontsize=7, color=INK2)
ax.set_xlabel("N (field degree)"); ax.set_ylabel("log2(IC cost / rho cost), bits")
ax.set_xticks(Ns); ax.grid(axis="y", color=GRID, lw=0.6); ax.set_axisbelow(True)
ax.set_title("Model P: best curve in each class is still above rho", loc="left", fontsize=10)
ax.legend(frameon=False, fontsize=8, loc="upper left")
fig.text(0.01, 0.01, "Model P = min(g!·q^(2−2/g), L_{q^g}[1/2,√2]) vs √(πr/4); best factorisation per curve.\nDerived from measured g, r (census.json).", fontsize=7, color=INK2)
ax.set_ylim(-2.5, None)
fig.tight_layout(rect=(0, 0.07, 1, 1)); fig.savefig(os.path.join(HERE, "fig2_model_p_margin.svg")); plt.close(fig)

# F3: LG5 walk: share of strong curves (in mixed classes) at each ℓ-distance, per N
fig, ax = plt.subplots(figsize=(6.4, 3.4))
k = 0
for r in walk:
    v = r["models"]["LG5"]
    if v["strong_curves_in_mixed"] == 0:
        continue
    tot = v["strong_curves_in_mixed"]
    h = {int(d): c for d, c in v["distance_hist_curves"].items()}
    ds = sorted(h)
    un = v["unreachable_strong_curves"] / tot
    ax.plot(ds, [h[d] / tot for d in ds], "-o", color=C[k], lw=2, ms=4,
            label=f"N={r['N']} ({v['mixed_classes']} classes; {un:.0%} unreachable)")
    for d in ds:
        rows.append(("F3", r["N"], d, "", h[d], tot, h[d] / tot, v["unreachable_strong_curves"], v["mixed_classes"]))
    k += 1
ax.set_xlabel("isogeny steps (ℓ ∈ {3,5,7}) to nearest low-genus curve")
ax.set_ylabel("share of strong curves")
ax.grid(color=GRID, lw=0.6); ax.set_axisbelow(True)
ax.set_title("Walk distance to a low-genus (n ≥ 5) curve, mixed classes", loc="left", fontsize=10)
ax.legend(frameon=False, fontsize=7)
fig.text(0.01, 0.01, "R5.2 walk.json; exhaustive over Galois-orbit reps, curve-weighted; measured. N=20 has no LG5 curves.", fontsize=7, color=INK2)
fig.tight_layout(rect=(0, 0.04, 1, 1)); fig.savefig(os.path.join(HERE, "fig3_walk_distance.svg")); plt.close(fig)

with open(os.path.join(HERE, "figure_data.tsv"), "w") as fh:
    fh.write("fig\tN\tl_or_dist\tn\tcount\ttotal\tshare_or_min\tcol8\tcol9\n")
    for row in rows:
        fh.write("\t".join(str(x) for x in row) + "\n")
