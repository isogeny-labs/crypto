#!/usr/bin/env python3
"""Render figures/quotient_solve.{svg,png} from analysis.json (layout only; run analyze.py first)."""
import json
import math
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
A = json.loads((HERE / "analysis.json").read_text())
FIG = HERE / "figures"
FIG.mkdir(exist_ok=True)
CLOCK = 2.8e9
COL = {4111: "#0072B2", 262231: "#009E73", 33554371: "#D55E00", "dev": "#777777"}
ARMS = ["raw_x", "sym_x", "raw_u", "sym_u", "quot"]
PRED = {"raw_x": 384, "sym_x": 64, "raw_u": 384, "sym_u": 64, "quot": 16}

fig, axs = plt.subplots(2, 2, figsize=(13.5, 9.6))
ax = axs.ravel()

# (a) m = 3 ideal degrees against the prediction
a = ax[0]
xs = range(len(ARMS))
a.plot(list(xs), [math.log2(PRED[k]) for k in ARMS], "k_", ms=28, mew=2, label="predicted")
off = {4111: -0.2, 262231: 0.0, 33554371: 0.2}
for r in A["m3"]:
    p = r["p"]
    if p not in off:
        continue
    ys = [math.log2(r["arms"][k]["D_log"]) if r["arms"][k]["D_log"] else None for k in ARMS]
    mk = "o" if r["target"] == "planted" else "s"
    for i, y in enumerate(ys):
        if y is not None:
            a.plot(i + off[p], y, mk, color=COL[p], ms=6, mfc=COL[p] if mk == "o" else "white",
                   label=f"p = {p}, {r['target']}" if i == 0 else None)
a.set_xticks(list(xs), ARMS)
a.set_ylabel("log2 D (msolve dimension of quotient)")
a.set_title("(a) m = 3: D for every arm, prime and target", fontsize=10)
a.legend(fontsize=7, frameon=False, loc="upper right")
a.set_ylim(3, 9.5)

# (b) cumulative cost against F4 degree
b = ax[1]
cells = A["m5"] + ([A["dev"]] if A.get("dev") else [])
for c in cells:
    rounds = c.get("rounds") or []
    if not rounds:
        continue
    cum, ys, ds = 0.0, [], []
    for r in rounds:
        cum += r["cpu_s"]
        if cum <= 0:
            continue
        ys.append(math.log2(cum * CLOCK))
        ds.append(r["deg"])
    key = c["p"] if c["p"] in COL and not c["cell"].startswith("dev") else "dev"
    b.plot(ds, ys, "-o", ms=3, color=COL[key], ls="--" if key == "dev" else "-",
           label=(f"p = {c['p']}" if key != "dev" else "dev p = 331 (not data)"))
    if c.get("log2_c_cyc") and key != "dev":
        b.plot([c["last_f4_degree_started"]], [c["log2_c_cyc"]], "*", ms=12, color=COL[key])
b.axhline(38, color="#E69F00", ls=":", lw=1.4)
b.text(b.get_xlim()[0] if False else 13.2, 38.3, "PROMOTE needs c_cyc ≤ 2^38 (c_ops ≤ 2^40 at 4 ops/cycle)", fontsize=7.5, color="#B07800")
b.axhline(46, color="#CC0000", ls=":", lw=1.4)
b.text(13.2, 46.3, "CLOSE needs c_cyc ≥ 2^46 (c_ops ≥ 2^44 at 1/4 op/cycle)", fontsize=7.5, color="#CC0000")
b.set_xlabel("F4 degree (msolve, DRL)")
b.set_ylabel("log2 cumulative CPU cycles (2.8 GHz)")
b.set_title("(b) m = 5 quotient: cost spent by F4 degree (★ = total at stop)", fontsize=10)
b.legend(fontsize=7.5, frameon=False, loc="lower right")

# (c) RSS over time
c_ = ax[2]
for c in cells:
    s = c.get("rss_samples") or []
    if not s:
        continue
    key = c["p"] if c["p"] in COL and not c["cell"].startswith("dev") else "dev"
    c_.plot([t / 3600 for t, _ in s], [k / 2 ** 20 for _, k in s], color=COL[key], ls="--" if key == "dev" else "-",
            label=(f"p = {c['p']} ({c['stop']})" if key != "dev" else "dev p = 331 (not data)"))
c_.axhline(12, color="#CC0000", ls=":", lw=1.4)
c_.text(0.02, 12.2, "tier-L RSS cap 12 GiB", fontsize=7.5, color="#CC0000")
c_.set_xlabel("wall time (h), 4 threads")
c_.set_ylabel("RSS (GiB)")
c_.set_title("(c) m = 5 quotient: resident memory", fontsize=10)
c_.legend(fontsize=7.5, frameon=False, loc="upper left", bbox_to_anchor=(0.0, 0.92))
# (d) probe arms at p = 4111 against W0's F4
d_ = ax[3]
W0 = A.get("W0_f4_cpu_s")
rows = [("W0  grevlex F4 (msolve)", W0, "complete")]
for r in A["probes"]["weighted"]:
    rows.append((f"{r['arm']}  W = {r['weights']} (msolve)", r["f4_cpu_s"], r["stop"]))
for r in A["probes"]["sba"]:
    rows.append((f"{r['arm']}  {r['call']} (Singular)", r["cpu_s"], r["stop"]))
lab = {"complete": "complete", "wall_cap": "stopped at cap", "owner_stop": "stopped by owner", "rss_cap": "stopped at RSS cap"}
for i, (name, cpu, st) in enumerate(rows):
    done = st == "complete"
    d_.barh(i, cpu, color="#0072B2" if done else "white", edgecolor="#0072B2", hatch=None if done else "//", lw=1.2)
    d_.text(cpu * 1.01, i, f"{cpu:,.0f} s  {lab.get(st, st)}" + ("" if done else " (lower bound)"), va="center", fontsize=7.5)
d_.axvline(W0, color="#555555", ls="-", lw=1)
d_.axvline(W0 / 1.5, color="#009E73", ls=":", lw=1.6)
d_.text(W0 / 1.5, len(rows) - 0.45, "improvement bar: W0 / 1.5", fontsize=7.5, color="#007A55", ha="center")
d_.set_yticks(range(len(rows)), [r[0] for r in rows], fontsize=8)
d_.invert_yaxis()
d_.set_xlim(0, max(r[1] for r in rows) * 1.45)
d_.set_xlabel("CPU seconds (user + sys) to a Gröbner basis, m = 5, p = 4111")
d_.set_title("(d) weighted orders (A2) and signature-based F5 (A3) against W0", fontsize=10)
for x in ax:
    for s in ("top", "right"):
        x.spines[s].set_visible(False)
fig.suptitle("EcGFp5-shaped (Z/2)^4 ⋊ S5 quotient: m = 3 validation and m = 5 msolve solves (toy primes, measured)", fontsize=11)
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(FIG / f"quotient_solve.{ext}", dpi=160, bbox_inches="tight")
print("wrote", sorted(p.name for p in FIG.iterdir()))
