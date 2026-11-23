#!/usr/bin/env python3
"""Figure for W5-SYN-3: recorded solving degree against the largest prime factor of d (exponent-study cells)."""
import glob, json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
H = os.path.dirname(os.path.abspath(__file__)); R = os.path.join(H, "..", "..")
def lpf(n):
    f, b = 2, 1
    while n > 1:
        while n % f == 0: b, n = f, n // f
        f += 1
    return b
pts = []
for f in sorted(glob.glob(os.path.join(R, "curve25519_exponent_study_20261108/cells/k2*_m[23]_sym_d*_g0.json"))) + \
         sorted(glob.glob(os.path.join(R, "curve25519_exponent_study_ext_20261110/cells/*.json"))):
    j = json.load(open(f))
    if j["capped"] or not j.get("per_target"): continue
    d = j["d_or_t"]; pts.append((j["k"], j["m"], d, lpf(d), max(r["solving_degree_max"] for r in j["per_target"])))
fig, ax = plt.subplots(figsize=(7, 4.6))
sty = {(2, True): ("o", "#1b6ca8", "m = 2, d prime"), (2, False): ("o", "#9cc3e0", "m = 2, d composite"),
       (3, True): ("s", "#c0501a", "m = 3, d prime"), (3, False): ("s", "#f0b08a", "m = 3, d composite")}
for key, (mk, c, lab) in sty.items():
    sel = [p for p in pts if p[1] == key[0] and (p[3] == p[2]) == key[1]]
    ax.scatter([p[3] for p in sel], [p[4] for p in sel], marker=mk, color=c, edgecolor="k", lw=0.4, s=34, label=f"{lab} ({len(sel)})", zorder=3)
x = [2, 100]
ax.plot(x, [v + 1 for v in x], "k-", lw=0.8, label="SD = e_max + 1")
ax.plot(x, [v + 5 for v in x], "k:", lw=0.8, label="SD = e_max + 5")
ax.set_xscale("log"); ax.set_yscale("log"); ax.grid(True, which="both", alpha=.2)
ax.set_xlabel("e_max: largest prime factor of d (|V| = d, base mu_d)")
ax.set_ylabel("recorded F4 solving degree (max over 6 targets)")
ax.set_title("Solving degree follows the largest prime factor of d\n(k = 20, 22, 24 exponent-study cells; unverified)", fontsize=10)
ax.legend(fontsize=7.5, loc="upper left")
fig.tight_layout(); fig.savefig(os.path.join(H, "..", "sd_law.svg")); fig.savefig(os.path.join(H, "..", "sd_law.png"), dpi=150)
print("points", len(pts))
