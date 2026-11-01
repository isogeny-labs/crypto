# Lays out recorded values only (no computation of results): F4 S before/after #1774 pricing
# (values from #2247's table and f6 ladder RESULT.md) and KS p recorded vs tie-fixed (ks_recompute/ks_recompute.tsv).
import csv
import matplotlib; matplotlib.use('svg')
import matplotlib.pyplot as plt
fig, ax = plt.subplots(1, 2, figsize=(11, 4.4))
rows = [("ladder n13\ninherited F4", 16.27, 488.89), ("ladder n23\ninherited F4", 2.134, 9479.7),
        ("K13 cross-family\ninherited F4", 16.49, 634.12), ("K13 cross-family\nmatrix F4", 16.44, 113830)]
x = range(len(rows)); w = 0.38
ax[0].bar([i-w/2 for i in x], [r[1] for r in rows], w, label="recorded (word XORs uncharged, lower bound)", color="#8fa8c8")
ax[0].bar([i+w/2 for i in x], [r[2] for r in rows], w, label="re-run on main (#2247; XORs charged)", color="#c0504d")
ax[0].axhline(14.71, ls=":", c="k"); ax[0].text(3.45, 15.5, "strong rho, n13 (S=14.71)", ha="right", fontsize=7)
ax[0].axhline(1.046, ls="--", c="k", lw=0.7); ax[0].text(3.45, 1.12, "strong rho, n23 (S=1.046)", ha="right", fontsize=7)
ax[0].set_yscale("log"); ax[0].set_xticks(list(x)); ax[0].set_xticklabels([r[0] for r in rows], fontsize=7)
ax[0].set_ylabel("S = GAE / sqrt(r)"); ax[0].legend(fontsize=7, loc="upper left")
ax[0].set_title("Item 2: F4 S before / after #1774 pricing\n(values from #2247, not re-run here)", fontsize=9)
rec, fix, chg = [], [], []
for r in csv.reader(open("ks_recompute/ks_recompute.tsv"), delimiter="\t"):
    p0, p1 = float(r[5]), float(r[9]); rec.append(p0); fix.append(p1); chg.append(abs(p0-p1) > 1e-12)
ax[1].scatter([a for a, c in zip(rec, chg) if not c], [b for b, c in zip(fix, chg) if not c], s=3, c="#8fa8c8", label="2,914 unchanged")
ax[1].scatter([a for a, c in zip(rec, chg) if c], [b for b, c in zip(fix, chg) if c], s=10, c="#c0504d", label="49 changed (max |dp| = 0.0159)")
ax[1].plot([0, 1], [0, 1], "k", lw=0.5); ax[1].axvline(0.05, ls=":", c="k", lw=0.6); ax[1].axhline(0.05, ls=":", c="k", lw=0.6)
ax[1].set_xlabel("KS p recorded (tie-blind merge)"); ax[1].set_ylabel("KS p, ties consumed (native recompute)"); ax[1].legend(fontsize=7)
ax[1].set_title("Item 4: all 2,963 KS tests of #2186; none crosses 0.05, 0.01 or 0.001", fontsize=9)
fig.tight_layout(); fig.savefig("validation_D.svg"); fig.savefig("validation_D.png", dpi=150)
