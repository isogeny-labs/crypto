"""Draw E0_E4.md's figure from recorded values only: e4/analysis.json
(written by examples/generator_effect_e4_analysis.rs) and e0/e0_sweep.log.
No number is computed here beyond reading and normalising for display."""
import json, re, sys
import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = sys.argv[1] if len(sys.argv) > 1 else "."
doc = json.load(open(f"{here}/e4/analysis.json"))
log = open(f"{here}/e0/e0_sweep.log").read()

ARMS = {"ic-abscissa": "abscissa base", "ic-knownlog-random": "known-log, random scalars"}
METRICS = ["yield", "S_w0", "S_w0.1", "S_w1"]
MLABEL = {"yield": "yield", "S_w0": "S, w=0", "S_w0.1": "S, w=0.1", "S_w1": "S, w=1"}
COL = {"ic-abscissa": "#2a6fb0", "ic-knownlog-random": "#d0612a"}

fig = plt.figure(figsize=(15, 8.6))
gs = fig.add_gridspec(2, 3, width_ratios=[1.25, 1.25, 1], height_ratios=[1, 1], hspace=0.42, wspace=0.32)

# A: primary ratio, A/A and planted control, per arm and metric.
ax = fig.add_subplot(gs[:, 0])
y = 0
for arm in ARMS:
    a = next(x for x in doc["arms"] if x["arm"] == arm)
    for m in a["metrics"]:
        for key, lab, mk in [("gstd_over_grand", "gstd/grand", "o"), ("aa_gstd_over_gstd_aa", "A/A", "s"), ("planted_plus3", "planted +3%", "^")]:
            v = m[key]
            lo, hi = v["ci90"]
            ax.errorbar(v["ratio"], y, xerr=[[v["ratio"] - lo], [hi - v["ratio"]]], fmt=mk, color=COL[arm],
                        alpha=1.0 if key == "gstd_over_grand" else 0.55, capsize=2, ms=5)
            y -= 1
        y -= 0.6
ax.axvspan(0.98, 1.02, color="#7fbf7f", alpha=0.25, label="±2% equivalence margin")
ax.axvline(1.0, color="k", lw=0.8)
ax.axvline(1.03, color="#999", lw=0.8, ls=":")
ys, yy = [], 0
for arm in ARMS:
    for m in METRICS:
        for lab in ["gstd/grand", "A/A", "planted +3%"]:
            ys.append((yy, f"{ARMS[arm][:9]} · {MLABEL[m]} · {lab}"))
            yy -= 1
        yy -= 0.6
ax.set_yticks([p for p, _ in ys])
ax.set_yticklabels([l for _, l in ys], fontsize=7)
ax.set_xlabel("ratio, 90% bootstrap interval (geometric mean over 8 curves)")
ax.set_title("A. gstd against four random generators", fontsize=10, loc="left")
ax.legend(loc="lower right", fontsize=7)

# B, C: per-cell yield and S (w=0) relative to the curve's mean over its five generator cells.
for k, (arm, metric) in enumerate([("ic-knownlog-random", "yield"), ("ic-abscissa", "yield")]):
    ax = fig.add_subplot(gs[k, 1])
    a = next(x for x in doc["arms"] if x["arm"] == arm)
    mi = METRICS.index(metric)
    cells = a["cells"]
    for c in range(1, 9):
        cc = [x for x in cells if x["curve"] == c]
        main = [x for x in cc if x["cell"] != "gstd-aa"]
        mean = sum(x["metrics"][mi]["mean"] for x in main) / len(main)
        for x in cc:
            off = {"gstd": -0.3, "grand1": -0.15, "grand2": 0, "grand3": 0.15, "grand4": 0.3, "gstd-aa": 0.42}[x["cell"]]
            m, se = x["metrics"][mi]["mean"] / mean, x["metrics"][mi]["se"] / mean
            color = "k" if x["cell"] == "gstd" else ("#888" if x["cell"] == "gstd-aa" else COL[arm])
            ax.errorbar(c + off, m, yerr=1.96 * se, fmt="o" if x["cell"] != "gstd-aa" else "s", color=color, ms=3.5, capsize=1.5, lw=0.8)
    ax.axhspan(0.98, 1.02, color="#7fbf7f", alpha=0.2)
    ax.axhline(1, color="k", lw=0.6)
    ax.set_xticks(range(1, 9))
    ax.set_xticklabels([f"c{i}" for i in range(1, 9)])
    ax.set_ylabel(f"{metric} / curve mean (95% CI)")
    ax.set_title(f"{'BC'[k]}. {ARMS[arm]}: {metric} per generator cell\n(black gstd, colour grand1–4, grey square gstd-aa; 2000 runs each)", fontsize=9, loc="left")

# D: E0b exact check, measured ΔGAE against the recomputed scalar-multiplication difference.
ax = fig.add_subplot(gs[0, 2])
pts = [(float(a), float(b)) for a, b in re.findall(r"dGAE=(-?[\d.]+) predicted=(-?[\d.]+) ok", log)]
ax.scatter([p[1] for p in pts], [p[0] for p in pts], s=10, color="#555")
lim = max(abs(v) for p in pts for v in p) * 1.08
ax.plot([-lim, lim], [-lim, lim], color="#c33", lw=0.8)
ax.set_xlabel("recomputed Σ scalar-mult ΔGAE")
ax.set_ylabel("measured ΔGAE of the run")
n_ok = len(re.findall(r"^E0b .* ok$", log, re.M))
n_a = len(re.findall(r"^E0a .*framework=ok", log, re.M))
ax.set_title(f"D. E0b/E0c: {n_ok} coupled comparisons, all exact\n(E0a: {n_a} base digests identical)", fontsize=9, loc="left")

# E: text summary of the verdicts.
ax = fig.add_subplot(gs[1, 2])
ax.axis("off")
lines = []
for arm in ARMS:
    a = next(x for x in doc["arms"] if x["arm"] == arm)
    lines.append(ARMS[arm])
    for m in a["metrics"]:
        g = m["gstd_over_grand"]
        h = m["heterogeneity_generators"]
        aa = m["aa_gstd_over_gstd_aa"]["verdict"]
        lines.append(f"  {MLABEL[m['metric']]:9s} {g['verdict']:11s} A/A {aa:12s} Q p={h['q_perm_p']:.2f}"
                     f" max|z| p={h['max_z_perm_p']:.2f} plant {'detected' if m['planted_plus3']['detected'] else 'MISSED'}")
ax.text(0, 1, "\n".join(lines), family="monospace", fontsize=6.6, va="top")
ax.set_title("E. verdicts (protocol: CI in [0.98, 1.02])", fontsize=9, loc="left")

fig.suptitle(f"Generator effect, E0 + E4: 8 curves (24-bit, a=−3, prime order), {doc['records']} records, all verified", fontsize=11)
fig.savefig(f"{here}/e4/e4.svg", bbox_inches="tight")
fig.savefig(f"{here}/e4/e4.png", dpi=150, bbox_inches="tight")
