#!/usr/bin/env python3
"""Lay out recorded round-4 values (README tables) as round4.svg; no computation."""
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
fig, (a1, a2, a3) = plt.subplots(1, 3, figsize=(12, 3.8))
# item 2: Singular std wall time (s); lower bounds open
labs = ["F dp", "F wp", "G dp", "G wp"]; vals = [105.1, 97.3, 1800, 1800]; lb = [0, 0, 1, 1]
for i, (v, l) in enumerate(zip(vals, lb)):
    a1.bar(i, v, color="#7f7f7f" if labs[i][0] == "F" else "#c0504d", fill=not l, edgecolor="k", hatch="//" if l else "")
a1.set_xticks(range(4)); a1.set_xticklabels(labs); a1.set_yscale("log"); a1.set_ylabel("Singular std wall s")
a1.set_title("Item 2: weights do not rescue G\n(hatched = cut off at 1,800 s)", fontsize=9)
# item 4: widest matrix, msolve
a2.bar([0, 1], [25742, 94422], color=["#2e75b6", "#4f9a4f"])
a2.set_xticks([0, 1]); a2.set_xticklabels(["T (697 s)", "swap-adapted (680 s)"]); a2.set_ylabel("widest Macaulay matrix (cols)")
a2.set_title("Item 4: same D = 7, 3.7x wider matrices", fontsize=9)
# item 3: log2 cost / log2 rho at 2^256
rows = ["w2 b0", "w2 b.5", "w2.8 b0", "w2.8 b.5"]; r = [1.22, 2.33, 1.49, 3.12]
a3.bar(range(4), r, color="#2e75b6"); a3.axhline(1, color="k", ls=":")
a3.set_xticks(range(4)); a3.set_xticklabels(rows, fontsize=8); a3.set_ylabel("log2 cost / log2 rho at 2^256")
a3.set_title("Item 3: model cost vs rho at 256 bits\n(below 1 would beat rho)", fontsize=9)
fig.tight_layout(); fig.savefig("round4.svg")
