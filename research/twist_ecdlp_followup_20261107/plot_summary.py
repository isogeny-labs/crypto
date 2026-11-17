"""Plot only recorded values: IC/rho gap per lead across the GAE band (w = 0, 0.1, 1). Log y.
Sources: L1 results.json (items, 'X / matched rho') and VERIFICATION.json verifier math verdict (setup-priced rho);
L2 results.json ic_vs_rho (arm F, measured d-determined, and the extrapolated k=29 N_dd rows);
L3 results.json per_size[k].ratios."""
import json, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
D = "research/twist_ecdlp_followup_20261107/"
L1 = json.load(open(D+"L1-descent-parity/results.json"))["items"]
L2 = json.load(open(D+"L2-binary-xonly-twist-filter/results.json"))["ic_vs_rho"]
L3 = json.load(open(D+"L3-genus2-measured-arm/results.json"))["per_size"]
X = [0, 1, 2]; XL = ["0", "0.1", "1"]
col = {"A": "#1f77b4", "D": "#d95f02", "C": "#7570b3", "G": "#1b9e77", "S": "#e7298a"}

fig, ax = plt.subplots(1, 3, figsize=(15, 5.2), sharey=True)
# ---- L1
a = ax[0]
def l1(arm, s):
    for it in L1:
        if it.get("comparison") == f"{arm} / matched rho" and it["s"] == s:
            return [it["ratio_c0"][0], it["ratio_c0.1"][0], it["ratio_c1"][0]]
# verifier recomputation with rho setup priced (VERIFICATION.json, L1 math verdict)
vp = {("A", 20): [3.9, 19, 123], ("A", 32): [6.1, 109, 562], ("D", 20): [3.8, 90, 375],
      ("D", 32): [5.7, 145, 381], ("C", 20): [21, 495, 1966], ("C", 32): [186, 3989, 10407]}
for arm in "ADC":
    for s, ls in ((20, "-"), (32, "--")):
        a.plot(X, l1(arm, s), ls, color=col[arm], alpha=0.35, marker="o", mfc="none")
        a.plot(X, vp[(arm, s)], ls, color=col[arm], marker="o", label=f"{arm}, s={s}")
a.set_title("L1 descent parity (F_p curves, s=20, 32)\nfaint open = run report (rho setup unpriced)\nsolid = verifier, rho setup priced", fontsize=9)
# ---- L2
a = ax[1]
for m, c in ((2, "#1f77b4"), (3, "#d95f02")):
    for k, ls, mk in ((17, ":", "s"), (19, ":", "^"), (23, "-", "o")):
        r = [x for x in L2 if x["m"] == m and x["k"] == k and x["arm"] == "F" and x["source"] == "measured_d_determined"][0]
        lab = f"m={m}, k={k}" + (" (rho flagged F8)" if k < 23 else " (rho in band)")
        a.plot(X, r["gap"], ls, color=c, marker=mk, mfc="none" if k < 23 else c, alpha=0.5 if k < 23 else 1, label=lab)
    r = [x for x in L2 if x["m"] == m and x["k"] == 29 and x["arm"] == "F" and "N_dd" in x["source"]][0]
    a.plot(X, r["gap"], "--", color=c, marker="x", label=f"m={m}, k=29 extrapolated")
a.set_title("L2 binary x-only twist filter, arm F\nmeasured d-determined (k=17,19,23) and extrapolated k=29", fontsize=9)
# ---- L3
a = ax[2]
for arm, key in (("D", "IC-D/R_E"), ("G", "IC-G/R_E"), ("S", "IC-S/R_E")):
    for k, ls in ((8, "--"), (16, "-")):
        rr = L3[f"k{k}"]["ratios"][key]
        a.plot(X, [rr["w0"]["gm"], rr["w01"]["gm"], rr["w1"]["gm"]], ls, color=col[arm], marker="o", label=f"IC-{arm}, k={k}")
a.set_title("L3 genus-2 Jacobian IC arms vs rho on E\nk=8 (dashed) and k=16 (solid), geometric mean", fontsize=9)
for a in ax:
    a.axhline(1, color="k", lw=1)
    a.set_yscale("log"); a.set_xticks(X); a.set_xticklabels(XL)
    a.set_xlabel("price w of unpriced native operations (GAE per operation)")
    a.grid(True, which="both", alpha=0.25); a.legend(fontsize=7, loc="upper left")
ax[0].set_ylabel("IC cost / matched rho cost  (1 = tie, below 1 = IC cheaper)")
ax[0].text(0.02, 0.97, "", transform=ax[0].transAxes)
fig.suptitle("Twist-ECDLP follow-up: gap to matched rho (distance remaining) across the GAE band. Only the L2 m=3 k=29 extrapolation at w=0 is below 1 (group operations only, unordered across the band).", fontsize=9.5)
fig.tight_layout(rect=(0, 0, 1, 0.94))
fig.savefig(D+"summary.svg"); fig.savefig(D+"summary.png", dpi=150)
