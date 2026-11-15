#!/usr/bin/env python3
"""U2 distance to matched negation rho (w = 0, pooled reference (b)) before and after the Hilbert stop.
Before: research/jv_cover_fbline_20261102/README.md (line search). After: rescore/table.md (this study)."""
import pathlib
import matplotlib; matplotlib.use("Agg"); import matplotlib.pyplot as plt
here = pathlib.Path(__file__).parent
p = [1301, 1451, 1601]
before = [(1.900, 1.735, 2.092), (1.585, 1.460, 1.725), (1.318, 1.217, 1.432)]
after = [(1.003, 0.916, 1.104), (0.837, 0.771, 0.910), (0.695, 0.642, 0.756)]
fig, ax = plt.subplots(figsize=(5.6, 3.8))
for data, lab, col, dx in ((before, "line search (fbline, 2026-11-02)", "#c0562b", -6), (after, "line search + Hilbert stop (this study)", "#1f5fa8", 6)):
    ax.errorbar([x + dx for x in p], [d[0] for d in data], yerr=[[d[0] - d[1] for d in data], [d[2] - d[0] for d in data]],
                fmt="o-", color=col, capsize=3, label=lab)
ax.axhline(1, color="#555", lw=0.9, ls=":"); ax.text(1605, 1.02, "matched negation rho", fontsize=7, ha="right", color="#555")
ax.set_xticks(p); ax.set_xlabel("p  (Joux–Vitse cover of E(F_{p^6}), Nagao arm)"); ax.set_ylabel("S_cover / S_rho, U2 textbook F_p mults, w = 0")
ax.set_title("Hilbert early stop at scale: distance to matched rho (local, unvalidated)", fontsize=9)
ax.legend(fontsize=8); fig.tight_layout()
for ext in ("svg", "png", "pdf"): fig.savefig(here / f"u2_before_after.{ext}", dpi=200 if ext == "png" else None)
