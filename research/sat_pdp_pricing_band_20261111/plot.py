#!/usr/bin/env python3
"""S / matched strong rho against the price of one SAT conflict, rho's native work kept in the §5a band."""
import json
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
b = json.load(open("band.json"))
G, U, n, sr = b["sum_total_gae"], b["sum_unpriced_ops"], 40, b["sqrt_r"]
S = lambda a, w: (G[a] + w * U[a]) / n / sr
wc = np.logspace(-4, 3, 400)
fig, ax = plt.subplots(figsize=(7.2, 4.4))
lo_r, hi_r = S("ic-sat-xor", wc) / S("rho-strong", 1.0), S("ic-sat-xor", wc) / S("rho-strong", 0.0)
ax.fill_between(wc, lo_r, hi_r, color="#c0392b", alpha=0.25, lw=0)
ax.plot(wc, hi_r, color="#c0392b", lw=1.6, label="IC native-XOR SAT / rho (band: rho native ops at 0–1 GAE)")
ax.plot(wc, lo_r, color="#c0392b", lw=1.0, ls="--")
f0, f1 = S("ic-mitm-frob", 0) / S("rho-strong", 0), S("ic-mitm-frob", 0) / S("rho-strong", 1)
ax.axhspan(f1, f0, color="#2471a3", alpha=0.3, label=f"IC Frobenius MITM / rho ({f1:.2f}–{f0:.2f}; no unpriced ops)")
ax.axhline(1, color="#555", lw=1, ls="--")
for x in (0.1, 1):
    ax.axvline(x, color="#999", lw=0.8, ls=":")
lo, hi = b["wall"]["implied_gae_per_conflict"]
ax.axvspan(lo, hi, color="#f5b041", alpha=0.35, label=f"same-host wall price of a conflict ({lo:.0f}–{hi:.0f} GAE)")
be, be1 = b["break_even_w"], b["break_even_w_conflict_rho_w1"]
ax.annotate(f"SAT = rho at {be:.4f}–{be1:.4f} GAE per conflict", (be1, 1), xytext=(2e-4, 18), fontsize=8,
            arrowprops=dict(arrowstyle="->", lw=0.6))
ax.set_xscale("log"); ax.set_yscale("log")
ax.set_xlabel("GAE charged per SAT conflict")
ax.set_ylabel("mean cold S / matched strong rho")
ax.set_title("n = 17 Koblitz, count-qualified-main, 40 targets per arm (re-priced, no new run)", fontsize=9)
ax.legend(fontsize=7.5, loc="upper left"); ax.grid(alpha=0.25, which="both")
fig.tight_layout(); fig.savefig("band.svg"); fig.savefig("band.png", dpi=200)
