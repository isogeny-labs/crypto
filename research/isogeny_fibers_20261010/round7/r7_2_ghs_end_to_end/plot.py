"""Plots recorded values only (no result is computed here; AGENTS.md "no Python" rule).

Inputs (all under raw/): run_*.json (examples/r7_2_ghs_end_to_end run), fit_genus4.json (the example's `fit` mode),
profile_l3_n4_m4.json (the example's `profile` mode) and r71_rerun.log (elapsed wall time of R7.1's own example).
Outputs: fig1..fig5 as SVG and PNG, and figure_data.tsv with every plotted number.
"""
import glob
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

D = os.path.dirname(os.path.abspath(__file__))
RAW = os.path.join(D, "raw")

inst = []
for f in sorted(glob.glob(os.path.join(RAW, "run_*.json"))):
    for i in json.load(open(f))["instances"]:
        if i.get("holdout"):
            inst.append(i)
fit = json.load(open(os.path.join(RAW, "fit_genus4.json")))
FAM = fit["families"]
prof2 = json.load(open(os.path.join(RAW, "profile_l3_n4_m4.json")))  # run 2: three of this study's jobs ran at the same time
prof = json.load(open(os.path.join(RAW, "profile_run1_less_contended.json")))  # run 1: fewer concurrent jobs
r71_elapsed = None
for line in open(os.path.join(RAW, "r71_rerun.log")):
    if line.startswith("elapsed_seconds="):
        r71_elapsed = float(line.split("=")[1])

COL = {2: "#4c78a8", 3: "#f58518", 4: "#54a24b", 5: "#b279a2", 6: "#e45756", 7: "#72b7b2", 8: "#9d755d"}
MARK = {2: "s", 4: "o", 7: "^", 8: "^"}


def lab(i):
    return f"q={2 ** i['l']}, N={i['N']}, g={i['genus']}"


def save(fig, name):
    fig.tight_layout()
    fig.savefig(os.path.join(D, name + ".svg"))
    fig.savefig(os.path.join(D, name + ".png"), dpi=150)
    plt.close(fig)


rows = []
for i in inst:
    h = i["holdout"]
    best = min(h["mean_gae"]["rho"][0], h["mean_gae"]["bsgs"][0])
    rows.append(
        [
            i["instance"], i["l"], i["n"], i["N"], i["genus"], i["ell"], i["cofactor"], i["chosen_mode"], i["chosen_fb_columns"],
            i["factor_base_available"]["columns"], h["mean_gae"]["ic_online"][0], h["mean_gae"]["ic_online"][1], h["mean_gae"]["ic_online"][2],
            h["mean_gae"]["rho"][0], h["mean_gae"]["bsgs"][0], best, h["ratio_ic_over_best_generic"]["point"],
            h["ratio_ic_over_best_generic"]["ci95"][0], h["ratio_ic_over_best_generic"]["ci95"][1],
            h["raw_group_op_ratio_ic_Jadds_over_best_generic_Eops"]["point"], h["mean_jacobian_additions"], h["solved_ic"],
        ]
    )
hdr = ("instance l n N genus ell cofactor mode fb_cols fb_avail ic_gae ic_lo ic_hi rho_gae bsgs_gae best_generic_gae "
       "ratio ratio_lo ratio_hi raw_J_over_E_ops mean_J_adds solved_of_20").split()
open(os.path.join(D, "figure_data.tsv"), "w").write("\t".join(hdr) + "\n" + "\n".join("\t".join(str(x) for x in r) for r in rows) + "\n")

# ── fig1: IC online cost against the best generic cost ──
fig, ax = plt.subplots(figsize=(6.6, 5.0))
for i in inst:
    h = i["holdout"]
    ic, lo, hi = h["mean_gae"]["ic_online"]
    arm = "rho" if h["mean_gae"]["rho"][0] <= h["mean_gae"]["bsgs"][0] else "bsgs"
    g, glo, ghi = h["mean_gae"][arm]
    ax.errorbar(g, ic, xerr=[[g - glo], [ghi - g]], yerr=[[ic - lo], [hi - ic]], fmt=MARK.get(i["genus"], "o"), color=COL[i["l"]],
                ms=6, lw=0.8, mfc=COL[i["l"]], alpha=0.9)
xs = [5, 8000]
ax.plot(xs, xs, "k-", lw=1)
ax.plot(xs, [10 * x for x in xs], "k:", lw=0.8)
ax.plot(xs, [100 * x for x in xs], "k:", lw=0.8)
ax.text(300, 300 * 1.25, "IC = best generic", rotation=34, fontsize=7)
ax.text(40, 40 * 12, "10x", fontsize=7)
ax.text(12, 12 * 120, "100x", fontsize=7)
for l in sorted({i["l"] for i in inst}):
    ax.plot([], [], "o", color=COL[l], label=f"q = {2 ** l}")
for g, m in [(2, "s"), (4, "o"), (8, "^")]:
    ax.plot([], [], m, color="gray", label=f"g = {g}" + (" (7 or 8)" if g == 8 else ""))
ax.set_xscale("log")
ax.set_yscale("log")
ax.set_xlim(8, 8000)
ax.set_ylim(80, 3e5)
ax.set_xlabel("best generic method on E (rho or BSGS, mean of 20), E-add equivalents")
ax.set_ylabel("index calculus on the descended Jacobian, mean of 20")
ax.set_title("R7.2 online cost per planted log (bitop/v1, in E additions)\nevery point above the diagonal is a loss; bars: 95% bootstrap CI", fontsize=9)
ax.legend(fontsize=7, ncol=2, loc="lower right")
save(fig, "fig1_ic_vs_generic")

# ── fig2: ratio against N, gates and raw group operations, with the n = 3, g = 4 fit ──
fig, ax = plt.subplots(figsize=(6.8, 4.8))
for i in inst:
    h = i["holdout"]
    r = h["ratio_ic_over_best_generic"]
    ax.errorbar(i["N"] + 0.08 * (i["l"] - 4), r["point"], yerr=[[r["point"] - r["ci95"][0]], [r["ci95"][1] - r["point"]]], fmt=MARK.get(i["genus"], "o"),
                color=COL[i["l"]], ms=6, lw=0.8)
    ax.plot(i["N"] + 0.08 * (i["l"] - 4), h["raw_group_op_ratio_ic_Jadds_over_best_generic_Eops"]["point"], marker=MARK.get(i["genus"], "o"), mfc="none",
            color=COL[i["l"]], ms=6, ls="none")
for r in fit["instances"]:
    ax.plot(r["N"] + 0.08 * (r["l"] - 4), r["derived_ratio_if_jacobian_additions_were_free"], marker="x", color=COL[r["l"]], ms=6, ls="none")
ax.axhline(1.0, color="k", lw=1)
for key, ls_, lab_, ls_n, lmax in [("n3_g4", "--", "n=3, g=4", 3, 8), ("n4_g4", "-.", "n=4, g=4", 4, 7)]:
    f = FAM[key]["fits_log2_cost_vs_l"]["ratio_ic_over_best_generic"]
    xs = [ls_n * l for l in range(4, lmax + 1)]
    ax.plot(xs, [2 ** (f["intercept"] + f["slope"] * (x / ls_n)) for x in xs], color="k", lw=0.9, ls=ls_,
            label=f"fit, {lab_} family: slope {f['slope']:.2f} per doubling of q")
ax.plot([], [], "o", color="gray", label="filled: bitop/v1 gates (library Cantor)")
ax.plot([], [], "o", mfc="none", color="gray", label="open: Jacobian additions counted as E additions")
ax.plot([], [], "x", color="gray", label="x: all other priced phases, Jacobian additions free (derived; n=3,4 g=4)")
ax.set_yscale("log")
ax.set_xlabel("field degree N (bits of the elliptic curve's field)")
ax.set_ylabel("IC / best generic")
ax.set_title("R7.2 ratio to the best generic method (below 1 = weak); colours as in fig. 1", fontsize=9)
ax.legend(fontsize=6.5, loc="upper right")
save(fig, "fig2_ratio_vs_N")

# ── fig3: ratio decomposed by phase, n = 3, g = 4 family ──
fam = [i for i in inst if i["n"] in (3, 4) and i["genus"] == 4]
fam.sort(key=lambda i: (i["N"], i["instance"]))
fig, ax = plt.subplots(figsize=(9.0, 4.6))
phases = [("phi_Q", "Phi(Q) on K", "#bab0ac"), ("step_setup", "step set-up", "#f2cf5b"), ("relation_search", "relation search (walk + tests)", "#4c78a8"),
          ("linear_algebra", "linear algebra mod ell", "#e45756"), ("verify", "verify on E", "#54a24b")]
bottom = [0.0] * len(fam)
for key, name, c in phases:
    vals = []
    for i in fam:
        h = i["holdout"]
        best = min(h["mean_gae"]["rho"][0], h["mean_gae"]["bsgs"][0])
        m = sum(r["ic"]["phase_gae"][key] for r in i["rows"]) / len(i["rows"])
        vals.append(m / best)
    ax.bar(range(len(fam)), vals, bottom=bottom, color=c, label=name, width=0.7)
    bottom = [b + v for b, v in zip(bottom, vals)]
ax.axhline(1.0, color="k", lw=1)
ax.set_xticks(range(len(fam)))
ax.set_xticklabels([f"q={2 ** i['l']}\nN={i['N']}\nn={i['n']}" for i in fam], fontsize=6)
ax.set_ylabel("phase cost / best generic cost on the same E")
ax.set_title("R7.2: where the IC cost goes (g = 4 instances, n = 3 or 4; the line at 1 is the best generic method)", fontsize=9)
ax.legend(fontsize=7, loc="upper right")
save(fig, "fig3_phase_breakdown")

# ── fig4: factor-base scan ──
fig, ax = plt.subplots(figsize=(6.8, 4.6))
shown = {}
for i in inst:
    if i["n"] == 3 and i["genus"] == 4 and i["l"] not in shown:
        shown[i["l"]] = i
for l, i in sorted(shown.items()):
    for mode, ls in [("Dense", "-"), ("Sparse", "--")]:
        pts = sorted((r["fb_columns"], r["mean_online_gae"]) for r in i["tuning_scan"] if r["mode"] == mode)
        if pts:
            ax.plot([p[0] for p in pts], [p[1] for p in pts], ls, marker="o", ms=4, color=COL[l], label=f"q={2 ** l} {mode.lower()}" if True else None)
ax.set_xscale("log")
ax.set_yscale("log")
ax.set_xlabel("factor-base columns B (x-coordinates of F_q-points of C)")
ax.set_ylabel("mean online cost on the 10 tuning logs, E-add equivalents")
ax.set_title("R7.2 factor-base scan: cost falls as the base grows to all F_q-points;\nsolid = dense walk, dashed = sparse (walk in <Phi(P)>)", fontsize=9)
ax.legend(fontsize=6, ncol=2)
save(fig, "fig4_factor_base_scan")

# ── fig5: the (3,4) g = 8 q = 8 'hang' ──
fig, (a1, a2) = plt.subplots(1, 2, figsize=(9.0, 4.0))
ks = [r["k"] for r in prof["count_points_extension"]]
ts = [r["seconds"] for r in prof["count_points_extension"]]
a1.semilogy(ks, ts, "o-", label="run 1, measured")
a1.semilogy([r["k"] for r in prof2["count_points_extension"]], [r["seconds"] for r in prof2["count_points_extension"]], "s:", color="C1", ms=4,
            label="run 2 (more concurrent jobs), measured")
last = ts[-1]
ext_k = list(range(ks[-1], 10))
ext_t = [last * 8 ** (k - ks[-1]) for k in ext_k]
a1.semilogy(ext_k, ext_t, "o--", color="gray", label="extrapolated (x8 per degree)")
a1.set_xlabel("extension degree k  (field F_{8^k}, 3k bits)")
a1.set_ylabel("count_points_extension seconds")
a1.set_title("point counting N_k (k up to g+1 = 9)", fontsize=9)
a1.legend(fontsize=7, loc="lower right")
cm = prof["closure_measured"]
parts = [
    ("N_1..N_8 (run 1, measured)", sum(ts)),
    ("N_9 (extrapolated)", ext_t[-1]),
    ("J0 closure, cap 400000 (extrapolated\nfrom 20000-element closure)", cm["extrapolated_seconds_for_R71_cap_400000"]),
    ("three walks (extrapolated)", 3 * prof["R71_run_curve_steps_estimate"]["walk_seconds_each"]),
]
names = [p[0] for p in parts]
vals = [p[1] for p in parts]
a2.barh(range(len(parts)), vals, color=["#4c78a8", "#9ecae9", "#f58518", "#54a24b"])
a2.set_yticks(range(len(parts)))
a2.set_yticklabels(names, fontsize=7)
tot = sum(vals)
a2.axvline(r71_elapsed, color="k", lw=1)
a2.text(r71_elapsed * 1.03, 0.2, f"R7.1 example, 1 curve,\n20000 steps: measured\n{r71_elapsed:.0f} s; sum of the\nbars {tot:.0f} s (derived)", fontsize=7)
a2.set_xlim(0, 1900)
a2.set_xlabel("seconds (contended container)")
a2.set_title("where R7.1's run_curve spends its time, (l,n)=(3,4) g=8", fontsize=9)
save(fig, "fig5_hang_profile")
# ── fig6: extrapolation of the two genus-4 families (extrapolation, not measurement) ──
fig, ax = plt.subplots(figsize=(7.4, 4.8))
for key, nfam, mk in [("n3_g4", 3, "o"), ("n4_g4", 4, "s")]:
    fits = FAM[key]["fits_log2_cost_vs_l"]
    for name, ls_, col in [("ratio_ic_over_best_generic", "-", "k"), ("derived_ratio_if_step_setup_free", ":", "C0"),
                           ("derived_ratio_if_jacobian_additions_were_free", "--", "C3")]:
        f = fits[name]
        xs = [4 + 0.25 * k for k in range(0, 41)]
        ax.plot(xs, [f["intercept"] + f["slope"] * x for x in xs], ls_, color=col, lw=1.0 if nfam == 4 else 0.6, alpha=1.0 if nfam == 4 else 0.55)
    pts = [(r["l"], r["ratio_ic_over_best"]) for r in fit["instances"] if r["n"] == nfam and r["genus"] == 4]
    import math
    ax.plot([p[0] for p in pts], [math.log2(p[1]) for p in pts], mk, color="k", ms=5, mfc="none" if nfam == 3 else "k")
f = FAM["n4_g4"]["fits_log2_cost_vs_l"]["ratio_ic_over_best_generic"]
ax.errorbar([f["x_at_zero"]], [0], xerr=[[f["x_at_zero"] - f["x_at_zero_ci95"][0]], [f["x_at_zero_ci95"][1] - f["x_at_zero"]]], fmt="*", color="C1", ms=11,
            label=f"n=4 fit crosses 1 at l={f['x_at_zero']:.1f} (N={4 * f['x_at_zero']:.0f}); bootstrap 95%: l {f['x_at_zero_ci95'][0]:.1f}-{f['x_at_zero_ci95'][1]:.1f}")
ax.axhline(0, color="k", lw=0.8)
ax.plot([], [], "-", color="k", label="priced total (fit)")
ax.plot([], [], ":", color="C0", label="derived: step set-up free")
ax.plot([], [], "--", color="C3", label="derived: Jacobian additions free")
ax.plot([], [], "s", color="k", label="n=4 measured (filled)")
ax.plot([], [], "o", color="k", mfc="none", label="n=3 measured (open); thin lines: n=3 fits")
ax.set_xlim(3.8, 14)
ax.set_ylim(-4, 5.5)
ax.set_xlabel("l = log2 q   (N = n l)")
ax.set_ylabel("log2 (IC / best generic)")
ax.set_title("R7.2 extrapolation of the fitted trend (l > 7 and all dashed/dotted lines are extrapolation or derived)", fontsize=8.5)
ax.legend(fontsize=6.5, loc="lower left")
save(fig, "fig6_extrapolation")
print("ok", len(inst), "instances")
