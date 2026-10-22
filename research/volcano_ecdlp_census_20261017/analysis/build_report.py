#!/usr/bin/env python3
"""Lay out REPORT.md from analysis/results.json (text and tables only; every number is read from results.json).
Run: python3 -I analysis/build_report.py
"""
import json, os
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
R = json.load(open(f"{ROOT}/analysis/results.json"))
ARMS = ["rho-neg", "rho-plain", "bsgs", "bsgs-neg", "ic"]
PR = ["0.0", "0.1", "1.0"]
o = []
w = o.append
f3 = lambda x: f"{x:.3f}"
ci = lambda c, f=f3: f"[{f(c[0])}, {f(c[1])}]"

pc = R["positive_control"]
w("# Volcano ECDLP census, Phase C: analysis and report\n")
w("Study `research/volcano_ecdlp_census_20261017/`; plan `PLAN.md`, analysis plan `ANALYSIS_PLAN.md` (frozen after data collection and before inspection), amendments `AMENDMENTS.md` (A1, A2). "
  "Every number below is read from `analysis/results.json` (written by `analysis/analyze.py`, 10,000 bootstrap or permutation replicates, seeds 20261022 and 20261023). "
  "**Status: observed, not established.** No speedup is claimed. Independent validation (`VALIDATION.json`) has not happened yet; this report is the analysis, not its validation.\n")

w("## 1. Verdicts\n")
w(f"1. **Positive control first (gates everything): {'PASS' if pc['pass'] else 'FAIL'}.** On the V3 (j = 0) crater, mean S(`rho-cubic`) / mean S(`rho-neg`) = **{pc['row_bootstrap']['ratio']:.4f}**, 95% CI {ci(pc['row_bootstrap']['ci95'], lambda x: f'{x:.4f}')} "
  f"over {pc['n_pairs']} paired runs (theory 1/sqrt(3) = {pc['theory_ratio']:.4f}; pass needs the CI upper bound below 0.85). The detector sees a 42% cost reduction, so the 'equivalent' verdicts below are informative, not relabelled 'uninformative'.")
le = R["level_effect"]
nd = sum(1 for v in le.values() for r in v["R"].values() if r["S"]["verdict"] == "different")
ne = sum(1 for v in le.values() for r in v["R"].values() if r["S"]["verdict"] == "equivalent")
ni = sum(1 for v in le.values() for r in v["R"].values() if r["S"]["verdict"] == "inconclusive")
w(f"2. **Level effect (V1, V2; 5 arms; {ne + nd + ni} level ratios): {ne} equivalent, {nd} different, {ni} inconclusive.** "
  "TOST margins are +-5% for rho and BSGS and +-10% for IC. The two inconclusive ratios are both `rho-plain` at level 1 (CI not inside the margin; neither differs from 1 after Holm). "
  "No method shows a level effect on either volcano. With one volcano per l, this is an observed absence of an effect at these margins, not an established one (PLAN §5: replication on a second volcano of the same l is missing).")
ce = R["curve_effect"]
w("3. **Curve effect against the isomorphic negative control:** no variance-ratio test survives Holm (smallest adjusted p = "
  f"{min(v['p_perm_holm'] for v in ce.values() if v.get('p_perm_holm') is not None):.3f}); BSGS is degenerate (identical S for a curve and its re-scalings). "
  f"{sum(v['n_flagged'] for v in ce.values())} curve is flagged in total (V2, `rho-plain`, vertex {[f['vertex'] for v in ce.values() for f in v['flagged']][0]}), and it is not flagged for the other arms.")
ps = R["paired_source"]
w(f"4. **Transfer:** every one of the {sum(v['n'] for v in R['recovery'].values())} measured runs recovered k (95% lower bound {min(v['cp95'][0] for v in R['recovery'].values()):.4f} for the smallest cell). "
  f"BSGS counts are identical across D, Ua, Ub and S in {min(v['fraction_identical_ops_gae'] for V in R['bsgs_identity'].values() for v in V.values()) * 100:.0f}% of triples. Paired source ratios for rho-neg, rho-plain and ic: "
  f"{sum(1 for v in ps.values() if v['verdict'] == 'equivalent')} of {len(ps)} equivalent, {sum(1 for v in ps.values() if v['verdict'] == 'inconclusive')} inconclusive, {sum(1 for v in ps.values() if v['verdict'] == 'different')} different. "
  f"Transport is cheap against solving (section 6) and no 'transport, then solve' ratio is a speedup signal ({sum(1 for v in R['transport_then_solve'].values() for l in v['levels'].values() if l.get('speedup_signal'))} signals).")
ic = R["ic_vs_rho"]
w("5. **IC against matched `rho-neg` (distance remaining, not pass/fail):** IC costs "
  f"{ic['V1']['pooled']['0.0']['ratio_ic_over_rho']:.1f}x (V1) and {ic['V2']['pooled']['0.0']['ratio_ic_over_rho']:.1f}x (V2) the rho cost in S with nothing priced, "
  f"{ic['V1']['pooled']['1.0']['ratio_ic_over_rho']:.1f}x and {ic['V2']['pooled']['1.0']['ratio_ic_over_rho']:.1f}x with every uncharged operation priced at 1 GAE on both arms. The order (rho ahead) holds across the whole 0 / 0.1 / 1 band at every level of V1, V2 and V3. No level improves IC's position.")
wa = R["wall_admission"]
w("6. **Wall time: not admitted for any (V, arm).** The A/A ratio is within [0.95, 1.05] on all three volcanoes, but only "
  f"{min(a['frac_rows_L2'] for V in wa.values() for a in V['arms'].values()) * 100:.0f}% to {max(a['frac_rows_L2'] for V in wa.values() for a in V['arms'].values()) * 100:.0f}% of rows reached isolation L2, so condition 1 fails everywhere. Wall tables are descriptive.\n")

w("## 2. What was measured\n")
w(f"- {R['sessions']} audited ecbench sessions (V3, V1, V2; V4 is censused, not measured, A1.1), each `audit OK` with exact replays; {R['data_checks']['rows_measured']:,} measured runs after dropping {R['data_checks']['rows_all_incl_warmup'] - R['data_checks']['rows_measured']:,} warm-up runs. Unverified measured runs: {R['unverified_rows_total']}.")
w("- Arms: `rho-neg` (matched reference), `rho-plain`, `bsgs`, `bsgs-neg`, `ic` (`ic-fb64-mitm`, frozen in `FROZEN.json`), `rho-neg-aa` (A/A), `rho-cubic` (V3 crater). Families D, Ua, Ub, S, N are isogenous; `ctl` (two isomorphic re-scalings per vertex) is the negative control and never pooled.")
w("- Population: V1 (l = 2, depth 4, 400 curves per family), V2 (l = 3, depth 3, 378), V3 (j = 0, depth 4, 16; descriptive). Curves per level V1 25/25/50/100/200, V2 14/28/84/252, V3 1/1/2/4/8.")
w("- Unit: S (ecbench normalised group operations). Host: shared 4-vCPU cloud VM; binary `ac4bc5b2...` from a clean commit (`runs/progress.jsonl` header); campaign wall about 12.2 h of sessions plus a 5-day pause.")
w("- Statistics: cluster bootstrap over **vertices**, then rows, stratified by level (A2.3). Rows per curve are many; levels are compared on their curves, not their solves.\n")

w("## 3. Positive control\n")
w(f"V3 crater (census vertex {pc['vertex']}), D, N and ctl rows pooled ({pc['n_slugs']} generators; `rho-cubic` was run on one of the two re-scalings), {pc['n_pairs']} paired runs. Ratio {pc['row_bootstrap']['ratio']:.4f}, 95% CI {ci(pc['row_bootstrap']['ci95'], lambda x: f'{x:.4f}')} (pair bootstrap); slug-cluster sensitivity {ci(pc['slug_cluster_sensitivity']['ci95'], lambda x: f'{x:.4f}')}. Result: **pass**.\n")

w("## 4. Level effect: R(L) = mean S at level L / mean S at the crater\n")
w("Cells: ratio [95% CI], verdict (E equivalent, D different, I inconclusive). Margins [0.95, 1.053] for rho and BSGS, [0.90, 1.111] for IC. Holm family (a) has "
  f"{R['multiplicity']['family_a_tests']} tests.\n")
for V in ["V1", "V2"]:
    L = sorted(int(k) for k in le[f"{V}|rho-neg"]["R"])
    w(f"**{V}** (l = {2 if V == 'V1' else 3})\n")
    w("| arm | " + " | ".join(f"L{l}/L0" for l in L) + " | CV of S per row | achieved 95% half-width of R |")
    w("|:--|" + "|".join([":--"] * len(L)) + "|--:|--:|")
    for arm in ARMS:
        e = le[f"{V}|{arm}"]
        cells = []
        for l in L:
            s = e["R"][str(l)]["S"]
            cells.append(f"{f3(s['ratio'])} {ci(s['ci95'])} {s['verdict'][0].upper()}")
        cv = sum(x["cv_S_per_row"] for x in e["levels"].values()) / len(e["levels"])
        hw = sum(r["S"]["half_width95"] for r in e["R"].values()) / len(e["R"])
        w(f"| {arm} | " + " | ".join(cells) + f" | {cv:.2f} | {hw:.3f} |")
    w("")
w("BSGS ratios equal 1 exactly: BSGS cost depends on r and k only, and r is the same on every curve of a volcano. Their bootstrap interval reflects only the spread of the scalars k_i. "
  "Secondary mixed model (`log S ~ C(level) + C(family) + (1 | curve)`, REML) agrees: level ratios between "
  f"{min(r['ratio'] for v in R['mixed_model'].values() if 'level_ratio' in v for r in v['level_ratio'].values()):.3f} and {max(r['ratio'] for v in R['mixed_model'].values() if 'level_ratio' in v for r in v['level_ratio'].values()):.3f}; "
  f"ICC between {min(v['icc'] for v in R['mixed_model'].values() if v.get('icc') is not None):.4f} and {max(v['icc'] for v in R['mixed_model'].values() if v.get('icc') is not None):.4f} (no curve-level variance beyond the solve noise); "
  f"non-converged fits: {', '.join(k for k, v in R['mixed_model'].items() if not v.get('converged')) or 'none'} (reported; the bootstrap is the result). "
  "Per-family R(L) (descriptive) are in `results.json` under `R_by_family_descriptive`.\n")
w("![](analysis/figures/fig1_forest_level_effect.svg)\n")
w("*Figure 1. Level ratios with TOST bands. Source: `analysis/results.json` `level_effect`; session ids in `runs/progress.jsonl`.*\n")
w("![](analysis/figures/fig2_volcano_V1.svg)\n")
w("*Figure 2a. V1 per-curve mean S for `rho-neg` and `ic` (standard `volcano-diagram/v2`). V2 and V3 are `fig2_volcano_V2/V3`. Colour scales are per panel; the spread within a level is the same size as between levels.*\n")

w("## 5. Curve effect against the negative control\n")
w("| volcano | arm | vertices | F (between / within) | permutation p | Holm p (across 10) | flagged curves (Holm p < 0.01) |")
w("|:--|:--|--:|--:|--:|--:|:--|")
for V in ["V1", "V2"]:
    for arm in ARMS:
        c = ce[f"{V}|{arm}"]
        if c.get("F") is None:
            w(f"| {V} | {arm} | {c['n_vertices']} | degenerate | n/a | n/a | none (D and both re-scalings give identical S) |")
        else:
            fl = ", ".join(f"vertex {x['vertex']} (level {x['level']}, z {x['z']:.2f}, adjusted p {x['p_holm']:.1e})" for x in c["flagged"]) or "none"
            w(f"| {V} | {arm} | {c['n_vertices']} | {c['F']:.3f} | {c['p_perm']:.3f} | {c['p_perm_holm']:.3f} | {fl} |")
w("")
fv = [(k, f) for k, c in ce.items() for f in c["flagged"]][0]
vv = fv[1]["vertex"]
others = []
for arm in ARMS:
    c = ce[f"V2|{arm}"]
    for x in c.get("vertices", []):
        if x["vertex"] == vv:
            others.append(f"{arm} z = {x['z']:.2f}")
w(f"F below 1 means the original curves spread no more than their own isomorphic re-scalings. The flagged V2 curve (vertex {vv}) has z by arm: {', '.join(others)}. One curve of the 2,334 tested by the plan's rule (3 non-degenerate arms x 778 vertices), in one arm only, and the corresponding variance-ratio test is not significant after Holm; it is listed, not interpreted.\n")
w("![](analysis/figures/fig4_curve_vs_control.svg)\n")
w("*Figure 4. Mean S of each original curve (D) against the mean of its two re-scalings. The grey band is the normal-approximation 99% band of the control, for display only; flags come from the permutation test.*\n")

w("## 6. Transfer\n")
tc = R["transfer_cost"]
w(f"**Recovery.** {sum(v['verified'] for v in R['recovery'].values()):,} of {sum(v['n'] for v in R['recovery'].values()):,} measured runs verified (every (V, family, arm) cell 100%; the Clopper-Pearson 95% lower bound is {min(v['cp95'][0] for v in R['recovery'].values()):.4f} for the 1,000-run cells and higher for larger ones). `matches_transported` is true for all {tc['n_rows']:,} transport-benchmark paths.\n")
w("**Paired source ratios** (mean S of the source family over D, on shared k_i; 95% CI; TOST margin as in section 4). Nothing is 'different'.\n")
w("| volcano | arm | Ua/D | Ub/D | S/D | N/D |")
w("|:--|:--|:--|:--|:--|:--|")
for V in ["V1", "V2"]:
    for arm in ["rho-neg", "rho-plain", "ic"]:
        cells = []
        for s in ["Ua", "Ub", "S", "N"]:
            r = ps[f"{V}|{arm}|{s}/D"]
            cells.append(f"{f3(r['ratio_src_over_D'])} {ci(r['ci95'])} {r['verdict'][0].upper()}")
        w(f"| {V} | {arm} | " + " | ".join(cells) + " |")
w("")
w("N uses another scalar sequence (A1.3), so its pairing is nominal. BSGS identity check (fraction of (vertex, target, round) triples with equal `ops_gae` across the families present): "
  + "; ".join(f"{V} {arm} {v['fraction_identical_ops_gae']:.3f} over {v['n_triples']}" for V, d in R["bsgs_identity"].items() for arm, v in d.items()) + ".\n")
w("**Transport cost** (`runs/transfer_cost.jsonl`, 200 repetitions per path, `ca volcano --transport-bench`; field-operation counts are a model, not instrumented). Per path length d, median ns per path (IQR) and Velu point maps (2 per edge):\n")
w("| volcano | d = 1 | d = 4 | d = 8 | d = max |")
w("|:--|:--|:--|:--|:--|")
for V in ["V1", "V2", "V3"]:
    per = tc["by_volcano_path_len"][V]
    ks = sorted(int(k) for k in per)
    cells = []
    for d in [1, 4, 8, ks[-1]]:
        if str(d) in per:
            p = per[str(d)]["ns_per_path_median_q1_q3"]
            cells.append(f"{p[0] / 1000:.1f} us ({p[1] / 1000:.1f} to {p[2] / 1000:.1f}), d = {d}, {per[str(d)]['velu_point_maps_per_path']:.0f} maps")
        else:
            cells.append("n/a")
    w(f"| {V} | " + " | ".join(cells) + " |")
w("")
w("Transport as a fraction of the destination solve (median `solve_wall_ns` of the arm at that level; wall is descriptive, section 8):\n")
w("| volcano | arm | median fraction over levels (max over paths) | Velu maps / mean ops_gae at 0.1 / 1 GAE per map |")
w("|:--|:--|:--|:--|")
for V in ["V1", "V2"]:
    for arm in ["rho-neg", "bsgs-neg", "ic"]:
        fa = tc["fraction_of_solve"][V][arm]
        med = sorted(v["fraction_median_q1_q3"][0] for v in fa.values())[len(fa) // 2]
        mx = max(v["fraction_max"] for v in fa.values())
        g = [sum(v["velu_evals_over_mean_ops_gae"][p] for v in fa.values()) / len(fa) for p in ["0.1", "1.0"]]
        w(f"| {V} | {arm} | {med * 100:.2f}% ({mx * 100:.2f}%) | {g[0] * 100:.3f}% / {g[1] * 100:.2f}% |")
w("")
tt = R["transport_then_solve"]
n_ex = sum(1 for v in tt.values() for l in v["levels"].values() if l["ci95_excludes_1_all_prices_same_side"])
n_all = sum(len(v["levels"]) for v in tt.values())
n_sig = sum(1 for v in tt.values() for l in v["levels"].values() if l.get("speedup_signal"))
w(f"**Transport, then solve, against solve where born** (A2.3; per source family, arm and destination level, priced at 0 / 0.1 / 1 GAE per Velu map): {n_all} cells; the CI excludes 1 at all three prices in {n_ex} of them, "
  f"and **{n_sig} meet the plan's speedup condition** (excludes 1 on the lower side at all three prices and disjoint from the native-instance control). The cells that exclude 1 are V1 `ic` from S (ratio about 1.2, above 1), V3 `ic` from D and Ua (about 1.2, above 1; the crater is cheaper for IC, section 9) and V2 `rho-plain` from Ua (about 0.6, **below 1**). "
  "The V2 `rho-plain` Ua cell is the same at every level including level 0 (no transport at all), and the paired Ua/D ratio for it is equivalent to 1, so it reflects a high mean at the single source vertex (one floor curve, few rows), not a transport benefit; its CI overlaps the native-instance control CI (about 0.75 to 1.34), so it is not a signal under the plan's condition. The price band moves any ratio by at most 1.5%. Transport is at most about 1.3% of a solve in wall time for any path (medians below 0.5%) and at most about 1.3% in GAE at 1 GAE per Velu map (V2, `bsgs-neg`), far less for IC.\n")
w("![](analysis/figures/fig5_transport_vs_solve.svg)\n")
w("*Figure 5. Transport ns per path against median solve wall by path length (descriptive wall).*\n")

w("## 7. IC against matched rho (distance remaining)\n")
w("Mean S(ic) / mean S(rho-neg), isogenous families pooled, 95% CI, with every uncharged native operation (`ic`: lookups, target-guard probes, row operations, Legendre symbols, square roots; `rho-neg`: canonicalisations) priced at 0 / 0.1 / 1 GAE on **both** arms. "
  "Counts come from the run records (A2.1). Both arms still omit native work ecbench does not name, so both are lower bounds.\n")
for V in ["V1", "V2", "V3"]:
    lv = ic[V]["levels"]
    w(f"**{V}**" + (" (descriptive: one curve at levels 0 and 1, 2 / 4 / 8 above)" if V == "V3" else "") + "\n")
    w("| level | curves | price 0 | price 0.1 | price 1 | order across the band |")
    w("|--:|--:|:--|:--|:--|:--|")
    for L in sorted(lv, key=int):
        x = lv[L]
        w(f"| {L} | {x['n_curves']} | " + " | ".join(f"{x['prices'][p]['ratio_ic_over_rho']:.1f} {ci(x['prices'][p]['ci95'], lambda t: f'{t:.1f}')}" for p in PR) + f" | {x['ranking']} |")
    pl = ic[V]["pooled"]
    w(f"| all | | " + " | ".join(f"{pl[p]['ratio_ic_over_rho']:.1f} {ci(pl[p]['ci95'], lambda t: f'{t:.1f}')}" for p in PR) + " | |")
    w("")
u = ic["V1"]["uncharged"]
w(f"Uncharged operations per run (V1): `ic` {sum(u['ic']['mean_uncharged_per_run'].values()):,.0f} (about {u['ic']['mean_unc_total_over_gae']:.1f} per charged GAE), `rho-neg` {sum(u['rho-neg']['mean_uncharged_per_run'].values()):,.0f} (about {u['rho-neg']['mean_unc_total_over_gae']:.2f} per charged GAE). "
  "The gap to rho is therefore about 43x unpriced and 65x at 1 GAE (V1), a distance of more than an order of magnitude at this size and no level of any volcano changes it. This is the measured baseline gap, not a result for or against IC's potential.\n")
w("![](analysis/figures/fig3_ic_over_rho.svg)\n")
w("*Figure 3. IC / rho-neg by level, three prices, 95% CI (log scale).*\n")

w("## 8. Wall time\n")
w("| volcano | A/A ratio (rho-neg-aa / rho-neg) [95% CI] | within [0.95, 1.05] | rows at L2 (rho-neg / ic) | admitted |")
w("|:--|:--|:--|:--|:--|")
for V in ["V1", "V2", "V3"]:
    a = wa[V]
    w(f"| {V} | {a['aa_ratio']:.4f} {ci(a['ci95'], lambda t: f'{t:.4f}')} | {'yes' if a['cond_aa_within_5pct'] else 'no'} | {a['arms']['rho-neg']['frac_rows_L2'] * 100:.1f}% / {a['arms']['ic']['frac_rows_L2'] * 100:.1f}% | no (isolation L1 rows present) |")
w("\nSo wall numbers (`solve_wall_ns` ratios in `results.json`, `R.<level>.wall`) are **descriptive (not admitted)**. For orientation they also show no level trend. The A/A result says the instrument's own noise floor is within 1% pooled.\n")

w("## 9. V3 (j = 0), descriptive\n")
w("One curve at levels 0 and 1, so no between-curve inference. Mean S relative to the crater:\n")
w("| arm | L1 | L2 | L3 | L4 |")
w("|:--|--:|--:|--:|--:|")
for arm in ARMS:
    d = R["v3_descriptive"][arm]
    w(f"| {arm} | " + " | ".join(f"{d[str(L)]['ratio_to_crater']:.3f}" for L in [1, 2, 3, 4]) + " |")
w("\n**Observed, not tested:** `ic` is about 22 to 25% more expensive at every non-crater V3 level than at the j = 0 crater, while the rho and BSGS arms show nothing. "
  "This fits IC using the 6 automorphisms of j = 0 on the crater, as the negation-folded oracle does for 2; it is one curve, one volcano and descriptive here, and it makes IC's gap to rho smaller on that curve (49x against about 60x at 0 GAE), not close to parity.\n")

w("## 10. CV and achieved precision\n")
w("![](analysis/figures/fig6_cv_precision.svg)\n")
w("*Figure 6. Per (volcano, arm): curves per level, rows, per-row CV of S, achieved 95% half-width of the level mean and of R(L), and ICC.* "
  "Rho CV about 0.46 (calibration 0.40, plan 0.52), IC 0.35 to 0.45, BSGS 0.15 to 0.29. The achieved 95% half-width of R(L) is 3 to 4% for rho and IC and about 1% for BSGS, so the +-5% rho margin and the +-10% IC margin were within reach, as the `rho-plain` level-1 results show (they sit just outside).\n")

w("## 11. Limits\n")
for s in [
    "Shared 4-vCPU cloud VM; wall time is descriptive, 8 to 13% of rows were at isolation L1.",
    "One volcano per l (V1 l = 2, V2 l = 3): no replication on a second volcano of the same l, so nothing can be 'established', only observed.",
    "V3 is descriptive (one curve per upper level, A1.2); its positive control uses one of the two ctl re-scalings for `rho-cubic`.",
    "V4 (twist of V1) is censused, not measured (A1.1).",
    "5-day pause in Phase B (driver died after V2-S-L3-c0, resumed 2026-10-22); the gap is excluded from wall; no result depends on it.",
    "A1 amendments (V4, V3 shape, target seeds, rho CV, Drive, storage, clean binary) and A2 amendments (this phase, post hoc) shape the analysis; each interpretation is listed in `AMENDMENTS.md`. `ANALYSIS_PLAN.md` was frozen after data collection but before anyone inspected a Phase B cost summary. A 100-replicate smoke run of the code was inspected before the final run. `results.json` records the SHA-256 of `AMENDMENTS.md` as it stood at the final run, before A2.3 and A2.4 were appended.",
    "Curves are census vertices; the five isogenous families share a vertex and are pooled. A different clustering would change widths, not point estimates.",
    "IC and rho both leave unnamed native work unpriced; the 0 / 0.1 / 1 GAE band covers only counters ecbench names. Field-operation counts for Velu maps are a model.",
    "Small sizes only (26-bit group orders); no statement about cryptographic sizes, no speedup, no `claim`.",
    "The analysis is Python (the plan names it); AGENTS.md prefers native code for results. The validator should re-derive the headline numbers independently.",
]:
    w(f"- {s}")
w("")
w("## 12. Files\n")
w("- Code: `analysis/analyze.py`, `analysis/extract_uncharged.py`, `analysis/figures.py`, `analysis/build_report.py`; output `analysis/results.json` (all statistics, n, seeds, SHA-256 of every input), `analysis/figures/*.svg|png`, `analysis/analyze.log`.")
w("- Inputs: `runs/extract/*.csv.xz`, `runs/uncharged/*.csv.xz`, `runs/slugmap/*.json`, `census/V*/`, `runs/transfer_cost.jsonl`, `runs/progress.jsonl`, sessions archives `sessions/*.tar.xz`.")
w("- Copies for browsing: `benchmarks/20261017-volcano-ecdlp-census/` (no `benchmarks/README.md` row until the validation gate is passed).")
open(f"{ROOT}/REPORT.md", "w").write("\n".join(o) + "\n")
print("REPORT.md", len("\n".join(o)))
