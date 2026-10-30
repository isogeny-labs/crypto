# plot_e0.py -- plots recorded rows only (out/*.txt); no new statistics are computed here beyond per-p sums of recorded weights.
import glob, collections, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
D = os.path.dirname(os.path.abspath(__file__))
os.chdir(D)

def rows(pattern, tag):
    out = []
    for fn in sorted(glob.glob(pattern)):
        with open(fn, errors="replace") as f:
            for line in f:
                if line.startswith(tag + "|"):
                    out.append(line.rstrip("\n").split("|"))
    return out

# Figure 1: weighted fraction of curves with #E even (== literal-lemma failure) per odd prime, T2 rows
t2 = rows("out/T2a.txt", "T2") + rows("out/T2b.txt", "T2") + rows("out/T2c.txt", "T2")
W = collections.defaultdict(float); WE = collections.defaultdict(float); WL = collections.defaultdict(float)
for r in t2:
    if r[2] == "1" and r[1] != "":
        p = int(r[1]); w = float(r[4]); nE = int(r[6]); lit = int(r[14])
        W[p] += w; WE[p] += w * (nE % 2 == 0); WL[p] += w * lit
ps = sorted(W)
fig, ax = plt.subplots(figsize=(8, 4.2))
ax.plot(ps, [WE[p] / W[p] for p in ps], "o-", ms=3, label="fraction of curves with #E even (recorded)")
ax.plot(ps, [WL[p] / W[p] for p in ps], "x", ms=5, label="fraction where literal lemma (B=O) fails (recorded)")
ax.axhline(2.0 / 3.0, color="gray", ls="--", label="heuristic 2/3")
ax.set_xlabel("odd prime p = q"); ax.set_ylabel("weighted fraction of curves")
ax.set_title("E0: literal lemma fails exactly when #E is even (T2, all classes)")
ax.legend(fontsize=7); fig.tight_layout()
fig.savefig("e0_fail_fraction_odd.png", dpi=150); fig.savefig("e0_fail_fraction_odd.svg"); plt.close(fig)

# Figure 2: decompositions per R vs g2 (T1 rows: min and max count per R)
t1 = []
for fn in ["out/T1a.txt", "out/T1b.txt", "out/T1c.txt", "out/T1d.txt", "out/T1_bin_ord.txt", "out/T1_bin_ss.txt"]:
    if os.path.exists(fn):
        t1 += rows(fn, "T1")
cnt = collections.Counter()
for r in t1:
    cnt[(int(r[8]), int(r[17]), int(r[18]))] += 1
fig, ax = plt.subplots(figsize=(5.5, 4.5))
for (g2, cmin, cmax), n in sorted(cnt.items()):
    ax.scatter([g2], [cmin], s=30 + 4 * n ** 0.5, alpha=0.6)
    ax.annotate(str(n), (g2, cmin), fontsize=6, xytext=(3, 3), textcoords="offset points")
ax.plot([0.5, 4.5], [0.5, 4.5], "k--", lw=0.8, label="decompositions per R = g2")
ax.set_xlabel("g2 = |E(F_q)[2]| (recorded)"); ax.set_ylabel("min # decompositions per R (recorded)")
ax.set_title("E0 T1: #(A,B) with A+B=R equals g2 (label = #classes)")
ax.legend(fontsize=7); fig.tight_layout()
fig.savefig("e0_decomp_vs_g2.png", dpi=150); fig.savefig("e0_decomp_vs_g2.svg"); plt.close(fig)

# Figure 3: T5 group-Hom count vs gcd(#E,#E')
t5 = rows("out/T5.txt", "T5")
fig, ax = plt.subplots(figsize=(5.5, 4.5))
xs = [int(r[7]) for r in t5]; ys = [int(r[10]) for r in t5]
ax.scatter(xs, ys, s=8, alpha=0.5)
ax.set_xscale("log"); ax.set_yscale("log")
ax.set_xlabel("gcd(#E,#E') (recorded)"); ax.set_ylabel("|Hom_grp(E(F_q),E'(F_q))| (recorded)")
ax.axhline(1, color="gray", lw=0.5)
ax.set_title("E0 T5: Hom is trivial iff gcd = 1")
fig.tight_layout(); fig.savefig("e0_hom_vs_gcd.png", dpi=150); fig.savefig("e0_hom_vs_gcd.svg"); plt.close(fig)
print("plots written")
