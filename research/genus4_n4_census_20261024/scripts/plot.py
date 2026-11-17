"""Draw the graphs from the recorded CSVs (layout only).
Usage: python plot.py <results-dir> <out-dir>"""
import csv, sys
from pathlib import Path
from collections import defaultdict
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

res, out = Path(sys.argv[1]), Path(sys.argv[2])
plt.rcParams.update({"font.size": 8, "svg.fonttype": "none"})
BLUE, ORANGE, GREY = "#2a78d6", "#d9822b", "#8a8a85"

def run(prefix):
    c = sorted(d for d in res.iterdir() if d.name.startswith(prefix + "-run"))
    return c[-1] if c else None

def rows(p):
    with open(p, newline="") as f:
        return list(csv.DictReader(f))

def save(fig, name):
    fig.tight_layout()
    fig.savefig(out / f"{name}.svg"); fig.savefig(out / f"{name}.png", dpi=150); plt.close(fig)

# 1. mass per t with predictor fit
qs = [q for q in (3, 5, 7) if run(f"join-q{q}")]
if qs:
    fig, ax = plt.subplots(1, len(qs), figsize=(3.6 * len(qs), 3.0), squeeze=False)
    for a, q in zip(ax[0], qs):
        r = rows(run(f"join-q{q}") / "join_by_t.csv")
        t = [int(x["t"]) for x in r]; m = [float(x["mass"]) for x in r]
        e = [float(x["mass_se"]) for x in r]; c = [float(x["arith_x_haar"]) for x in r]
        k = sum(m) / max(sum(c), 1e-12)
        a.errorbar(t, m, yerr=e, fmt="o", ms=3, color=BLUE, label="mass of hits (weighted classes)")
        a.plot(t, [k * x for x in c], "x", color=ORANGE, label="c·Haar·H/√Δ (fit)")
        a.set_title(f"q = {q}"); a.set_xlabel("trace t of E/F_{q^4}"); a.set_ylabel("mass")
        a.set_yscale("symlog", linthresh=1)
        if a is ax[0][0]: a.legend(frameon=False, fontsize=6)
    save(fig, "fig_mass_by_t")

# 2. controls: parity of Res-shaped traces
fig, ax = plt.subplots(1, 2, figsize=(7, 2.8))
labels, ev, od = [], [], []
for q in (3, 5, 7):
    d = run(f"ctrlH-q{q}")
    if d:
        h = rows(d / "hyper_t_hist.csv")
        labels.append(f"q={q}"); ev.append(sum(int(x["count"]) for x in h if int(x["t"]) % 2 == 0))
        od.append(sum(int(x["count"]) for x in h if int(x["t"]) % 2 != 0))
x = range(len(labels))
ax[0].bar([i - .2 for i in x], ev, .4, color=BLUE, label="even t"); ax[0].bar([i + .2 for i in x], od, .4, color=ORANGE, label="odd t")
ax[0].set_xticks(list(x)); ax[0].set_xticklabels(labels); ax[0].set_yscale("symlog"); ax[0].set_title("hyperelliptic Res-shaped (Theorem 5)")
ax[0].legend(frameon=False)
labels, ev, od = [], [], []
for q in (3, 5, 7):
    c = defaultdict(lambda: [0, 0])
    for ty in ("split", "nonsplit", "cone"):
        d = run(f"cen-q{q}-{ty}") or run(f"ctrlK-q{q}-{ty}")
        if d and (d / "cand_t_hist.csv").exists():
            for r in rows(d / "cand_t_hist.csv"):
                c[0][int(r["t"]) % 2] += int(r["candidates"])
    if c:
        labels.append(f"q={q}"); ev.append(c[0][0]); od.append(c[0][1])
x = range(len(labels))
ax[1].bar([i - .2 for i in x], ev, .4, color=BLUE); ax[1].bar([i + .2 for i in x], od, .4, color=ORANGE)
ax[1].set_xticks(list(x)); ax[1].set_xticklabels(labels); ax[1].set_yscale("symlog"); ax[1].set_title("non-hyperelliptic Res-shaped (sampled forms)")
save(fig, "fig_controls")

# 3. Tian heat map
qs = [q for q in (3, 5, 7) if run(f"tian-q{q}")]
if qs:
    fig, ax = plt.subplots(1, len(qs), figsize=(3.2 * len(qs), 3.6), squeeze=False)
    col = {"yes": BLUE, "depends": ORANGE, "no": "#e8e6df"}
    for a, q in zip(ax[0], qs):
        r = rows(run(f"tian-q{q}") / "tian_by_t.csv")
        ts = sorted({int(x["t"]) for x in r}); ells = sorted({int(x["ell"]) for x in r})
        for x in r:
            a.add_patch(plt.Rectangle((ells.index(int(x["ell"])), ts.index(int(x["t"]))), 1, 1, color=col[x["tian_exists"]]))
        a.set_xlim(0, len(ells)); a.set_ylim(0, len(ts))
        a.set_xticks([i + .5 for i in range(len(ells))]); a.set_xticklabels(ells)
        a.set_yticks([i + .5 for i in range(len(ts))]); a.set_yticklabels(ts, fontsize=5)
        a.set_title(f"q={q}: Tian kernel (blue yes, orange depends)"); a.set_xlabel("l")
    save(fig, "fig_tian")
