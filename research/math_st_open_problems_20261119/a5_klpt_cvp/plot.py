"""Plot recorded A5 results: median log_p(ell^e) per bit band for A4 (basic) and A5 (CVP),
read from results/paired.csv (written by the Rust `pair` binary). Display only."""
import csv
import matplotlib; matplotlib.use("svg")
import matplotlib.pyplot as plt

rows = [r for r in csv.reader(open("results/paired.csv"))]
hdr = rows[0]; data = []
for r in rows[1:]:
    if r[0] == "ell": break
    data.append(dict(zip(hdr, r)))
BLUE, ORANGE, INK, MUTED = "#2a78d6", "#eb6834", "#0b0b0b", "#8a8984"
fig, ax = plt.subplots(figsize=(8.0, 5.0))
for ell, col in (("2", BLUE), ("3", ORANGE)):
    d = [r for r in data if r["ell"] == ell]
    x = [int(r["p_bits"]) for r in d]
    ax.plot(x, [float(r["median_A4_log_p_ell_e"]) for r in d], color=col, ls="--", marker="o", mfc="white",
            label=f"A4 basic KLPT, ℓ={ell}")
    ax.plot(x, [float(r["median_A5_log_p_ell_e"]) for r in d], color=col, ls="-", marker="o",
            label=f"A5 CVP strong approximation, ℓ={ell}")
for y, name in ((3.5, "p^3.5"), (3.0, "p^3"), (1.0, "exact optimum ≈ p/12")):
    ax.axhline(y, color=MUTED, lw=1, ls=":", zorder=0)
    ax.text(28.6, y + 0.03, name, color=INK, fontsize=9, ha="right")
ax.set_xticks([12, 20, 28]); ax.set_xlabel("bits of p")
ax.set_ylabel("median log_p of the ℓ-power norm returned")
ax.set_ylim(0.8, 4.3)
ax.set_title("KLPT on 105 matched ideals: CVP strong approximation vs basic (paired, 5 runs each)", fontsize=10)
ax.legend(fontsize=8, loc="center right", frameon=False)
for s in ("top", "right"): ax.spines[s].set_visible(False)
fig.tight_layout(); fig.savefig("a5_paired.svg")
