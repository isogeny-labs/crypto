"""Lay out recorded values from the cited files; computes nothing new.

Sources (all in aburan28/crypto):
  research/notes/ecc2k130/RESEARCH_ECC2K130_DESCENT_DEGREE.md (rows 6-8): m=4 prices
  research/dreg_closure_driver_20261016/RESULTS.md: (4,6) needs ~70 GB dense at degree 5
  research/ic_next10_20261014/03-pkm-degree/README.md: msolve 5 GB cap, f4_fp_tower N=20 > 14 GB
  backlog/XP-20261015-pkm-m3-n21-m4-n20: 256-512 GB host
  research/ic_next10_20261014/09-equal-advice/README.md: 1 GiB cap, n <= 63 instrument
  research/ic_next10_20261014/README.md: shared 4-vCPU 15 GB container
"""
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

# Figure 1: ECC2K-130, m = 4 Weil-descent decomposition, log2 cost (unconverted column-touch units) vs rho
rho = 60.81
rows = [
    ("rho (signed Frobenius,\nmeasured constant)", rho, "#555555"),
    ("D = 4 held to n = 131\n(contradicted 2026-10-16)", 68.43, "#9ecae1"),
    ("D = 5 held to n = 131\n(optimistic row, open)", 71.13, "#3182bd"),
    ("XL fit (ell + 4 law)\n(three-rung extrapolation)", 173.13, "#de2d26"),
]
fig, ax = plt.subplots(figsize=(7.2, 3.6))
labels = [r[0] for r in rows]
vals = [r[1] for r in rows]
cols = [r[2] for r in rows]
bars = ax.barh(labels, vals, color=cols)
for b, v in zip(bars, vals):
    ax.text(v + 1.5, b.get_y() + b.get_height() / 2, f"2^{v:.2f}" + ("" if v == rho else f"  (+{v - rho:.2f} bits)"),
            va="center", fontsize=8)
ax.axvline(rho, color="#555555", ls="--", lw=0.8)
ax.set_xlim(0, 200)
ax.set_xlabel("log2 cost at ECC2K-130 (m = 4 rows: unconverted column touches; rho: group operations)", fontsize=8)
ax.set_title("ECC2K-130: what the open m = 4 solving-degree question is worth", fontsize=10)
ax.invert_yaxis()
fig.tight_layout()
fig.savefig("ecc2k130_degree_rows.svg")
fig.savefig("ecc2k130_degree_rows.png", dpi=150)

# Figure 2: memory needed by the deciding cells vs memory the program has had
cells = [
    ("ic_next10 per-item cap (3 GB)", 3, "have"),
    ("msolve PKM cap (5 GB)", 5, "have"),
    ("shared container (15 GB)", 15, "have"),
    ("PKM f4_fp_tower N = 20 (> 14 GB)", 14, "need"),
    ("Weil descent (4,6) / degree 5 at N = 40 (~70 GB)", 70, "need"),
    ("PKM m = 3 N = 21 .. m = 4 N = 20 (256-512 GB)", 512, "need"),
]
fig, ax = plt.subplots(figsize=(7.2, 3.4))
names = [c[0] for c in cells]
gb = [c[1] for c in cells]
colors = ["#74c476" if c[2] == "have" else "#fd8d3c" for c in cells]
bars = ax.barh(names, gb, color=colors)
for b, v, c in zip(bars, gb, cells):
    ax.text(v * 1.08, b.get_y() + b.get_height() / 2, f"{v} GB" + (" (lower bound)" if c[0].startswith("PKM f4") else ""), va="center", fontsize=8)
ax.set_xscale("log")
ax.set_xlim(1, 3000)
ax.set_xlabel("RAM, GB (log scale). Green: what runs have had. Orange: what the blocked cells need.", fontsize=8)
ax.set_title("Open exponent questions: memory needed vs memory had", fontsize=10)
ax.invert_yaxis()
fig.tight_layout()
fig.savefig("memory_wall.svg")
fig.savefig("memory_wall.png", dpi=150)
print("ok")
