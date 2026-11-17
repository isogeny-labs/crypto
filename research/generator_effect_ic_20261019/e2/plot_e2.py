"""Lay out recorded values of e2/summary.json (written by the Rust example
generator_effect_e2_summary). No result is computed here."""
import json, os
os.environ.setdefault("MPLCONFIGDIR", os.environ.get("TMPDIR", "/tmp"))
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
d = json.load(open(os.path.join(here, "summary.json")))
lad = d["ladder"]
hs = [x["h"] for x in lad]
fig, ax = plt.subplots(1, 3, figsize=(17, 5.2), constrained_layout=True)

# A. measured / predicted yield ratio
a = ax[0]
a.axhspan(0.95, 1.05, color="#eeeeee", label="±5% (sizing target)")
a.axhline(1.0, color="black", lw=0.8)
series = [("knownlog_over_abscissa_n0", "known-log / abscissa, N₀ prediction", "#c0392b", -0.12),
          ("knownlog_over_abscissa_sumset", "known-log / abscissa, sumset prediction", "#4c78a8", 0.0),
          ("aa_over_knownlog_sumset", "A/A (seed 20261020 / 20261019), sumset", "#6b8e23", 0.12)]
for key, label, col, off in series:
    for i, x in enumerate(lad):
        y = x["yield_ratios"][key]
        m, (lo, hi) = y["measured_over_predicted"], y["measured_over_predicted_ci90"]
        a.errorbar(i + off, m, yerr=[[m - lo], [hi - m]], fmt="o", color=col, capsize=3,
                   label=label if i == 0 else None)
a.set_xticks(range(len(hs)))
a.set_xticklabels([f"h = {h}" for h in hs])
a.set_ylabel("online yield ratio, measured / predicted (90% CI)")
a.set_title("A. Yield ratio against the pre-registered prediction")
a.legend(fontsize=7, loc="lower left")

# B. measured yield ratio against h
b = ax[1]
b.plot(hs, hs, color="#999999", ls=":", label="y = h")
b.plot(hs, [x["yield_ratios"]["knownlog_over_abscissa_n0"]["predicted_mean"] for x in lad],
       "x", color="#c0392b", ms=9, label="predicted, N₀ (mean of 8 curves)")
b.plot(hs, [x["yield_ratios"]["knownlog_over_abscissa_sumset"]["predicted_mean"] for x in lad],
       "+", color="#4c78a8", ms=11, label="predicted, sumset (mean of 8 curves)")
for i, x in enumerate(lad):
    y = x["yield_ratios"]["knownlog_over_abscissa_sumset"]
    m, (lo, hi) = y["measured_ratio_pooled"], y["measured_ratio_pooled_ci90"]
    b.errorbar(x["h"], m, yerr=[[m - lo], [hi - m]], fmt="o", color="black", capsize=3,
               mfc="none", label="measured, pooled over 8 curves (90% CI)" if i == 0 else None)
b.set_xscale("log", base=2)
b.set_yscale("log", base=2)
b.set_xticks(hs)
b.set_xticklabels([str(h) for h in hs])
b.set_yticks(hs)
b.set_yticklabels([str(h) for h in hs])
b.set_xlabel("cofactor h")
b.set_ylabel("relations per target tried, known-log / abscissa")
b.set_title("B. Online yield ratio against cofactor")
b.legend(fontsize=7, loc="upper left")

# C. total S per arm against h, w = 0 and w = 1
c = ax[2]
arms = [("rho-neg", "rho-neg", "#999999"), ("ic-abscissa", "abscissa", "#e0a030"),
        ("ic-knownlog-random", "known-log random", "#4c78a8")]
for w, ls in ((0.0, "-"), (1.0, "--")):
    for arm, name, col in arms:
        ms, lo, hi = [], [], []
        for x in lad:
            bw = [b_ for b_ in x["arms"][arm]["by_weight"] if b_["w"] == w][0]
            ms.append(bw["s_mean"]); lo.append(bw["s_mean"] - bw["s_ci90"][0]); hi.append(bw["s_ci90"][1] - bw["s_mean"])
        c.errorbar(hs, ms, yerr=[lo, hi], color=col, ls=ls, marker="o", ms=4, capsize=2,
                   label=f"{name}, w = {w:g}")
c.set_xscale("log", base=2)
c.set_yscale("log")
c.set_xticks(hs)
c.set_xticklabels([str(h) for h in hs])
c.set_xlabel("cofactor h")
c.set_ylabel("total S = GAE / sqrt(r), mean per target (90% CI)")
c.set_title("C. Total S at T = 1 (w = unpriced ops in GAE)")
c.legend(fontsize=7, ncol=2, loc="center right")

fig.suptitle("E2 cofactor ladder: 24-bit a = −3 curves, 8 per h, 768 planted targets per curve, "
             "B = ⌈#E^(1/3)⌉, mitm m = 2 negation-folded; 40 ecbench sessions (e2/records/)", fontsize=10)
fig.savefig(os.path.join(here, "e2.svg"))
fig.savefig(os.path.join(here, "e2.png"), dpi=150)
