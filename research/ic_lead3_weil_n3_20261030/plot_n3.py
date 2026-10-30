"""Plot the recorded rows of results.json (written by fit_n3.rs). Lays out recorded values and the fits
that fit_n3.rs already computed; computes nothing new."""
import json, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

res = json.load(open(sys.argv[1]))
out = sys.argv[2]
rows = res["rows"]
names = {"dense": "m=3 dense LA", "wiedemann": "m=3 Wiedemann", "2lp": "m=3 Wiedemann + 2 large primes",
         "M": "m=3 MitM pair tests", "J": "m=2 (Joux-Vitse)"}
col = {"dense": "#1f77b4", "wiedemann": "#17becf", "2lp": "#d62728", "M": "#7f7f7f", "J": "#8c564b"}

fig, ax = plt.subplots(1, 2, figsize=(12, 4.6))
for arm in ["dense", "wiedemann", "2lp", "M", "J"]:
    g = [r for r in rows if r["arm"] == arm and r["verified"]]
    if not g:
        continue
    ax[0].scatter([r["log2n"] for r in g], [r["s"] / r["rho_s"] for r in g], s=14, color=col[arm], alpha=0.5)
    ps = sorted(set(r["p"] for r in g))
    xs = [sum(r["log2n"] for r in g if r["p"] == p) / sum(1 for r in g if r["p"] == p) for p in ps]
    ys = [sum(r["s"] / r["rho_s"] for r in g if r["p"] == p) / sum(1 for r in g if r["p"] == p) for p in ps]
    ax[0].plot(xs, ys, "-o", color=col[arm], label=names[arm], ms=4)
ax[0].axhline(1.0, color="k", lw=1)
ax[0].text(24.3, 1.15, "rho (S/S_rho = 1)", fontsize=8)
ax[0].set_yscale("log")
ax[0].set_xlabel("log2 n  (n = #E(F_{p^3}), prime)")
ax[0].set_ylabel("counted cost / pooled rho cost on the same group")
ax[0].set_title("Total counted cost over rho, per instance (points) and mean (lines)")
ax[0].legend(fontsize=7)
ax[0].grid(alpha=0.3, which="both")

for arm, ls in [("2lp", "-"), ("dense", "--")]:
    g = [r for r in rows if r["arm"] == arm and r["verified"]]
    ps = sorted(set(r["p"] for r in g))
    xs = [sum(r["log2n"] for r in g if r["p"] == p) / sum(1 for r in g if r["p"] == p) for p in ps]
    rel = [sum(r["rel_phase"] for r in g if r["p"] == p) / sum(1 for r in g if r["p"] == p) for p in ps]
    la = [sum(r["la_phase"] for r in g if r["p"] == p) / sum(1 for r in g if r["p"] == p) for p in ps]
    ax[1].plot(xs, rel, ls, marker="o", color="#d62728" if arm == "2lp" else "#1f77b4", ms=4, label=f"{names[arm]}: relation phase")
    ax[1].plot(xs, la, ls, marker="s", color="#ff9896" if arm == "2lp" else "#aec7e8", ms=4, label=f"{names[arm]}: LA phase")
g = [r for r in rows if r["verified"]]
ps = sorted(set(r["p"] for r in g))
xs = [sum(r["log2n"] for r in g if r["p"] == p) / sum(1 for r in g if r["p"] == p) for p in ps]
rho = [res_r for res_r in [next(r for r in g if r["p"] == p) for p in ps]]
ax[1].plot(xs, [r["rho_s"] * 2 ** (r["log2n"] / 2) for r in rho], "k:", marker="^", ms=4, label="rho (pooled S_rho * sqrt n)")
ax[1].set_yscale("log")
ax[1].set_xlabel("log2 n")
ax[1].set_ylabel("counted ops (affine additions + Fp muls / 63)")
ax[1].set_title("Phases: relation collection vs linear algebra vs rho")
ax[1].legend(fontsize=7)
ax[1].grid(alpha=0.3, which="both")
fig.tight_layout()
fig.savefig(out + ".svg")
fig.savefig(out + ".png", dpi=150)
