# Lays out recorded values from raw/assess-r2-summarize.stdout.log (b06 summarize over raw/, repaired instrument);
# per-target ranges from the same raw rows. No result is computed here.
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
cells = ["c13-3-4", "c15-3-4", "c15-5-3", "c17-4-3"]
sig = [0.494, 1.677, 0.497, 0.492]          # upper bounds (stopped jobs)
f4r = [0.885, 0.630, 0.741, 0.587]
f4n = [1.120, 1.026, 1.413, 1.160]
lab = ["sig 16/16 touchcap\nf4r,f4n 16/16", "sig mixed caps (5/4/4/3)\nf4r,f4n 15/16 (f4 t1 timelimit)",
       "sig 16/16 touchcap\nf4r,f4n 16/16", "sig 16/16 touchcap\nf4r,f4n 16/16"]
fig, ax = plt.subplots(figsize=(9, 5))
w = 0.26
for j, (name, v, col) in enumerate((("sig (upper bound, stopped jobs)", sig, "#b2182b"),
                                    ("f4r exact-key replay", f4r, "#2166ac"),
                                    ("f4n negative-filter replay (descriptive)", f4n, "#4d9221"))):
    xs = [i + (j - 1) * w for i in range(4)]
    bars = ax.bar(xs, v, w, color=col, label=name)
    for x, y in zip(xs, v):
        ax.text(x, y + 0.03, ("<=" if j == 0 else "") + f"{y:.2f}", ha="center", fontsize=8)
bars = ax.patches
bars[1].set_hatch("//")  # c15-3-4 sig: bound does not certify regression
ax.axhline(1.5, color="k", ls="--", lw=1); ax.text(3.45, 1.53, "qualify 1.5", ha="right", fontsize=8)
ax.axhline(1 / 1.5, color="#777", ls=":", lw=1); ax.text(-0.45, 0.69, "regress line 1/1.5", fontsize=8, color="#555")
ax.set_xticks(range(4)); ax.set_xticklabels([f"{c}\n{l}" for c, l in zip(cells, lab)], fontsize=7.5)
ax.set_ylabel("R = touches(f4) / cost(X), geometric mean over targets"); ax.set_ylim(0, 2.0)
ax.set_title("Item 06 (repair pass): paired touch ratio vs F4, by cell. Verdict REGRESSES (constant)", fontsize=10)
ax.legend(fontsize=8, loc="upper right")
fig.text(0.01, 0.005, "Hatched sig bar: c15-3-4 bound 1.68 is not below 1/1.5 (time/mem/column stops), so that cell is not regress-certain. "
         "Replay cost includes the held-out trace /16. Toy scale n=13-17.", fontsize=6.5)
fig.tight_layout(rect=(0, 0.03, 1, 1))
fig.savefig("figures/ratio-by-cell.svg"); fig.savefig("figures/ratio-by-cell.png", dpi=150)
