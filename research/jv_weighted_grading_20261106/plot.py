"""Plots the probe output in raw/: Macaulay echelon multiplications against
the Macaulay degree, unweighted and weighted (1,2,3,4,2), target 0, with
f4_fp's whole-solve count as a line.  python3 plot.py"""
import re
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

pat = re.compile(r"target 0 weights \[([0-9, ]+)\] D (\d+): rows (\d+) cols (\d+) .*?echelon_muls (\d+) .*?outcome \"([^\"]+)\"")
series = {}
f4 = None
for name in ("raw/probe_unweighted_p271.txt", "raw/probe_weighted_p271.txt"):
    for line in open(name):
        m = pat.search(line)
        if m:
            w = m.group(1).replace(" ", "")
            series.setdefault(w, {})[int(m.group(2))] = (int(m.group(5)), m.group(6))
        m2 = re.search(r"target 0 f4_fp: field_ops (\d+)", line)
        if m2:
            f4 = int(m2.group(1))
fig, ax = plt.subplots(figsize=(8, 4.5))
for w, label, color in (("1,1,1,1,1", "unweighted (degree)", "#2a6fbb"), ("1,2,3,4,2", "weighted (1,2,3,4,2) (weighted degree)", "#d9822b")):
    pts = sorted(series[w].items())
    xs = [d for d, _ in pts]
    ys = [v[0] for _, v in pts]
    ax.plot(xs, ys, "o-", color=color, label=label, ms=3)
    for d, (v, outcome) in pts:
        if outcome in ("solved",) or "inconsistent" in outcome:
            ax.plot([d], [v], "*", color=color, ms=12)
            break
ax.axhline(f4, color="grey", ls="--", lw=1, label=f"f4_fp, whole solve ({f4:.2e})")
ax.set_yscale("log")
ax.set_xlabel("Macaulay degree (weighted degree for the weighted grading)")
ax.set_ylabel("echelon F_p multiplications")
ax.set_title("JV k=5 Edwards residual, p = 271: first decided degree marked ★", fontsize=10)
ax.legend(fontsize=8)
fig.tight_layout()
fig.savefig("fig_weighted.svg")
fig.savefig("fig_weighted.png", dpi=120)
