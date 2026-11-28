#!/usr/bin/env python3
"""Lays out DEEPDIVE.md from results_deepdive.json (and quotes REPORT.md / analysis/results.json). No statistics are computed here beyond formatting
and one Fisher exact test on counts already in the results (V5 vs V6 share of significant curves)."""
import json, os, re
from scipy import stats
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
J = json.load(open(f"{HERE}/results_deepdive.json"))
RT = json.load(open(f"{ROOT}/analysis/results.json"))
REP = open(f"{ROOT}/REPORT.md").read().splitlines()
v4 = next(l for l in REP if l.startswith("4. **H-T"))[3:]
v5 = next(l for l in REP if l.startswith("5. **H-M"))[3:]
f1 = next(l for l in REP if l.startswith("- **F1 (material)"))


def c3(x, d=3):
    return f"{x:.{d}f}"


def vci(o, i=0, d=3):
    return f"{o['value'][i]:.{d}f} [{o['ci95'][i][0]:.{d}f}, {o['ci95'][i][1]:.{d}f}]"


def sci(x):
    return f"{x:.2e}"


H = J["H-T_decomposition"]; G = J["H-T_gap"]; CR = J["crater_rows"]; PV = J["H-T_vertices"]; HM = J["H-M"]; MC = J["mechanism"]; V6 = J["V6_model_effect"]; PW = J["H-M_power"]
ov = H["overall"]
vl = H["log_variance_share_p1"]
ld = H["log_decomp_overall_p1"]
ym = MC["yield_mechanism"]
bd = HM["boundary"]
pc = HM["pooled"]
vc = HM["variance_components"]
ic_rt = {k: v["arms"]["ic"] for k, v in RT["H-M"]["curves"].items()}
vids = HM["curves"]
rows_u = {r["vid"]: r for r in HM["per_curve"]["ic|unpaired"]["rows"]}
rows_p = {r["vid"]: r for r in HM["per_curve"]["ic|paired"]["rows"]}
cal = HM["maxmin_calibration"]
import numpy as np
cvp = {a: float(np.sqrt(np.mean([x["cv_between_models"] ** 2 for x in vc[a]]))) for a in vc}
icc_ic = float(np.mean([x["icc_model"] for x in vc["ic"]]))
blk_ic = float(np.mean([x["block_icc"] for x in vc["ic"]]))
msr_ic = float(np.mean([x["mse_paired_over_msw"] for x in vc["ic"]]))
fish = stats.fisher_exact([[1, 3], [4, 4]])[1]
nsig_cal = {k: sum(r["observed"] > r["null_p95"] for r in cal[k]) for k in cal}
nsig99 = {k: sum(r["observed"] > r["null_p99"] for r in cal[k]) for k in cal}
lo_gt1 = {a: sum(1 for v in vids if RT["H-M"]["curves"][v]["arms"][a]["max_over_min_model_mean"]["ci95"][0] > 1) for a in ARMS} if False else None
ARMS = ["ic", "rho-neg", "bsgs"]
lo_gt1 = {a: sum(1 for v in vids if RT["H-M"]["curves"][v]["arms"][a]["max_over_min_model_mean"]["ci95"][0] > 1) for a in ARMS}
pt_out = {a: sum(1 for v in vids if not (RT["H-M"]["curves"][v]["arms"][a]["max_over_min_model_mean"]["ci95"][0] <= RT["H-M"]["curves"][v]["arms"][a]["max_over_min_model_mean"]["value"] <= RT["H-M"]["curves"][v]["arms"][a]["max_over_min_model_mean"]["ci95"][1])) for a in ARMS}
null_med = {a: (min(r["null_median"] for r in cal[a + "|unpaired"]), max(r["null_median"] for r in cal[a + "|unpaired"])) for a in ARMS}
null_p95 = {a: (min(r["null_p95"] for r in cal[a + "|unpaired"]), max(r["null_p95"] for r in cal[a + "|unpaired"])) for a in ARMS}
obs_rng = {a: (min(r["observed"] for r in cal[a + "|unpaired"]), max(r["observed"] for r in cal[a + "|unpaired"])) for a in ARMS}
pw = PW["scenarios"]
n80 = PW["n_for_80pct_power"]
hp = bd["high_precision_ic"]; rr = bd["repeat_runs_10k_each"]
ex_u = pc["ic|unpaired|excluding_the_5_frozen_significant_curves"]; ex_p = pc["ic|paired|excluding_the_5_frozen_significant_curves"]
t6 = lambda L: H["by_level"][str(L)]
bk = PV["breakeven"]
sh = PV["transport_share_by_level"]
ex = PV["extrapolation"]["table"]


def ex_row(bits, scen, pr):
    return next(r for r in ex if r["bits_r"] == bits and r["depth_scenario"] == scen and r["price"] == pr)


q = G["tcr_overall"]
qs = G["quantile_ratios"]
shp = G["shapes_tcr"]
CS = CR["canon_shape"]
tr = PV["trend_level_price1"]
cq = PV["cochran_Q_price1.0"]

def _bv(v):
    t = RT['H-T']['by_vertex'][str(v)]['T']['1.0']
    return f"{v}: {t['value']:.2f} [{t['ci95'][0]:.2f}, {t['ci95'][1]:.2f}]"


vtxt = ', '.join(_bv(v) for v in PV['vertices_with_hi_ge_1_price1'])
md = []
A = md.append
A("# Deep dive on H-T and H-M: carry-up cost decomposition and the Weierstrass-model effect on IC")
A("")
A("**EXPLORATORY, POST HOC.** Written 2026-11-06, after the preregistered analysis and while the independent validation of the frozen snapshot is running. Study `research/volcano_ecdlp_census_followup_20261023/`. Nothing in this document changes a preregistered verdict, a frozen result or `analysis/results.json`, and no statement here is a speedup claim. Numbers come from `results_deepdive.json` (`deepdive.py`, seed 20261106, 10,000 replicates unless stated) over the committed extracts, census files and, for the IC mechanism, the 12 M session archives streamed through `tar`. Unpriced native work is priced at 0, 0.1 and 1 GAE per operation (ANALYSIS_PLAN s1).")
A("")
A("## Summary (one page)")
A("")
A("**Preregistered verdicts, unchanged, quoted from `REPORT.md`:**")
A("")
A(v4)
A("")
A(v5)
A("")
A("Erratum F1 of `REPORT.md` (quoted, last sentence of the erratum entry): **The H-M verdict therefore sits on the Monte Carlo boundary between inconclusive and supported for IC and is not robust either way.**")
A("")
A("**Exploratory findings (all post hoc; n and intervals in the sections):**")
A("")
A("| # | finding | key numbers (n) |")
A("|--:|---|---|")
A(f"| 1 | T factors exactly as T = F x T0; the automorphism factor F carries the gain, the control T0 is consistent with 1 | at 1 GAE: F {vci(ov['F'],2)}, T0 {vci(ov['T0'],2)}, T {vci(ov['T'],2)}; n = 63 vertices |")
A(f"| 2 | the ~6% gap to 1/sqrt3 is a charged additive part (ops minus canonicalisations = {G['resid_mean_sd_min_max'][0]:.0f} +- {G['resid_mean_sd_min_max'][1]:.0f}) that does not shrink; the iteration count itself matches theory | charged ratio {vci(q['S_ratio'])}; canonicalisation ratio {vci(q['canon_ratio'])}; predicted {c3(q['pred_ops_ratio_if_iterations_exactly_theory']['value'][0])}; n = {G['n_pairs_tcr']} pairs |")
A(f"| 3 | per-vertex T_v is homogeneous: no trend with level; {PV['n_ci_includes_1_price1.0']} of 63 CIs include 1 | Cochran Q {cq['Q']:.1f} on {cq['df']} df, p = {cq['p']:.2f}; slope {tr['ratio_per_level']:.4f} per level, p = {tr['perm_p_two_sided']:.2f} |")
A(f"| 4 | break-even price of a Velu point map is about {bk['pooled_a']:.0f} GAE (charged ops) and has no finite value under the plan's same-price pricing; transport share {PV['transport_share_overall']['1.0']['value']*100:.2f}% at 1 GAE | model extrapolation (not measured): {sci(ex_row(64,'depth 6',1.0)['transport_share'])} at 64 bits, {sci(ex_row(256,'depth 6',1.0)['transport_share'])} at 256 |")
A("| 5 | H-T is a known composition: the volcano preserves r, the crater adds the order-6 automorphism for the price of a few point maps | no new method; see section 5 |")
A(f"| 6 | significant IC curves (frozen seed): V5:14, V6:2, V6:99, V6:214, V6:223; V6:98 is the sixth | no V5-vs-V6 or level pattern (Fisher p = {fish:.2f}); n = 12 curves |")
A(f"| 7 | paired/blocked tests agree with the unpaired ones (blocking gains nothing); pooled across 12 curves the IC model effect is strong, rho-neg and bsgs show none | sum of F {pc['ic|unpaired']['sum_F']:.1f} vs null mean {pc['ic|unpaired']['sum_F_null_mean']:.1f}, p <= {pc['ic|unpaired']['p_sum_F']:.0e}; rho-neg p = {pc['rho-neg|unpaired']['p_sum_F']:.2f}; n = 6,144 rows per arm |")
A(f"| 8 | max/min of model means is biased up by noise alone (null median {min(null_med['ic'][0], null_med['rho-neg'][0]):.2f}-{max(null_med['ic'][1], null_med['rho-neg'][1]):.2f}); excess over null p95 is visible for IC only | IC above null p95 on {nsig_cal['ic|unpaired']} of 12 curves, rho-neg on {nsig_cal['rho-neg|unpaired']}, bsgs on {nsig_cal['bsgs|unpaired']} |")
A(f"| 9 | IC cost tracks relation yield, which tracks how many factor-base relation sources fall in the subgroup on each model; the Legendre count (factor-base length) does not | log trials per relation vs log H: r = {ym['corr_log_trials_per_relation_vs_log_H']['r_within_curve']:.2f}; log S vs log H: r = {ym['corr_log_S_ic_vs_log_H']['r_within_curve']:.2f}, p = {ym['corr_log_S_ic_vs_log_H']['perm_p']:.4f}; n = 96 models |")
A(f"| 10 | Q1's curve-effect test and H-M do not contradict: Q1's within-vertex spread already contains the model effect, which Q1's own data show (V6, 648 vertices x 3 models) | pooled F {V6['arms']['ic']['F_pooled_unpaired']:.2f}, permutation p = {V6['arms']['ic']['perm_p_unpaired']:.4f}; between-model CV {V6['arms']['ic']['cv_between_models']*100:.1f}% [{V6['arms']['ic']['cv_between_models_ci95'][0]*100:.1f}, {V6['arms']['ic']['cv_between_models_ci95'][1]*100:.1f}] |")
A(f"| 11 | design recommendation: about {n80['shrunk']} rows per model (1.5x the 64 used) gives 80% power for >= 6 of 12 under moment-shrunk truth | simulated, {PW['reps']} studies per point |")
A("")
A("```{=typst}\n#pagebreak()\n```")
A("")
# ---------------- section 1
A("## 1. H-T decomposition (question 1)")
A("")
A(f"*Exploratory, post hoc. n = 63 source vertices (levels 1-6 with 1/2/4/8/16/32 vertices), 3,064 (target, round) pairs per arm; vertex-weighted ratios of means as in A3.2; cluster bootstrap, 10,000 replicates, seed 20261106, 95% percentile CI.*")
A("")
A("With C the vertex-averaged priced `ops_gae`, M the mean number of Velu point maps, p the price per point map and N_v the source rows:")
A("")
A("- F = (C(rho-cubic, Tcr) + p M) / (C(rho-neg, Tcr) + p M): the crater factor, the same transport in both terms.")
A("- T0 = (C(rho-neg, Tcr) + p M) / C(rho-neg, N): the control of the plan.")
A(f"- T = F x T0 holds exactly (maximum error {H['identity_max_abs_err']:.1e} at every price, overall and per level).")
A("")
A("| price per point map | F (crater factor) | T0 (control) | T (verdict quantity) | transport share of T numerator |")
A("|---|---|---|---|---|")
for i, p in enumerate([0.0, 0.1, 1.0]):
    A(f"| {p:g} GAE | {vci(ov['F'], i)} | {vci(ov['T0'], i)} | {vci(ov['T'], i)} | {ov['share']['value'][i]*100:.3f}% |")
A("")
A("Per level at 1 GAE (descriptive; the plan's verdict is the overall row only):")
A("")
A("| level | vertices | F | T0 | T |")
A("|--:|--:|---|---|---|")
for L in range(1, 7):
    b = t6(L)
    A(f"| {L} | {b['n_vertices']} | {vci(b['F'],2,3)} | {vci(b['T0'],2,3)} | {vci(b['T'],2,3)} |")
A(f"| all | 63 | {vci(ov['F'],2,3)} | {vci(ov['T0'],2,3)} | {vci(ov['T'],2,3)} |")
A("")
A(f"**Which factor carries the gain.** F does. In logs at 1 GAE, log T = {ld['log_T']:.4f} splits into {ld['log_G']:.4f} from the pure automorphism ratio (rho-cubic over rho-neg on the crater rows, uncharged work priced), {ld['log_transport_corr']:+.4f} from transport, and {ld['log_T0']:.4f} from the control: {ld['log_G']/ld['log_T']*100:.0f}% of the log-gain is the automorphism, {ld['log_T0']/ld['log_T']*100:.0f}% is T0 sitting below 1. **T0 is consistent with 1**: its CI {vci(ov['T0'],2,3)} includes 1 at all three prices, as it should, because the cost of rho depends on the group order r and the volcano preserves r. Its 3% shortfall is consistent with sampling noise: the bootstrap variance of log T is shared between the numerator and the denominator draws (var log F = {vl['var_logF']:.1e}, var log T0 = {vl['var_logT0']:.1e}, covariance {vl['cov']:.1e}), so the T0 draw does about as much to the width of T's CI as F does. Level 6 alone has T0 = {t6(6)['T0']['value'][2]:.3f} (CI upper end {t6(6)['T0']['ci95'][2][1]:.3f}), which pulls its T to {t6(6)['T']['value'][2]:.3f}, about level 3's {t6(3)['T']['value'][2]:.3f}, the lowest.")
A("")
A("![fig1](figures/fig1_T_decomposition_by_level.png)")
A("")
A("*fig1: H-T decomposed (exploratory, post hoc). Source `results_deepdive.json`, key `H-T_decomposition`; SVG beside the PNG.*")
A("")
# ---------------- section 2
A("## 2. Gap to the 1/sqrt3 theory (question 2)")
A("")
A(f"*Exploratory, post hoc. Tcr pairs: n = {G['n_pairs_tcr']} (63 vertices); crater positive-control pairs (D, N, ctl pooled): n = {CR['n_pairs']}. CIs: vertex-cluster bootstrap (Tcr) and pair bootstrap (crater), 10,000 replicates.*")
A("")
A(f"Theory: with equivalence classes of order 6 against 2, rho needs sqrt(pi r / 12) against sqrt(pi r / 4) iterations, ratio 1/sqrt3 = {G['theory']:.4f}. Measured charged-ops ratio rho-cubic / rho-neg: **{vci(q['S_ratio'])}** on the carried instances and **{vci(CR['S_ratio'], None) if False else c3(CR['S_ratio']['value'],4)} [{c3(CR['S_ratio']['ci95'][0],4)}, {c3(CR['S_ratio']['ci95'][1],4)}]** on the crater (the report's 0.6113). Both CIs exclude 1/sqrt3: the gap is +{q['S_ratio']['value'][0]-G['theory']:.3f} (6.5%) and +{CR['S_ratio']['value']-G['theory']:.3f} (5.9%).")
A("")
A("**What explains it, derived from the recorded fields.** Every V5 rho row records `ops_gae` (charged) and `canonicalisations_uncharged`. Their difference is:")
A("")
A(f"- {G['resid_mean_sd_min_max'][0]:.1f} +- {G['resid_mean_sd_min_max'][1]:.1f} operations (min {G['resid_mean_sd_min_max'][2]:.0f}, max {G['resid_mean_sd_min_max'][3]:.0f}), **identical in the rho-cubic and rho-neg rows of a pair in {G['resid_cub_equals_resid_neg_fraction']*100:.0f}% of the {G['n_pairs_tcr']} pairs**, and uncorrelated with the walk length (correlation {G['corr_resid_with_canon_neg']:+.3f} for rho-neg, {G['corr_resid_with_canon_cub']:+.3f} for rho-cubic). It is an additive part that belongs to the instance, not to the walk: the automorphism does not shrink it. (What it is, setup or collision handling, is not derivable from the CSV; only that it does not scale with the walk.)")
A(f"- The canonicalisation count, the natural proxy for walk iterations, has the ratio **{vci(q['canon_ratio'])}** on Tcr and {c3(CR['canon_ratio']['value'],4)} [{c3(CR['canon_ratio']['ci95'][0],4)}, {c3(CR['canon_ratio']['ci95'][1],4)}] on the crater: both include 1/sqrt3 = {G['theory']:.4f}.")
A(f"- If the iterations scale by exactly 1/sqrt3 and the additive part stays, the charged ratio would be (theta C + f) / (C + f) = **{c3(q['pred_ops_ratio_if_iterations_exactly_theory']['value'][0],4)}** on Tcr (measured {c3(q['S_ratio']['value'][0],4)}) and {c3(CR['pred_ops_ratio_if_iterations_exactly_theory'],4)} [{c3(CR['pred_ops_ratio_ci95'][0],4)}, {c3(CR['pred_ops_ratio_ci95'][1],4)}] on the crater (measured {c3(CR['S_ratio']['value'],4)}). The gap is therefore accounted for by the additive part, with nothing left over that the data can resolve.")
A("")
A(f"Absolute levels: canonicalisations / sqrt(r) are {G['measured_canon_over_sqrt_r']['rho-neg']:.3f} (rho-neg) and {G['measured_canon_over_sqrt_r']['rho-cubic']:.3f} (rho-cubic) against the idealised sqrt(pi/4) = {G['theory_iterations_over_sqrt_r']['k2']:.3f} and sqrt(pi/12) = {G['theory_iterations_over_sqrt_r']['k6']:.3f}: both arms sit {(G['measured_over_theory']['rho-neg']-1)*100:.1f}% and {(G['measured_over_theory']['rho-cubic']-1)*100:.1f}% above the formula. That excess (distinguished-point delay, fruitless cycles, collision detection) is equal in the two arms within noise, so it cancels in the ratio; the data **cannot separate** its causes, and in particular cannot say whether rho-cubic loses more to fruitless cycles than rho-neg.")
A("")
A("Distribution shape (Tcr pairs, n = 3,064 each):")
A("")
A("| quantity | mean | median | CV | median/mean | skew |")
A("|---|--:|--:|--:|--:|--:|")
for nm, k in [("S rho-cubic", "S_cubic"), ("S rho-neg", "S_neg"), ("canonicalisations rho-cubic", "canon_cubic"), ("canonicalisations rho-neg", "canon_neg")]:
    s_ = shp[k]
    A(f"| {nm} | {s_['mean']:.4g} | {s_['median']:.4g} | {s_['cv']:.3f} | {s_['median_over_mean']:.3f} | {s_['skew']:.2f} |")
A(f"| Rayleigh reference | | | {shp['rayleigh_reference']['cv']:.3f} | {shp['rayleigh_reference']['median_over_mean']:.3f} | |")
A("")
A(f"The canonicalisation counts have the CV and median/mean of a Rayleigh-like collision time in both arms (CV 0.50-0.52 against 0.523), so rho-cubic's walk length is the same shape as rho-neg's, scaled. Quantile ratios cubic / neg (5, 25, 50, 75, 95, 99%): charged S {', '.join(f'{x:.3f}' for x in qs['S']['ratio_cubic_over_neg'])}; canonicalisations {', '.join(f'{x:.3f}' for x in qs['canon']['ratio_cubic_over_neg'])}. The charged ratio is highest at the low quantiles (the additive part is a larger share of a short solve), the canonicalisation ratio is flat near 0.56-0.59 (row-bootstrap CIs in fig8, n = 1,000 replicates).")
A("")
A("**Not charged for rho-cubic**: the class-representative work per iteration (up to six images) is counted in `canonicalisations_uncharged` for both arms and priced at the same p for both. A canonicalisation under the order-6 action is plausibly dearer than a sign choice; a per-arm price was not measured, so T at 0.1 and 1 GAE treats them alike.")
A("")
A("![fig8](figures/fig8_gap_to_theory.png)")
A("")
A("*fig8: gap to theory. Source `results_deepdive.json`, keys `H-T_gap`, `crater_rows`.*")
A("")
# ---------------- section 3
A("## 3. Heterogeneity across the 63 vertices (question 3)")
A("")
A(f"*Exploratory, post hoc. n = 63 vertices; per-vertex CIs are the preregistered pair-bootstrap CIs in `analysis/results.json` (descriptive there), reused here.*")
A("")
A(f"T_v at 1 GAE ranges {PV['T_range_price1'][0]:.2f} to {PV['T_range_price1'][1]:.2f}. The per-vertex CI includes 1 for **{PV['n_ci_includes_1_price0.0']} of 63 vertices at 0 GAE, {PV['n_ci_includes_1_price0.1']} at 0.1 and {PV['n_ci_includes_1_price1.0']} at 1 GAE** (vertices {', '.join(str(v) for v in PV['vertices_with_hi_ge_1_price1'])} at 1 GAE; largest upper end {PV['max_hi_price1.0']:.2f}); their T_v are {vtxt} (8 targets x few rounds each). The control T0_v includes 1 for {PV['n_T0_ci_includes_1_price1.0']} of 63.")
A("")
A(f"Heterogeneity beyond sampling noise is not detected: Cochran's Q on log T_v = {cq['Q']:.1f} on {cq['df']} df, p = {cq['p']:.2f}, I2 = {cq['I2']*100:.0f}% (standard errors from the percentile CI widths, approximate). Trend with source level, which here equals path length to the crater (M_v = 2 x level): weighted slope of log T_v = {tr['wls_slope_logT_per_level']:+.4f} per level (ratio {tr['ratio_per_level']:.4f}), permutation p = {tr['perm_p_two_sided']:.2f} (10,000 shuffles); Spearman rho = {tr['spearman_rho']:+.2f}, permutation p = {tr['spearman_perm_p']:.2f} (2,000 shuffles). Level means of T_v at 1 GAE: {', '.join(f'L{k} {v:.3f}' for k, v in tr['level_means_T'].items())}. No trend, no outlier beyond what 63 noisy ratios give.")
A("")
A("![fig2](figures/fig2_T_v_forest.png)")
A("")
A("*fig2: per-vertex T_v at 1 GAE with 95% CIs, grey diamonds the control T0_v. Source `analysis/results.json` `H-T.by_vertex`.*")
A("")
# ---------------- section 4
A("## 4. Break-even price and transport share (question 4)")
A("")
A("*Exploratory, post hoc. n = 63 vertices. Break-even prices are point values per vertex (no CI); pooled transport shares carry the bootstrap CI of section 1.*")
A("")
A("p* is the price per Velu point map at which T = 1: p* = (C(rho-neg, N_v) - C(rho-cubic, Tcr_v)) / M_v. It depends on what C includes:")
A("")
A(f"- (a) charged ops only: per vertex min {bk['a_per_vertex']['min']:.0f}, median {bk['a_per_vertex']['median']:.0f}, max {bk['a_per_vertex']['max']:.0f} GAE per point map; **pooled {bk['pooled_a']:.0f} GAE**. By level 1-6: {', '.join(f'{v:.0f}' for v in bk['by_level_a'].values())}.")
A(f"- (b) every uncharged operation priced at 1 GAE: min {bk['b_per_vertex']['min']:.0f}, median {bk['b_per_vertex']['median']:.0f}, max {bk['b_per_vertex']['max']:.0f}; pooled {bk['pooled_b']:.0f}.")
A(f"- (c) the plan's definition, the same p on the uncharged counters and on the point maps: **no finite p* exists for any of the {PV['n_vertices']} vertices**. Raising p adds more to the rho-neg denominator (about 3,900 canonicalisations) than to the rho-cubic numerator (about 2,300) and the transport term (2 to 12), so T falls with p, from {ov['T']['value'][0]:.4f} at 0 to {ov['T']['value'][2]:.4f} at 1.")
A("")
A("A Velu point map at an order-2 isogeny costs a handful of field operations, so the study's upper price of 1 GAE is already conservative, and the break-even is two to three orders of magnitude above it.")
A("")
A("Transport share of the T numerator, measured, at 1 GAE (95% CI): " + "; ".join(f"level {L} {sh[str(L)]['1.0']['value']*100:.3f}% [{sh[str(L)]['1.0']['ci95'][0]*100:.3f}, {sh[str(L)]['1.0']['ci95'][1]*100:.3f}]" for L in range(1, 7)) + f"; overall {PV['transport_share_overall']['1.0']['value']*100:.3f}% [{PV['transport_share_overall']['1.0']['ci95'][0]*100:.3f}, {PV['transport_share_overall']['1.0']['ci95'][1]*100:.3f}]. It grows linearly with depth (2 point maps per level) and stays below 0.25%.")
A("")
A("**MODEL EXTRAPOLATION, not a measurement.** Assumptions: solve cost = c sqrt(r) GAE with c = " + f"{PV['extrapolation']['c_S_cubic']:.3f}" + " (the vertex-averaged rho-cubic S on Tcr; the additive part scales too); transport = 2 x depth point maps priced at p, independent of bit size; depth scenarios: the measured mean depth (5.1), 6 held fixed, and 6 x bits / 26. Not modelled: finding the path to the crater, whether a j = 0 volcano of that depth exists at that size, cofactors, and the class machinery at that size.")
A("")
A("| bits of r | depth scenario | depth | transport GAE (p = 1) | projected transport share, p = 0.1 | p = 1 |")
A("|--:|---|--:|--:|--:|--:|")
for bits in (26, 64, 128, 256):
    for scen in ("measured mean depth", "depth 6", "depth 6*bits/26"):
        a1, a2 = ex_row(bits, scen, 0.1), ex_row(bits, scen, 1.0)
        A(f"| {bits}{' (measured)' if bits == 26 else ''} | {scen} | {a2['depth']:.1f} | {a2['transport_gae']:.0f} | {sci(a1['transport_share'])} | {sci(a2['transport_share'])} |")
A("")
A("The share falls like 2^(-bits/2). The projection says transport stays negligible if the model holds; it does not say a crater of such depth exists at cryptographic size, nor that the path can be found cheaply, and it is not evidence.")
A("")
A("![fig3](figures/fig3_breakeven_and_extrapolation.png)")
A("")
A("*fig3: (a) and (b) measured, (c) model extrapolation. Source `results_deepdive.json`, key `H-T_vertices`.*")
A("")
# ---------------- section 5
A("## 5. What H-T means (question 5)")
A("")
A("An l-isogeny between two curves over the same prime field preserves the group order up to the isogeny's kernel: the volcano's curves all have the same order, so the same prime r in the same size subgroup, and Pollard rho on any of them costs the same, which is what the control T0 = 0.97 (CI includes 1) shows. The j = 0 crater, though, has an extra automorphism of order 6 that the other curves lack; rho can quotient by it and run on classes of size 6 instead of 2, which saves a factor 1/sqrt3, about 0.58. A Velu isogeny is a rational map of known formula, so carrying the instance (G, Q) up to the crater costs two point maps per level: 2 to 12 maps, well under 1% of a 26-bit solve even at 1 GAE each. H-T measures that composition end to end and finds 0.58 of the cost of solving where the instance was born with rho-neg, at 63 vertices.")
A("")
A("It is a known composition, the census's A2.3 transport-then-solve with the crater's automorphism as the destination gain, not a new method: the gain is the j = 0 automorphism and the isogeny is only the road to it. It is relative to rho-neg at the source (the strongest rho available on a curve with j not 0 or 1728), and it is a statement about this 26-bit family, where an isogeny path to a j = 0 crater of depth 1 to 6 exists. It says nothing about curves that have no such path, and nothing about index calculus, which is 53 to 141 times the matched rho in this study (REPORT section 9).")
A("")
# ---------------- section 6
A("## 6. Which five curves, and where the near misses are (question 6)")
A("")
A("*Exploratory, post hoc. n = 12 curves, 8 models x 64 (target, round) rows per curve and arm; the table is the frozen-seed result of `analysis/results.json` (permutation p with 10,000 shuffles, seed 20261024, Holm over the 12 curves).*")
A("")
A("| curve | level | ic F | ic p_perm | ic p_Holm (frozen) | significant | rows: unpaired Holm, 20,000 shuffles, new seed | paired Holm |")
A("|---|--:|--:|--:|--:|---|--:|--:|")
for v in vids:
    a = ic_rt[v]
    A(f"| {v} | {RT['H-M']['curves'][v]['level']} | {a['F']:.2f} | {a['p_perm']:.4f} | {a['p_holm']:.4f} | {'yes' if a['significant'] else 'no'} | {rows_u[v]['holm_p']:.4f} | {rows_p[v]['holm_p']:.4f} |")
A("")
A(f"The five frozen-seed curves are V5:14 (level 4), V6:2 (0), V6:99 (1), V6:214 (1) and V6:223 (3). Near misses: V6:98 (Holm 0.0511, the sixth), then V6:43 (0.161), V6:599 (0.145), V5:19 (0.279), V5:0 (0.252). **No clustering**: V5 has 1 of 4 curves significant and V6 4 of 8 (Fisher exact p = {fish:.2f}); by level (frozen run), level 0 has 1 of 3 (V6:2; V5:0 and V6:98 not), level 1 has 2 of 2, level 2 has 0 of 2, level 3 has 1 of 1, level 4 has 1 of 2 (V5:14 yes, V6:599 no), levels 5 and 6 have 0 of 1 each. Most of the non-significant curves still have F above the null mean of about 1 (V5:0 1.95, V5:19 1.77, V6:43 2.21, V6:599 2.33, V6:98 2.81); only V6:262 (1.00) and V5:44 (0.77) do not. That is why the pooled test of section 7 is the better instrument.")
A("")
# ---------------- section 7
A("## 7. Paired and pooled tests (question 7), and the V6:98 boundary")
A("")
A(f"*Exploratory, post hoc; seed 20261106. n = 12 curves x 8 models x 64 blocks per arm. The frozen reproduction through the committed `hypothesis_M` matched `results.json` exactly (F and p for all 36 curve-arm cells).*")
A("")
A("**The boundary.** The frozen rule reads 5 of 12 as inconclusive; erratum F1 says the 6th curve, V6:98, sits on the Monte Carlo boundary. Here is how far:")
A("")
A(f"- V6:98's raw permutation p is {bd['p_at_max']:.5f} +- {bd['mc_se_at_max']:.5f} at {bd['max_shuffles']:,} shuffles (running estimates by shuffle count: " + ", ".join(f"{int(k):,}: {v:.4f}" for k, v in bd['running_p_one_stream'].items()) + f"); the parametric F(7, 504) gives {bd['p_parametric_F_7_504']:.5f}. Holm's multiplier at its rank is 7, so the adjusted p is about 7 x {bd['p_at_max']:.5f} = {7*bd['p_at_max']:.4f}, just under 0.05.")
A(f"- At {hp['unpaired']['B']:,} shuffles per curve the unpaired Holm p is {hp['unpaired']['holm']['V6:98']:.4f} and the unpaired count is {hp['unpaired']['n_sig']} of 12; the paired count is {hp['paired']['n_sig']} of 12 (V6:98 Holm {hp['paired']['holm']['V6:98']:.4f}).")
A(f"- {rr['unpaired']['n_runs']} independent whole-table reruns with 10,000 shuffles each (the frozen setting, different seeds): unpaired design 6 of 12 in {rr['unpaired']['n_sig_counts'].get('6',0)} runs and 5 of 12 in {rr['unpaired']['n_sig_counts'].get('5',0)}, V6:98 Holm quantiles 2.5/50/97.5%: {', '.join(f'{x:.4f}' for x in rr['unpaired']['holm_V698_quantiles'])}; paired design 6 of 12 in {rr['paired']['n_sig_counts'].get('6',0)} runs and 5 in {rr['paired']['n_sig_counts'].get('5',0)}.")
A("")
A(f"So the preregistered statistic landed on 5 on the one stream it used ({rr['unpaired']['n_sig_counts'].get('5',0)}% of independent 10,000-shuffle streams do the same): with an exact evaluation of the same rule the verdict would read 6 of 12, rho-neg 0, bsgs 0, supported for IC. This agrees with erratum F1; it does not change the reported verdict, and it does not rest on any other curve being borderline (at the high-precision setting the sixth curve is at {hp['unpaired']['holm']['V6:98']:.4f} and the seventh, V6:43, at {hp['unpaired']['holm']['V6:43']:.3f}).")
A("")
A("![fig7](figures/fig7_V698_boundary.png)")
A("")
A("*fig7: V6:98 against shuffle count and across independent 10,000-shuffle runs. Source `results_deepdive.json`, `H-M.boundary`.*")
A("")
A(f"**Paired (blocked) test.** The 8 models of a curve solve the same 64 (target, round) instances (BSGS gives identical cost in 100% of blocks), so a blocked permutation (shuffle model labels inside each block) is available. It buys nothing here: the residual mean square after removing blocks is {msr_ic:.3f} of the within-model mean square for IC, and the block share of variance is {blk_ic*100:.1f}% (instance difficulty does not carry across models). Per curve the paired and unpaired p's differ by Monte Carlo and small shifts only (fig4); both designs give **6 of 12** for IC at 20,000 shuffles and 0 for rho-neg; for bsgs every block is identical, so F is 0 (unpaired) and the blocked statistic is undefined.")
A("")
A("![fig4](figures/fig4_HM_paired_vs_unpaired.png)")
A("")
A("*fig4: per-curve Holm p, unpaired against paired, 20,000 shuffles per curve. Source `results_deepdive.json`, `H-M.per_curve`.*")
A("")
A("**Pooled across the 12 curves** (the per-curve permutations are independent, so the null is the sum of independent permutation F values; Fisher's combination uses each curve's permutation ranks; 20,000 shuffles, so p cannot go below 5e-5):")
A("")
A("| arm | design | sum of F (12 curves) | null mean | null 95th pct | p (sum of F) | Fisher p |")
A("|---|---|--:|--:|--:|--:|--:|")
for arm in ("ic", "rho-neg", "bsgs"):
    for des in ("unpaired", "paired"):
        r = pc[f"{arm}|{des}"]
        if "sum_F" in r:
            A(f"| {arm} | {des} | {r['sum_F']:.1f} | {r['sum_F_null_mean']:.1f} | {r['sum_F_null_p95']:.1f} | {r['p_sum_F']:.1e} | {r['p_fisher']:.1e} |")
        else:
            A(f"| {arm} | {des} | undefined | | | | |")
A(f"| ic, excluding the 5 frozen-significant curves (n = 7) | unpaired | {ex_u['sum_F']:.1f} | {ex_u['sum_F_null_mean']:.1f} | {ex_u['sum_F_null_p95']:.1f} | {ex_u['p_sum_F']:.1e} | {ex_u['p_fisher']:.1e} |")
A(f"| ic, excluding the 5 frozen-significant curves (n = 7) | paired | {ex_p['sum_F']:.1f} | {ex_p['sum_F_null_mean']:.1f} | {ex_p['sum_F_null_p95']:.1f} | {ex_p['p_sum_F']:.1e} | {ex_p['p_fisher']:.1e} |")
A("")
A(f"The evidence for *some* model effect on IC is strong overall (mean F {pc['ic|unpaired']['mean_F']:.2f} per curve against a null mean of about 1.0), and it does not rest on the five significant curves. rho-neg is below its null (mean F {pc['rho-neg|unpaired']['mean_F']:.2f}; p = {pc['rho-neg|unpaired']['p_sum_F']:.2f}) and bsgs is exactly 0. The pooled test was chosen after the data; it is a post hoc strengthening of the question, not a replacement for the preregistered per-curve rule.")
A("")
# ---------------- section 8
A("## 8. Effect-size calibration of max/min (question 8): a methodological finding")
A("")
A(f"*Exploratory, post hoc. n = 12 curves per arm, 8 model means each of 64 rows. Null: model labels permuted across the 512 rows of a curve, 20,000 shuffles, seed 20261106.*")
A("")
A(f"`REPORT.md` section 8 gives, per curve and arm, max/min of the eight model means with a percentile bootstrap CI (rows resampled within model). That ratio is biased upward by noise alone: even when no model effect exists, the largest of eight noisy means over the smallest is above 1. Under the permutation null its **median is {null_med['rho-neg'][0]:.2f} to {null_med['rho-neg'][1]:.2f} for rho-neg and {null_med['ic'][0]:.2f} to {null_med['ic'][1]:.2f} for IC** (95th percentile {null_p95['rho-neg'][0]:.2f} to {null_p95['rho-neg'][1]:.2f}). The reported bootstrap CIs inherit the same bias: {lo_gt1['rho-neg']} of 12 rho-neg CIs have a lower end above 1 although rho-neg has no significant curve, and for BSGS, where the models carry identical instances and cost, the observed ratio is exactly 1.000 but all {lo_gt1['bsgs']} CIs lie above 1 (about [1.03, 1.11]) and {pt_out['bsgs']} of 12 exclude their own point value. The bootstrap CI of a max/min is a CI of a biased statistic, not of the model effect.")
A("")
A("| arm | observed max/min, range over 12 curves | null median, range | null 95th pct, range | curves above null 95th pct | above null 99th pct |")
A("|---|---|---|---|--:|--:|")
for a in ARMS:
    A(f"| {a} | {obs_rng[a][0]:.3f} to {obs_rng[a][1]:.3f} | {null_med[a][0]:.3f} to {null_med[a][1]:.3f} | {null_p95[a][0]:.3f} to {null_p95[a][1]:.3f} | {nsig_cal[a+'|unpaired']} of 12 | {nsig99[a+'|unpaired']} of 12 |")
A("")
A(f"Read against the null, **rho-neg's observed {obs_rng['rho-neg'][0]:.2f}-{obs_rng['rho-neg'][1]:.2f} is entirely null-sized (0 of 12 above the null 95th percentile)**, so the percentile CIs in REPORT section 8 should not be read as showing a rho-neg model effect, and IC's observed ratio exceeds the null 95th percentile on {nsig_cal['ic|unpaired']} of 12 curves ({nsig99['ic|unpaired']} above the 99th percentile; about 0.6 would be expected by chance): the 'excess over null median' for IC runs from {min(r['excess_over_null_median'] for r in cal['ic|unpaired']):.2f} (no excess) to {max(r['excess_over_null_median'] for r in cal['ic|unpaired']):.2f} (V6:214). The unpaired null for bsgs is not a fair benchmark because BSGS values are identical across models by construction (observed 1.000 against a null median {null_med['bsgs'][0]:.2f}): that cell is an illustration of the bias, not a test. Methodologically, an effect size here should be quoted as observed / null median or as the between-model coefficient of variation (method of moments: IC {cvp['ic']*100:.1f}% pooled over the 12 curves, rho-neg {cvp['rho-neg']*100:.1f}%, bsgs 0), not as a raw max/min with a bootstrap CI. I did not edit `REPORT.md`.")
A("")
A("![fig5](figures/fig5_null_calibrated_maxmin.png)")
A("")
A("*fig5: observed max/min of model means (dots), the reported bootstrap CI (grey) and the 95th percentile under model-label permutation (bars). Source `results_deepdive.json`, `H-M.maxmin_calibration`.*")
A("")
# ---------------- section 9
A("## 9. Mechanism: why IC depends on the model (question 9)")
A("")
A(f"*Exploratory, post hoc. n = 96 models (12 curves x 8), 64 ic rows each. IC features come from the committed extracts; relation counters come from streaming `records.jsonl` of the 12 M session archives (no unpacking). The factor base and the subgroup count H were rebuilt from the census coefficients in 30 lines of exact modular arithmetic (not an ecbench measurement) and checked against the recorded `abscissae_scanned` of all 96 models: all 96 match.*")
A("")
A("What `prime-abscissa:size=64` is, from `src/cryptanalysis/ic_boundary.rs` (`prime_factor_base`): the 64 smallest abscissae x = 1, 2, 3, ... for which x^3 + ax + b is a square, both signs. 'Prime' means prime field; the base is not prime-valued x. An isomorphism (x, y) -> (u^2 x, u^3 y) moves which small x lie on the curve, so each model has a different factor base. Each IC solve stops after about " + f"{ym['relations_per_solve_mean']:.0f}" + f" relations on average" + f" and spends almost all its cost in the relation phase: S tracks `trials` (log S against log trials per relation: r = {ym['corr_log_S_ic_vs_log_trials_per_relation']['r_within_curve']:.2f} within curve).")
A("")
A(f"Findings, within-curve (each curve's models centred, so curve and cofactor effects are removed), permutation p from 10,000 shuffles of model labels inside a curve:")
A("")
A("| per-model feature | against | r | p |")
A("|---|---|--:|--:|")
for lab, key, tgt in [("log H (relation sources inside the subgroup)", "corr_log_trials_per_relation_vs_log_H", "log trials per relation"), ("log H", "corr_log_S_ic_vs_log_H", "log mean S (ic)"), ("log H", "corr_log_S_rho_neg_vs_log_H_negative_control", "log mean S (rho-neg), negative control"), ("k0 + 0.5 (factor-base columns inside the subgroup)", "corr_log_S_ic_vs_k0", "log mean S (ic)"), ("npair + 0.5 (same-coset column pairs)", "corr_log_S_ic_vs_npair", "log mean S (ic)")]:
    r = ym[key]
    A(f"| {lab} | {tgt} | {r['r_within_curve']:+.3f} | {r['perm_p']:.4f} |")
leg = MC["correlations_within_curve"]["ic_log_S"]["legendre_symbols_uncharged"]
A(f"| Legendre symbols to build the base (= abscissae scanned) | log mean S (ic) | {leg['r_within_curve']:+.3f} | {leg['perm_p']:.2f} |")
for nm in ("u_over_p", "a_over_p", "b_over_p", "gx_over_p", "gy_over_p"):
    r = MC["correlations_within_curve"]["ic_log_S"][nm]
    A(f"| model coefficient {nm.replace('_over_p','')} / p | log mean S (ic) | {r['r_within_curve']:+.3f} | {r['perm_p']:.2f} |")
mi = MC["correlations_within_curve"]["ic_log_S"]["model"]
A(f"| model index 0-7 | log mean S (ic) | {mi['r_within_curve']:+.3f} | {mi['perm_p']:.3f} (one of 13 uncorrected tests; Bonferroni {min(1, mi['perm_p']*13):.2f}) |")
A("")
A(f"H = 2 k0 + 4 C(k0, 2) + 2 npair counts the distinct elements +-P_i and +-P_i +- P_j (P in the 64 factor-base points) that lie in the subgroup of order r. A relation needs a target combination equal to such an element, so the per-trial yield is expected to be proportional to H / r (a counting argument; the data below test its consequence). H varies by model (for example {', '.join(f'{k} {v[0]}-{v[1]}' for k, v in list(ym['H_range_by_curve'].items())[:4])}), because which small x are on the curve, and in which coset of the subgroup (cofactors {', '.join(str(c) for c in ym['cofactors'])}), changes with the model. Across the 96 models the within-curve correlation between log H and log trials per relation is **{ym['corr_log_trials_per_relation_vs_log_H']['r_within_curve']:+.2f}** and with log mean S **{ym['corr_log_S_ic_vs_log_H']['r_within_curve']:+.2f}** (within-curve regression slope {ym['within_curve_regression_logS_on_logH']['slope']:.2f}, R2 {ym['within_curve_regression_logS_on_logH']['R2_within_curve']:.2f}; capped below 1 by row noise on 64-row means). The negative control, rho-neg's cost on the same models, does not correlate (r = {ym['corr_log_S_rho_neg_vs_log_H_negative_control']['r_within_curve']:+.2f}, p = {ym['corr_log_S_rho_neg_vs_log_H_negative_control']['perm_p']:.2f}). The factor-base length (Legendre count) is not the mechanism (r = {leg['r_within_curve']:+.2f}), nor are the model coefficients.")
A("")
A("Per curve (n = 8 models each; Spearman rank correlation of H with the model's trials per relation and with its mean S; negative means higher H, cheaper IC):")
A("")
A("| curve | cofactor | rho(H, trials per relation) | rho(H, S ic) |")
A("|---|--:|--:|--:|")
for v, x in ym["per_curve"].items():
    A(f"| {v} | {x['cofactor']} | {x['spearman_H_vs_trials_per_relation']:+.2f} | {x['spearman_H_vs_S_ic']:+.2f} |")
bv_ = ym["by_volcano"]
A("")
A(f"The association is negative on {ym['n_curves_negative_spearman_tpr']} of 12 curves for trials per relation and {ym['n_curves_negative_spearman_S']} of 12 for S. It is strongest on V6 (cofactor 47; within-curve r = {bv_['V6']['r_log_tpr_vs_log_H']:+.2f} for trials per relation and {bv_['V6']['r_log_S_vs_log_H']:+.2f} for S, n = {bv_['V6']['n_models']} models) and weaker on V5 (cofactor 4; {bv_['V5']['r_log_tpr_vs_log_H']:+.2f} and {bv_['V5']['r_log_S_vs_log_H']:+.2f}, n = {bv_['V5']['n_models']}; the V5 crater V5:0 has the wrong sign). Neither k0 nor npair alone reaches significance (table above); only their combination H does. The log scale is used so that V5's larger absolute H range does not dominate the pooled within-curve correlation; the rank-based per-curve table shows the pattern without it. That the predictor's form matters is a reason to fix it before testing it again.")
A("")
A(f"This supplies the mechanism that REPORT section 12 declined to claim: the x-coordinate dependence of the factor base acts through how many relation sources fall in the subgroup. It is a measured association on 96 models with a derived predictor chosen after seeing the relation counters, so it is exploratory; a follow-up should fix H before the run. Phase counters other than trials, relations and `abscissae_scanned` (linear algebra rank, row operations) were not used.")
A("")
A("![fig11](figures/fig11_ic_yield_mechanism.png)")
A("")
A("*fig11: within-curve log trials per relation (left) and log mean S (right) against log H. Source `results_deepdive.json`, `mechanism`.*")
A("")
A("![fig6](figures/fig6_IC_model_means_significant_curves.png)")
A("")
A("*fig6: IC model means (+-1.96 SE, n = 64 rows each) on the five frozen-significant curves and V6:98; brackets under the model index give H.*")
A("")
# ---------------- section 10
A("## 10. Reconciling with Q1's curve-effect result (question 10)")
A("")
A(f"*Exploratory, post hoc. n = {V6['arms']['ic']['n_vertices']} V6 vertices, each with its D curve and two ctl re-scalings (three isomorphic models, the same instances); rows per model 8 to 56 depending on level.*")
A("")
A("The two tests differ in design and in what varies. Q1's curve-effect test asks whether the D curves of different vertices spread more (variance of the D means within level) than the three isomorphic copies of one vertex (within-vertex variance of {D, ctl1, ctl2}); the ratio was F = 0.99 for IC (p 0.42), and erratum F6 notes that its per-curve flags cannot fire at 648 curves (smallest attainable Holm p about 0.33). H-M asks whether the 8 models of one curve differ. They address different questions: Q1 compares *curves* against a yardstick built from *models*, and that yardstick (variance of the three model means, 196.4 for IC) already contains whatever model effect exists. H-M measures the yardstick itself.")
A("")
A(f"Q1's data can be asked the H-M question directly: within each vertex, shuffle the three model labels (unpaired, and inside (target, round) blocks), pooled over vertices:")
A("")
A("| arm | pooled F (3 models within vertex) | permutation p, unpaired | blocked | between-model CV (95% CI, vertex bootstrap) | within-vertex variance of model means, observed vs row-noise expectation |")
A("|---|--:|--:|--:|---|---|")
for a in ARMS:
    r = V6["arms"][a]
    if "F_pooled_unpaired" in r and r.get("F_pooled_unpaired") is not None and a != "bsgs":
        A(f"| {a} | {r['F_pooled_unpaired']:.2f} | {r['perm_p_unpaired']:.4f} | {r['perm_p_paired']:.4f} | {r['cv_between_models']*100:.1f}% [{r['cv_between_models_ci95'][0]*100:.1f}, {r['cv_between_models_ci95'][1]*100:.1f}] | {r['within_vertex_var_of_model_means_obs']:.4g} vs {r['within_vertex_var_expected_from_row_noise_only']:.4g} |")
    else:
        A(f"| {a} | 0 (identical across models) | 1.0 | 1.0 | 0 | 0 |")
A("")
a = V6["arms"]["ic"]
A(f"On Q1's own data the IC model effect is present (p at the 2,000-shuffle floor, 5e-4), with a between-model CV of {a['cv_between_models']*100:.1f}% [{a['cv_between_models_ci95'][0]*100:.1f}, {a['cv_between_models_ci95'][1]*100:.1f}], in line with the {cvp['ic']*100:.1f}% estimated from the 12 M curves; rho-neg shows none (F {V6['arms']['rho-neg']['F_pooled_unpaired']:.2f}). Of the within-vertex variance of IC model means (196.4), about {a['within_vertex_var_expected_from_row_noise_only']:.0f} is row noise and the remaining {a['within_vertex_var_of_model_means_obs']-a['within_vertex_var_expected_from_row_noise_only']:.0f} is model effect; the between-vertex variance of D means was 194.4, which is the same size, so **vertex-to-vertex differences in IC cost are not distinguishable from the model effect plus row noise**. That is what Q1 reports, and it is consistent with H-M: no contradiction. The reading to avoid is 'a curve does not differ from its re-scalings', which suggests the re-scalings are interchangeable; for IC they are not (CV about 9%), and Q1 could not see it because it was built to detect curve effects, not model effects. Q1's rho-neg and bsgs conclusions are unaffected.")
A("")
A("![fig10](figures/fig10_model_effect_by_arm.png)")
A("")
A("*fig10: between-model CV by arm, V6 Q1 data (3 models, 648 vertices) and H-M (8 models, 12 curves). Source `results_deepdive.json`, `V6_model_effect` and `H-M.variance_components`.*")
A("")
# ---------------- section 11
A("## 11. Power for a follow-up (question 11)")
A("")
A(f"*Exploratory, post hoc design recommendation, not a result. {PW['reps']} simulated studies per point, seed 20261106.*")
A("")
A("Simulation: for each of the 12 curves, true model means are the grand mean plus the observed deviations shrunk by sqrt(max(0, 1 - 1/F)) (so that the simulated between-model mean square matches the observed one in expectation), each model's 64 observed rows are rescaled to its true mean and resampled with replacement n times, parametric F(7, 8n-8) p-values, Holm over the 12 curves, outcome = ic significant on at least 6 of 12. 'Observed' uses the unshrunk means (optimistic); 'null' sets all model means equal.")
A("")
A("| rows per model | P(>= 6 of 12), shrunk | mean significant, shrunk | P(>= 6), observed means | P(>= 6), null |")
A("|--:|--:|--:|--:|--:|")
for s_, o_, n_ in zip(pw["shrunk"], pw["observed"], pw["null"]):
    A(f"| {s_['n_rows_per_model']} | {s_['P_at_least_6_of_12']:.3f} | {s_['mean_n_sig']:.1f} | {o_['P_at_least_6_of_12']:.3f} | {n_['P_at_least_6_of_12']:.3f} |")
A("")
A(f"At the 64 rows used, the shrunk scenario has power {pw['shrunk'][0]['P_at_least_6_of_12']:.2f} for >= 6 of 12 and expects {pw['shrunk'][0]['mean_n_sig']:.1f} significant curves, which matches the observed 5 or 6: the study sat on the boundary because its power for the 6-of-12 rule was about one third. **{n80['shrunk']} rows per model** (for example 16 targets x 6 rounds) reaches 80% under the shrunk truth; {n80['observed']} under the optimistic one. The null scenario never reaches 6 (mean {pw['null'][0]['mean_n_sig']:.2f} false positives of 12), so the rule keeps its size. A follow-up could also fix H (section 9) beforehand and test the yield prediction directly, which needs far fewer rows.")
A("")
A("![fig9](figures/fig9_power.png)")
A("")
A("*fig9: power of the preregistered H-M rule against rows per model. Source `results_deepdive.json`, `H-M_power`.*")
A("")
# ---------------- limits
A("## Limits")
A("")
A("- Everything here is exploratory and post hoc; the analyses were chosen after the frozen results were known, several tests are uncorrected across this document, and no number changes a verdict. The H-M pooled test and the yield mechanism in particular are hypotheses for a follow-up.")
A("- Section 2: the additive part and the canonicalisation proxy are read from recorded counters; their meaning (setup, or something else) is not derivable from them. Canonicalisations are assumed to count walk iterations.")
A("- Section 3: per-vertex SEs for Q come from percentile CI widths and are approximate; 8 targets per vertex.")
A("- Section 4: the extrapolation is a model with stated assumptions; the existence and path-finding cost of deep j = 0 volcanoes at cryptographic size are not modelled.")
A("- Section 7: permutation p-values have Monte Carlo error (stated); the frozen verdict was computed with one stream and is kept.")
A("- Section 9: H is a derived predictor with weights set by counting elements, chosen after the relation counters were seen; the factor base is rebuilt in Python (checked against the 96 recorded scan counts) and is not an ecbench measurement; relation counters come from the M archives only.")
A("- Section 10: Q1's rows per model are small (8 to 56); the pooled F uses all 648 vertices and the 2,000-shuffle permutation has a floor of 5e-4.")
A("- Section 11: the simulation resamples the 64 observed rows per model, so it cannot be heavier-tailed than the data; it assumes the 12 curves are representative.")
A("- One shared VM; validator running concurrently; seeds fixed at 20261106.")
A("")
A("## Files")
A("")
A("`deepdive.py` (reads only committed inputs, writes `results_deepdive.json` with the sha256 of every input), `build_md.py` (lays this file out from the JSON), `build_pdf.sh`, `figures/` (SVG + PNG), `DEEPDIVE.md`, `deepdive.pdf`. Copies of the PDF, SVGs and PNGs are in `benchmarks/20261106-volcano-HT-HM-deepdive/` (no `benchmarks/README.md` row: exploratory, no validation). Run `python3 deepdive.py` (about 8 minutes on 4 vCPUs), then `python3 build_md.py` and `./build_pdf.sh`.")
txt = "\n".join(md) + "\n"
txt = re.sub(r"!\[(fig\d+)\]\((.*?)\)\n\n\*(.*?)\*", lambda m: f"![{m.group(3)}]({m.group(2)})", txt)
open(f"{HERE}/DEEPDIVE.md", "w").write(txt)
print("wrote DEEPDIVE.md", len(txt))
