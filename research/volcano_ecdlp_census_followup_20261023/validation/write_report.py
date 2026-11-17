#!/usr/bin/env python3
"""Write validator-report.json and VALIDATOR_REPORT.md (follow-up study) from
the validator's outputs (build_validation.py -> build_parts.json, recompute.json,
hm_precise.json, extras.json, report_check.json) and the replay certificate.
Validator-written. Usage:
python3 -I write_report.py STUDY_DIR SCRATCH_DIR BENCH_DIR SNAPSHOT_SHA"""
import json
import sys

STUDY, SCR, BENCH, SNAP = sys.argv[1:5]
VAL_ID = "VAL-20261106-54796a"
J = lambda p: json.load(open(p))  # noqa: E731
res = J(f"{STUDY}/analysis/results.json")
me = J(f"{SCR}/recompute.json")
hmp = J(f"{SCR}/hm_precise.json")
ex = J(f"{SCR}/extras.json")
rchk = J(f"{SCR}/report_check.json")
parts = J(f"{SCR}/build_parts.json")
cert = J(f"{BENCH}/replay-certificate.json")
summ = parts["summary"]
tot = cert["totals"]
rr = cert["raw_record_rederivation"]

hm98p = res["H-M"]["curves"]["V6:98"]["arms"]["ic"]
hm98m = me["H-M"]["curves"]["V6:98"]["ic"]
hp98 = hmp["V6:98"]
le = res["level_effect"]
lm = me["level_effect"]
ce = res["curve_effect"]["V6|rho-neg"]
z545 = max(ce["vertices"], key=lambda x: abs(x["z"]))
w = ex["wall"]


def f(x, d=4):
    return f"{x:.{d}f}"


findings = [
    {"id": "F1", "severity": "material", "status": "open",
     "where": "REPORT.md s1 item 5 and s8 (and the same text in report.typ / report.pdf)",
     "claim": ("'H-M ...: INCONCLUSIVE. ic is significant (Holm, alpha 0.05) on 5 of 12 chosen curves ... IC's model dependence is "
               "therefore present on some curves and absent on others: it is neither a general property nor noise.' and s8 'With 5 of "
               "12 the verdict is inconclusive by the plan's rule, which needed 6.'"),
     "evidence": (f"The verdict turns on one curve that sits on the Monte Carlo boundary. V6:98 ic: F = {hm98p['F']:.4f} (validator "
                  f"{hm98m['F']:.4f}, equal to 1e-15); it is the 6th-smallest p of 12, so its Holm multiplier is 7. Producer, 10,000 "
                  f"shuffles: p = {hm98p['p_perm']:.4f}, Holm {hm98p['p_holm']:.4f} -> not significant -> 5 of 12 -> 'inconclusive'. "
                  f"Validator, independent code, same frozen seed (20261024) and count (10,000), different draw order: p = "
                  f"{hm98m['p_perm']:.4f}, Holm {hm98m['p_holm']:.4f} -> significant -> 6 of 12 with rho-neg 0 and bsgs 0, which the "
                  f"frozen rule (ANALYSIS_PLAN s5c) reads as 'supported for IC'. The same test evaluated to high precision "
                  f"(validation/hm_precise.py, 2,000,000 shuffles): p = {hp98['p_perm']:.6f} +- {hp98['p_perm_mc_sd']:.6f} (MC SE), "
                  f"Holm {hp98['p_perm_holm']:.4f} +- {7 * hp98['p_perm_mc_sd']:.4f} -> 6 of 12; the parametric one-way ANOVA F(7, 504) "
                  f"gives p = {hp98['p_anova']:.5f}, Holm {hp98['p_anova_holm']:.4f} -> 6 of 12. Both 10,000-shuffle Holm values agree "
                  "within Monte Carlo error (the numbers are not wrong); the categorical verdict does not reproduce. The report states "
                  "'inconclusive' as a settled outcome and interprets it ('absent on others', 'neither a general property nor noise'); "
                  "a curve that is not significant after Holm is not shown to lack a model effect, and the five-versus-six split is "
                  "within Monte Carlo noise of the frozen threshold."),
     "fix": ("Erratum (a new AMENDMENTS record, REPORT.md and report.pdf; the frozen rule, seed and shuffle count are not changed): "
             "report the frozen-seed count (5 of 12, inconclusive) together with the fact that the 6th curve (V6:98, Holm p 0.0511) "
             "lies within Monte Carlo error of 0.05 and that a precise evaluation of the same permutation test (and the parametric F "
             "test) gives 6 of 12, which the rule reads as supported for IC; state that the H-M verdict is on the boundary between "
             "'inconclusive' and 'supported for IC' and is not robust either way; withdraw 'absent on others' and 'neither a general "
             "property nor noise'. The figure 8 caption / s8 table can keep the frozen-seed values.")},
    {"id": "F2", "severity": "minor", "status": "open",
     "where": "REPORT.md s1 item 2 heading",
     "claim": "'Q1, V6 ... level effect: no method depends on level.'",
     "evidence": (f"1 of 20 V6 cells is 'inconclusive' (rho-neg L2: 90% CI upper {le['V6|rho-neg']['R']['2']['S']['ci90'][1]:.4f} "
                  f"against the margin 1.0526; validator {lm['V6|rho-neg|2']['ci90'][1]:.4f}), so equivalence is not shown for every "
                  f"(arm, level). Two V6 TOST cells sit within about 2 Monte Carlo SE of the margin and were reproduced by the "
                  f"validator's independent bootstrap on the same side: rho-neg L2 (above) and rho-plain L1 (90% lower "
                  f"{le['V6|rho-plain']['R']['1']['S']['ci90'][0]:.4f}, validator {lm['V6|rho-plain|1']['ci90'][0]:.4f}, margin 0.95)."),
     "fix": "Heading: '19 of 20 equivalent, 1 inconclusive, none different' (the sentence that follows already says so); optionally note the two near-margin cells."},
    {"id": "F3", "severity": "minor", "status": "open",
     "where": "REPORT.md s1 item 4 (last sentence) and s12 (H-T bullet)",
     "claim": ("'it is relative to rho-neg on the same instance, not to the strongest rho at the source' / 'it does not compare with "
               "the strongest rho at the source (on j != 0 vertices rho-neg is the strongest)'"),
     "evidence": ("Every one of the 63 Tcr source vertices has j not in {0, 1728} (validator recomputed j(a, b) for all 64 V5 vertices; "
                  "only the crater has j = 0) and the field is prime, so Aut = {+-1} and rho-neg is the strongest available rho on every "
                  "source curve: H-T's denominator is the matched, strongest rho at the source. The two sentences contradict each "
                  "other and the first understates the comparison."),
     "fix": "Wording: 'relative to rho-neg at the source, which is the strongest rho available on every (j != 0) source curve'."},
    {"id": "F4", "severity": "minor", "status": "open",
     "where": "REPORT.md s1 item 7 and s10; results.json wall_admission",
     "claim": "A/A ratio V5 1.042, V6 1.001; 'only 87% (V5) and 84% (V6) of rows reached isolation L2'.",
     "evidence": (f"The A/A ratio and the per-arm L2 fractions use the isogenous families only (pairs {w['V5|iso']['n_pairs']} / "
                  f"{w['V6|iso']['n_pairs']}), while ANALYSIS_PLAN s8 lists D, Ua, Ub, S, N and ctl as the families that enter. With "
                  f"ctl: A/A {w['V5|iso+ctl']['aa_ratio']:.4f} (V5, {w['V5|iso+ctl']['n_pairs']} pairs) and "
                  f"{w['V6|iso+ctl']['aa_ratio']:.4f} (V6, {w['V6|iso+ctl']['n_pairs']}), still inside [0.95, 1.05]; per-arm L2 "
                  "fractions rise by 0.4-3.6 points and stay below 100%, so 'not admitted' holds either way. The 87% / 84% of s1 is over "
                  "all rows of the volcano (M and Tcr included), the s10 per-arm values over isogenous rows only; neither is stated."),
     "fix": "State the row set used (or follow s8's family list); no verdict changes."},
    {"id": "F5", "severity": "minor", "status": "open",
     "where": "AMENDMENTS.md A2.1",
     "claim": "'Between 05:41Z and 05:43Z, ecbench refused to start 9 short Tcr sessions'",
     "evidence": ("The 9 status 'failed' lines of runs/progress.jsonl are timestamped 05:32:32Z to 05:43:01Z (V5-Tcr-L5-c19 05:32:32, "
                  "V5-Tcr-L5-c27 05:36:10, V5-Tcr-L6-c45 05:40:01, then six between 05:41:11 and 05:43:01). All 9 carry the refusal "
                  "signature A2.1 names (rc_run 2, session_status 'missing', runs 0, wall 2-3 s); the refusal message itself is not "
                  "preserved (no session log contains 'not quiet'; the redone sessions' logs replaced them). All 9 specs later ran and "
                  "count once; no data is affected."),
     "fix": "A new amendment record correcting the time span (AMENDMENTS.md is append-only)."},
    {"id": "F6", "severity": "minor", "status": "open",
     "where": "REPORT.md s5 last sentence",
     "claim": "'No variance-ratio test is significant and no curve is flagged: no V6 curve differs from its own isomorphic re-scalings beyond the control spread'",
     "evidence": (f"With 648 curves and the pooled-|z*| reference of s4 (3 label outcomes per vertex) the smallest attainable p_v is "
                  f"about 1/1944 and the smallest attainable Holm-adjusted p about 648/1944 = 0.33, so no curve can reach adj. p < 0.01 "
                  f"whatever the data. results.json: the most extreme rho-neg curve (vertex {z545['vertex']}, level {z545['level']}, "
                  f"z = {z545['z']:.2f}) has Holm p {z545['p_holm']:.3f}. 'No curve is flagged' is true but uninformative; the claim "
                  "about curves rests on the aggregate F tests (p 0.15 / 0.67 / 0.42), which do support 'no curve effect beyond the "
                  "control spread' in aggregate. The design is frozen (and is the census's)."),
     "fix": "Say that the per-curve flag cannot fire at this size and that the statement rests on the F test."},
    {"id": "F7", "severity": "minor", "status": "open",
     "where": "REPORT.md s4 (V5 bsgs and bsgs-neg rows) and s1 item 3",
     "claim": "V5 bsgs R(L) = 0.988 and bsgs-neg R(L) = 0.982 at every level 1-6, listed without comment.",
     "evidence": ("A constant ratio at every level is a family-mix artifact, not a level effect: Ua and Ub have no V5 level-0 rows, so "
                  "the crater mean is D + N while every other level is D + Ua + Ub + N; BSGS cost depends only on k, D/Ua/Ub share k "
                  "(target seed 20261123) and N has its own (20261125), with mean S(N)/S(D) = 1.0484 (bsgs) and 1.0753 (bsgs-neg). "
                  "(3 + x) / (2 (1 + x)) = 0.98819 and 0.98186 reproduce the constants exactly. The same mix difference enters every "
                  "V5 arm's R(L) (negligibly for rho and ic). The census validation flagged the same pattern (census F8)."),
     "fix": "One explanatory sentence in s4."},
]
notes = [
    {"id": "N1", "severity": "methodological note", "status": "note",
     "where": "REPORT.md s8 table (max/min columns)",
     "text": ("The percentile bootstrap CI of the max/min of 8 model means is biased upward and does not cover the point estimate when "
              "there is no effect (bsgs, whose model means are identical: 1.000 with CI [1.03, 1.11]; rho-neg V6:2: 1.10 [1.11, 1.38]). "
              "These CIs cannot be read against 1; IC's max/min (1.12-1.58) must be read against rho-neg's on the same curves (1.10-1.26), "
              "the null object, as the report's sentence on rho-neg partly does.")},
    {"id": "N2", "severity": "cosmetic", "status": "note",
     "where": "ANALYSIS_PLAN.md s10",
     "text": ("The frozen plan names the benchmarks copy benchmarks/20261023-volcano-ecdlp-census-followup/; the copy is in "
              "benchmarks/20261023-volcano-census-followup/ (the directory this validation writes to). Contents identical to "
              "research/.../report.pdf and analysis/figures (sha256 checked).")},
    {"id": "N3", "severity": "note", "status": "note",
     "where": "H-T (s5b, A3.2)",
     "text": (f"The vertex-weighted pooling (A3.2) gives T = {res['H-T']['overall']['T']['0.0']['value']:.4f} / "
              f"{res['H-T']['overall']['T']['1.0']['value']:.4f} (0 / 1 GAE); the row-pooled alternative gives "
              f"{me['H-T']['row_pooled_sensitivity']['T|0.0']:.4f} / {me['H-T']['row_pooled_sensitivity']['T|1.0']:.4f}. The verdict "
              "(supported) does not depend on the choice; the T0 control's 95% CI includes 1 at all three prices with its upper end "
              "about 0.014 above 1, many Monte Carlo SE away.")},
]

rep = {
    "schema": "validation-gate/validator-report/v1",
    "status": "fail",
    "independent": True,
    "study": "research/volcano_ecdlp_census_followup_20261023",
    "pr": "isogeny-labs/crypto#3476",
    "branch": "claude/volcano-census-followup-20261023",
    "snapshot_commit": SNAP,
    "validation_id": VAL_ID,
    "round": 1,
    "status_reason": (f"{summ['agree']} of {summ['numbers']} recomputed numbers agree (point estimates to "
                      f"{summ['max_rel_diff_point_estimates']:.1e} relative; bootstrap endpoints within {summ['bootstrap_endpoints_max_mc_z']:.2f} "
                      "Monte Carlo SD); the replay sample verifies; but the H-M headline verdict does not reproduce: the validator's "
                      "independent 10,000-shuffle permutation test and a 2,000,000-shuffle evaluation both give ic significant on 6 of 12 "
                      "curves ('supported for IC' by the frozen rule) where the report states 5 of 12, 'INCONCLUSIVE', with an "
                      "interpretation beyond the verdict (F1, material). Minor items F2-F7."),
    "independence": {
        "validator": "independent validator of the follow-up study (did not write its plan, specs, sessions, driver, analysis or report)",
        "producer_code_read": False,
        "producer_code_not_opened": ["analysis/analyze.py", "analysis/extract_uncharged.py", "analysis/figures.py",
                                     "analysis/build_report.py", "analysis/build_pdf.sh"],
        "producer_derived_files_not_used_as_input": ["runs/slugmap/*", "analysis/results.json (claims only)"],
        "validator_code": "research/volcano_ecdlp_census_followup_20261023/validation/ (draw_sample.sh and replay_sample.sh adapted from the census validation; the Python is new)",
        "rng": "numpy PCG64 with the plan's seeds but the validator's own draw order; bootstrap and permutation values agree with the producer's only up to Monte Carlo error",
    },
    "raw_data": parts["raw"],
    "headline_summary": summ,
    "headline": parts["headline"],
    "report_vs_results": {
        "table_cells_checked": rchk["checked"], "equal_to_results_json_at_report_rounding": rchk["equal_results"],
        "differ_from_results_json": rchk["differ_results"], "differ_from_validator_recompute": rchk["differ_recompute"],
        "text_claims_checked": [
            "s1.1 positive control 0.6113 [0.5995, 0.6237], 4000 pairs, 3 models, slug-cluster [0.5967, 0.6288]: equal",
            "s1.2 V6 19 equivalent / 1 inconclusive / 0 different: equal (wording F2)",
            "s1.3 V5 L4-6 15 equivalent; L1-3 within -0.031..+0.011: equal",
            "s1.4 H-T T and T0 at 3 prices with CIs, 0.21% transport share, 0.6148 [0.5895, 0.6420]: equal (wording F3)",
            "s1.5 H-M 5 / 0 / 0 of 12: equal to results.json; not reproduced (F1)",
            "s1.6 IC/rho V6 60.4x / 90.8x, V5 L1-6 53.3x / 81.8x, crater rho-cubic 89.3 / 98.2 / 141.2: equal",
            "s1.7 A/A 1.042 / 1.001, L2 87% / 84%: equal (row sets, F4)",
            "s2 147 sessions by family, 428,976 measured, 241,712 warm-up, 0 unverified, p and curve counts, 8.94 h, 7 pauses: equal",
            "s4 mixed model convergence pattern and ICC 0.0050 / 0.0035: equal (statsmodels re-run)",
            "s6 recovery CP lower bound 0.9963 (n = 1000), BSGS identity 100%, 12/12 paired equivalent (points), ns/edge 3172 / 3669, "
            "2 Velu maps per edge, 2 of 100 transport-then-solve signal cells (V6|S|ic L1, L3) and their point ratios: equal",
            "s7 M_v 2..12, unpaired 0, path-check failures 0, T_v at 1 GAE 0.43..0.89, level-6 T0 0.931, level-1 0.956: equal",
            "s8 IC crater models 59.3 / 59.2 / 58.1, IC max/min up to 1.58, rho-neg 1.10-1.26: equal",
            "s9 M curves IC/rho-neg 51.8-65.2 (0 GAE), 79.5-98.4 (1 GAE): equal",
            "s12 7 restarts, 3 redone sessions, 3 orphans, 9 refusals: equal (A2.1 time span, F5)"],
        "report_pdf": "report.typ carries the same text as REPORT.md (key phrases checked); report.pdf identical in research/ and benchmarks/ (sha256 107ade10...); figures identical"},
    "verdict_vocabulary": {
        "follows_analysis_plan": True,
        "detail": ("Level ratios: equivalent / different / inconclusive, and 'descriptive' for V5 L1-3, with TOST margins [0.95, 1/0.95] "
                   "(rho, BSGS) and [0.90, 1/0.90] (ic) as frozen; Q2a pass/fail at 0.85; H-T supported / refuted / inconclusive; H-M "
                   "supported for IC / refuted / inconclusive; wall 'descriptive (not admitted)'; s5 'signal' with 'not a speedup claim'; "
                   "IC vs rho as distance remaining, ranked only where the order holds at all three prices. Departures are interpretive "
                   "sentences, not vocabulary: F1 ('absent on others', 'neither a general property nor noise') and F2 ('no method depends "
                   "on level').")},
    "amendment_audit": {
        "A1": {"committed": "2f80fe625f 2026-10-23T00:50:25Z, before the progress header (01:09:44Z) and the first session (01:15:29Z)",
               "changes_estimand_margin_family_test_or_threshold": False,
               "detail": ("Operational (disk guard, per-session push and prune, driver duties) plus declared interpretations before data: "
                          "(a) H-T in priced ops_gae instead of S fixes PLAN s4's mixed units; every V5 curve has the same r, so the "
                          "S ratio equals the ops_gae ratio and the estimand is unchanged; (b) joint pair resampling; (c) H-M degenerate "
                          "handling, alpha 0.05, p = (1 + #)/(1 + 10,000); (d) pooled = ratio of means, only the overall value decides.")},
        "A2": {"committed": "141d319090 2026-10-23T09:08:56Z, during Phase B; A1 text is an exact byte prefix",
               "changes_estimand_margin_family_test_or_threshold": False,
               "detail": "Busy-host refusals retried and kept as 'failed' records; orchestration; base merge. Time span wording: F5."},
        "A3": {"committed": "12e5eb3f7d 2026-11-06T03:26:29Z with the analysis (post hoc, labelled so); A1 + A2 text is an exact byte prefix",
               "changes_estimand_margin_family_test_or_threshold": False,
               "detail": ("Fills gaps the frozen plan leaves open (session naming, vertex-weighted H-T pooling as the plan's 'averaged over "
                          "the vertices', per-vertex CI scheme, closed-interval verdict arithmetic, Holm family of 35, per-family V5 R(L), "
                          "unpaired H-M permutation with default_rng(20261024), s9 matched-rho blocks). None alters an estimand, margin, "
                          "family, test, seed or threshold. results.json pins the A1-A3 AMENDMENTS.md (all 521 pinned inputs match the "
                          "snapshot). A3.7 fixes the generator scheme only after data existed, which is why the H-M verdict's Monte "
                          "Carlo dependence (F1) matters: the frozen plan fixed the seed and count, not the draw order.")},
        "plan_files_unchanged_since_freeze": "PLAN.md, ANALYSIS_PLAN.md, specs/ and census/ have no change between dbd0f9f274 (ANALYSIS_PLAN freeze) and the snapshot",
    },
    "rho_reference_audit": {
        "protocol_reference": "rho-neg (rho.negation) in every isogenous, ctl and M session; rho-cubic (rho.cubic) on the V5 crater D, N, ctl specs and every Tcr spec",
        "V5": (f"p = 66678169 = 1 (mod 3) and r = 16665673 = 1 (mod 3): the order-6 automorphism of the j = 0 crater is defined over F_p "
               f"and acts on the order-r subgroup, so rho-cubic (automorphism fold) is the strongest rho on the crater; the report uses it "
               f"there (s9 V5 L0, results.json ic_vs_rho_cubic_crater: 89.3 / 98.2 / 141.2). The other 63 V5 vertices have j not in "
               f"{{0, 1728}} (recomputed): Aut = {{+-1}}, prime field, so rho-neg is the strongest."),
        "V6": "No V6 vertex (648, j recomputed from (a, b)) has j in {0, 1728}; prime field: rho-neg is the strongest available on every V6 curve.",
        "H-T": "Denominator rho-neg at the 63 j != 0 source vertices: the strongest rho at the source (F3 is wording only).",
        "same_unit_and_binary": "Both arms in the same sessions, same binary (ac4bc5b2...), unit S / priced S; rho-cubic and rho-neg priced with their canonicalisations, ic with its five uncharged counters.",
        "matched_rho_is_strongest_available_per_curve": True,
    },
    "section_5a_band": {
        "applied_to_both_arms": True,
        "unpriced_counters": {"ic": ["sqrt_solves_uncharged", "lookups_uncharged", "row_ops_uncharged", "legendre_symbols_uncharged",
                                     "target_guard_probes_uncharged"], "rho-neg": ["canonicalisations_uncharged"],
                              "rho-cubic": ["canonicalisations_uncharged"], "bsgs": ["inserts_uncharged", "lookups_uncharged"]},
        "formula": "priced S = S * (total_gae + p * uncharged) / total_gae, p in {0, 0.1, 1}, same p on both arms (A2.1); uncharged re-derived from records.jsonl for all 428,976 measured rows",
        "order_holds_across_band": "rho ahead of ic at every price on every level of both volcanoes and on the V5 crater against rho-cubic (lower CI bound of IC/rho far above 1)",
        "band_means": {"V6_all_levels": {p: {"ic": ex["band"][f"V6|ic|{p}"], "rho_neg": ex["band"][f"V6|rho-neg|{p}"]} for p in ("0.0", "0.1", "1.0")},
                       "V5_levels_1_6": {p: {"ic": ex["band"][f"V5|ic|{p}"], "rho_neg": ex["band"][f"V5|rho-neg|{p}"]} for p in ("0.0", "0.1", "1.0")},
                       "V5_crater_D+N": {p: {"ic": me["ic_vs_rho"]["V5|crater_rho_cubic"][p]["ic"],
                                             "rho_cubic": me["ic_vs_rho"]["V5|crater_rho_cubic"][p]["rho_cubic"],
                                             "rho_neg": me["ic_vs_rho"]["V5|crater_rho_cubic"][p]["rho_neg"]} for p in ("0.0", "0.1", "1.0")}},
    },
    "controls": {
        "positive_control_Q2a": f"pass: rho-cubic / rho-neg = {me['positive_control.ratio']:.4f}, validator CI {me['positive_control.ci95']} (upper < 0.85); the detector for every 'equivalent' verdict works",
        "negative_control_ctl_V6": "F = 1.04 / 0.98 / 0.99 (rho-neg / rho-plain / ic), p 0.15 / 0.67 / 0.42 (validator: identical F and p); BSGS degenerate (zero within-vertex variance), as expected",
        "H-T_control_T0": "T0 CI includes 1 at all three prices (validator: [0.923, 1.014] / [0.922, 1.014] / [0.921, 1.016])",
        "H-M_controls": "rho-neg significant on 0 of 12 curves, bsgs 0 of 12; bsgs ops identical across the 8 models for every (target, round) on all 12 curves (the models carry the same instances)",
        "bsgs_identity": "identical ops across D/Ua/Ub(/S) for 100% of 3,064 (V5) and 6,336 (V6) triples, both BSGS arms",
        "null_object_note": "max/min effect sizes do not decay to 1 under no effect (N1); read IC against rho-neg",
    },
    "heuristic_validation_checks": [
        "Pre-registered predictions: theory 1/sqrt(3) for rho-cubic / rho-neg and the H-T composition (PLAN s0, frozen before data); measured 0.611 (Q2a) and T = 0.578-0.595, reported as measurements with the theory quoted, not fitted.",
        "Sample integrity: 147 sessions, seeds in specs and plan.json; every record re-derived from the archives; sample sizes per level per the manifest (estimate 15.4 h <= 20 h, no ctl reduction).",
        "Scale binding: 26-bit p (V5 p = 66678169, r = 16665673; V6 p = 38002051, r = 808589), one volcano per question; stated in REPORT s12."],
    "cost_model_checks": [
        "Unit declared: S = total_gae / sqrt(r) (validator checked S == total_gae / sqrt(r) on all 670,688 records) and ops_gae; uncharged native work priced at 0 / 0.1 / 1 GAE on both arms; both arms flagged as lower bounds (cost.lower_bound = true).",
        "Memory is not reported; at 26-bit group orders it is not binding and no time-memory claim is made (limitation).",
        "No success-probability scaling is involved: every measured run verified (0 failures), so expected cost = mean measured cost."],
    "findings": findings + notes,
    "limitations": [
        f"Replay (ecbench verify --replay-all) covers {tot['sessions']} of 147 sessions (seeded stratified sample: every volcano x family stratum, the three V5 crater rho-cubic sessions, {len(cert['sample']['tcr_sessions'])} Tcr, {len(cert['sample']['m_sessions'])} M and one redone .r2 session); the other sessions rest on their Phase B audits (12 replays each, 1,764 identical) and on the validator's raw-record re-derivation, which covers all 147 archives ([k]G = Q with independent arithmetic, S, uncharged counts, extracts row by row).",
        "The replay used the same shared cloud VM type and the same ecbench binary as the sessions; it is an independent re-execution of operation counts, not an independent solver implementation or independent hardware.",
        "Bootstrap CIs and permutation p-values were recomputed with independent generators: they agree within Monte Carlo error, not bit for bit; verdicts that sit within about 2 Monte Carlo SE of a threshold (F1, F2) can differ between implementations.",
        "Not re-run: the s5 transport-then-solve and paired-source bootstrap CIs and signal flags (all 100 point ratios and native controls recomputed exactly), per-vertex H-T CIs and the V5 per-family R(L); the s4 per-curve p-values were computed from the exact three-outcome reference distribution instead of 10,000 label shuffles; the s3 mixed model was re-run with the same library (statsmodels) for ic and rho-neg only.",
        "Wall-time results are descriptive in the study and were checked only as A/A ratios and L2 fractions.",
        "This validation does not support an ECDLP claim, a speedup, or promotion; a pass would make the receipts admissible evidence for the benchmarks row only.",
    ],
    "proposed_validation_json_rho_reference": {
        "kind": "negation",
        "protocol": "PLAN s2 / ANALYSIS_PLAN s9: rho-neg (rho.negation) paired with ic on the same (workload, target, round); on the V5 j = 0 crater the matched rho is the automorphism fold rho-cubic (rho.cubic), results.json ic_vs_rho_cubic_crater",
        "unit": "S (total_gae / sqrt(r)), unpriced native operations priced at 0 / 0.1 / 1 GAE on both arms",
        "binary_or_process": "ecbench sha256 ac4bc5b26f858a4d8a374208297157840af25fb1dc25ea1c19dd41360735fb35, both arms in the same sessions",
        "gae_band": {k: {"ic": ex["band"][f"V6|ic|{p}"], "rho": ex["band"][f"V6|rho-neg|{p}"]} for k, p in (("0", "0.0"), ("0.1", "0.1"), ("1", "1.0"))},
        "order_holds_across_band": True,
        "note": "not written as VALIDATION.json: the validation failed (F1)"},
    "replay_certificate": {"path": "replay-certificate.json", "status": cert["status"]},
    "validation_report": {
        "id": VAL_ID,
        "task_id": "none issued (workflow step 9, independent validation of PR isogeny-labs/crypto#3476)",
        "run_ids": "the 147 Phase B sessions of runs/progress.jsonl (status done); 19 replayed, listed in replay-certificate.json",
        "artifact_checks": [
            "binary sha256 ac4bc5b2... equals the progress header; checked before every replayed session",
            "147 archives hash to their progress lines; 147 Phase B audit receipts ok, complete, bound to the session id, 1,764/1,764 replays identical",
            "snapshot 12e5eb3f7d committed and pushed (local HEAD = origin branch); working tree clean",
            "PLAN, ANALYSIS_PLAN, specs, census unchanged since the freeze; ANALYSIS_PLAN and A1 committed before the first session",
            "results.json pins 521 inputs, all equal to the snapshot files",
            "3 orphan partial sessions: every partial record identical (seq, arm, workload, total_gae, recovered) to the redone .r2 session",
            "spec.json inside all 147 archives equals the committed specs/<spec>.json; session status complete; records_written = progress runs; git_dirty true in 146 sessions (driver commits per session; binary pinned by sha256)"],
        "metric_recomputations": f"{summ['numbers']} numbers, {summ['agree']} agree, {summ['disagree']} disagree ({', '.join(summ['disagreeing'])})",
        "control_checks": "positive control pass; ctl negative control F tests reproduced; T0 control includes 1; H-M rho-neg / bsgs controls 0 of 12; BSGS identity 100%",
        "heuristic_validation_checks": "see heuristic_validation_checks",
        "cost_model_checks": "see cost_model_checks",
        "proof_architecture_checks": [],
        "verdict": "failed",
        "limitations": "see limitations",
        "artifact_paths": ["benchmarks/20261023-volcano-census-followup/replay-certificate.json",
                           "benchmarks/20261023-volcano-census-followup/validator-report.json",
                           "benchmarks/20261023-volcano-census-followup/VALIDATOR_REPORT.md",
                           "benchmarks/20261023-volcano-census-followup/replay-receipts/",
                           "research/volcano_ecdlp_census_followup_20261023/validation/"]},
    "review_attestation": {
        "joints_owned": ["replay certificate", "own-code recompute of the headline numbers", "REPORT vs results", "verdict vocabulary",
                         "amendments", "matched rho"],
        "sources_read": ["docs/VALIDATION_GATE.md", "PLAN.md", "EXECUTION.md", "ANALYSIS_PLAN.md", "AMENDMENTS.md", "DRIVE.md", "REPORT.md",
                         "report.typ (phrase check)", "analysis/results.json", "analysis/analyze.log", "runs/progress.jsonl", "runs/manifest.json",
                         "runs/transfer_cost.jsonl", "runs/extract/*", "runs/uncharged/*", "sessions/*.tar.xz (records.jsonl, session.json, plan.json)",
                         "sessions/*.audit.json", "runs/orphans/*", "specs/*", "census/V5/*, census/V6/*",
                         "census validation: benchmarks/20261017-volcano-ecdlp-census/{VALIDATOR_REPORT.md, VALIDATION.json, replay-certificate.json, validator-report.json (structure)}",
                         "research/volcano_ecdlp_census_20261017/validation/{draw_sample.sh, replay_sample.sh, curve_effect.awk, point_estimates.awk}",
                         "scripts/validation_gate.py (check functions)"],
        "read_sibling_reports": False,
        "producer_code_opened": False,
        "verdict": "failed"},
}
json.dump(rep, open(f"{BENCH}/validator-report.json", "w"), indent=1)

# ------------------------------------------------------------------ markdown
L = []
A = L.append
A("# Validator report: volcano ECDLP census follow-up")
A("")
A(f"Study `research/volcano_ecdlp_census_followup_20261023/`, PR isogeny-labs/crypto#3476, branch "
  f"`claude/volcano-census-followup-20261023` at `{SNAP}`. Validation ID `{VAL_ID}`, round 1. The validator did not write "
  "the study and did not open or run any producer code (`analysis/*.py`, `analysis/build_pdf.sh`) or use its derived "
  "`runs/slugmap/`; every number below was recomputed from the committed session archives with validator-written code in "
  "`research/volcano_ecdlp_census_followup_20261023/validation/` (the census's `draw_sample.sh` and `replay_sample.sh`, adapted; "
  "the Python is new).")
A("")
A("## Verdicts")
A("")
A("| file | status | why |")
A("|:--|:--|:--|")
A(f"| `replay-certificate.json` | **{cert['status']}** | {tot['sessions']} of 147 sessions replayed (seeded stratified sample: all "
  f"{len(cert['sample']['strata_covered'])} volcano x family strata, the 3 V5 crater rho-cubic sessions, {len(cert['sample']['tcr_sessions'])} Tcr, "
  f"{len(cert['sample']['m_sessions'])} M, 1 redone `.r2`); {tot['measured_runs_replayed']} of {tot['measured_runs_replayed']} measured runs "
  f"re-executed with identical counts; {tot['records_scalar_verified']} of {tot['records_audited']} records scalar-verified; "
  f"{tot['sessions_replay_all']} sessions with `--replay-all`, fallback: {'none' if not tot['sessions_replay_50_fallback'] else ', '.join(tot['sessions_replay_50_fallback'])}; "
  "archive hashes and receipt file maps all match |")
A(f"| `validator-report.json` | **fail** | {summ['agree']} of {summ['numbers']} recomputed numbers agree; the H-M verdict does not "
  "reproduce (F1, material) |")
A("| `VALIDATION.json` | **not written** | the gate needs a passing validator report |")
A("")
A(f"Raw-record re-derivation, all 147 archives: {rr['records']} records ({rr['measured']} measured, {rr['warmup']} warm-up) streamed "
  "from the committed archives (each archive's SHA-256 equal to its progress line); [recovered]G = Q, Q on the curve and [r]G = O "
  f"checked with validator-written affine arithmetic: {rr['scalar_check_failures']} failures; S = total_gae / sqrt(r): "
  f"{rr['s_recompute_failures']} failures; 0 unverified measured runs. Every row equals `runs/extract` and `runs/uncharged` "
  f"(the A2.1 uncharged count re-derived independently): "
  f"{sum(rr['extract_and_uncharged_comparison']['mismatches'].values())} mismatches over "
  f"{rr['extract_and_uncharged_comparison']['totals']['extract_rows']} extract rows and "
  f"{rr['extract_and_uncharged_comparison']['totals']['uncharged_rows']} uncharged rows. Census-vertex mapping by recomputed "
  "j-invariant (D rows carry the vertex's own model, ctl rows one of its two re-scalings, M rows one of its 8 models, Tcr rows "
  "the crater model with the source's generator): 0 failures; level from the vertex equals the spec's level everywhere.")
A("")
A("The replay ran on the same shared 4-vCPU cloud VM type with the same ecbench binary as the sessions; it re-executes "
  "operation counts, it is not an independent solver implementation or independent hardware.")
A("")
A("## Recomputed headline numbers")
A("")
A(f"{summ['numbers']} numbers compared, {summ['agree']} agree, {summ['disagree']} disagree "
  f"({', '.join(summ['disagreeing'])}). Point estimates agree to {summ['max_rel_diff_point_estimates']:.1e} relative; "
  f"bootstrap endpoints within {summ['bootstrap_endpoints_max_mc_z']:.2f} Monte Carlo SD of the difference (tolerance 4); "
  "permutation p within binomial Monte Carlo error.")
A("")
A("| quantity | claimed (results.json) | recomputed |")
A("|:--|--:|--:|")
pick = ["positive_control.ratio_rho-cubic_over_rho-neg.V5_crater.D+N+ctl", "positive_control.row_bootstrap.ci_hi",
        "level_ratio.R(L).V6.rho-neg.L2/L0", "level_ratio.V6.rho-neg.L2.ci90.ci_hi", "level_ratio.R(L).V6.ic.L4/L0",
        "level_ratio.R(L).V5.ic.L6/L0", "level_effect.verdict_count.V6.equivalent", "level_effect.verdict_count.V5.equivalent",
        "H-T.overall.T.price_0.0", "H-T.overall.T.price_0.1", "H-T.overall.T.price_1.0", "H-T.overall.T.price_1.0.ci_hi",
        "H-T.overall.T0.price_1.0", "H-T.overall.T0.price_1.0.ci_hi", "H-T.cubic_over_neg_S_on_Tcr",
        "H-M.V6:98.ic.F", "H-M.V6:98.ic.p_perm", "H-M.V6:214.ic.max_over_min", "H-M.n_significant_holm.ic",
        "H-M.n_significant_holm.rho-neg", "H-M.n_significant_holm.bsgs",
        "ic_over_rho-cubic.V5_crater.D+N.price_0.0", "ic_over_rho-cubic.V5_crater.D+N.price_0.1", "ic_over_rho-cubic.V5_crater.D+N.price_1.0",
        "ic_over_rho-neg.V6.pooled.price_0.0", "ic_over_rho-neg.V6.pooled.price_1.0",
        "ic_over_rho-neg.V5.pooled_levels_ge1.price_0.0", "ic_over_rho-neg.V5.pooled_levels_ge1.price_1.0",
        "counts.rows_measured", "counts.rows_warmup_dropped", "counts.unverified_rows_total",
        "bsgs_identity.V6.bsgs.fraction_identical_ops", "bsgs_identity.V6.bsgs.n_triples", "bsgs_identity.V5.bsgs.n_triples",
        "curve_effect.V6.rho-neg.F", "mixed_model.V6|ic.icc", "wall.V5.aa_ratio(isogenous families, as results.json)"]
byname = {x["name"]: x for x in parts["headline"]}
for n in pick:
    x = byname[n]
    A(f"| {n} | {x['claimed']:.10g} | {x['recomputed']:.10g}{' **(differs)**' if not x['agrees'] else ''} |")
A("")
A("All 50 R(L) values with their 90% and 95% CIs and verdicts, the 63 per-vertex and 6 per-level H-T values at three prices, "
  "every H-M F, permutation p, model mean and max/min, every IC/rho cell (levels, pooled, crater against rho-cubic, M curves), "
  "the 100 transport-then-solve point ratios and native controls, the curve-effect F and p, the paired-source ratios, the A/A and "
  "L2 fractions and the mixed-model ICCs are in `validator-report.json` (`headline`). "
  f"REPORT.md tables: {rchk['checked']} cells checked, {rchk['equal_results']} equal to results.json at the report's rounding, "
  f"{len(rchk['differ_recompute'])} not reproduced by the validator (the V6:98 significance mark, F1).")
A("")
A("## Findings")
A("")
for x in findings:
    A(f"- **{x['id']} ({x['severity']}; {x['status']})**, {x['where']}. Report: {x['claim']} Evidence: {x['evidence']} *Fix:* {x['fix']}")
for x in notes:
    A(f"- **{x['id']} ({x['severity']})**, {x['where']}. {x['text']}")
A("")
A("## Audits")
A("")
A("- **Snapshot.** `12e5eb3f7d` is committed and pushed (local HEAD equals the origin branch), working tree clean. PLAN.md, "
  "ANALYSIS_PLAN.md, specs/ and census/ are unchanged since the freeze; ANALYSIS_PLAN.md (00:42Z) and A1 (00:50Z) precede the "
  "progress header (01:09Z) and the first session (01:15Z). results.json pins 521 inputs, all equal to the snapshot.")
A("- **Verdict vocabulary** follows ANALYSIS_PLAN (equivalent / different / inconclusive / descriptive; pass/fail; supported / "
  "refuted / inconclusive; supported for IC; descriptive (not admitted); signal, not a speedup claim; distance remaining). The "
  "departures are interpretive sentences (F1, F2).")
A("- **Amendments.** A1 and A2 change no estimand, margin, family, test or threshold; A1's H-T unit change (S to priced ops_gae) "
  "leaves the ratio unchanged because every V5 curve has the same r. A3 (post hoc, labelled so) fills gaps without changing an "
  "estimand, margin, family, test, seed or threshold; A1 text is a byte prefix of A1+A2, which is a byte prefix of the current "
  "file. A3.7 fixes the H-M generator scheme only after data existed; the frozen plan fixed the seed and the count, not the draw "
  "order, which is why F1 matters. A2.1's time span is wrong (F5).")
A("- **Matched rho.** V5: p = 1 (mod 3) and r = 1 (mod 3), so the j = 0 crater's order-6 automorphism is defined over F_p and "
  "rho-cubic is the strongest rho there; the report compares IC with rho-cubic on the crater (89.3x / 98.2x / 141.2x). The other 63 "
  "V5 vertices and all 648 V6 vertices have j not in {0, 1728} (recomputed): negation is the strongest there, and it is the "
  "reference. H-T's rho-neg denominator at j != 0 sources is the strongest rho at the source (F3 is wording).")
A("- **AGENTS.md section 5a band** applied to both arms with their named uncharged counters; rho ahead at every price, every "
  "level, both volcanoes and the crater; both arms remain lower bounds.")
A("- **Controls.** Q2a positive control passes (0.6113, validator CI [0.5996, 0.6235]); the V6 ctl negative control's F tests "
  "reproduce exactly; H-T's T0 control includes 1 at all prices; H-M's rho-neg and bsgs controls are significant on 0 of 12 curves "
  "and bsgs ops are identical across all 8 models for every instance.")
A("- **Redone sessions.** The three `runs/orphans/` partial sessions (3,013 / 5,211 / 547 records, status 'running') have every "
  "record identical (seq, arm, workload, total_gae, recovered) to the same record of the redone `.r2` session: the redo selected "
  "nothing.")
A("")
A("## Limitations")
A("")
for x in rep["limitations"]:
    A(f"- {x}")
A("")
open(f"{BENCH}/VALIDATOR_REPORT.md", "w").write("\n".join(L))
print("wrote", f"{BENCH}/validator-report.json", f"{BENCH}/VALIDATOR_REPORT.md")
