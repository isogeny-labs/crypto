"""E9b run report figure: mean S per arm per curve from ../../sessions/e9b_a1.table.md
(ecbench table; S with the solver priced at its pinned word-XOR ratio), with matched rho.
Lays out recorded values only."""
import os, re
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt
here = os.path.dirname(os.path.abspath(__file__))
rows = {}
for line in open(os.path.join(here, "../../sessions/e9b_a1.table.md")):
    c = [x.strip() for x in line.strip().strip("|").split("|")]
    if len(c) < 8 or not c[0].startswith("ECBS"):
        continue
    arm, curve, s, ci = c[1], c[3], float(c[6]), c[7]
    lo, hi = [float(v) for v in re.findall(r"[\d.]+", ci)[:2]]
    rows.setdefault(curve, {})[arm] = (s, lo, hi)
arms = ["mitm-default", "mitm-coset", "f4-default", "f4early-default", "f4-coset", "f4early-coset"]
colors = ["#c0632b", "#3b6ea8", "#e3a47a", "#c0632b", "#8fb0d6", "#3b6ea8"]
hatches = ["", "", "", "//", "", "//"]
curves = sorted(rows, key=lambda k: int(re.search(r"f2m(\d+)", k).group(1)))
fig, ax = plt.subplots(figsize=(9, 4.4))
w = 0.13
for i, arm in enumerate(arms):
    xs = [j + (i - 2.5) * w for j in range(len(curves))]
    ys = [rows[c][arm][0] / rows[c]["rho-neg"][0] for c in curves]
    err = [[(rows[c][arm][0] - rows[c][arm][1]) / rows[c]["rho-neg"][0] for c in curves],
           [(rows[c][arm][2] - rows[c][arm][0]) / rows[c]["rho-neg"][0] for c in curves]]
    ax.bar(xs, ys, w, yerr=err, color=colors[i], hatch=hatches[i], edgecolor="black", lw=0.4, label=arm, capsize=2)
ax.axhline(1, color="black", ls=":", lw=1); ax.text(-0.45, 1.15, "rho", fontsize=8)
ax.set_yscale("log", base=2)
ax.set_xticks(range(len(curves))); ax.set_xticklabels([c.replace("icv1-", "") for c in curves], fontsize=8)
ax.set_ylabel("mean S ÷ mean rho S (same curve)")
ax.set_title("E9b, a = 1 Koblitz curves: coset MITM wins at n = 19, 23; F4 stays 10–700× above rho", fontsize=9)
ax.legend(fontsize=7, ncol=3); ax.grid(axis="y", alpha=0.3)
fig.tight_layout(); fig.savefig(os.path.join(here, "e9b_s_over_rho.svg")); print("ok")
