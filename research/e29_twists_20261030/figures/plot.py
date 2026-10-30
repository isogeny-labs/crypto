"""Plot E29's recorded values (experiments/34_twist24_stage_r/summary.json); computes nothing new."""
import json, sys, pathlib
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
here = pathlib.Path(__file__).parent
s = json.load(open(sys.argv[1]))
r = [x for x in s["stage_r"] if not x["karatsuba"]]; rk = [x for x in s["stage_r"] if x["karatsuba"]]
ps = [x["p"] for x in r]
fig, ax = plt.subplots(figsize=(8, 3.4))
ax.plot(ps, [x["steps_ratio"] for x in r], "o-", color="#4a7fb5", label="steps, order 6 / order 24")
ax.plot(ps, [x["muls_ratio"] for x in r], "s-", color="#8c8c8c", label="F_p mults, order 6 / order 24 (schoolbook)")
ax.plot(ps, [x["muls_ratio"] for x in rk], "^-", color="#d9772b", label="F_p mults, order 6 / order 24 (Karatsuba)")
ax.axhline(1, color="k", lw=0.6); ax.axhline(2, color="k", lw=0.4, ls=":")
ax.set_xlabel("p (E18's eight j = 0 curves, canonical models)"); ax.set_ylabel("ratio (16 walks each)")
ax.set_title("E29 Stage R: the order-24 walk takes fewer steps but is not cheaper as built", fontsize=10)
ax.legend(fontsize=8, frameon=False); fig.tight_layout()
for ext, kw in (("svg", {}), ("png", {"dpi": 150})): fig.savefig(here / f"stage_r.{ext}", **kw)
a = s["stage_a"]
fig, ax = plt.subplots(figsize=(6.5, 3.4))
for arm, col in (("cubic", "#8c8c8c"), ("twist24", "#d9772b")):
    xs = [x for x in a if x["arm"] == arm]
    ax.plot([x["p"] for x in xs], [x["nstar6_log2"] for x in xs], "o-", color=col, label=f"{arm} arm (vs order-6 walk)")
ax.axhline(90.1, color="#2ca02c", ls="--", label="E24 V4 line, 2^90.1 (§9.14)")
ax.axhspan(139, 144, color="#d9772b", alpha=0.12, label="registered band 2^139-2^144")
ax.set_xlabel("p (Stage A)"); ax.set_ylabel("log2 n* (model, §11.16)")
ax.set_title("E29 Stage A: modelled S5 handover", fontsize=10); ax.legend(fontsize=7, frameon=False); fig.tight_layout()
for ext, kw in (("svg", {}), ("png", {"dpi": 150})): fig.savefig(here / f"handover_model.{ext}", **kw)
