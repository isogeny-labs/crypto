"""Figures for Chapter 'The record, 6-10 November 2026', from data-november.json
(recorded values only; nothing is computed except the plotting layout).

    python3 make_fig_november.py   # writes fig-nov-{generic,reference,k4,prime}.{pdf,svg,png}
"""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(HERE, "data-november.json")))
plt.rcParams.update({"font.size": 8.5, "font.family": "serif", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.alpha": 0.25,
                     "svg.fonttype": "none"})
C = ["#0072B2", "#E69F00", "#009E73", "#CC79A7", "#999999", "#D55E00"]


def save(fig, stem):
    fig.tight_layout()
    for ext in ("pdf", "svg"):
        fig.savefig(os.path.join(HERE, f"{stem}.{ext}"), bbox_inches="tight")
    fig.savefig(os.path.join(HERE, f"{stem}.png"), dpi=150, bbox_inches="tight")
    plt.close(fig)


def errbar(ax, x, s, color, marker, label, dx=0.0):
    xs = [v + dx for v in x]
    if "lo" in s:
        ax.errorbar(xs, s["ratio"], yerr=[[r - lo for r, lo in zip(s["ratio"], s["lo"])],
                                          [hi - r for r, hi in zip(s["ratio"], s["hi"])]],
                    fmt=marker + "-", color=color, ms=4, lw=1.3, capsize=2, label=label)
    else:
        ax.plot(xs, s["ratio"], marker + "-", color=color, ms=4, lw=1.3, label=label)


# 1. Curve25519-family generic arms, and the table-work pricing correction
g = D["c25519_generic"]
fig, (a, b) = plt.subplots(1, 2, figsize=(7.2, 3.0), gridspec_kw={"width_ratios": [1.25, 1]})
errbar(a, g["snc_incumbent"]["k"], g["snc_incumbent"], C[4], "s", "C=2 incumbent (SNC panel)", -0.3)
errbar(a, g["snc_tw2cj_lazy"]["k"], g["snc_tw2cj_lazy"], C[3], "D", "tw2cj-lazy (SNC panel)", -0.1)
errbar(a, g["f1_x07"]["k"], g["f1_x07"], C[1], "^", "F=1 table, f1-x07", 0.1)
errbar(a, g["twcj_j16"]["k"], g["twcj_j16"], C[0], "o", "target in walk, twcj-j16", 0.3)
errbar(a, g["twcj_cv0"]["k"], g["twcj_cv0"], C[2], "v", "twcj, cv = 0", 0.5)
a.axhline(1, color="#222222", ls="--", lw=0.9)
a.set_xticks([20, 24, 28, 32])
a.set_ylim(0.9, 2.1)
a.set_xlabel("$k$ (bits of $p$)")
a.set_ylabel("arm / rho-neg, $w=0$")
a.set_title("Generic arms on Curve25519 miniatures", fontsize=9)
a.legend(fontsize=6.3, loc="lower left", frameon=False, ncol=2)
rows = D["table_work_pricing"]["rows"]
for i, r in enumerate(rows):
    y = len(rows) - 1 - i
    b.annotate("", xy=(r["after"], y), xytext=(r["before"], y),
               arrowprops=dict(arrowstyle="->", color="#555555", lw=1))
    b.plot(r["before"], y, "o", color=C[1], ms=5)
    b.plot(r["after"], y, "o", color=C[0], ms=5)
b.plot([], [], "o", color=C[1], label="inserts unpriced (before 8 Nov)")
b.plot([], [], "o", color=C[0], label="inserts priced (default)")
b.axvline(1, color="#222222", ls="--", lw=0.9)
b.set_yticks(range(len(rows)))
b.set_yticklabels([r["label"] for r in rows][::-1])
b.set_xlabel("arm / rho-neg, $w=1$")
b.set_title("Pricing table inserts", fontsize=9)
b.legend(fontsize=6.3, loc="upper center", bbox_to_anchor=(0.45, -0.2), frameon=False, ncol=1)
save(fig, "fig-nov-generic")

# 2. The rho reference: Brent cycle detection (Koblitz) and the p36 floor study
br = D["brent"]
fl = D["rho_floor_p36"]
fig, (a, b) = plt.subplots(1, 2, figsize=(7.2, 2.8), gridspec_kw={"width_ratios": [1.4, 1]})
a.errorbar(br["n"], br["ratio"], yerr=[[r - lo for r, lo in zip(br["ratio"], br["lo"])],
                                       [hi - r for r, hi in zip(br["ratio"], br["hi"])]],
           fmt="o", color=C[0], ms=4, capsize=2)
a.axhline(1, color="#222222", ls="--", lw=0.9)
a.set_xlabel("$n$ (Koblitz, $\\mathbb{F}_{2^n}$)")
a.set_ylabel("legacy rho-frob / Brent v2")
a.set_title("Brent cycle detection in the Koblitz reference", fontsize=9)
labels = ["rho-neg / ideal", "res1eq / rho-neg", "res1 / rho-neg"]
vals = [fl["rho_neg_over_ideal"], fl["res1eq_over_rho"], fl["res1_over_rho"]]
for i, (lab, v) in enumerate(zip(labels, vals)):
    y = len(vals) - 1 - i
    b.plot([v[1], v[2]], [y, y], color=C[0], lw=2)
    b.plot(v[0], y, "o", color=C[0])
for kk, gg in zip(fl["floor_g"]["k"], fl["floor_g"]["g"]):
    b.axvline(gg, color=C[5], ls=":", lw=0.9)
    b.text(gg + 0.03, 1.45, f"$g({kk})$", color=C[5], fontsize=7, ha="left")
b.axvline(1, color="#222222", ls="--", lw=0.9)
b.set_yticks(range(3))
b.set_yticklabels(labels[::-1])
b.set_xlabel("ratio, $p_{36}$, 128 targets (95% CI)")
b.set_title("Collision floor at $p_{36}$", fontsize=9)
save(fig, "fig-nov-reference")

# 3. k = 4 in characteristic 2
k4 = D["k4_char2"]
fig, (a, b) = plt.subplots(1, 2, figsize=(7.2, 2.6))
a.plot(k4["m"], k4["ic_over_rho_theory"], "o-", color=C[0])
a.axhline(1, color="#222222", ls="--", lw=0.9)
a.set_yscale("log")
a.set_xticks(k4["m"])
a.set_yticks([0.5, 1, 2, 4])
a.set_yticklabels(["0.5", "1", "2", "4"])
a.minorticks_off()
a.set_xlabel("$m$ (curve over $\\mathbb{F}_{2^{4m}}$)")
a.set_ylabel("IC / rho theory (geo. mean)")
a.set_title("IC against rho's theoretical cost", fontsize=9)
b.bar(k4["m"], k4["matrix_share_pct"], color=C[2], width=0.6)
b.set_xticks(k4["m"])
b.set_xlabel("$m$")
b.set_ylabel("Wiedemann share of IC cost (%)")
b.set_title("Linear algebra share", fontsize=9)
save(fig, "fig-nov-k4")

# 4. Prime fields: minors envelope, and PKM fall degree tower vs null
mi = D["minors"]
pk = D["pkm_falldegree"]
fig, (a, b) = plt.subplots(1, 2, figsize=(7.2, 2.7))
a.errorbar(mi["bits"], mi["ratio"], yerr=[[r - lo for r, lo in zip(mi["ratio"], mi["lo"])],
                                          [hi - r for r, hi in zip(mi["ratio"], mi["hi"])]],
           fmt="o-", color=C[5], ms=4, capsize=2)
a.axhline(1, color="#222222", ls="--", lw=0.9)
a.set_yscale("log")
a.set_xlabel("bits of $p$")
a.set_ylabel("cheapest minors arm / rho")
a.set_title("Minors guess-and-determine", fontsize=9)
b.plot(pk["t"], pk["dF_tower"], "o-", color=C[0], label="Kummer tower")
b.plot(pk["t"], pk["dF_null"], "x--", color=C[1], label="random-coefficient null")
b.set_xlabel("$t$ (tower height)")
b.set_ylabel("median last fall degree $d_F$")
b.set_yticks([3, 4, 5, 6])
b.set_title("PKM, $m=2$, prime $P_1$", fontsize=9)
b.legend(fontsize=7, frameon=False, loc="upper left")
save(fig, "fig-nov-prime")
print("ok")
