# F3 figure. This script draws only from results/run.json (recorded values: curves[].rows eta_over_null_median,
# curves[].energy z / ratio_to_null_mean / band_3sd_over_mean); it computes no result.
# Usage: python plot.py <path-to-results/run.json> <out-dir>
# Run F3-RUN-02, study coord_spectrum_20261110. matplotlib only. Also writes table fragments (figures/table_*.typ,
# table_*.md) from the same recorded values, formatting only.
import json, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.lines import Line2D

run = json.load(open(sys.argv[1])); out = sys.argv[2]
curves = run["curves"]
GS = ["x", "y", "chi(x+0)", "chi(x+1)", "chi(x+2)", "dickson"]
COL = {"x": "#1f5fa8", "y": "#c45a00", "chi(x+0)": "#2a9d4b", "chi(x+1)": "#8a4fbf", "chi(x+2)": "#c2185b", "dickson": "#555555"}
MK = {"x": "o", "y": "s", "chi(x+0)": "^", "chi(x+1)": "v", "chi(x+2)": "D", "dickson": "P"}
LAB = {"x": "x", "y": "y", "chi(x+0)": "χ(x)", "chi(x+1)": "χ(x+1)", "chi(x+2)": "χ(x+2)", "dickson": "Dickson D_n"}
names = [c["name"] for c in curves]
pos = {}
i = 0; prev = None
for c in curves:
    if prev is not None and c["set"] != prev: i += 1.0
    pos[c["name"]] = i; i += 1; prev = c["set"]

fig, (axl, axr) = plt.subplots(1, 2, figsize=(14, 5.6), gridspec_kw=dict(width_ratios=[1.15, 1]))

# ---- left: eta / class-null median per function, per curve ----
axl.axhspan(0.8, 1.25, color="#2a9d4b", alpha=0.13, lw=0)
axl.axhline(1.0, color="#999999", lw=0.8)
axl.axhline(0.8, color="#2a9d4b", lw=0.9, ls="--"); axl.axhline(1.25, color="#2a9d4b", lw=0.9, ls="--")
axl.axhline(1.5, color="#c62828", lw=1.4, ls="-")
axl.text(len(names) + 0.9, 1.5, "rule 2: 1.5", color="#c62828", fontsize=8, va="bottom", ha="right")
axl.text(len(names) + 0.9, 1.255, "P1 band [0.8, 1.25]", color="#2a9d4b", fontsize=8, va="bottom", ha="right")
for c in curves:
    rows = {r["g"]: r for r in c["rows"]}
    for g in GS:
        axl.plot(pos[c["name"]] + 0.11 * (GS.index(g) - 2.5), rows[g]["eta_over_null_median"], MK[g], color=COL[g], ms=6, mec="white", mew=0.5, zorder=3)
axl.set_xticks([pos[n] for n in names]); axl.set_xticklabels(names, rotation=70, fontsize=7.5)
axl.set_ylim(0.3, 1.85); axl.set_ylabel("η (b≠0, max over 64 a) / median η of its symmetry-class null (20 nulls)")
axl.grid(alpha=0.25, axis="y")
axl.legend(handles=[Line2D([], [], marker=MK[g], color=COL[g], ls="", ms=6, label=LAB[g]) for g in GS],
           fontsize=8, loc="upper center", ncol=3, frameon=True, bbox_to_anchor=(0.5, 0.995))
axl.set_title("Fourier statistic η of the coordinate functions vs matched null", fontsize=10)

# ---- right: energy z per (curve, K) with +-3 band; ratio in the label ----
axr.axhspan(-3, 3, color="#2a9d4b", alpha=0.13, lw=0)
axr.axhline(0, color="#999999", lw=0.8)
axr.axhline(3, color="#2a9d4b", lw=0.9, ls="--"); axr.axhline(-3, color="#2a9d4b", lw=0.9, ls="--")
axr.text(len(names) + 0.9, 3.05, "P3 band: |z| ≤ 3 (ratio within 1 ± 3·sd/mean)", color="#2a9d4b", fontsize=8, va="bottom", ha="right")
for c in curves:
    for j, e in enumerate(c["energy"]):
        axr.plot(pos[c["name"]] + (-0.12 if j == 0 else 0.12), e["z"], "o" if j == 0 else "s",
                 color="#1f5fa8" if j == 0 else "#c45a00", ms=6, mec="white", mew=0.5, zorder=3)
axr.set_xticks([pos[n] for n in names]); axr.set_xticklabels(names, rotation=70, fontsize=7.5)
axr.set_ylim(-4.2, 4.2); axr.set_ylabel("z = (E(A_K) − null mean) / null sd   (50 random K-subsets of F_p)")
axr.grid(alpha=0.25, axis="y")
allr = [e["ratio_to_null_mean"] for c in curves for e in c["energy"]]
axr.legend(handles=[Line2D([], [], marker="o", color="#1f5fa8", ls="", ms=6, label="K = ⌈r^(1/2)⌉"),
                    Line2D([], [], marker="s", color="#c45a00", ls="", ms=6, label="K = ⌈r^(2/3)⌉")],
           fontsize=8, loc="lower center", ncol=2, frameon=True)
axr.set_title(f"Additive energy of A_K: z against the null, 36 (curve, K) pairs\n(recorded ratios {min(allr):.4f} to {max(allr):.4f}; band half-width 6e-5 to 1.2e-2)", fontsize=10)

fig.suptitle("F3-RUN-02 (coord_spectrum_20261110): spectra and additive energy of coordinate functions along <G>; 18 curves, local host", fontsize=11)
fig.text(0.01, 0.005, "Source: results/run.json (curves[].rows, curves[].energy). 20 nulls per class, 64 a-values, 50 energy nulls. "
         "Controls (λ, PLANT) not drawn: η = 1 (see tables). [pcc:unverified]", fontsize=7.5)
fig.tight_layout(rect=(0, 0.025, 1, 0.95))
for ext in ("svg", "png"):
    fig.savefig(f"{out}/eta_energy.{ext}", dpi=150)
plt.close(fig)

# ---- table fragments (formatting of recorded values only) ----
def f(v, d=3): return f"{v:.{d}f}"
def zs(v): return f"{v:+.2f}"
for S in ("R16", "R20", "ANOM"):
    cs = [c for c in curves if c["set"] == S]
    cols = GS + (["lambda"] if S == "ANOM" else []) + ["PLANT"]
    hdr = ["curve", "r", "x", "y", "χ(x)", "χ(x+1)", "χ(x+2)", "D_n", ] + (["λ η"] if S == "ANOM" else []) + ["PLANT η", "E ratio, z (K1)", "E ratio, z (K2)"]
    lines = []
    for c in cs:
        rows = {r["g"]: r for r in c["rows"]}
        cells = [c["name"], str(c["r"])]
        for g in GS: cells.append(f(rows[g]["eta_over_null_median"]))
        if S == "ANOM": cells.append(f(rows["lambda"]["eta"], 6))
        cells.append(f(rows["PLANT"]["eta"], 6))
        for e in c["energy"]:
            cells.append(f"K={e['K']}: {e['ratio_to_null_mean']:.5f}, {zs(e['z'])}")
        lines.append(cells)
    with open(f"{out}/table_{S}.md", "w") as fh:
        fh.write("| " + " | ".join(hdr) + " |\n|" + "---|" * len(hdr) + "\n")
        for l in lines: fh.write("| " + " | ".join(l) + " |\n")
    with open(f"{out}/table_{S}.typ", "w") as fh:
        fh.write(f"#table(columns: {len(hdr)}, align: (left,) + (right,) * {len(hdr)-1}, stroke: 0.4pt, inset: 2.5pt,\n  text(size: 7pt)[*" +
                 "*], text(size: 7pt)[*".join(hdr) + "*],\n")
        for l in lines:
            fh.write("  " + ", ".join(f"text(size: 7pt)[{x}]" for x in l) + ",\n")
        fh.write(")\n")
