# Layout only: plots the embedding degree of the pairing-friendly rows (exact)
# against the lower bound for the special-shape curves (k > 4000), from
# ../constructions.tsv and ../../snfs_prime_families_20261023/verified_families.tsv.
import csv
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
sp = list(csv.DictReader(open(os.path.join(here, "..", "constructions.tsv")), delimiter="\t"))
pf = list(csv.DictReader(open(os.path.join(here, "..", "..", "snfs_prime_families_20261023",
                                           "verified_families.tsv")), delimiter="\t"))
fig, ax = plt.subplots(figsize=(7.6, 4.4))
names = []
for r in pf[::2]:
    names.append(r["family"].split(" ")[0])
    ax.scatter(len(names) - 1, float(r["k(verified)"]), color="#2c6fbb", s=40, zorder=3)
base = len(names)
order = ["proth", "phi_d", "smooth_p_minus_1", "smooth_p_plus_1", "generic_control"]
labels = {"proth": "Proth/FFT", "phi_d": "Phi_d(m)", "smooth_p_minus_1": "smooth p-1",
          "smooth_p_plus_1": "smooth p+1", "generic_control": "random (control)"}
for i, f in enumerate(order):
    n = sum(1 for r in sp if r["family"].split("_")[0] == f.split("_")[0] and
            (r["family"] == f or r["family"].startswith(f + "_")))
    x = base + i
    ax.annotate("", xy=(x, 4000 * 3), xytext=(x, 4000), arrowprops=dict(arrowstyle="->", color="#c0392b", lw=1.6))
    ax.scatter(x, 4000, color="#c0392b", s=140, zorder=3, marker="_")
    ax.text(x, 4000 * 4, f"{n} curves", ha="center", fontsize=7)
    names.append(labels[f])
ax.set_yscale("log")
ax.set_ylim(5, 4e4)
ax.axhline(4000, color="#555", ls=":", lw=1)
ax.text(0, 4300, "search cap k = 4000", fontsize=7.5, color="#555")
ax.set_xticks(range(len(names)))
ax.set_xticklabels(names, rotation=30, ha="right", fontsize=8)
ax.set_ylabel("embedding degree k")
ax.set_title("Pairing families: k exact (blue). Other special shapes: k > 4000 (red, lower bound)", fontsize=9)
ax.grid(True, which="both", axis="y", alpha=0.2)
fig.text(0.01, 0.01, "192-bit primes; each red point is a CM-constructed curve certified by the group law (22 curves). "
         "Source: constructions.tsv, verified_families.tsv", fontsize=6.3, color="#444")
fig.tight_layout(rect=(0, 0.04, 1, 1))
fig.savefig(os.path.join(here, "k_lower_bounds.svg"))
fig.savefig(os.path.join(here, "k_lower_bounds.png"), dpi=150)
