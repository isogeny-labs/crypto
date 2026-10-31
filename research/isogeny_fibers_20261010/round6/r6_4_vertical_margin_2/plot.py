"""Plots for R6.4 from the recorded JSON (no computation beyond reading)."""
import json, os, sys
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt

D = os.path.dirname(os.path.abspath(__file__))
plt.rcParams.update({"font.size": 9, "svg.fonttype": "none"})


def rows(*names):
    out = []
    for n in names:
        p = os.path.join(D, n)
        if os.path.exists(p):
            out += json.load(open(p))["rows"]
    return out

lad = rows("ladder_auto.json", "ladder_auto_hi.json", "ladder_auto_top.json")
proj = rows("project.json")
psi = rows("psi.json", "psi_hi.json", "psi_top.json")
mp = rows("modpoly.json")
an = json.load(open(os.path.join(D, "analysis.json")))

# Fig 1: generic kernel finding, all arms, descending and ascending.
fig, ax = plt.subplots(figsize=(6.4, 4.4))
for arm, c in [("karatsuba", "#9aa0a6"), ("auto", "#1a73e8"), ("ntt", "#e8710a")]:
    ax.plot([r["ell"] for r in proj], [r["arms"][arm]["find_expected_Meq"] for r in proj],
            "-", color=c, lw=1.2, label=f"(a') ladder projected, {arm}")
ax.plot([r["ell"] for r in lad], [r["desc"]["find_expected_Meq"] for r in lad], "o", ms=4,
        color="#1a73e8", label="(a) ladder measured, desc")
la = [r for r in lad if r["asc_reaches_crater"]]
ax.plot([r["ell"] for r in la], [r["asc"]["find_expected_Meq"] for r in la], "x", ms=5,
        color="#174ea6", label="(a) ladder measured, asc")
ax.plot([r["ell"] for r in psi], [r["desc"]["find"]["Meq"] for r in psi], "s", ms=4,
        color="#188038", label="(b) psi + CZ, desc")
pa = [r for r in psi if r["asc_reaches_crater"]]
ax.plot([r["ell"] for r in pa], [r["asc"]["find"]["Meq"] for r in pa], "+", ms=6,
        color="#0d652d", label="(b) psi + gcd, asc")
ax.plot([r["ell"] for r in mp], [r["desc"]["find"]["Meq"] for r in mp], "^", ms=4,
        color="#a142f4", label="(c) Phi eval+roots, desc (Phi not counted)")
ma = [r for r in mp if r["asc_reaches_crater"]]
ax.plot([r["ell"] for r in ma], [r["asc"]["find"]["Meq"] for r in ma], "v", ms=4,
        color="#681da8", label="(c) Phi eval+roots, asc")
xs = [r["ell"] for r in proj]
ax.plot(xs, [1e3 * x * x for x in xs], ":", color="k", lw=0.8, label="slope 2 reference")
ax.set_xscale("log", base=2); ax.set_yscale("log", base=2)
ax.set_xlabel("ℓ"); ax.set_ylabel("counted cost (Meq, 61-bit p)")
ax.set_title("Generic kernel finding at j = −3375, 61-bit p")
ax.legend(fontsize=6.5, loc="upper left"); ax.grid(alpha=0.3)
fig.tight_layout(); fig.savefig(os.path.join(D, "fig1_generic_find.svg"))

# Fig 2: local exponents of the projected ladder.
fig, ax = plt.subplots(figsize=(6.0, 3.6))
loc = an["projected_local_exponents"]
mid = [(w["log2_ell_from"] + w["log2_ell_to"]) / 2 for w in loc]
for arm, c in [("karatsuba", "#9aa0a6"), ("auto", "#1a73e8"), ("ntt", "#e8710a")]:
    y = [w[arm][0] for w in loc]
    lo = [w[arm][0] - w[arm][1] for w in loc]
    hi = [w[arm][2] - w[arm][0] for w in loc]
    ax.errorbar(mid, y, yerr=[lo, hi], fmt="o-", ms=3, color=c, capsize=2, label=arm)
ax.axhline(2.0, color="k", ls=":", lw=0.8)
ax.set_xlabel("log2 ℓ (window centre, 2 octaves)"); ax.set_ylabel("local exponent (95% CI)")
ax.set_title("Projected ladder find: local exponent by product arm")
ax.legend(fontsize=7); ax.grid(alpha=0.3)
fig.tight_layout(); fig.savefig(os.path.join(D, "fig2_local_exponent.svg"))

# Fig 3: ascending / descending ratio per ell, per arm.
fig, ax = plt.subplots(figsize=(6.0, 3.6))
def ratio(rs, key):
    out = []
    for r in rs:
        if not r["asc_reaches_crater"]:
            continue
        out.append((r["ell"], key(r, "asc") / key(r, "desc")))
    return out
for lab, rs, key, c, m in [
    ("(a) ladder, expected", lad, lambda r, d: r[d]["find_expected_Meq"], "#1a73e8", "o"),
    ("(b) psi", psi, lambda r, d: r[d]["find"]["Meq"], "#188038", "s"),
    ("(c) Phi eval+roots", mp, lambda r, d: r[d]["find"]["Meq"], "#a142f4", "^"),
]:
    v = ratio(rs, key)
    ax.plot([a for a, _ in v], [b for _, b in v], m, ms=4, color=c, label=lab)
ax.axhline(1.0, color="k", lw=0.8)
ax.set_xscale("log", base=2); ax.set_yscale("log", base=2)
ax.set_xlabel("ℓ"); ax.set_ylabel("ascending / descending (counted)")
ax.set_title("Ascending vs descending edge at equal ℓ")
ax.legend(fontsize=7); ax.grid(alpha=0.3)
fig.tight_layout(); fig.savefig(os.path.join(D, "fig3_asc_desc_ratio.svg"))

# Fig 4: crossover table, g = 6.
fig, axs = plt.subplots(1, 3, figsize=(9.6, 4.6), sharey=True)
for ax, t in zip(axs, [t for t in an["crossover"] if t["g_mults_per_group_op"] == 6.0]):
    names = [r["model"] for r in t["rows"]]
    vals = [r["log2_ell_star"] if r["log2_ell_star"] is not None else 0 for r in t["rows"]]
    cols = ["#9aa0a6" if n.startswith("[R5.3]") else "#1a73e8" for n in names]
    ax.barh(range(len(vals)), vals, color=cols)
    ax.axvline(t["log2_ell_max"], color="#d93025", ls="--", lw=1, label="ℓ_max = 2√p")
    ax.set_title(f"r = 2^{int(t['log2_r'])}, g = 6")
    ax.set_xlabel("log2 ℓ*")
    ax.set_yticks(range(len(names)))
    ax.set_yticklabels([n[:60] for n in names], fontsize=5.5)
    ax.invert_yaxis()
axs[0].legend(fontsize=7)
fig.tight_layout(); fig.savefig(os.path.join(D, "fig4_crossover.svg"))
print("ok")
