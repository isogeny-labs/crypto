"""Render the paper-opportunities figure from data.json (recorded values only).

    python3 make_figures.py   # writes fall-degrees.{pdf,svg,png}
"""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(HERE, "data.json")))
plt.rcParams.update({"font.size": 8.5, "font.family": "serif", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.alpha": 0.25,
                     "svg.fonttype": "none"})
C = ["#0072B2", "#E69F00", "#009E73", "#CC79A7", "#999999", "#D55E00"]

fig, (a, b) = plt.subplots(1, 2, figsize=(7.2, 3.0))

r = D["binary_m2"]["random_V"]
for n, d, lb in zip(r["n"], r["dLFD"], r["lower_bound_only"]):
    a.plot(n, d, "^" if lb else "o", color=C[0], ms=6 if lb else 5, mfc="white" if lb else C[0], zorder=3)
p = D["binary_m2"]["polynomial_V"]
for n, ds, lb in zip(p["n"], p["dLFD_values"], p["lower_bound_only"]):
    for d in ds:
        a.plot(n + 0.35, d, "^" if lb else "s", color=C[1], ms=6 if lb else 4.5, mfc="white" if lb else C[1], zorder=3)
ns = list(range(10, 47))
a.plot(ns, [2] * len(ns), color=C[2], lw=1.6, label="first fall degree = 2 (proved, every n)")
a.plot(ns, [(n + 1) // 2 + 2 for n in ns], color=C[4], ls=":", lw=1, label="upper bound k+2 (proved)")
a.plot([], [], "o", color=C[0], label="last fall, random V (exact)")
a.plot([], [], "s", color=C[1], label="last fall, polynomial V (exact)")
a.plot([], [], "^", color="#222222", mfc="white", label="certified lower bound (1 not in W_4)")
a.set_ylim(1.5, 7.5)
a.set_xlabel("n  (field F_{2^n}, k = ceil(n/2))")
a.set_ylabel("degree")
a.set_title("Binary Semaev m=2 Weil descent", fontsize=9)
a.legend(fontsize=6.3, loc="upper left", frameon=False)

k = D["prime_kummer_m2"]
b.plot(k["t"], k["dF_session_reported"], "o--", color=C[0], mfc="white", label="last fall d_F, ours (session-reported; raw data lost)")
b.plot(k["pkm_n1"], k["pkm_Dcorr"], "s-", color=C[1], ms=4, label="PKM 2016 Table 4: corrected D_reg")
b.plot(k["t"][1:], k["ffd_proved"][1:], color=C[2], lw=1.6, label="first fall degree = 4 (proved, t >= 2)")
b.plot(k["t"], [2 * t + 2 for t in k["t"]], color=C[4], ls=":", lw=1, label="certificate degree 2t+2 (proved)")
b.set_ylim(1.5, 9.5)
b.set_xlabel("tower height t (factor base mu_{2^t})")
b.set_title("Prime-field Kummer tower, m=2 (PKM)", fontsize=9)
b.legend(fontsize=6.3, loc="upper left", frameon=False)

fig.tight_layout()
for ext in ("pdf", "svg"):
    fig.savefig(os.path.join(HERE, f"fall-degrees.{ext}"), bbox_inches="tight")
fig.savefig(os.path.join(HERE, "fall-degrees.png"), dpi=150, bbox_inches="tight")
