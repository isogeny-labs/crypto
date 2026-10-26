"""Draw the recorded E3a results (workloads.csv, regression.json,
stats-unadjusted.json, stats-adjusted.json). Arranges recorded values only;
every number is computed by generator-n0 and volcano-stats (native)."""
import csv, json, math
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = Path(__file__).resolve().parent
rows = list(csv.DictReader(open(here / "workloads.csv")))
reg = json.load(open(here / "regression.json"))
un = json.load(open(here / "stats-unadjusted.json"))
ad = json.load(open(here / "stats-adjusted.json"))
ym = lambda d: next(m for m in d["metrics"] if m["metric"] == "relation_yield")

fig, ax = plt.subplots(1, 3, figsize=(15, 4.6), gridspec_kw={"width_ratios": [1.5, 1, 1]})

# (a) yield against N0/r, one point per workload curve.
cols = {"l2-ramified": "#d95f02", "l2-split": "#e6ab02", "l2-inert": "#a6761d",
        "l3-ramified": "#1b9e77", "l3-split": "#7570b3", "l3-inert": "#386cb0"}
for c, col in cols.items():
    pts = [(float(r["w"]), float(r["yield"])) for r in rows if r["class"] == c]
    ax[0].scatter(*zip(*pts), s=9, alpha=.6, color=col, label=c, lw=0)
xs = [float(r["w"]) for r in rows]; ys = [float(r["yield"]) for r in rows]
gx = math.exp(sum(map(math.log, xs)) / len(xs)); gy = math.exp(sum(map(math.log, ys)) / len(ys))
lo, hi = min(xs), max(xs)
ax[0].plot([lo, hi], [gy * lo / gx, gy * hi / gx], "k--", lw=1, label="slope 1 (through geometric means)")
est = {e["name"]: e for e in reg["g1"]["estimates"]}["wls_pooled"]
ax[0].set_xscale("log"); ax[0].set_yscale("log")
ax[0].set_xlabel("N0/r  (exact class-cancelling ceiling, m = 2)")
ax[0].set_ylabel("measured IC yield  (relations / targets tried)")
ax[0].set_title(f"(a) 800 workload curves: WLS slope {est['slope']:.3f}\n95% cluster CI [{est['ci95'][0]:.3f}, {est['ci95'][1]:.3f}]; gate [0.9, 1.1]", fontsize=10)
ax[0].legend(fontsize=7, loc="upper left")

# (b) tau against its permutation null.
labels = ["horizontal", "coordinate"]
for j, (nm, d, off) in enumerate([("measured", un, -.18), ("N0-adjusted", ad, .18)]):
    m = ym(d); v = [m["horizontal"], m["model"]]
    ax[1].bar([i + off for i in range(2)], [100 * x["tau_rel"] for x in v], .34,
              color=["#b2182b", "#2166ac"][j], label=nm)
    for i, x in enumerate(v):
        ax[1].plot([i + off - .17, i + off + .17], [100 * x["tau_null_p95"]] * 2, "k-", lw=2)
ax[1].plot([], [], "k-", lw=2, label="permutation-null 95th pct")
ax[1].set_xticks(range(2), labels); ax[1].set_ylabel("DerSimonian-Laird tau (%)")
ax[1].set_title("(b) IC-yield heterogeneity\nbefore and after dividing by N0/r", fontsize=10)
ax[1].legend(fontsize=8)

# (c) l = 2 floor/crater contrast.
g4 = {g["set"]: g for g in reg["g4"]}
sets = ["l2_all (primary)", "l2_ramified_split", "l3_all (control)"]
for j, (key, lab, col, off) in enumerate([("raw", "measured", "#b2182b", -.18), ("adj", "N0-adjusted", "#2166ac", .18)]):
    ax[2].bar([i + off for i in range(3)], [100 * (math.exp(g4[s][f"c_{key}"]) - 1) for s in sets], .34, color=col, label=lab)
ax[2].axhline(0, color="k", lw=.6)
ax[2].set_xticks(range(3), ["l=2 (3 classes)\nprimary", "l=2 ramified,\nsplit", "l=3 (3 classes)\ncontrol"], fontsize=8)
ax[2].set_ylabel("floor / crater yield, % change")
ax[2].set_title(f"(c) floor vs crater; primary explained {100*g4[sets[0]]['explained']:.0f}%\n(secondary {100*g4[sets[1]]['explained']:.0f}%, gate 80%)", fontsize=10)
ax[2].legend(fontsize=8)
fig.suptitle("E3a: the base's cofactor-class count N0/r vs IC relation yield (volcano v1.1 data, 24-bit, prime-abscissa:size=32, n = 12,800 runs)", fontsize=10)
fig.tight_layout()
fig.savefig(here / "figures" / "e3a_n0_yield.svg", bbox_inches="tight")
fig.savefig(here / "figures" / "e3a_n0_yield.png", dpi=150, bbox_inches="tight")
