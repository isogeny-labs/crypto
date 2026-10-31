"""Lay out the recorded rho reference and the §5a band of the validation.

Plots recorded values only (validation/rho_reference.json and the band in
validation/validator-report.json, recomputed there by jq); computes nothing
the report quotes.
"""

import json
import os
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
VAL = os.path.join(HERE, "..", "validation")
rho = json.load(open(os.path.join(VAL, "rho_reference.json")))
rep = json.load(open(os.path.join(VAL, "validator-report.json")))["recomputed_all"]

fig, (ax, bx) = plt.subplots(1, 2, figsize=(12.5, 4.8), gridspec_kw={"width_ratios": [1.4, 1]})

runs = rho["runs"]
xs = list(range(len(runs)))
ax.bar(xs, [r["fp_muls"] for r in runs], color="#4e79a7", label="negation rho, one target (verified)")
ax.axhline(rep["rho_mean_fp_muls"], color="#4e79a7", ls=":", lw=1)
ax.text(len(runs) - 0.5, rep["rho_mean_fp_muls"] * 1.3, f"rho mean {rep['rho_mean_fp_muls']:.3g}",
        ha="right", fontsize=8, color="#4e79a7")
ax.axhline(rep["band_ic_lower_bound"], color="#e15759", ls="--", lw=1.2,
           label="IC lower bound: one decomposition, C4 (sparsest+chains+replay)")
ax.text(0, rep["band_ic_lower_bound"] * 1.3, f"C4 = {rep['band_ic_lower_bound']:.3g}", fontsize=8, color="#e15759")
ax.set_yscale("log")
ax.set_ylim(1e5, 1e12)
ax.set_xticks(xs)
ax.set_xticklabels([str(i) for i in xs], fontsize=7)
ax.set_xlabel("rho run (seed index)")
ax.set_ylabel("$F_p$ multiplications, counted")
ax.set_title(f"Matched rho on the §11.17 curve: prime subgroup ℓ = {rho['subgroup_order']:,} "
             f"($2^{{24.72}}$), {sum(r['verified'] for r in runs)}/{len(runs)} verified", fontsize=9)
ax.legend(fontsize=7, loc="center right")

ws = ["0", "0.1", "1"]
ic = [rep["band_ic_lower_bound"]] * 3
rh = [rep["band_rho_w0"], rep["band_rho_w01"], rep["band_rho_w1"]]
bx.plot(ws, ic, "s--", color="#e15759", label="IC lower bound (unpriced work not emitted)")
bx.plot(ws, rh, "o-", color="#4e79a7", label="rho, unpriced ops priced at w GAE")
for w, r, i in zip(ws, rh, ic):
    bx.text(w, r * 0.45, f"{i / r:.3g}×", ha="center", fontsize=8)
bx.set_yscale("log")
bx.set_ylim(1e5, 1e12)
bx.set_xlabel("w, GAE per unpriced native operation\n(1 GAE = 97 $F_p$ mult., the measured cost of one addition)")
bx.set_ylabel("$F_p$ multiplications")
bx.set_title("§5a band: the order holds at every weight", fontsize=9)
bx.legend(fontsize=7, loc="center right")

fig.suptitle("Validation of the plain-$S_5$ $C_4$ row: rho reference and §5a band "
             "(run-20261021T0501Z; replay 91/91 identical)", fontsize=10)
fig.tight_layout()
for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
    fig.savefig(os.path.join(HERE, f"validation_rho_band.{ext}"), **kw)
print("wrote validation_rho_band.svg/png", file=sys.stderr)
