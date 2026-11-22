#!/usr/bin/env python3
"""Lay out recorded values from cofactor_data.json (no computation of results).

Figure 3: cofactor popcount bound Pi(w) for the Semaev instance against the box size, with the
counting prediction and the consistency floor 4w - 9 implied by the LFD law and Theorem A.
Figure 4: LFD(k) for three generators (Semaev, Mobius, affine): the common slope 1/4.
"""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(here, "cofactor_data.json")))

# ---------------------------------------------------------------- figure 3
fig, ax = plt.subplots(figsize=(7.2, 4.4))
sem = D["pi_measured"]["semaev_F2"]["per_coordinate_max_popcount"]
colors = {"2": "#7f7f7f", "3": "#2ca02c", "4": "#d62728", "5": "#1f77b4"}
for w, series in sem.items():
    xs = sorted(int(x) for x in series)
    ax.plot([x.bit_length() - 1 for x in xs], [series[str(x)] for x in xs], marker="o", color=colors[w],
            label=f"w = {w}: measured max popcount of cofactors in box 2^m")
pred = D["counting_prediction_m_of_w"]
for w in ("3", "4", "5"):
    ax.axhline(pred[w], color=colors[w], ls=":", lw=1)
    ax.text(11.05, pred[w], f"counting prediction w={w}: {pred[w]}", color=colors[w], fontsize=7, va="center")
ax.plot([3, 11], [3, 11], color="#999999", lw=0.8, ls="--", label="box edge (popcount = log2 box)")
ax.set_xlabel("log2 of the cofactor box size X")
ax.set_ylabel("max per-coordinate popcount of h  (S*h of weight <= w)")
ax.set_xlim(2.5, 14.5)
ax.set_ylim(0, 15)
ax.grid(alpha=0.25)
ax.legend(fontsize=6.8, loc="upper left")
ax.set_title("Pi(w): cofactor popcount of weight-w multiples of S in F_p[x1,x2] (F2 instance)", fontsize=9.5)
fig.tight_layout()
fig.savefig(os.path.join(here, "pi_vs_box.svg"))
fig.savefig(os.path.join(here, "pi_vs_box.png"), dpi=150)

# ---------------------------------------------------------------- figure 4
fig, ax = plt.subplots(figsize=(7.2, 4.0))
ks = list(range(2, 10))
u = D["universality_lfd"]
for name, color, marker in (("semaev", "#d62728", "o"), ("mobius", "#1f77b4", "s"), ("affine", "#2ca02c", "^")):
    ax.plot(ks, u[name], marker=marker, color=color, label=f"{name} (computed LFD)")
ax.plot(ks, [(k + 14) // 4 for k in ks], color="#d62728", ls=":", lw=1, label="floor((k+14)/4)")
ax.plot(ks, [k // 4 + 3 for k in ks], color="#1f77b4", ls=":", lw=1, label="floor(k/4)+3")
ax.plot(ks, [(k - 1) // 4 + 3 for k in ks], color="#2ca02c", ls=":", lw=1, label="floor((k-1)/4)+3")
ax.set_xlabel("depth k")
ax.set_ylabel("LFD(k)")
ax.set_yticks([3, 4, 5, 6])
ax.grid(alpha=0.25)
ax.legend(fontsize=7)
ax.set_title("Slope 1/4 for three couplings of two 2-power chains", fontsize=10)
fig.tight_layout()
fig.savefig(os.path.join(here, "lfd_universality.svg"))
fig.savefig(os.path.join(here, "lfd_universality.png"), dpi=150)
print("wrote pi_vs_box.{svg,png} lfd_universality.{svg,png}")
