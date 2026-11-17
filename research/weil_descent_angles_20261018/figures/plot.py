"""Draw the round-3 graphs from the recorded CSVs (layout only; computes nothing
beyond selecting and tallying recorded rows).

Usage: python plot.py <results-dir> <out-dir>
Writes SVG and a 150 dpi PNG of each graph.
"""

import csv
import sys
from collections import Counter, defaultdict
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

res, out = Path(sys.argv[1]), Path(sys.argv[2])
plt.rcParams.update({"font.size": 8, "svg.fonttype": "none"})


def run(prefix):
    c = sorted(d for d in res.iterdir() if d.name.startswith(prefix))
    return c[-1]


def rows(path):
    with open(path, newline="") as f:
        return list(csv.DictReader(f))


def save(fig, name):
    fig.tight_layout()
    fig.savefig(out / f"{name}.svg")
    fig.savefig(out / f"{name}.png", dpi=150)
    plt.close(fig)


D = run("item1D-run")
M = run("item1D-mult-run")
PK = run("item2P-run")
K4 = run("item4-run")
OJ = run("itemOJ-run")

# 1. Part D: paired residual difference per (char, n), and binary n = 2 per curve
s = rows(D / "coverage_summary.csv")
fig, ax = plt.subplots(1, 2, figsize=(9, 3.2))
labels = [f"{'char 2' if r['char'] == '2' else 'odd'} n={r['n']}" for r in s]
vals = [float(r["resid_diff"]) for r in s]
errs = [1.96 * float(r["sd_resid_diff"]) / float(r["curves"]) ** 0.5 for r in s]
cols = ["#c0392b" if abs(float(r["z_resid_diff"])) > 3 else "#4a6fa5" for r in s]
ax[0].bar(range(len(s)), vals, yerr=errs, color=cols, capsize=2)
ax[0].axhline(0, color="k", lw=0.6)
ax[0].set_xticks(range(len(s)), labels, rotation=40, ha="right")
ax[0].set_ylabel("paired residual: subfield − null")
ax[0].set_title("Part D: coverage residual vs Poisson (±1.96 s.e.)")
cv = [r for r in rows(D / "coverage.csv") if r["char"] == "2" and r["n"] == "2" and r["arm"] == "subfield"]
xs = [int(r["q"]).bit_length() - 1 + 0.08 * int(r["curve"]) for r in cv]
ax[1].plot(xs, [float(r["pi"]) for r in cv], "o", ms=3, label="π subfield (exact)")
ax[1].plot(xs, [float(r["poisson"]) for r in cv], "x", ms=3, label="Poisson 1 − e^{−λ}")
ax[1].axhline(3 / 8, color="#c0392b", lw=0.8, ls="--", label="3/8 (Amendment D law)")
ax[1].set_xlabel("m  (q = 2^m, field F_{q^2})")
ax[1].set_ylabel("coverage π")
ax[1].set_title("Binary n = 2: exact coverage per curve")
ax[1].legend(fontsize=7)
fig.suptitle(f"run {D.name}", fontsize=7, y=0.02)
save(fig, "item1_coverage")

# 2. Amendment D: multiplicity histograms (m = 10 binary, p = 809 odd)
mr = rows(M / "mult.csv")
fig, ax = plt.subplots(1, 2, figsize=(9, 3.0))
for a, (ch, q) in zip(ax, [("2", "1024"), ("0", "809")]):
    for arm, mk in [("subfield", "o-"), ("null", "s--")]:
        sel = [r for r in mr if r["char"] == ch and r["q"] == q and r["arm"] == arm and r["curve"] == "0"]
        if not sel:
            continue
        h = [int(sel[0][k]) for k in ["h0", "h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8plus"]]
        a.semilogy(range(9), [max(v, 0.8) for v in h], mk, ms=3, label=f"{arm} (λ={float(sel[0]['lambda']):.3f})")
    a.set_xlabel("decompositions of a point (multiset count)")
    a.set_ylabel("points (log; 0 drawn at 0.8)")
    a.set_title(("binary q = 2^10, n = 2" if ch == "2" else "odd p = 809, n = 2") + ", curve 0")
    a.legend(fontsize=7)
fig.suptitle(f"run {M.name}: binary subfield multiplicities stop at 2 (Bin(2, 1/2) on 2E)", fontsize=7, y=0.02)
save(fig, "item1_multiplicity")

# 3. Part P: z of the paired residual per cell
ps = rows(PK / "pkm_summary.csv")
fig, ax = plt.subplots(figsize=(9, 3.0))
lab = [f"{r['family']} m={r['m']} {int(r['t_offset']):+d}" for r in ps]
z = [float(r["z_resid_diff"]) for r in ps]
ax.bar(range(len(ps)), z, color="#4a6fa5")
for y in (-2, 2):
    ax.axhline(y, color="#c0392b", lw=0.7, ls="--")
ax.axhline(0, color="k", lw=0.6)
ax.set_xticks(range(len(ps)), lab, rotation=55, ha="right", fontsize=6.5)
ax.set_ylabel("z (paired residual, family − null)")
ax.set_title(f"Part P: PKM tower factor bases vs random x-sets (≥ 23 primes per cell); run {PK.name}")
save(fig, "item2_pkm")

# 4. Item 4: trace histograms, hyperelliptic models vs non-hyperelliptic classes
h4 = rows(K4 / "h4_res_shaped.csv")
kc = rows(K4 / "k_classes.csv")
fig, ax = plt.subplots(1, 2, figsize=(9, 3.0))
ch = Counter(int(r["t"]) for r in h4)
ts = sorted(ch)
ax[0].bar(ts, [ch[t] for t in ts], color="#4a6fa5")
ax[0].set_xlabel("trace t of E over F_81")
ax[0].set_ylabel("Res-shaped hyperelliptic models")
ax[0].set_title(f"genus 4 hyperelliptic over F_3: {len(h4)} models, every t even")
byt = defaultdict(lambda: [0, 0, 0])
for r in kc:
    t = int(r["t"])
    if r["numE_prime"] == "true" and r["elliptic_trace_exists"] == "true":
        byt[t][2] += 1
    elif r["elliptic_trace_exists"] == "true":
        byt[t][1] += 1
    else:
        byt[t][0] += 1
ts = sorted(byt)
b0 = [byt[t][0] for t in ts]
b1 = [byt[t][1] for t in ts]
b2 = [byt[t][2] for t in ts]
ax[1].bar(ts, b0, color="#bbbbbb", label="no elliptic curve with this t")
ax[1].bar(ts, b1, bottom=b0, color="#4a6fa5", label="E exists, #E composite")
ax[1].bar(ts, b2, bottom=[x + y for x, y in zip(b0, b1)], color="#c0392b", label="E exists, #E prime (P6)")
ax[1].set_xlabel("trace t")
ax[1].set_ylabel("Res-shaped non-hyperelliptic classes")
ax[1].set_title(f"genus 4 non-hyperelliptic over F_3: {len(kc)} classes")
ax[1].legend(fontsize=6.5)
fig.suptitle(f"run {K4.name}", fontsize=7, y=0.02)
save(fig, "item4_traces")

# 5. Parts O/J: steps per p and the IC/rho ratio band
rs = rows(OJ / "routes.csv")
fig, ax = plt.subplots(1, 2, figsize=(9, 3.4))
for arm, c, mk, colr in [("R", "0", "o", "#333333"), ("J", "2", "^", "#4a6fa5"), ("D", "2", "s", "#c0392b")]:
    sel = [r for r in rs if r["arm"] == arm and r["c"] == c]
    xs = [int(r["bits"]) + {"R": -0.4, "J": 0, "D": 0.4}[arm] for r in sel]
    ys = [int(r["steps"]) / int(r["p"]) for r in sel]
    ax[0].plot(xs, ys, mk, ms=3, color=colr, alpha=0.7, label={"R": "rho (steps)", "J": "Arm J, c=2 (Jacobian ops in walk)", "D": "Arm D, c=2 (oracle calls)"}[arm])
ax[0].axhline(1 / 3, color="#4a6fa5", ls="--", lw=0.8, label="1/3 (first cycle, P8b)")
for h in (2, 4, 8):
    ax[0].axhline(0.886 / h ** 0.5, color="#333333", ls=":", lw=0.7)
    ax[0].text(41, 0.886 / h ** 0.5, f"0.886/√{h}", fontsize=6, va="center")
ax[0].set_xlabel("log2 #E(F_{p^2})")
ax[0].set_ylabel("walk steps / p")
ax[0].set_title("Group operations per p (recorded steps)")
ax[0].legend(fontsize=6)
rk = rows(OJ / "ranking.csv")
for arm, off, colr in [("J", -0.3, "#4a6fa5"), ("D", 0.3, "#c0392b")]:
    for c, dc in [("1.0", -0.1), ("2.0", 0), ("4.0", 0.1)]:
        sel = [r for r in rk if r["arm"] == arm and r["c"] in (c, c.rstrip("0").rstrip("."))]
        for r in sel:
            x = int(r["bits"]) + off + dc
            ax[1].vlines(x, float(r["min_ratio"]), float(r["max_ratio"]), color=colr, lw=2)
ax[1].axhline(1, color="k", lw=0.7)
ax[1].set_yscale("log")
ax[1].set_xlabel("log2 #E(F_{p^2})   (bars: c = 1, 2, 4)")
ax[1].set_ylabel("IC cost / rho cost (band over pricing cells)")
ax[1].set_title("Pricing band: blue Arm J, red Arm D")
fig.suptitle(f"run {OJ.name}", fontsize=7, y=0.02)
save(fig, "itemOJ_routes")
print("wrote graphs to", out)

# 6. Amendment F control: Arm J walk vs independent trials, by cofactor
F = run("itemF-control-run")
cr = rows(F / "control.csv")
fig, ax = plt.subplots(figsize=(6.5, 3.2))
for arm, off, colr in [("Jwalk", -0.15, "#4a6fa5"), ("Jind", 0.15, "#c0392b")]:
    for h, base in [("4", 0), ("8", 1)]:
        sel = [r for r in cr if r["arm"] == arm and r["h"] == h]
        ax.plot([base + off] * len(sel), [int(r["steps"]) / int(r["p"]) for r in sel], "o", ms=3, alpha=0.6, color=colr,
                label=f"{arm}" if h == "4" else None)
ax.axhline(1 / 3, color="k", ls="--", lw=0.8, label="1/3 (random-graph first cycle)")
for h, base in [(4, 0), (8, 1)]:
    ax.hlines(0.886 / h ** 0.5, base - 0.35, base + 0.35, color="#333333", ls=":", lw=1)
    ax.text(base + 0.37, 0.886 / h ** 0.5, f"rho 0.886/√{h}", fontsize=6.5, va="center")
ax.set_xticks([0, 1], ["h = 4", "h = 8"])
ax.set_xlim(-0.6, 1.9)
ax.set_ylabel("trials (Jacobian ops) / p")
ax.set_title(f"Amendment F: walk vs independent trials, c = 2; run {F.name}", fontsize=7.5)
ax.legend(fontsize=6.5)
save(fig, "itemF_control")
print("wrote control graph")
