#!/usr/bin/env python3
"""Plot recorded values only (AGENTS.md: Python lays out what was recorded).
Inputs: ../data/recount.json (this study's native recount) and
../data/uploaded_report_values.json (numbers transcribed from the uploaded report).
Outputs: recount_z.svg/.png, obstruction_diagram.svg/.png (same source for SVG and PNG)."""
import json, os, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch, FancyArrowPatch

HERE = os.path.dirname(os.path.abspath(__file__))
rec = json.load(open(os.path.join(HERE, "..", "data", "recount.json")))
rep = json.load(open(os.path.join(HERE, "..", "data", "uploaded_report_values.json")))
cutoffs = rec["cutoffs"]

def save(fig, stem):
    fig.savefig(os.path.join(HERE, stem + ".svg"), format="svg")
    fig.savefig(os.path.join(HERE, stem + ".png"), format="png", dpi=150)
    plt.close(fig)

# ---------- Figure 1: z against cutoff, native recount (lines) vs uploaded report (markers)
fig, axes = plt.subplots(1, 2, figsize=(12, 6.4), gridspec_kw={"width_ratios": [3, 2]})
ax = axes[0]
colors = plt.cm.tab10.colors
for i, c in enumerate(rec["curves"]):
    zs = [row["z"] for row in c["cutoffs"]]
    ax.plot(cutoffs, zs, "-", color=colors[i % 10], lw=1.4, label=c["name"])
    rv = rep["curves"][c["name"]]
    ax.plot([691, 5000, 50000], [rv["z_691"], rv["z_5000"], rv["z_50000"]], "o", mfc="none",
            color=colors[i % 10], ms=7, mew=1.5)
pz = [p["z_pooled_counts"] for p in rec["pooled"]]
ax.plot(cutoffs, pz, "k-", lw=2.6, label="pooled (10 curves), native recount")
ax.plot([int(k) for k in rep["pooled_z"]], list(rep["pooled_z"].values()), "ks", mfc="none", ms=8, mew=1.6,
        label="pooled, uploaded report")
ax.axhline(0, color="grey", lw=0.8)
ax.axhline(-1.96, color="grey", lw=0.8, ls=":")
ax.axhline(1.96, color="grey", lw=0.8, ls=":")
ax.set_xscale("log")
ax.set_xlabel("cutoff B (odd primes ℓ ≤ B, ℓ ∤ n)")
ax.set_ylabel("z = (observed split − expected) / sd")
ax.set_title("Split-prime count against the conditioned GL2 baseline\nlines: native recount (tools/splitcount.rs); hollow markers: uploaded report")
ax.legend(fontsize=7, ncol=4, loc="upper center", bbox_to_anchor=(0.5, -0.14), frameon=False)

ax = axes[1]
names = [c["name"] for c in rec["curves"]]
obs = [c["cutoffs"][3]["split"] for c in rec["curves"]]
exp = [c["cutoffs"][3]["expected_split"] for c in rec["curves"]]
sd = [c["cutoffs"][3]["sd"] for c in rec["curves"]]
assert all(c["cutoffs"][3]["cutoff"] == 691 for c in rec["curves"])
y = range(len(names))
ax.barh(y, obs, color=[colors[i % 10] for i in range(len(names))], alpha=0.85, label="observed split ℓ ≤ 691 (of 124 odd primes)")
ax.errorbar(exp, y, xerr=sd, fmt="k|", capsize=3, lw=1.2, label="expected ± 1 sd (model)")
rep_obs = [rep["curves"][n]["split_691"] for n in names]
ax.plot(rep_obs, y, "kx", ms=7, mew=1.8, label="uploaded report's count")
ax.set_yticks(list(y)); ax.set_yticklabels(names, fontsize=8)
ax.invert_yaxis()
ax.set_xlim(35, 70)
ax.set_xlabel("number of split primes ℓ ≤ 691")
ax.set_title("Counts at B = 691 (the report's cutoff)")
ax.legend(fontsize=7, loc="upper center", bbox_to_anchor=(0.5, -0.14), frameon=False)
fig.suptitle("Independent native recount of the split-prime deficit, 10 published prime curves (research/split_prime_ecdlp_20261107, data/recount.json)", fontsize=10)
fig.tight_layout()
save(fig, "recount_z")

# ---------- Figure 2: the obstruction diagram (hand-laid boxes; no computation)
fig, ax = plt.subplots(figsize=(12, 6.2))
ax.set_xlim(0, 12); ax.set_ylim(0, 6.2); ax.axis("off")
def box(x, y, w, h, text, fc="#f2f2f2", ec="#333", fs=9, bold=False):
    ax.add_patch(FancyBboxPatch((x, y), w, h, boxstyle="round,pad=0.08", fc=fc, ec=ec, lw=1.2))
    ax.text(x + w / 2, y + h / 2, text, ha="center", va="center", fontsize=fs, weight="bold" if bold else "normal", wrap=True)
def arrow(x1, y1, x2, y2, text="", color="#333", ls="-"):
    ax.add_patch(FancyArrowPatch((x1, y1), (x2, y2), arrowstyle="-|>", mutation_scale=14, color=color, lw=1.4, ls=ls))
    if text:
        ax.text((x1 + x2) / 2, (y1 + y2) / 2 + 0.12, text, ha="center", va="bottom", fontsize=8, color=color)

# left column: ℓ-adic / class-group world (ℓ ≠ n)
box(0.3, 4.6, 3.4, 1.2, "split / inert pattern of small ℓ\n= Kronecker symbol (Δ/ℓ), Δ = t² − 4p\n(the uploaded report's observation)", fc="#e8f0fe", bold=False)
box(0.3, 2.9, 3.4, 1.2, "ℓ-torsion E[ℓ], eigenlines, ℓ-isogenies,\nhorizontal class Cl(O), End(E) = O\n(all for ℓ ≠ n)", fc="#e8f0fe")
box(0.3, 1.2, 3.4, 1.2, "what reaches E(F_p):\nendomorphisms α ∈ O and\nisogeny transports φ: E → E′", fc="#e8f0fe")
arrow(2.0, 4.6, 2.0, 4.1)
arrow(2.0, 2.9, 2.0, 2.4)

# middle: the group
box(4.6, 2.6, 3.0, 1.6, "E(F_p) ≅ O/(π − 1) ≅ Z/n\ncyclic of prime order n\nDLP: Q = [k]P, find k", fc="#fff4e5", bold=True, fs=10)
arrow(3.7, 1.8, 4.6, 2.9, "", color="#b00020")
ax.text(4.7, 2.35, "acts as a scalar s_α ∈ Z/n;\nφ(Q) = [k]φ(P), same k", ha="left", va="top", fontsize=8.5, color="#b00020")
ax.text(4.0, 0.75, "scalar-only: generic-group bound √n still applies\n(Shoup 1997; Jao–Miller–Venkatesan 2005 self-reduction)", fontsize=8.5, color="#b00020", ha="left")

# right column: coordinate world
box(8.4, 4.6, 3.3, 1.2, "F_p coordinates of points:\nx(P), y(P), x(P₁ + ⋯ + P_m)", fc="#e6f4ea")
box(8.4, 2.9, 3.3, 1.2, "polynomial relations:\nsummation / division polynomials,\npairings (need small embedding degree)", fc="#e6f4ea")
box(8.4, 1.2, 3.3, 1.2, "index calculus / decomposition\n(the lab's algebraic and generic tracks;\nprime-field factor base is the open problem)", fc="#e6f4ea")
arrow(10.05, 4.6, 10.05, 4.1)
arrow(10.05, 2.9, 10.05, 2.4)
arrow(7.6, 3.4, 8.4, 3.5, "only non-linear\nhandle", color="#1a7f37")
# model choice link, measured null
arrow(3.7, 5.35, 8.4, 5.35, "", color="#666", ls="--")
ax.text(6.05, 5.45, "model choice within the class: IC yield / Dreg / F4 cost flat\n(measured null, research/isogeny_class_ic_yield_20261102)", ha="center", va="bottom", fontsize=8, color="#666")
ax.set_title("Why the split-prime pattern cannot feed a sub-generic ECDLP algorithm on a prime-order curve (derived; see the obstruction section of the study note)", fontsize=10)
fig.tight_layout()
save(fig, "obstruction_diagram")
print("ok")
