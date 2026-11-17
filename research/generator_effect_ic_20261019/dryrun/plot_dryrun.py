"""Lay out recorded values of summary.json (written by the Rust example
generator_effect_dryrun_summary). No result is computed here."""
import json, os
os.environ.setdefault("MPLCONFIGDIR", os.environ.get("TMPDIR", "/tmp"))
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

d = json.load(open("summary.json"))
cur = d["curves"]
labels = ["c1 (r=2^23.03, B=205)", "c2 (r=2^23.73, B=240)"]
cols = {"factor_base": "#6b8e23", "oracle_setup": "#4c78a8", "relations": "#e0a030",
        "linear_algebra": "#c0392b", "verify": "#888888"}
names = {"factor_base": "base construction", "oracle_setup": "pair-table build",
         "relations": "target draw + probes", "linear_algebra": "linear algebra",
         "verify": "verify"}
fig, ax = plt.subplots(1, 2, figsize=(11, 5.2), constrained_layout=True)

# A: stacked per-phase S, abscissa vs knownlog, w = 0 and 1, both curves
a = ax[0]
xs, ticks, tl = 0, [], []
for ci, c in enumerate(cur):
    for w in (0.0, 1.0):
        for arm, short in (("ic-abscissa", "abs"), ("ic-knownlog", "kl")):
            bw = [b for b in c["arms"][arm]["by_weight"] if b["w"] == w][0]
            bottom = 0
            for ph in cols:
                v = bw["phases_s"][ph]
                a.bar(xs, v, bottom=bottom, color=cols[ph],
                      label=names[ph] if (ci == 0 and w == 0.0 and arm == "ic-abscissa") else None)
                bottom += v
            lo, hi = bw["s_ci90"]
            a.errorbar(xs, bw["s_mean"], yerr=[[bw["s_mean"] - lo], [hi - bw["s_mean"]]],
                       color="black", capsize=3, lw=1)
            ticks.append(xs); tl.append(f"{short}\nw={w:g}\nc{ci+1}")
            xs += 1
        xs += 0.6
    xs += 0.8
a.set_xticks(ticks); a.set_xticklabels(tl, fontsize=7)
a.set_ylabel("S = GAE / sqrt(r), mean per target (90% bootstrap CI)")
a.set_title("A. Per-phase S, abscissa vs known-log")
a.legend(fontsize=7, loc="upper left")

# B: online yield
b = ax[1]
for ci, c in enumerate(cur):
    for ai, arm in enumerate(("ic-abscissa", "ic-knownlog")):
        v = c["arms"][arm]
        x = ci * 3 + ai
        b.errorbar(x, v["yield"], yerr=[[v["yield"] - v["yield_ci90"][0]], [v["yield_ci90"][1] - v["yield"]]],
                   fmt="o", color=["#4c78a8", "#c0392b"][ai], capsize=4,
                   label=arm if ci == 0 else None)
        b.annotate(f"{v['yield']:.2e}", (x, v["yield"]), textcoords="offset points", xytext=(8, 4), fontsize=7)
b.set_yscale("log"); b.set_xticks([0.5, 3.5]); b.set_xticklabels(labels, fontsize=8)
b.set_ylabel("relations found / targets tried (90% CI)")
b.set_title("B. Online yield")
b.legend(fontsize=8)

fig.suptitle("SCRATCH dry run, not data. Generator-effect E1, two 24-bit a=-3 prime-order curves, "
             "16 planted_shared targets x 2 rounds, mitm negation_folded m=2. Sessions ECBS1hdfb4b2694578, ECBS1h3e4cb8782f3c.",
             fontsize=7)
fig.savefig("dryrun.svg")
fig.savefig("dryrun.png", dpi=150)
