"""Plot recorded values from ../../summary (pairs.tsv, gap_to_rho.txt); no new computation beyond log2 of recorded ratios."""
import csv, math, re
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
H = Path(__file__).resolve().parent
S = H.parent.parent / "summary"
rows = list(csv.DictReader(open(S / "pairs.tsv"), delimiter="\t"))
cells = sorted({(int(r["n"]), int(r["l"])) for r in rows})
fig, (a1, a2) = plt.subplots(1, 2, figsize=(11, 4.2))
for i, c in enumerate(cells):
    rs = [r for r in rows if (int(r["n"]), int(r["l"])) == c]
    for j, (w, col) in enumerate((("w0", "#1f5fa8"), ("w0.1", "#e69138"), ("w1", "#b45f06"))):
        ys = [-math.log2(float(r["band_" + w])) for r in rs]
        a1.scatter([i + (j - 1) * 0.22] * len(ys), ys, color=col, s=18, label=f"{w[1:]} GAE" if i == 0 else None, zorder=3)
    lo = [-math.log2(float(r["ci_hi"])) for r in rs]; hi = [-math.log2(float(r["ci_lo"])) for r in rs]
    a1.vlines([i - 0.22 + 0.04 * k for k in range(len(rs))], lo, hi, color="#1f5fa8", lw=0.8)
a1.axhline(0, color="k", lw=0.7)
a1.set_xticks(range(len(cells))); a1.set_xticklabels([f"n={n} l={l}" for n, l in cells], fontsize=8)
a1.set_ylabel("log2(S_prefix / S_strict4e)  (positive = strict4e cheaper)")
a1.set_title("Per sieve seed (4 per cell), 8 targets each; bars: ecbench compare 95% CI at 0 GAE", fontsize=8)
a1.legend(fontsize=8, title="unpriced ops priced at", title_fontsize=8); a1.grid(alpha=0.25, axis="y")
g = {}
for line in open(S / "gap_to_rho.txt"):
    m = re.match(r"(\d+) l=(\d) (prefix|strict).*w0=([-\d.]+) w0.1=([-\d.]+) w1=([-\d.]+)", line)
    if m: g[(int(m[1]), int(m[2]), m[3])] = [float(m[4]), float(m[5]), float(m[6])]
x = range(len(cells))
for arm, mk, col in (("prefix", "s", "#888"), ("strict", "o", "#1f5fa8")):
    a2.plot(x, [g[(n, l, arm)][0] for n, l in cells], mk + "-", color=col, label=f"{arm}, 0 GAE")
    a2.plot(x, [g[(n, l, arm)][2] for n, l in cells], mk + ":", color=col, alpha=0.6, label=f"{arm}, 1 GAE")
a2.axhline(0, color="k", lw=0.7)
a2.set_xticks(list(x)); a2.set_xticklabels([f"n={n} l={l}" for n, l in cells], fontsize=8)
a2.set_ylabel("log2(S_IC / S_rho-strong)  (above 0 = rho cheaper)")
a2.set_title("Gap to matched rho.signed_frobenius_strong (strict: mean of 4 seeds)", fontsize=8)
a2.legend(fontsize=8); a2.grid(alpha=0.25)
fig.suptitle("research/fb_strict4e_20261027, sessions n13/n19/n23, run 2026-10-27T08:02Z", fontsize=9)
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(H / f"fig-delta-and-gap.{ext}", dpi=150)
