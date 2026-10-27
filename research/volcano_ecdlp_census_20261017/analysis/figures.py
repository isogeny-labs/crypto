#!/usr/bin/env python3
"""Draw the ANALYSIS_PLAN s10 figures from analysis/results.json (plots recorded
statistics only; computes nothing new except axis layout).

Run: python3 -I analysis/figures.py
Writes analysis/figures/<name>.svg and .png (150 dpi) and the volcano diagram inputs.
"""
import json, os, sys
import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
FIG = f"{ROOT}/analysis/figures"
os.makedirs(FIG, exist_ok=True)
R = json.load(open(f"{ROOT}/analysis/results.json"))
ARMS = ["rho-neg", "rho-plain", "bsgs", "bsgs-neg", "ic"]
COL = {"rho-neg": "#1b6ca8", "rho-plain": "#7a9cc6", "bsgs": "#2a9d8f", "bsgs-neg": "#8ab17d", "ic": "#d1495b"}
plt.rcParams.update({"font.size": 9, "axes.spines.top": False, "axes.spines.right": False, "svg.fonttype": "none"})


def save(fig, name):
    fig.savefig(f"{FIG}/{name}.svg", bbox_inches="tight")
    fig.savefig(f"{FIG}/{name}.png", dpi=150, bbox_inches="tight")
    plt.close(fig)


# 1. forest plot of R(L) with TOST bands
fig, axs = plt.subplots(1, 2, figsize=(11, 6), sharex=True)
for ax, V in zip(axs, ["V1", "V2"]):
    ys, labels, y = [], [], 0
    for arm in ARMS:
        m = 0.90 if arm == "ic" else 0.95
        le = R["level_effect"][f"{V}|{arm}"]["R"]
        ax.axvspan(m, 1 / m, ymin=0, ymax=1, color=COL[arm], alpha=0.06)
        for L, r in le.items():
            s = r["S"]
            ax.plot(s["ci95"], [y, y], color=COL[arm], lw=1.2)
            ax.plot(s["ci90"], [y, y], color=COL[arm], lw=3.5, alpha=0.8)
            ax.plot(s["ratio"], y, "o", color="white", mec=COL[arm], ms=4, zorder=3)
            ax.text(1.115, y, s["verdict"], va="center", fontsize=7, color="#444")
            labels.append(f"{arm}  L{L}/L0")
            ys.append(y)
            y -= 1
        y -= 0.6
    ax.axvline(1, color="k", lw=0.7)
    ax.set_yticks(ys)
    ax.set_yticklabels(labels, fontsize=7)
    ax.set_xlim(0.9, 1.2)
    ax.set_xlabel("R(L) = mean S at level L / mean S at crater")
    ax.set_title(f"{V} (l = {2 if V == 'V1' else 3}): thin = 95% CI, thick = 90% CI, band = TOST margin")
fig.suptitle("Level effect on S, isogenous families pooled; cluster bootstrap (10,000 reps, seed 20261022), curves = vertices", fontsize=10)
fig.tight_layout()
save(fig, "fig1_forest_level_effect")

# 3. IC / rho by level with the pricing band
fig, axs = plt.subplots(1, 3, figsize=(11, 3.8), sharey=True)
for ax, V in zip(axs, ["V1", "V2", "V3"]):
    lv = R["ic_vs_rho"][V]["levels"]
    Ls = sorted(int(k) for k in lv)
    for p, c in zip(["0.0", "0.1", "1.0"], ["#7fb069", "#e6aa68", "#ca3c25"]):
        y = [lv[str(L)]["prices"][p]["ratio_ic_over_rho"] for L in Ls]
        lo = [lv[str(L)]["prices"][p]["ci95"][0] for L in Ls]
        hi = [lv[str(L)]["prices"][p]["ci95"][1] for L in Ls]
        ax.errorbar(np.array(Ls) + {"0.0": -0.08, "0.1": 0, "1.0": 0.08}[p], y, yerr=[np.array(y) - lo, np.array(hi) - y], fmt="o-", ms=3, color=c, label=f"{p} GAE / uncharged op", capsize=2)
    ax.axhline(1, color="k", lw=0.8)
    ax.set_yscale("log")
    ax.set_title(V + (" (descriptive: 1 curve at upper levels)" if V == "V3" else ""), fontsize=9)
    ax.set_xlabel("level")
axs[0].set_ylabel("mean S(ic) / mean S(rho-neg)")
axs[0].legend(fontsize=7)
fig.suptitle("Distance remaining: IC over matched rho, priced at 0 / 0.1 / 1 GAE per uncharged operation on both arms (ratio 1 = parity)", fontsize=10)
fig.tight_layout()
save(fig, "fig3_ic_over_rho")

# 4. per-curve mean S against the control band
arms4 = ["rho-neg", "rho-plain", "ic"]
fig, axs = plt.subplots(2, 3, figsize=(12, 8.5), constrained_layout=True)
for i, V in enumerate(["V1", "V2"]):
    for j, arm in enumerate(arms4):
        ax = axs[i, j]
        ce = R["curve_effect"][f"{V}|{arm}"]
        vs = ce["vertices"]
        cbar = np.array([(v["c1"] + v["c2"]) / 2 for v in vs])
        m = np.array([v["m"] for v in vs])
        lev = np.array([v["level"] for v in vs])
        sd = ce["control_sd_pooled"] * np.sqrt(1.5)
        xs = np.linspace(cbar.min(), cbar.max(), 20)
        ax.fill_between(xs, xs - 2.576 * sd, xs + 2.576 * sd, color="#bbb", alpha=0.5, label="approx. 99% control band (display only)")
        ax.plot(xs, xs, "k", lw=0.6)
        sc = ax.scatter(cbar, m, c=lev, cmap="viridis", s=10)
        fl = {f["vertex"] for f in ce["flagged"]}
        for v in vs:
            if v["vertex"] in fl:
                ax.plot((v["c1"] + v["c2"]) / 2, v["m"], "o", mfc="none", mec="red", ms=9)
        ax.set_title(f"{V} {arm}: F={ce['F']:.2f}, p={ce['p_perm']:.2f}, flagged {ce['n_flagged']}", fontsize=8)
        ax.set_xlabel("mean S of the two isomorphic re-scalings")
        ax.set_ylabel("mean S of the original (D)")
fig.colorbar(sc, ax=axs, label="level", shrink=0.5)
fig.suptitle("Curve effect against the negative control: each point is one vertex; spread beyond the band would be a curve effect", fontsize=10)
save(fig, "fig4_curve_vs_control")

# 5. transfer cost against solve cost by path length
fig, axs = plt.subplots(1, 3, figsize=(11, 3.8), sharey=True)
for ax, V in zip(axs, ["V1", "V2", "V3"]):
    per = R["transfer_cost"]["by_volcano_path_len"][V]
    ds = sorted(int(k) for k in per)
    med = np.array([per[str(d)]["ns_per_path_median_q1_q3"][0] for d in ds])
    q1 = np.array([per[str(d)]["ns_per_path_median_q1_q3"][1] for d in ds])
    q3 = np.array([per[str(d)]["ns_per_path_median_q1_q3"][2] for d in ds])
    ax.errorbar(ds, med, yerr=[med - q1, q3 - med], fmt="o-", ms=3, color="k", label="transport ns per path (median, IQR)")
    for arm in ["rho-neg", "bsgs-neg", "ic"]:
        fr = R["transfer_cost"]["fraction_of_solve"][V][arm]
        w = np.mean([v["median_solve_wall_ns"] for v in fr.values()])
        ax.axhline(w, color=COL[arm], ls="--", label=f"{arm} median solve wall")
    ax.set_yscale("log")
    ax.set_xlabel("path length d (edges)")
    ax.set_title(V, fontsize=9)
axs[0].set_ylabel("ns")
axs[0].legend(fontsize=6)
fig.suptitle("Transport cost against solve cost (wall, descriptive: not all rows at isolation L2); Velu point maps are 2 per edge", fontsize=10)
fig.tight_layout()
save(fig, "fig5_transport_vs_solve")

# 6. CV and achieved-precision table as a figure
rows = []
for V in ["V1", "V2"]:
    for arm in ARMS:
        le = R["level_effect"][f"{V}|{arm}"]
        mm = R["mixed_model"][f"{V}|{arm}"]
        lv = le["levels"]
        rows.append([V, arm,
                     "/".join(str(lv[str(L)]["n_curves"]) for L in sorted(map(int, lv))),
                     f"{le['n_rows']}",
                     f"{np.mean([lv[L]['cv_S_per_row'] for L in lv]):.2f}",
                     f"{np.mean([lv[L]['mean_S_rel_half_width95'] for L in lv]) * 100:.1f}%",
                     f"{np.mean([r['S']['half_width95'] for r in le['R'].values()]):.3f}",
                     "n/a" if mm.get("icc") is None else f"{mm['icc']:.3f}",
                     "yes" if mm.get("converged") else "no"])
fig, ax = plt.subplots(figsize=(11, 3.9))
ax.axis("off")
t = ax.table(cellText=rows, colLabels=["volcano", "arm", "curves per level", "rows", "CV of S\nper row", "95% half-width\nof level mean", "95% half-width\nof R(L)", "ICC (mixed\nmodel)", "converged"], loc="center", cellLoc="center")
t.auto_set_font_size(False)
t.set_fontsize(7.5)
t.scale(1, 1.6)
ax.set_title("CV and achieved precision (mean over levels); BSGS is deterministic in r and k, so its CV is the scalar's spread only", fontsize=10)
save(fig, "fig6_cv_precision")

# 2. volcano diagrams (standard renderer) for rho-neg and ic mean S
sys.path.insert(0, f"{ROOT}/tools/volcano-diagram")
from volcano_diagram import render
for V in ["V1", "V2", "V3"]:
    verts = [json.loads(l) for l in open(f"{ROOT}/census/{V}/vertices.jsonl")]
    vm = R["vertex_means"][V]
    spec = {"schema": "volcano-diagram/v2", "ell": verts[0]["ell"], "depth": R["depth"][V],
            "title": f"{V}: measured mean S per curve, rho-neg (left) and ic (right), isogenous families pooled",
            "subtitle": "volcano-ecdlp-census-20261017 Phase B, 79 sessions; 5 families x 8 targets x rounds per curve",
            "footnote": "Colour = mean S per vertex (observed range per panel); labels are census vertex ids (census/%s/vertices.jsonl)." % V,
            "metrics": [{"key": "rho", "title": "rho-neg: mean S", "label": "S (normalised group operations)"},
                        {"key": "ic", "title": "ic: mean S", "label": "S (normalised group operations)"}],
            "vertices": [{"id": v["id"], "label": f"v{v['id']}", "level": v["level"], "nbrs": v["nbrs"],
                          "values": {"rho": vm["rho-neg"][str(v["id"])]["mean_S"], "ic": vm["ic"][str(v["id"])]["mean_S"]},
                          **({"highlight": "root"} if v["id"] == 0 else {})} for v in verts]}
    json.dump(spec, open(f"{FIG}/fig2_volcano_{V}.diagram.json", "w"))
    render(spec, f"{FIG}/fig2_volcano_{V}")
print("figures written")
