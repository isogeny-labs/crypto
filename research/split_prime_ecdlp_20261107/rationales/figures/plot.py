#!/usr/bin/env python3
"""Plot recorded values only (AGENTS.md: Python lays out what was recorded; it
computes no result). Inputs: ../data/weightings.json (native Rust tool
tools/weightings.rs) and, when present, ../data/classgroups.json (PARI/GP script
tools/classgroup.gp, one JSON object per line). Outputs: weightings.svg/.png,
band_profile.svg/.png, classgroups.svg/.png, rationales_diagram.svg/.png (same
source for SVG and PNG)."""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch, FancyArrowPatch

HERE = os.path.dirname(os.path.abspath(__file__))
W = json.load(open(os.path.join(HERE, "..", "data", "weightings.json")))
CG_PATH = os.path.join(HERE, "..", "data", "classgroups.json")
CG = []
if os.path.exists(CG_PATH):
    for line in open(CG_PATH):
        line = line.strip()
        if line.startswith("{"):
            CG.append(json.loads(line))


def save(fig, stem):
    fig.savefig(os.path.join(HERE, stem + ".svg"), format="svg")
    fig.savefig(os.path.join(HERE, stem + ".png"), format="png", dpi=150)
    plt.close(fig)


PUB = "#1f77b4"
SKP = "#d62728"

# ---------- Figure 1: weighted split sums at B = 691 per field, published vs skipped, one panel per weight
weights = [w for w in W["weights"]]
names = [w["name"] for w in weights]
fields = W["paired_fields"]
fig, axes = plt.subplots(2, 4, figsize=(14, 6.8), sharex=False)
for ax, w in zip(axes.flat, weights):
    for j, b in enumerate(fields):
        for c in W["curves"]:
            if c["bits"] != b:
                continue
            y = c["w691"][w["name"]]
            ax.plot([j], [y], "o" if c["role"] == "published" else "x",
                    color=PUB if c["role"] == "published" else SKP, ms=7, mew=1.6)
    sep = w["lower_count"]
    ax.set_title(f'w = {w["name"]}: separates at {sep} of {W["odd_prime_cutoffs"]} cutoffs'
                 + ("" if sep == 0 else f'\n{w["lower_separating_cutoffs"][:6]}'), fontsize=9)
    ax.set_xticks(range(len(fields)))
    ax.set_xticklabels([f"P-{b}" for b in fields], fontsize=8)
    ax.grid(alpha=0.3)
axes[0, 0].plot([], [], "o", color=PUB, label="published")
axes[0, 0].plot([], [], "x", color=SKP, label="skipped")
axes[0, 0].legend(fontsize=8, loc="lower left")
fig.suptitle("Weighted split-prime sums W_691(E) = Σ_{split odd ℓ ≤ 691} w(ℓ) for the 21 census curves "
             "(native recount, tools/weightings.rs); each rationale implies one w", fontsize=10)
fig.tight_layout()
save(fig, "weightings")

# ---------- Figure 2: band profile of the deficit (pooled published vs skipped split fraction)
bp = W["band_profile_paired_fields"]
labels = [f"{lo}–{hi}" for lo, hi in (b["band"] for b in bp)]
fig, ax = plt.subplots(figsize=(8, 4.2))
x = range(len(bp))
ax.bar([i - 0.2 for i in x], [b["published_fraction"] for b in bp], width=0.4, color=PUB, label="published (8 curves, 4 fields)")
ax.bar([i + 0.2 for i in x], [b["skipped_fraction"] for b in bp], width=0.4, color=SKP, label="skipped (11 curves)")
for i, b in enumerate(bp):
    ax.text(i, max(b["published_fraction"], b["skipped_fraction"]) + 0.02,
            f'Δ = {b["difference"]:+.3f}\nz = {b["z_two_proportion"]:+.2f}\n{b["published_cells"]}/{b["skipped_cells"]} cells',
            ha="center", fontsize=7.5)
ax.axhline(0.5, color="gray", lw=0.8, ls="--")
ax.set_xticks(list(x))
ax.set_xticklabels(labels)
ax.set_xlabel("band of odd primes ℓ")
ax.set_ylabel("fraction of (curve, ℓ) cells with χ(ℓ) = +1")
ax.set_ylim(0, 0.75)
ax.set_title("Where the published/skipped deficit sits: pooled split fraction by ℓ-band (paired fields P-192/224/239/256)", fontsize=9.5)
ax.legend(fontsize=8, loc="upper left")
ax.grid(alpha=0.3, axis="y")
fig.tight_layout()
save(fig, "band_profile")

# ---------- Figure 3: class-number facts (PARI facts stage) and exact class groups where computed
import glob
FACTS = []
fp = os.path.join(HERE, "..", "data", "cl", "facts.jsonl")
if os.path.exists(fp):
    for line in open(fp):
        if line.startswith("{"):
            FACTS.append(json.loads(line))
EXACT = {}
for f in glob.glob(os.path.join(HERE, "..", "data", "cl", "cl_*.jsonl")):
    for line in open(f):
        if line.startswith("{"):
            r = json.loads(line)
            EXACT[r["label"]] = r
roles = {c["label"]: (c["role"], c["bits"]) for c in W["curves"]}
if FACTS:
    fig, axes = plt.subplots(1, 2, figsize=(12, 4.6))
    ax = axes[0]
    fields = sorted(set(b for _, b in roles.values()))
    for r in FACTS:
        role, b = roles[r["label"]]
        col = PUB if role == "published" else SKP
        x = fields.index(b) + (-0.08 if role == "published" else 0.08)
        ax.plot([x], [r["L1_partial_1e6"]], "o" if role == "published" else "x", color=col, ms=7, mew=1.6)
        if r["conductor"] != 1:
            ax.annotate(f'f = {r["conductor"]}', (x, r["L1_partial_1e6"]), textcoords="offset points", xytext=(6, -3), fontsize=7)
    ax.plot([], [], "o", color=PUB, label="published")
    ax.plot([], [], "x", color=SKP, label="skipped")
    ax.set_xticks(range(len(fields)))
    ax.set_xticklabels([f"P-{b}" for b in fields])
    ax.set_ylabel("partial L(1, χ) over primes ≤ 10⁶  (h ≈ √|D_K| · L(1,χ) / π)")
    ax.set_title("Class-number size factor of the Frobenius field, published vs skipped", fontsize=9.5)
    ax.legend(fontsize=8)
    ax.grid(alpha=0.3)
    ax = axes[1]
    for r in FACTS:
        role, b = roles[r["label"]]
        col = PUB if role == "published" else SKP
        ax.plot([b], [r["h_estimate_log2"]], "o" if role == "published" else "x", color=col, ms=6, alpha=0.8)
    for lab, e in EXACT.items():
        role, b = roles[lab]
        ax.plot([b], [e["h_K_bits"]], "s", mfc="none", color="k", ms=9, label="exact h (quadclassunit, GRH)" if lab == sorted(EXACT)[0] else None)
    ax.plot(fields, [b / 2 for b in fields], "--", color="gray", lw=0.8, label="log₂ √p")
    ax.set_xlabel("field size (bits)")
    ax.set_ylabel("log₂ h(O_K) (analytic estimate; squares exact)")
    ax.set_title("Class number of the Frobenius field against field size", fontsize=9.5)
    ax.legend(fontsize=8)
    ax.grid(alpha=0.3)
    fig.tight_layout()
    save(fig, "classgroups")

# ---------- Figure 4: diagram of the three rationales and where each meets the obstruction
fig, ax = plt.subplots(figsize=(12, 5.2))
ax.set_xlim(0, 12)
ax.set_ylim(0, 5.2)
ax.axis("off")


def box(x, y, w, h, text, fc="#f4f4f4", ec="#444", fs=8.5):
    ax.add_patch(FancyBboxPatch((x, y), w, h, boxstyle="round,pad=0.04", fc=fc, ec=ec, lw=1.2))
    ax.text(x + w / 2, y + h / 2, text, ha="center", va="center", fontsize=fs, wrap=True)


def arrow(x0, y0, x1, y1, text=None, color="#444", ls="-"):
    ax.add_patch(FancyArrowPatch((x0, y0), (x1, y1), arrowstyle="-|>", mutation_scale=14, color=color, lw=1.2, ls=ls))
    if text:
        ax.text((x0 + x1) / 2, (y0 + y1) / 2 + 0.12, text, ha="center", fontsize=7.5, color=color)


box(0.2, 1.9, 2.6, 1.4, "split prime ℓ:\nχ(ℓ) = (Δ/ℓ) = +1\ntwo rational ℓ-isogenies\n(Elkies kernel polynomial)", fc="#fff3cd")
box(3.6, 3.7, 2.8, 1.3, "1. class group Cl(O_K)\nrelations among [𝔩]\n(Hafner–McCurley,\nBisson–Sutherland)", fc="#e8f1fb")
box(3.6, 1.95, 2.8, 1.3, "2. lift to Q with extra\nisogeny structure\n(xedni-style, 1998)", fc="#e8f1fb")
box(3.6, 0.2, 2.8, 1.3, "3. j(E) ∈ X_0(ℓ)(F_p)\nper-ℓ Frobenius data,\nCRT-combined (SEA-like)", fc="#e8f1fb")
arrow(2.8, 2.9, 3.6, 4.3)
arrow(2.8, 2.6, 3.6, 2.6)
arrow(2.8, 2.3, 3.6, 0.9)
box(7.3, 3.7, 2.4, 1.3, "acts on E(F_p) as a\nscalar α mod (π − 1)\n(Lenstra)", fc="#fde2e2")
box(7.3, 1.95, 2.4, 1.3, "needs ℓ ∈ Mazur set\n(ℓ ≤ 163): weight\n[ℓ ∈ M], not a count", fc="#fde2e2")
box(7.3, 0.2, 2.4, 1.3, "determines t mod ℓ,\ni.e. n, already public;\nE[ℓ] orthogonal to E(F_p)\n(ℓ-pairing is trivial)", fc="#fde2e2")
arrow(6.4, 4.35, 7.3, 4.35, "generic (Thm §3)")
arrow(6.4, 2.6, 7.3, 2.6, "no separation (§4)")
arrow(6.4, 0.85, 7.3, 0.85, "point-independent")
box(10.2, 1.6, 1.7, 2.0, "E(F_p)\ncyclic, prime\norder n ≫ ℓ\n\nDLP untouched", fc="#e2f0d9", fs=9)
arrow(9.7, 4.35, 10.2, 3.2, color="#999", ls="--")
arrow(9.7, 2.6, 10.2, 2.6, color="#999", ls="--")
arrow(9.7, 0.85, 10.2, 2.0, color="#999", ls="--")
ax.text(6, 5.1, "Three rationales for weighting split primes, and where each meets the scalar-only obstruction (derived; dashed = no information flow)",
        ha="center", fontsize=9.5)
fig.tight_layout()
save(fig, "rationales_diagram")
print("figures written")
