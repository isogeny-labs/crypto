"""Plot recorded A6 results: median log_p(ell^e) per bit band against kappa.
Reads results/k*/paired_vs_a5.csv (written by the Rust `pair` binary). Display only."""
import csv
import matplotlib; matplotlib.use("svg")
import matplotlib.pyplot as plt

K = [1, 2, 4, 8, 16]
med = {}
for k in K:
    rows = list(csv.reader(open(f"results/k{k}/paired_vs_a5.csv")))
    hdr = rows[0]
    for r in rows[1:]:
        if r[0] == "ell": break
        d = dict(zip(hdr, r))
        med[(int(d["p_bits"]), d["ell"], k)] = float(d["median_A5_log_p_ell_e"])
BLUE, ORANGE = "#2a78d6", "#eb6834"
fig, ax = plt.subplots(figsize=(8.0, 4.8))
for ell, col in (("2", BLUE), ("3", ORANGE)):
    for bits, ls in ((12, ":"), (20, "--"), (28, "-")):
        ys = [med[(bits, ell, k)] for k in K]
        ax.plot(K, ys, color=col, ls=ls, marker="o", label=f"ℓ={ell}" if bits == 28 else None)
        ax.text(17.5, ys[-1], f"{bits}-bit, ℓ={ell}", color=col, fontsize=8, va="center")
ax.set_xscale("log", base=2); ax.set_xticks(K); ax.set_xticklabels([str(k) for k in K])
ax.set_xlabel("κ in the rule ℓ^e₁ ≥ κ·p·N³ (κ = 1 is A5)")
ax.set_ylabel("median log_p of the returned norm")
ax.set_title("A6: headroom in the e₁ rule never helps (paired, 1050 runs per κ)", fontsize=10)
ax.set_xlim(0.85, 34)
ax.legend(fontsize=8, frameon=False, loc="upper left")
for s in ("top", "right"): ax.spines[s].set_visible(False)
fig.tight_layout(); fig.savefig("a6_kappa.svg")
