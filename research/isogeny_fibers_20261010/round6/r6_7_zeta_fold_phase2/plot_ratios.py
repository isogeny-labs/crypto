"""Plot recorded ratios and intervals from the R6.7 JSON outputs (reads fields only)."""
import json
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt

def rows(n): return json.load(open(n))["rows"]
m3 = rows("m3_9_10.json") + rows("m3_11.json")
m2t, m2p = rows("m2_torsion.json"), rows("m2_prime.json")
cats = [
    ("B/A m=3\nsize-matched", m3, "ratio_B_over_A_size_matched", 3.0),
    ("C/A m=3\nsize-matched", m3, "ratio_C_over_A_size_matched", 1.0),
    ("B/C m=3", m3, "ratio_B_over_C", 3.0),
    ("(b)/(a) m=2\nall curves", m2t + m2p, "ratio_b_over_a", 1 / 3),
    ("(c)/(b) m=2\nsize-matched", m2t, "ratio_c_over_b_size_matched", 1.0),
]
pr = [("unpriced", "#2f6f9f", -0.12), ("priced_as_run", "#d08a1f", 0.0), ("priced_essential", "#4b9a5b", 0.12)]
fig, ax = plt.subplots(figsize=(12, 5.2))
for i, (lab, rs, key, law) in enumerate(cats):
    for name, col, off in pr:
        for k, r in enumerate(rs):
            b = r[key][name]; x = i + off + (k - (len(rs) - 1) / 2) * 0.004
            ax.errorbar(x, b["point"], yerr=[[b["point"] - b["ci95"][0]], [b["ci95"][1] - b["point"]]],
                        fmt="o", ms=2.5, color=col, alpha=0.6, lw=0.6,
                        label=name.replace("_", " ") if i == 0 and k == 0 else None)
    ax.hlines(law, i - 0.3, i + 0.3, colors="#c0392b", lw=2, label="Prop. 15.2 law" if i == 0 else None)
ax.set_yscale("log"); ax.set_xticks(range(len(cats))); ax.set_xticklabels([c[0] for c in cats], fontsize=8)
ax.set_ylabel("cost ratio (F_p multiplications; inversions priced at Fermat I_p)")
ax.grid(axis="y", which="both", alpha=0.3); ax.legend(fontsize=8, loc="upper right")
ax.set_title("R6.7: zeta-fold phase two, one point per curve with bootstrap 95% CI\n"
             f"m=3: {len(m3)} torsion curves (p 2^9-2^11); m=2: {len(m2t)} torsion + {len(m2p)} prime curves, 1000 targets", fontsize=9)
fig.tight_layout(); fig.savefig("ratios.svg")
