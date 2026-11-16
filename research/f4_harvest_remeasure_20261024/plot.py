# Lays out recorded values only: analysis/<spec>.<a>__<b>.json are the outputs of
# `ecbench compare --json` on each committed session. Computes no result.
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

H = Path(__file__).parent
SPECS = [  # (spec, label, priced word XORs?)
    ("K1n11", "n=11, K1\ndim 4", True),
    ("K0n13", "n=13, K0\ndim 5", True),
    ("K1n17", "n=17, K1\ndim 6", True),
    ("K1n19", "n=19, K1\ndim 7", True),
    ("K0n19", "n=19, K0\ndim 7", False),
]


def load(spec, a, b):
    return json.load(open(H / "analysis" / f"{spec}.{a}__{b}.json"))


fig, (ax, bx) = plt.subplots(1, 2, figsize=(11.5, 4.6), gridspec_kw={"width_ratios": [1.35, 1]})

# (a) per-target ratio, default / off at m=3, and the harness's totals ratio with its interval
for i, (spec, label, priced) in enumerate(SPECS):
    d = load(spec, "f4-m3-off", "f4-m3-default")
    per = [w["ratio_b_over_a"] for w in d["workloads"]]
    if priced:
        xs = [i + (k - 11.5) * 0.012 for k in range(len(per))]
        ax.scatter(xs, per, s=12, color="#8aa4c8", zorder=2,
                   label="one target (24 per curve), ratio of its S" if i == 0 else None)
        o = d["ops"]
        lo, hi = o["ci95_bootstrap"]
        ax.errorbar([i], [o["ratio_b_over_a"]], yerr=[[o["ratio_b_over_a"] - lo], [hi - o["ratio_b_over_a"]]],
                    fmt="o", color="#b2432f", capsize=4, zorder=3, markersize=7,
                    label="totals ratio, 95% cluster bootstrap" if i == 0 else None)
    else:
        w = d["ops"]["band"]["ratio_b_over_a"][1]
        ax.scatter([i], [w], marker="D", s=55, color="#b2432f", zorder=3,
                   label="word XORs unpriced: point ratio at 0.1 GAE per XOR (no interval)")
ax.axhline(1.0, color="k", lw=0.8, ls="--")
ax.set_xticks(range(len(SPECS)))
ax.set_xticklabels([s[1] for s in SPECS], fontsize=8)
ax.set_ylabel("S(default) / S(off), m=3, inherited F4")
ax.set_title("(a) the root harvest default against no harvest", fontsize=10)
ax.legend(fontsize=7, loc="upper right")
ax.grid(alpha=0.3)

# (b) the m=2 negative control, and gpv-only and default against off
rows = [("m=2 default / off (control)", "f4-m2-off", "f4-m2-default"),
        ("m=3 gpv / off", "f4-m3-off", "f4-m3-gpv"),
        ("m=3 default / gpv", "f4-m3-gpv", "f4-m3-default")]
cols = ["#999999", "#4c78a8", "#e39b3c"]
for j, (name, a, b) in enumerate(rows):
    ys = []
    for spec, _, priced in SPECS:
        d = load(spec, a, b)
        ys.append(d["ops"]["band"]["ratio_b_over_a"][1])
    bx.plot(range(len(SPECS)), ys, "o-", color=cols[j], label=name, markersize=5 + 2 * (2 - j))
bx.axhline(1.0, color="k", lw=0.8, ls="--")
bx.set_xticks(range(len(SPECS)))
bx.set_xticklabels([s[1] for s in SPECS], fontsize=8)
bx.set_ylabel("totals ratio at 0.1 GAE per unpriced XOR")
bx.set_title("(b) control and the two gates", fontsize=10)
bx.legend(fontsize=7)
bx.grid(alpha=0.3)

fig.suptitle("PR #1862 linear harvest: counted S per target, inherited-F4 pdp3-Koblitz, x86-64 L0, 5 sessions of 144 verified records",
             fontsize=10)
fig.tight_layout()
fig.savefig(H / "harvest_ratio.svg")
fig.savefig(H / "harvest_ratio.png", dpi=150)
