#!/usr/bin/env python3
"""Rank deficit of the selector system vs the trivial-kernel prediction M(k-1, e-1)
(LEAD-A-selector-eta-kernel), from the independently re-checked certificates in
research/lfd_lower_bound_20261111/out/c/verify_*.txt. Post-hoc consistency check."""
import glob, json, re, pathlib
from math import comb
import matplotlib; matplotlib.use("Agg"); import matplotlib.pyplot as plt
here = pathlib.Path(__file__).parent
src = here.parent.parent / "lfd_lower_bound_20261111" / "out" / "c"
M = lambda k, e: sum(comb(k, d) for d in range(e + 1))
pts = set()
for f in glob.glob(str(src / "verify_*.txt")):
    txt = open(f).read()
    for m in re.finditer(r'n=(\d+) s(?:eed)?=(\d+) e=(\d+).*?"rank_A": (\d+)', txt):
        pts.add(tuple(map(int, m.groups())))
    if re.search(r"verify_35_s(\d)", f) and txt.strip().startswith("{"):
        s = int(re.search(r"verify_35_s(\d)", f).group(1)); pts.add((35, s, 4, json.loads(txt)["rank_A"]))
xs, ys, lab = [], [], []
for n, s, e, r in sorted(pts):
    k = (n + 1) // 2; xs.append(M(k - 1, e - 1)); ys.append(n * M(k, e) - r); lab.append(f"n={n}, e={e}")
fig, ax = plt.subplots(figsize=(5.2, 4))
lim = max(xs + ys) * 1.1
ax.plot([0, lim], [0, lim], ":", color="#555", label="deficit = M(k-1, e-1)")
ax.scatter(xs, ys, color="#1f5fa8", zorder=3, label=f"{len(xs)} certificates")
for i, (x, y, l) in enumerate(sorted(set(zip(xs, ys, lab)))):
    ax.annotate(l, (x, y), xytext=(8, -12 + 14 * (i % 3)), textcoords="offset points", fontsize=7)
ax.set_xlabel("predicted trivial kernel M(k-1, e-1)"); ax.set_ylabel("measured rank deficit (columns - rank)")
ax.set_title("Selector system: rank deficit vs trivial kernel (post hoc)", fontsize=9)
ax.legend(fontsize=8, loc="upper left"); fig.tight_layout()
for ext in ("svg", "png", "pdf"):
    fig.savefig(here / f"deficit_vs_kernel.{ext}", dpi=200 if ext == "png" else None)
print(len(xs), "points; all equal:", all(x == y for x, y in zip(xs, ys)))
