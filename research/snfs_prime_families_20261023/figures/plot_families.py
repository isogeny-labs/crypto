# Layout only: plots log2 p^k (the size of the field NFS must work in) per
# verified family, from ../verified_families.tsv. Computes nothing else.
import csv
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
rows = list(csv.DictReader(open(os.path.join(here, "..", "verified_families.tsv")), delimiter="\t"))
rows = [r for r in rows if r["checks"].startswith("ok")]
seen, fam = set(), []
for r in rows:
    if r["family"] not in seen:
        seen.add(r["family"])
        fam.append(r)
fig, ax = plt.subplots(figsize=(7.2, 4.0))
xs = range(len(fam))
ax.bar(xs, [float(r["log2 p^k"]) for r in fam], color="#2c6fbb")
for x, r in zip(xs, fam):
    ax.text(x, float(r["log2 p^k"]) + 120, f"k={r['k(verified)']}\nlog2 p={r['log2 p']}", ha="center", fontsize=7.5)
ax.set_xticks(list(xs))
ax.set_xticklabels([r["family"].split(" ")[0] for r in fam], fontsize=9)
ax.set_ylabel("log2 of the pairing target field size p^k (bits)")
ax.set_ylim(0, 13500)
ax.set_title("Verified pairing-friendly families at about 256-bit subgroup order", fontsize=10)
ax.grid(True, axis="y", alpha=0.25)
fig.text(0.01, 0.01, "Each row verified by exact arithmetic: p, r prime; r divides p+1-t; embedding degree equals k. "
         "Source: verified_families.tsv", fontsize=6.5, color="#444")
fig.tight_layout(rect=(0, 0.04, 1, 1))
fig.savefig(os.path.join(here, "families.svg"))
fig.savefig(os.path.join(here, "families.png"), dpi=150)
