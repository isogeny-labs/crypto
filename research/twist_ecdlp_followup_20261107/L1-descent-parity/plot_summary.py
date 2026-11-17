# Plots recorded values from results.json only (no computation of new numbers).
import json, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
d = json.load(open("results.json"))["items"]
cmp_ = [x for x in d if "comparison" in x]
fig, ax = plt.subplots(1, 3, figsize=(15, 4.4))
# panel 1: attempts ratio A/B with CI
rows = [x for x in cmp_ if x["comparison"].startswith("A / B (doubling isolated)") and "no comb" not in x["comparison"]]
s = [r["s"] for r in rows]; m = [r["attempts_ratio"][0] for r in rows]
lo = [r["attempts_ratio"][0]-r["attempts_ratio"][1] for r in rows]; hi = [r["attempts_ratio"][2]-r["attempts_ratio"][0] for r in rows]
ax[0].errorbar(s, m, yerr=[lo, hi], fmt="o-", capsize=3)
ax[0].axhline(0.354, ls="--", c="g", label="structure-aware 0.354"); ax[0].axhline(0.5, ls=":", c="r", label="naive 0.5")
ax[0].set_xlabel("s (group order 2^s)"); ax[0].set_ylabel("attempts A/B"); ax[0].legend(); ax[0].set_title("Doubling isolated: A vs B")
# panel 2: priced A/B at three c
for k, c in enumerate(["ratio_c0", "ratio_c0.1", "ratio_c1"]):
    ax[1].plot(s, [r[c][0] for r in rows], "o-", label="c=" + c[7:])
ax[1].axhline(1, c="k", lw=0.5); ax[1].set_xlabel("s"); ax[1].set_ylabel("priced cost A/B"); ax[1].legend(); ax[1].set_title("Priced A/B (with comb precomputation)")
# panel 3: IC / matched rho at three c
for arm in ["A", "B", "D", "C"]:
    rr = [x for x in cmp_ if x["comparison"].startswith(arm + " / matched rho")]
    for c, ls in zip(["ratio_c0", "ratio_c0.1", "ratio_c1"], ["-", "--", ":"]):
        ax[2].plot([r["s"] for r in rr], [r[c][0] for r in rr], "o" + ls, label=arm + " " + c[6:] if False else arm + " c=" + c[7:], color={"A": "C0", "B": "C1", "D": "C2", "C": "C3"}[arm])
ax[2].set_yscale("log"); ax[2].axhline(1, c="k", lw=0.5); ax[2].set_xlabel("s"); ax[2].set_ylabel("IC priced / matched rho priced"); ax[2].set_title("Distance remaining to matched rho"); ax[2].legend(fontsize=6, ncol=2)
plt.tight_layout(); plt.savefig("summary.svg"); plt.savefig("summary.png", dpi=110)
print("plots written")
