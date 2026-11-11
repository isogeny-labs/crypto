#!/usr/bin/env python3
"""Plot the degree table of NOTE.md section 6 (d_F, d_SEL, ceil(k/3)+2 against k on the diagonal).

Values are copied from the table in NOTE.md, which pools out/n*.txt and out/emin_*.txt.
Ranges are drawn as their maximum with a vertical bar down to the minimum.
"""
import math, pathlib
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = pathlib.Path(__file__).parent
# k: (d_F min, d_F max), (d_SEL min, d_SEL max); d_SEL at k=9..12 is e_min+2 from out/emin_*.txt
rows = {
    4: ((2, 3), (3, 4)), 5: ((3, 3), (4, 4)), 6: ((3, 3), (4, 5)), 7: ((3, 3), (5, 5)),
    8: ((4, 4), (5, 5)), 9: ((4, 4), (5, 5)), 10: ((4, 4), (6, 6)), 11: ((4, 4), (6, 6)),
    12: (None, (6, 6)),
}
ks = sorted(rows)
fig, ax = plt.subplots(figsize=(6.4, 3.8))
for idx, (label, color, marker) in enumerate([("last fall degree $d_F$", "#1f5fa8", "o"),
                                              ("selector degree $d_{SEL}$", "#c0562b", "s")]):
    xs, ys, lo = [], [], []
    for k in ks:
        v = rows[k][idx]
        if v is None:
            continue
        xs.append(k + (-0.08 if idx == 0 else 0.08)); ys.append(v[1]); lo.append(v[1] - v[0])
    ax.errorbar(xs, ys, yerr=[lo, [0] * len(lo)], fmt=marker + "-", color=color, label=label, capsize=3)
ax.plot(ks, [math.ceil(k / 3) + 2 for k in ks], ":", color="#555", label=r"$\lceil k/3\rceil+2$ (conjecture S)")
ax.axhline(5, color="#999", lw=0.8)
ax.text(12.1, 5.05, "certified $d_F\\geq5$ at $n=33$–45", fontsize=7, ha="right", color="#666")
ax.set_xlabel("$k=\\lceil n/2\\rceil$  ($n = 2k-1$)")
ax.set_ylabel("degree")
ax.set_xticks(ks)
ax.set_yticks(range(2, 7))
ax.set_title("Binary $m=2$ descent, unsatisfiable random-subspace instances, $n\\leq23$", fontsize=9)
ax.legend(fontsize=8, loc="upper left")
fig.tight_layout()
for ext in ("svg", "png", "pdf"):
    fig.savefig(here / f"degrees.{ext}", dpi=200 if ext == "png" else None)
