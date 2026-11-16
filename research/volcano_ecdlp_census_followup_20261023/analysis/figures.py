#!/usr/bin/env python3
"""Draw the ANALYSIS_PLAN s10 figures from analysis/results.json (plots recorded
statistics only; computes nothing new except axis layout).

Run: python3 -I analysis/figures.py [results.json] [outdir]
Writes <outdir>/<name>.svg and .png (150 dpi).
"""
import json, os, sys
import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
RES = sys.argv[1] if len(sys.argv) > 1 else f"{ROOT}/analysis/results.json"
FIG = sys.argv[2] if len(sys.argv) > 2 else f"{ROOT}/analysis/figures"
os.makedirs(FIG, exist_ok=True)
R = json.load(open(RES))
ARMS = ["rho-neg", "rho-plain", "bsgs", "bsgs-neg", "ic"]
COL = {"rho-neg": "#1b6ca8", "rho-plain": "#7a9cc6", "bsgs": "#2a9d8f", "bsgs-neg": "#8ab17d", "ic": "#d1495b", "rho-cubic": "#6a4c93"}
PCOL = {"0.0": "#4f9d3a", "0.1": "#d98324", "1.0": "#b2301c"}
plt.rcParams.update({"font.size": 9, "axes.spines.top": False, "axes.spines.right": False, "svg.fonttype": "none"})


def save(fig, name):
    fig.savefig(f"{FIG}/{name}.svg", bbox_inches="tight")
    fig.savefig(f"{FIG}/{name}.png", dpi=150, bbox_inches="tight")
    plt.close(fig)


# 1. forest plot of R(L) with TOST bands, V6 and V5
fig, axs = plt.subplots(1, 2, figsize=(12, 7.5), sharex=True)
for ax, V in zip(axs, ["V6", "V5"]):
    ys, labels, y = [], [], 0
    for arm in ARMS:
        m = 0.90 if arm == "ic" else 0.95
        le = R["level_effect"][f"{V}|{arm}"]["R"]
        ax.axvspan(m, 1 / m, color=COL[arm], alpha=0.06)
        for L, r in le.items():
            s = r["S"]
            desc = s["verdict"] == "descriptive"
            ax.plot(s["ci95"], [y, y], color=COL[arm], lw=1.2, ls=":" if desc else "-")
            ax.plot(s["ci90"], [y, y], color=COL[arm], lw=3.5, alpha=0.35 if desc else 0.8)
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
    ax.set_title(f"{V} (l = {2 if V == 'V5' else 3}): thin = 95% CI, thick = 90% CI, band = TOST margin" + ("; L1-3 descriptive (dotted)" if V == "V5" else ""), fontsize=8)
fig.suptitle("Level effect on S, isogenous families pooled; cluster bootstrap (10,000 reps, seed 20261022), curves = census vertices", fontsize=10)
fig.tight_layout()
save(fig, "fig1_forest_level_effect")

# 3. IC / rho by level with the pricing band
fig, axs = plt.subplots(1, 2, figsize=(11, 4.2), sharey=True)
for ax, V in zip(axs, ["V5", "V6"]):
    lv = R["ic_vs_rho"][V]["levels"]
    Ls = sorted(int(k) for k in lv)
    for p in ["0.0", "0.1", "1.0"]:
        off = {"0.0": -0.08, "0.1": 0, "1.0": 0.08}[p]
        y = np.array([lv[str(L)]["prices"][p]["ratio_ic_over_rho"] for L in Ls])
        lo = np.array([lv[str(L)]["prices"][p]["ci95"][0] for L in Ls])
        hi = np.array([lv[str(L)]["prices"][p]["ci95"][1] for L in Ls])
        ax.errorbar(np.array(Ls) + off, y, yerr=[y - lo, hi - y], fmt="o-", ms=3, color=PCOL[p], label=f"{p} GAE / uncharged op (vs rho-neg)", capsize=2)
        if V == "V5":
            c = R["ic_vs_rho_cubic_crater"]["prices"][p]
            ax.errorbar([0 + off], [c["ratio_ic_over_rho_cubic"]], yerr=[[c["ratio_ic_over_rho_cubic"] - c["ci95"][0]], [c["ci95"][1] - c["ratio_ic_over_rho_cubic"]]],
                        fmt="s", ms=6, mfc="white", color=PCOL[p], capsize=2, label="crater vs rho-cubic (matched)" if p == "0.0" else None)
    ax.axhline(1, color="k", lw=0.8)
    ax.set_yscale("log")
    ax.set_title(V + (": level 0 squares = matched rho-cubic" if V == "V5" else ""), fontsize=9)
    ax.set_xlabel("level")
axs[0].set_ylabel("mean S(ic) / mean S(rho)")
axs[0].legend(fontsize=7)
fig.suptitle("Distance remaining: IC over matched rho, uncharged operations priced at 0 / 0.1 / 1 GAE on both arms (ratio 1 = parity)", fontsize=10)
fig.tight_layout()
save(fig, "fig3_ic_over_rho")

# 4. per-curve mean S against the control band (V6)
arms4 = ["rho-neg", "rho-plain", "ic"]
fig, axs = plt.subplots(1, 3, figsize=(13, 4.6), constrained_layout=True)
for j, arm in enumerate(arms4):
    ax = axs[j]
    ce = R["curve_effect"][f"V6|{arm}"]
    vs = ce["vertices"]
    cbar = np.array([(v["c1"] + v["c2"]) / 2 for v in vs])
    m = np.array([v["m"] for v in vs])
    lev = np.array([v["level"] for v in vs])
    sd = ce["control_sd_pooled"] * np.sqrt(1.5)
    xs = np.linspace(cbar.min(), cbar.max(), 20)
    ax.fill_between(xs, xs - 2.576 * sd, xs + 2.576 * sd, color="#bbb", alpha=0.5, label="approx. 99% control band (display only)")
    ax.plot(xs, xs, "k", lw=0.6)
    sc = ax.scatter(cbar, m, c=lev, cmap="viridis", s=8)
    for v in vs:
        if v["vertex"] in {f["vertex"] for f in ce["flagged"]}:
            ax.plot((v["c1"] + v["c2"]) / 2, v["m"], "o", mfc="none", mec="red", ms=9)
    ax.set_title(f"V6 {arm}: F={ce['F']:.2f}, p={ce['p_perm']:.2f}, flagged {ce['n_flagged']}", fontsize=8)
    ax.set_xlabel("mean S of the two isomorphic re-scalings")
    ax.set_ylabel("mean S of the original (D)")
fig.colorbar(sc, ax=axs, label="level", shrink=0.7)
fig.suptitle("Curve effect against the negative control (V6, 648 vertices): spread beyond the band would be a curve effect", fontsize=10)
save(fig, "fig4_curve_vs_control")

# 5. transfer cost against solve cost by path length
fig, axs = plt.subplots(1, 2, figsize=(11, 4), sharey=True)
for ax, V in zip(axs, ["V5", "V6"]):
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

# 6. CV and achieved-precision table
rows = []
for V in ["V5", "V6"]:
    for arm in ARMS:
        le = R["level_effect"][f"{V}|{arm}"]
        mm = R["mixed_model"][f"{V}|{arm}"]
        lv = le["levels"]
        rows.append([V, arm, "/".join(str(lv[str(L)]["n_curves"]) for L in sorted(map(int, lv))), f"{le['n_rows']}",
                     f"{np.mean([lv[L]['cv_S_per_row'] for L in lv]):.2f}",
                     f"{np.mean([lv[L]['mean_S_rel_half_width95'] for L in lv]) * 100:.1f}%",
                     f"{np.mean([r['S']['half_width95'] for r in le['R'].values()]):.3f}",
                     "n/a" if mm.get("icc") is None else f"{mm['icc']:.3f}", "yes" if mm.get("converged") else "no"])
fig, ax = plt.subplots(figsize=(11, 4.4))
ax.axis("off")
t = ax.table(cellText=rows, colLabels=["volcano", "arm", "curves per level", "rows", "CV of S\nper row", "95% half-width\nof level mean", "95% half-width\nof R(L)", "ICC (mixed\nmodel)", "converged"], loc="center", cellLoc="center")
t.auto_set_font_size(False)
t.set_fontsize(7.5)
t.scale(1, 1.5)
ax.set_title("CV and achieved precision (mean over levels); BSGS is deterministic in r and k, so its CV is the scalar's spread only", fontsize=10)
save(fig, "fig6_cv_precision")

# 7. H-T: T_v by source level with CIs at the three prices, with the T0 control
H = R["H-T"]
fig, axs = plt.subplots(1, 2, figsize=(13, 4.8), sharey=True, gridspec_kw={"width_ratios": [3, 1.3]})
ax = axs[0]
vs = sorted(H["by_vertex"], key=lambda k: (H["by_vertex"][k]["level"], int(k)))
for pi, p in enumerate(["0.0", "0.1", "1.0"]):
    off = (pi - 1) * 0.22
    x = np.arange(len(vs)) + off
    y = np.array([H["by_vertex"][k]["T"][p]["value"] for k in vs])
    lo = np.array([H["by_vertex"][k]["T"][p]["ci95"][0] for k in vs])
    hi = np.array([H["by_vertex"][k]["T"][p]["ci95"][1] for k in vs])
    ax.errorbar(x, y, yerr=[y - lo, hi - y], fmt="o", ms=2.5, lw=0.7, color=PCOL[p], label=f"T_v, {p} GAE per point map")
y0 = np.array([H["by_vertex"][k]["T0"]["1.0"]["value"] for k in vs])
ax.plot(np.arange(len(vs)), y0, "x", color="k", ms=3, label="T0_v (control, rho-neg on the carried instance), p = 1")
ax.axhline(1, color="k", lw=0.8)
ax.axhline(H["theory_ratio"], color="#6a4c93", ls="--", lw=0.9, label="theory 1/sqrt(3) = 0.577")
lv_ = [H["by_vertex"][k]["level"] for k in vs]
for L in sorted(set(lv_)):
    i0 = lv_.index(L)
    ax.axvline(i0 - 0.5, color="#ccc", lw=0.5)
    ax.text(i0, 1.45, f"L{L}", fontsize=8)
ax.set_ylim(0.2, 1.5)
ax.set_xticks([])
ax.set_xlabel("source vertex v (ordered by level, then id); 63 vertices")
ax.set_ylabel("T_v = cost(carry up, rho-cubic) / cost(solve at v, rho-neg)")
ax.legend(fontsize=6.5, loc="lower right")
ax.set_title("per vertex (pair-bootstrap 95% CI, descriptive)", fontsize=9)
ax = axs[1]
labs, xs = [], 0
for key, nm in [("overall", "overall")] + [(str(L), f"L{L}") for L in sorted(set(lv_))]:
    r = H["overall"] if key == "overall" else H["by_level"][key]
    for pi, p in enumerate(["0.0", "0.1", "1.0"]):
        t_ = r["T"][p]
        ax.errorbar([xs + (pi - 1) * 0.25], [t_["value"]], yerr=[[t_["value"] - t_["ci95"][0]], [t_["ci95"][1] - t_["value"]]], fmt="o", ms=3, color=PCOL[p], lw=1)
        t0 = r["T0"][p]
        ax.errorbar([xs + (pi - 1) * 0.25], [t0["value"]], yerr=[[t0["value"] - t0["ci95"][0]], [t0["ci95"][1] - t0["value"]]], fmt="x", ms=3, color="k", lw=0.6, alpha=0.7)
    labs.append(nm)
    xs += 1
ax.axhline(1, color="k", lw=0.8)
ax.axhline(H["theory_ratio"], color="#6a4c93", ls="--", lw=0.9)
ax.set_xticks(range(len(labs)))
ax.set_xticklabels(labs, fontsize=7)
ax.set_title("pooled: overall carries the verdict (cluster CI); x = T0 control", fontsize=9)
fig.suptitle(f"H-T on V5: carry up to the j = 0 crater and solve with rho-cubic, transport charged. Verdict: {H['verdict']}", fontsize=10)
fig.tight_layout()
save(fig, "fig7_hT")

# 8. H-M: model means per curve for the three arms with permutation verdicts
M = R["H-M"]
keys = sorted(M["curves"], key=lambda k: (M["curves"][k]["volcano"], M["curves"][k]["level"], M["curves"][k]["vertex"]))
fig, axs = plt.subplots(1, 3, figsize=(14, 5.2))
for ax, arm in zip(axs, ["rho-neg", "bsgs", "ic"]):
    for i, k in enumerate(keys):
        c = M["curves"][k]["arms"][arm]
        mu = np.array(c["model_mean_S"])
        se = np.array(c["model_se_S"])
        base = mu.mean()
        ax.errorbar(mu / base, np.full(len(mu), i) + np.linspace(-0.25, 0.25, len(mu)), xerr=1.96 * se / base, fmt="o", ms=2.5, lw=0.7, color=COL[arm])
        ax.text(1.42, i, ("sig " if c.get("significant") else "ns ") + ("F=%.2f p=%.3f" % (c["F"], c.get("p_holm", 1)) if c["F"] is not None else "F n/a"), fontsize=6.5, va="center")
    ax.axvline(1, color="k", lw=0.6)
    ax.set_yticks(range(len(keys)))
    ax.set_yticklabels([f"{k} L{M['curves'][k]['level']}" for k in keys], fontsize=7)
    ax.set_xlim(0.6, 1.8)
    ax.set_xlabel("model mean S / curve mean (+-1.96 SE)")
    s = M["summary"][arm]
    ax.set_title(f"{arm}: {s['n_significant_holm']} of {s['n_curves']} curves significant (Holm)", fontsize=9)
fig.suptitle(f"H-M: 8 isomorphic Weierstrass models per curve, 64 solves per model and arm. Verdict: {M['verdict']}", fontsize=10)
fig.tight_layout()
save(fig, "fig8_hM")

# 2. volcano diagrams (standard renderer) for rho-neg and ic mean S
sys.path.insert(0, f"{ROOT}/../../tools/volcano-diagram")
from volcano_diagram import render
for V in ["V5", "V6"]:
    verts = [json.loads(l) for l in open(f"{ROOT}/census/{V}/vertices.jsonl")]
    vm = R["vertex_means"][V]
    spec = {"schema": "volcano-diagram/v2", "ell": verts[0]["ell"], "depth": R["depth"][V],
            "title": f"{V}: measured mean S per curve, rho-neg (left) and ic (right), isogenous families pooled",
            "subtitle": f"volcano-ecdlp-census-followup-20261023 Phase B, {R['sessions']} sessions; D, Ua, Ub, N" + (", S" if V == "V6" else "") + " families x 8 targets x rounds per curve",
            "footnote": f"Colour = mean S per vertex (observed range per panel); labels are census vertex ids (census/{V}/vertices.jsonl)." + (" The crater (v0) is j = 0: rho-neg does not use the order-6 automorphism there." if V == "V5" else ""),
            "metrics": [{"key": "rho", "title": "rho-neg: mean S", "label": "S (normalised group operations)"},
                        {"key": "ic", "title": "ic: mean S", "label": "S (normalised group operations)"}],
            "vertices": [{"id": v["id"], "label": f"v{v['id']}", "level": v["level"], "nbrs": v["nbrs"],
                          "values": {"rho": vm["rho-neg"][str(v["id"])]["mean_S"], "ic": vm["ic"][str(v["id"])]["mean_S"]},
                          **({"highlight": "root"} if v["id"] == 0 else {})} for v in verts]}
    json.dump(spec, open(f"{FIG}/fig2_volcano_{V}.diagram.json", "w"))
    render(spec, f"{FIG}/fig2_volcano_{V}")
print("figures written")
