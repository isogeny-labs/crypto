#!/usr/bin/env python3
"""Round-2 validator: write VALIDATOR_REPORT.md from the two JSON outputs, and VALIDATION.json
(validation-gate/v1) only when both pass. Validator-written.
Usage: write_md.py EVIDENCE_DIR
"""
import hashlib, json, os, sys

ev = sys.argv[1]
C = json.load(open(os.path.join(ev, "replay-certificate.json")))
V = json.load(open(os.path.join(ev, "validator-report.json")))


def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()


H = {h["name"]: h for h in V["headline"]}
CAT = {c["name"]: c for c in V["categorical"]}
S = V["headline_summary"]
t = C["totals"]
rr = C["raw_record_rederivation"]
ex = rr["extract_comparison"]
pb = C["phase_b_audits_all_sessions"]
bt = [h for h in V["headline"] if h["kind"] in ("bootstrap_endpoint", "permutation_p")]
import re
def sdv(h):
    m = re.search(r"\|diff\| = ([0-9.]+) SD", h.get("note", ""))
    return float(m.group(1)) if m else 0.0
max_boot = max(sdv(h) for h in bt if h["kind"] == "bootstrap_endpoint")
max_perm = max(sdv(h) for h in bt if h["kind"] == "permutation_p")
pt = [h for h in V["headline"] if h["kind"] == "point" and h["rel_tol"] == 1e-9]
max_pt = max(h["rel_diff"] for h in pt)

L = []
w = L.append
w("# Validator report: volcano ECDLP census follow-up (round 2)\n")
w(f"Study `research/volcano_ecdlp_census_followup_20261023/`, PR isogeny-labs/crypto#3476, branch `claude/volcano-census-followup-20261023` at `{V['snapshot_commit']}` (the round-1 erratum commit). Validation ID `{V['validation_report_id']}`, round 2. "
  "The validator did not write the study, its sessions, its analysis, its report, the erratum or the round-1 validation, and did not open producer code (`analysis/*.py`, `analysis/build_pdf.sh`) or the round-1 validator's scripts; it did not use `runs/slugmap/`. "
  "Every number below was recomputed from the committed session archives with validator-written code in `research/volcano_ecdlp_census_followup_20261023/validation/round2/` (the census's `draw_sample.sh` and `replay_sample.sh`, adapted; the Python is new).\n")
w(f"Previous validation `VAL-20261106-54796a` at `12e5eb3f7d` (validator commit `abd653551e`): **fail**, 1587 of 1588 numbers agreed; material F1 (H-M verdict), minor F2-F7, notes N1-N3. They are kept below with their resolution.\n")
w("## Verdicts\n")
w("| file | status | why |\n|:--|:--|:--|")
w(f"| `replay-certificate.json` | **{C['status']}** | {t['sessions_replayed']} of {t['sessions_in_study']} sessions replayed from the committed archives with `ecbench verify --replay-all` (new seeded draw, nonce = snapshot: all 14 volcano x family strata, the 3 V5 crater rho-cubic sessions, {len(C['sample']['tcr_sessions'])} Tcr, {len(C['sample']['m_sessions'])} M, {len(C['sample']['redone_r2_sessions'])} redone `.r2`); {t['measured_runs_identical']} of {t['measured_runs_replayed']} measured runs re-executed with identical counts; {t['records_scalar_verified']} of {t['records']} records scalar-verified; fallback: {', '.join(t['fallback_sessions']) or 'none'}; archive hashes, binary sha256 and receipt file maps all match |")
w(f"| `validator-report.json` | **{V['status']}** | {S['n_numeric_agree']} of {S['n_numeric']} recomputed numbers and {S['n_categorical_agree']} of {S['n_categorical']} categorical outcomes agree; {V['report_vs_results']['cells_equal']} of {V['report_vs_results']['cells_checked']} REPORT.md cells equal results.json; F1-F7 resolved by the round-1 erratum |")
w(f"| `VALIDATION.json` | **{'written' if V['status'] == 'pass' and C['status'] == 'verified' else 'not written'}** | pins both files by sha256 with the matched rho reference and the section 5a band |\n")
w(f"Raw-record re-derivation, all {rr['archives']} archives (streamed, nothing unpacked to disk): {rr['records']} records; archive SHA-256 equal to the progress line {rr['archive_sha256_equal_progress']}/{rr['archives']}; archived `spec.json` equal to `specs/` {rr['spec_json_equal_committed_spec']}/{rr['archives']}; "
  f"[recovered]G = Q, Q on the curve, [r]G = O ({rr['generators_order_r_checked']} generator checks), S = total_gae / sqrt(r), outcome verified, frozen binary in every record, census vertex by recomputed j-invariant, level = spec level: failures {sum(rr['failures'].values())}. "
  f"Equal to `runs/extract` and `runs/uncharged` row by row: {ex['extract_rows']} extract rows and {ex['uncharged_rows']} uncharged rows, 0 missing, mismatches {sum(ex['mismatch'].values()) + ex['uncharged_mismatch'] + ex['uncharged_detail_mismatch']}. "
  f"Phase B audits: {pb['ok_complete']}/{pb['sessions']} OK, {pb['replays_reproduced']}/{pb['replays']} replays reproduced.\n")
w("## Recomputed headline numbers\n")
w(f"{S['n_numeric']} numbers compared, {S['n_numeric_agree']} agree. Point estimates agree to {max_pt:.1e} relative; bootstrap endpoints within {max_boot:.2f} Monte Carlo SD of the difference and permutation p-values within {max_perm:.2f} binomial SD (tolerance 4). Values quoted in the erratum from the round-1 validator (with-ctl A/A, F7 ratios, row-pooled H-T) agree at their printed rounding.\n")
w("| quantity | claimed | recomputed |\n|:--|--:|--:|")
keys = ["positive_control.ratio_rho-cubic_over_rho-neg.V5_crater.D+N+ctl", "positive_control.row_bootstrap.ci95_hi",
        "level_effect.R(L).V6.rho-neg.L2", "level_effect.V6.rho-neg.L2.ci90_hi", "level_effect.V6.rho-plain.L1.ci90_lo",
        "level_effect.R(L).V6.ic.L4", "level_effect.R(L).V5.ic.L6", "level_effect.verdict_count.V6.equivalent",
        "level_effect.verdict_count.V6.inconclusive", "level_effect.verdict_count.V5.equivalent",
        "H-T.overall.T.price_0.0", "H-T.overall.T.price_0.1", "H-T.overall.T.price_1.0", "H-T.overall.T.price_1.0.ci95_hi",
        "H-T.overall.T0.price_0.0", "H-T.overall.T0.price_1.0", "H-T.overall.T0.price_1.0.ci95_lo", "H-T.overall.T0.price_1.0.ci95_hi",
        "H-T.cubic_over_neg_S_on_Tcr", "H-T.transport_share_of_numerator.price_1.0",
        "H-M.V6:98.ic.F", "H-M.V6:98.ic.p_perm", "H-M.V6:98.ic.p_holm", "H-M.V6:214.ic.max_over_min", "H-M.V6:214.ic.F",
        "H-M.n_significant_holm.rho-neg", "H-M.n_significant_holm.bsgs",
        "ic_over_rho-cubic.V5_crater.D+N.price_0.0", "ic_over_rho-cubic.V5_crater.D+N.price_0.1", "ic_over_rho-cubic.V5_crater.D+N.price_1.0",
        "ic_over_rho-neg.V6.pooled.price_0.0", "ic_over_rho-neg.V6.pooled.price_1.0",
        "ic_over_rho-neg.V5.pooled_levels_ge1.price_0.0", "ic_over_rho-neg.V5.pooled_levels_ge1.price_1.0",
        "counts.rows_measured", "counts.rows_warmup_dropped", "counts.unverified_rows_total",
        "bsgs_identity.V6.bsgs.fraction_identical_ops", "bsgs_identity.V6.bsgs.n_triples", "bsgs_identity.V5.bsgs.n_triples",
        "curve_effect.V6.rho-neg.F", "wall.V5.aa_ratio(isogenous)", "wall.V5.aa_ratio(isogenous+ctl) (erratum F4, rounded)",
        "wall.V6.aa_ratio(isogenous+ctl) (erratum F4, rounded)", "mixed_model.V6|ic.icc"]
for k in keys:
    h = H[k]
    w(f"| {k} | {h['claimed']:.10g} | {h['recomputed']:.10g} |")
c = CAT["H-M.n_significant_holm.ic"]
w(f"| H-M.n_significant_holm.ic | {c['claimed']} (frozen seed) | {c['recomputed']['validator_10000_shuffles_seed_20261024']} (own 10,000 shuffles), {c['recomputed']['validator_1e6_shuffles']} (1,000,000 shuffles), {c['recomputed']['parametric_F_7_504']} (parametric) |")
w(f"| H-T.verdict | {CAT['H-T.verdict']['claimed']} | {CAT['H-T.verdict']['recomputed']} |")
w(f"| positive_control.pass | {CAT['positive_control.pass (95% CI upper < 0.85)']['claimed']} | {CAT['positive_control.pass (95% CI upper < 0.85)']['recomputed']} |\n")
w("All 50 R(L) cells with their 90% and 95% CIs and verdicts, the 6 per-level and 63 per-vertex H-T values with M_v, every H-M model mean, F, permutation and Holm p and max/min, every IC/rho cell (levels, pooled, the crater against rho-cubic, the 12 M curves), BSGS identity, the curve-effect F tests, the A/A ratios and L2 fractions and the mixed-model ICCs are in `validator-report.json` (`headline`, `categorical`).\n")
w("## Round-1 findings and their resolution\n")
for f in V["round1_findings_resolution"]:
    w(f"- **{f['id']} ({f['severity']}; {f['status']}).** Round 1: {f['round1']} Resolution: {f['resolution']} Round-2 check: {f['round2_evidence']}")
w("")
w("## Round-2 findings\n")
w("None. Four notes, none affecting a number or a verdict:\n")
for n in V["notes"]:
    w(f"- **{n['id']}** ({n['where']}): {n['text']}")
w("")
w("## Audits\n")
sa = V["snapshot_audit"]
w(f"- **Snapshot.** `{V['snapshot_commit'][:10]}` is committed and pushed (local HEAD equals the origin branch), working tree clean. The erratum commit touches only REPORT.md, report.typ, report.pdf (both copies) and AMENDMENTS.md (appended). results.json is unchanged since round 1; {sa['results_json_inputs_equal_now']} of its {sa['results_json_inputs_pinned']} pinned inputs hash equal now, and the one that differs is AMENDMENTS.md, pinned at its A1-A3 version, a byte prefix of the current file. {sa['frozen_files_unchanged']}.")
w(f"- **Amendments.** {V['amendment_audit']} {sa['amendments_append_only']}.")
w(f"- **Verdict vocabulary.** {V['verdict_vocabulary']}")
w(f"- **Matched rho.** {V['matched_rho_audit']}")
band = V["proposed_validation_json_rho_reference"]["gae_band_by_volcano"]
w("- **AGENTS.md section 5a band** (mean priced S over matched pairs; both arms priced with their own named uncharged counters):\n")
w("| population | price | ic | rho | ic / rho |\n|:--|--:|--:|--:|--:|")
for k, v in band.items():
    pop, pr = k.split("|")
    if "rho_cubic" in v:
        w(f"| {pop} (rho-cubic) | {pr} | {v['ic']:.3f} | {v['rho_cubic']:.4f} | {v['ic_over_cubic']:.1f} |")
    else:
        w(f"| {pop} (rho-neg) | {pr} | {v['ic']:.3f} | {v['rho_neg']:.4f} | {v['ratio']:.1f} |")
w("\n  Rho is ahead at every price, at every level of both volcanoes (every recomputed cluster-bootstrap lower end above 1) and on the crater against rho-cubic; both arms remain lower bounds. The ratio is the distance remaining, not a pass/fail.")
ht = H
w(f"- **Controls.** Q2a positive control passes ({H['positive_control.ratio_rho-cubic_over_rho-neg.V5_crater.D+N+ctl']['recomputed']:.4f}, own 95% CI upper {H['positive_control.row_bootstrap.ci95_hi']['recomputed']:.4f} < 0.85), so the equivalence verdicts are informative. H-T's T0 control: own 95% CI includes 1 at all three prices (upper end {H['H-T.overall.T0.price_1.0.ci95_hi']['recomputed']:.4f} at 1 GAE); T's CI is below 1 at all three; verdict supported, reproduced. H-M's null objects: rho-neg significant on 0 of 12 curves in every evaluation; bsgs between-model variance 0 with identical ops across all 8 models for every instance. The V6 ctl negative control's F tests reproduce exactly (own permutation p {H['curve_effect.V6.rho-neg.p_perm']['recomputed']:.3f} / {H['curve_effect.V6.rho-plain.p_perm']['recomputed']:.3f} / {H['curve_effect.V6.ic.p_perm']['recomputed']:.3f} for rho-neg / rho-plain / ic).")
ro = V["redone_sessions_audit"]["result"]
w("- **Redone sessions.** " + "; ".join(f"`{k}`: {v['orphan_records']} partial records (status '{v['orphan_status']}'), {v['orphan_records_identical_in_r2']} identical in `{k}.r2`" for k, v in ro.items()) + ". The redo selected nothing; the nine busy-host `failed` lines (05:32:32Z-05:43:01Z) started no session, and their specs count once.")
w(f"- **Report and PDF.** {V['report_vs_results']['cells_equal']} of {V['report_vs_results']['cells_checked']} REPORT.md cells (s1, s2, s4, s5, s7, s8, s9, s10 tables and verdict sentences) equal results.json at the report's rounding. {V['report_vs_results']['pdf']}; {V['report_vs_results']['figures']}.")
w("")
w("## Limitations\n")
for l in V["limitations"]:
    w(f"- {l}")
w("")
open(os.path.join(ev, "VALIDATOR_REPORT.md"), "w").write("\n".join(L))

if V["status"] == "pass" and C["status"] == "verified":
    ref = dict(V["proposed_validation_json_rho_reference"])
    vj = {"schema": "validation-gate/v1",
          "replay_certificate": {"path": "replay-certificate.json", "sha256": sha(os.path.join(ev, "replay-certificate.json"))},
          "validator_report": {"path": "validator-report.json", "sha256": sha(os.path.join(ev, "validator-report.json"))},
          "rho_reference": ref}
    json.dump(vj, open(os.path.join(ev, "VALIDATION.json"), "w"), indent=2)
    print("VALIDATION.json written")
else:
    print("VALIDATION.json not written")
