"""Plot the recorded values of the two ecbench sessions of research/ic_cofactor_clear_ab_20261103.

Layout only: every number is read from the `ecbench compare` JSON files under
sessions/<s>/analysis/ (ratio, linearised 95% interval).  Nothing is computed here.
Usage: python3 figures.py   (from this directory; needs matplotlib)
"""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
matplotlib.rcParams.update({"font.size": 7})
import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parents[2]
SESS = {"V2 (h=32)": ROOT / "sessions/v2/analysis", "h=1 control": ROOT / "sessions/h1/analysis"}
WEIGHTS = ["0", "0.1", "1"]


def rec(path):
    o = json.loads(path.read_text())["ops"]
    return o["ratio_b_over_a"], o["ci95"][0], o["ci95"][1]


data = {"gain": {}, "aa": {}, "positive": {}, "gap": {}}
for name, d in SESS.items():
    data["gain"][name] = {w: rec(d / f"main_w{w}.json") for w in WEIGHTS}
    data["aa"][name] = rec(d / "aa.json")
    data["positive"][name] = rec(d / "positive.json")
for arm in ("raw", "clear"):
    data["gap"][arm] = {w: rec(SESS["V2 (h=32)"] / f"gap_{arm}_w{w}.json") for w in WEIGHTS}
(Path(__file__).parent / "figure-data.json").write_text(json.dumps(data, indent=1))

fig, ax = plt.subplots(1, 3, figsize=(8.6, 3.6))

# Panel 1: gain G_w = sum S(ic-raw) / sum S(ic-clear)
xs = range(len(WEIGHTS))
for k, (name, col, off) in enumerate((("V2 (h=32)", "#1f77b4", -0.08), ("h=1 control", "#7f7f7f", 0.08))):
    v = [data["gain"][name][w] for w in WEIGHTS]
    ax[0].errorbar([x + off for x in xs], [r for r, _, _ in v],
                   yerr=[[r - lo for r, lo, _ in v], [hi - r for r, _, hi in v]],
                   fmt="o", color=col, capsize=3, label=name)
ax[0].axhline(1, color="k", lw=0.8)
ax[0].axhline(3, color="#d62728", lw=0.8, ls="--")
ax[0].text(2.45, 3.1, "3x (stated falsifier)", color="#d62728", ha="right", fontsize=6)
ax[0].set_xticks(list(xs), [f"{w} GAE/op" for w in WEIGHTS])
ax[0].set_yscale("log")
ax[0].set_ylabel("G = sum S(ic-raw) / sum S(ic-clear)")
ax[0].set_title("S gain of clearing the cofactor", fontsize=9)
ax[0].legend(fontsize=6)

# Panel 2: controls
labels, vals, cols = [], [], []
for name in SESS:
    labels.append(f"A/A\n{name}"); vals.append(data["aa"][name]); cols.append("#7f7f7f")
    labels.append(f"rho-plain/rho-neg\n{name}"); vals.append(data["positive"][name]); cols.append("#2ca02c")
ax[1].errorbar(range(len(vals)), [r for r, _, _ in vals],
               yerr=[[r - lo for r, lo, _ in vals], [hi - r for r, _, hi in vals]], fmt="o", color="k", capsize=3)
ax[1].axhline(1, color="k", lw=0.8)
ax[1].axhline(1.2, color="#d62728", lw=0.8, ls="--")
ax[1].text(3.4, 1.205, "1.2 (positive-control line)", color="#d62728", ha="right", fontsize=6)
ax[1].set_xticks(range(len(vals)), labels, fontsize=6)
ax[1].set_ylabel("ratio of totals B/A")
ax[1].set_title("controls (counted S, w=0)", fontsize=9)

# Panel 3: distance remaining to matched rho-neg on V2
for arm, col, off in (("raw", "#d62728", -0.08), ("clear", "#1f77b4", 0.08)):
    v = [data["gap"][arm][w] for w in WEIGHTS]
    ax[2].errorbar([x + off for x in xs], [r for r, _, _ in v],
                   yerr=[[r - lo for r, lo, _ in v], [hi - r for r, _, hi in v]],
                   fmt="o", color=col, capsize=3, label=f"ic-{arm}")
ax[2].axhline(1, color="k", lw=0.8)
ax[2].set_xticks(list(xs), [f"{w} GAE/op" for w in WEIGHTS])
ax[2].set_yscale("log")
ax[2].set_ylabel("S(ic arm) / S(rho-neg)")
ax[2].set_title("V2: distance remaining to matched rho", fontsize=9)
ax[2].legend(fontsize=6)

fig.suptitle("ecbench sessions ECBS1hd7774432e6f8 (V2) and ECBS1ha02d58b9932d (h=1); 448 matched pairs each; "
             "bars: ecbench linearised 95% interval", fontsize=8)
fig.tight_layout(rect=(0, 0, 1, 0.94))
fig.savefig(Path(__file__).parent / "fig1.svg")
print("wrote fig1.svg and figure-data.json")
