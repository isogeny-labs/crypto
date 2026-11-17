#!/usr/bin/env python3
"""Figure for the wave-6 runs: (left) L3 flat against chain cost at equal (m, D) on the k = 20 member, with the
flat-cell power-law fits; (right) L2 paired cost ratio of the ladder-check arm against its baseline at k = 20, 24
across the pricing band, and L1 faithful-member cost against the smooth-family records at equal (m, D)."""
import glob, json, math, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
H = os.path.dirname(os.path.abspath(__file__)); R = os.path.join(H, "..")
rec = {}
for f in glob.glob(os.path.join(R, "curve25519_exponent_study_20261108/cells/k20_m[23]_sym_d*_g0.json")) + \
         glob.glob(os.path.join(R, "curve25519_exponent_study_ext_20261110/cells/k20_m[23]_sym_d*_g0.json")):
    j = json.load(open(f))
    if not j["capped"] and j.get("per_target"): rec[(j["m"], j["d_or_t"])] = j["mean_field_ops"]
flat = {}
for f in glob.glob(os.path.join(H, "L3_flat/cells/*subflat*.json")):
    j = json.load(open(f))
    if not j.get("capped"): flat[(j["m"], j["d_or_t"])] = j["mean_field_ops"]
fig, axs = plt.subplots(1, 3, figsize=(15, 4.8))
a = axs[0]; col = {2: "#1b6ca8", 3: "#c0501a"}
for m in (2, 3):
    ch = sorted((d, t) for (mm, d), t in rec.items() if mm == m and d >= 3)
    fl = sorted((d, t) for (mm, d), t in flat.items() if mm == m)
    a.plot([d for d, _ in ch], [t for _, t in ch], "o", mfc="white", color=col[m], ms=5, label=f"m = {m} chain (recorded)")
    a.plot([d for d, _ in fl], [t for _, t in fl], "D-", color=col[m], ms=6, lw=1, label=f"m = {m} flat y^D = 1 (L3)")
a.set_xscale("log"); a.set_yscale("log"); a.grid(True, which="both", alpha=.25)
a.set_xlabel("|V| = D (k = 20 member, base mu_D)"); a.set_ylabel("T: F_p multiplications per system (mean of 6 targets)")
a.set_title("L3: one variable of degree D beats the chain by 10 to 360x at m = 3", fontsize=9.5); a.legend(fontsize=7.5)
b = axs[1]
for k, ratios, pred in ((20, (0.9669, 0.9692, 0.9809), (0.959, 0.950, 0.970)), (24, (0.9862, 0.9873, 0.9927), (0.983, 0.975, 0.990))):
    b.plot([0, 0.1, 1], ratios, "o-", label=f"k = {k} measured (tw2cj-lazy-lcheck / tw2cj-lazy)")
    b.axhspan(pred[1], pred[2], alpha=0.12, color="C0" if k == 20 else "C1"); b.axhline(pred[0], ls=":", lw=0.8, color="C0" if k == 20 else "C1")
b.set_xscale("symlog", linthresh=0.1); b.set_xticks([0, 0.1, 1]); b.set_xticklabels(["w = 0", "0.1", "1"])
b.set_ylim(0.94, 1.0); b.axhline(1, color="k", lw=0.6); b.set_ylabel("paired cost ratio (480 pairs, 5 curves)")
b.set_title("L2: both [d]G checks from the stored ladder; bands = frozen predictions", fontsize=9.5); b.legend(fontsize=7.5, loc="lower right"); b.grid(True, alpha=.25)
c = axs[2]
recall = {}
for f in glob.glob(os.path.join(R, "curve25519_exponent_study_20261108/cells/k*_m[23]_sym_d*_g0.json")) + glob.glob(os.path.join(R, "curve25519_exponent_study_ext_20261110/cells/k*_m[23]_sym_d*_g0.json")):
    j = json.load(open(f))
    if not j["capped"] and j.get("per_target"): recall.setdefault((j["m"], j["d_or_t"]), []).append(j["mean_field_ops"])
xs, ys = [], []
for f in glob.glob(os.path.join(H, "L1_faithful/cells/*.json")):
    j = json.load(open(f))
    if j.get("capped"): continue
    for t in recall.get((j["m"], j["d_or_t"]), []):
        xs.append(t); ys.append(j["mean_field_ops"])
if xs:
    c.plot(xs, ys, "s", color="#2a8a4a", ms=6, label=f"{len(xs)} shared (m, D) cells")
    lo, hi = min(xs) / 2, max(xs) * 2; c.plot([lo, hi], [lo, hi], "k:", lw=0.8, label="equality"); c.plot([lo, hi], [2 * lo, 2 * hi], "k--", lw=0.5); c.plot([lo, hi], [lo / 2, hi / 2], "k--", lw=0.5, label="factor 2 (P1 band)")
c.set_xscale("log"); c.set_yscale("log"); c.grid(True, which="both", alpha=.25)
c.set_xlabel("T on the smooth family (k = 20, 22, 24 records)"); c.set_ylabel("T on the faithful member p = 2^24 - 75, A = 7842")
c.set_title("L1: cost depends on (m, D) only; faithful = smooth to 4 digits", fontsize=9.5); c.legend(fontsize=7.5, loc="upper left")
fig.tight_layout(); fig.savefig(os.path.join(H, "wave6_runs.svg")); fig.savefig(os.path.join(H, "wave6_runs.png"), dpi=150); print("figure ok", len(flat), "flat cells,", len(xs), "L1 shared cells")
