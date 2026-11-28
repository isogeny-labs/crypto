#!/usr/bin/env python3
"""Assemble the validator's replay certificate, validator report (JSON) and
VALIDATOR_REPORT.md for the follow-up study from the validator's own outputs
(rederive.py, compare_extracts.py, recompute.py, hm_precise.py,
mixed_check.py, report_check.py, replay_sample.sh) and the claims in
analysis/results.json. Validator-written; reads no producer code.

Usage: python3 -I build_validation.py STUDY_DIR SCRATCH_DIR BENCH_DIR SNAPSHOT_SHA
"""
import glob
import hashlib
import json
import math
import os
import shutil
import sys

STUDY, SCR, BENCH, SNAP = sys.argv[1:5]
VAL_ID = "VAL-20261106-54796a"
REL = "../../research/volcano_ecdlp_census_followup_20261023"
B = 10000


def J(p):
    return json.load(open(p))


def sha(p):
    h = hashlib.sha256()
    with open(p, "rb") as f:
        for c in iter(lambda: f.read(1 << 20), b""):
            h.update(c)
    return h.hexdigest()


res = J(f"{STUDY}/analysis/results.json")
me = J(f"{SCR}/recompute.json")
hmp = J(f"{SCR}/hm_precise.json")
mix = J(f"{SCR}/mixed_check.json")
cmpx = J(f"{SCR}/compare_extracts.json")
rchk = J(f"{SCR}/report_check.json")
pba = J(f"{SCR}/phase_b_audits.json")
orph = J(f"{SCR}/orphans.json")
prov = J(f"{SCR}/provenance.json")
sess = [json.loads(l) for l in open(f"{SCR}/rows/sessions.jsonl")]
replay = [json.loads(l) for l in open(f"{SCR}/replay.jsonl")]
sample = [l.strip() for l in open(f"{SCR}/sample.txt") if l.strip()]
started = open(f"{SCR}/replay_started_utc").read().strip()
prog = [json.loads(l) for l in open(f"{STUDY}/runs/progress.jsonl")]
header = prog[0]

# ------------------------------------------------------------------ headline
H = []


def h(name, claimed, recomputed, rel_tol=1e-9, kind="point"):
    claimed = float(claimed)
    recomputed = float(recomputed)
    scale = max(abs(claimed), abs(recomputed), 1e-300)
    rd = abs(claimed - recomputed) / scale
    H.append({"name": name, "claimed": claimed, "recomputed": recomputed, "rel_tol": rel_tol,
              "rel_diff": rd, "agrees": rd <= rel_tol, "kind": kind})


def mc_tol(sd, q, value):
    """4 x sqrt(2) x Monte Carlo SE of a percentile endpoint, relative."""
    z = {0.025: 1.959964, 0.05: 1.644854}[q]
    phi = math.exp(-z * z / 2) / math.sqrt(2 * math.pi)
    se = math.sqrt(q * (1 - q) / B) / phi * sd
    return 4 * math.sqrt(2) * se / max(abs(value), 1e-300)


def ci_h(name, ce, cm, sd, q=0.025):
    h(name + ".ci_lo", ce[0], cm[0], mc_tol(sd, q, ce[0]), "bootstrap endpoint")
    h(name + ".ci_hi", ce[1], cm[1], mc_tol(sd, q, ce[1]), "bootstrap endpoint")


# counts
dc = res["data_checks"]
h("counts.rows_all_incl_warmup", dc["rows_all_incl_warmup"], me["counts.rows_all_incl_warmup"])
h("counts.rows_measured", dc["rows_measured"], me["counts.rows_measured"])
h("counts.rows_warmup_dropped", dc["rows_all_incl_warmup"] - dc["rows_measured"], me["counts.rows_warmup"])
h("counts.unverified_rows_total", res["unverified_rows_total"], me["counts.unverified_measured"], kind="count (abs)")
h("counts.sessions", res["sessions"], me["counts.sessions"])
h("counts.recovery_smallest_cell_cp_lower95", 0.9963, me["counts.recovery_smallest_cp_lower95"], 1e-4, "report rounding")
# positive control
pc = res["positive_control"]
h("positive_control.ratio_rho-cubic_over_rho-neg.V5_crater.D+N+ctl", pc["row_bootstrap"]["ratio"], me["positive_control.ratio"])
h("positive_control.n_pairs", pc["n_pairs"], me["positive_control.n_pairs"])
ci_h("positive_control.row_bootstrap", pc["row_bootstrap"]["ci95"], me["positive_control.ci95"], me["positive_control.boot_sd"])
sdc = (pc["slug_cluster_sensitivity"]["ci95"][1] - pc["slug_cluster_sensitivity"]["ci95"][0]) / 3.92
ci_h("positive_control.slug_cluster", pc["slug_cluster_sensitivity"]["ci95"], me["positive_control.slug_cluster_ci95"], sdc)
# level effect
for k, d in sorted(me["level_effect"].items()):
    V, arm, L = k.split("|")
    e = res["level_effect"][f"{V}|{arm}"]["R"][L]["S"]
    h(f"level_ratio.R(L).{V}.{arm}.L{L}/L0", e["ratio"], d["R"])
    h(f"level_ratio.R(L)_ops_gae.{V}.{arm}.L{L}/L0", res["level_effect"][f"{V}|{arm}"]["R"][L]["ops_gae"]["ratio"], d["R_ops_gae"])
    ci_h(f"level_ratio.{V}.{arm}.L{L}.ci95", e["ci95"], d["ci95"], e["boot_sd"], 0.025)
    ci_h(f"level_ratio.{V}.{arm}.L{L}.ci90", e["ci90"], d["ci90"], e["boot_sd"], 0.05)
for V, want in (("V6", {"equivalent": 19, "inconclusive": 1, "different": 0}),
                ("V5", {"equivalent": 15, "descriptive": 15, "different": 0})):
    claimed = {}
    for k in res["level_effect"]:
        if k.startswith(V + "|"):
            for L, x in res["level_effect"][k]["R"].items():
                v = x["S"].get("verdict")
                claimed[v] = claimed.get(v, 0) + 1
    for verdict in want:
        h(f"level_effect.verdict_count.{V}.{verdict}", claimed.get(verdict, 0), me["level_effect.verdict_counts"][V].get(verdict, 0))
# curve effect
for arm in ("rho-neg", "rho-plain", "ic"):
    e = res["curve_effect"][f"V6|{arm}"]
    d = me["curve_effect"][arm]
    h(f"curve_effect.V6.{arm}.F", e["F"], d["F"])
    h(f"curve_effect.V6.{arm}.p_perm", e["p_perm"], d["p_perm"], 1e-9, "permutation (same seed and scheme: identical)")
    h(f"curve_effect.V6.{arm}.n_flagged", e["n_flagged"], d["n_flagged"])
# paired source
for k, d in me["paired_source"].items():
    h(f"paired_source.{k}", res["paired_source"][k]["ratio_src_over_D"], d["ratio"])
# BSGS identity
for k, d in me["bsgs_identity"].items():
    V, arm = k.split("|")
    e = res["bsgs_identity"][V][arm]
    h(f"bsgs_identity.{V}.{arm}.fraction_identical_ops", e["fraction_identical_ops_gae"], d["fraction_identical"])
    h(f"bsgs_identity.{V}.{arm}.n_triples", e["n_triples"], d["n_triples"])
# H-T
ht, mh = res["H-T"], me["H-T"]
for t in ("T", "T0"):
    for p in ("0.0", "0.1", "1.0"):
        e = ht["overall"][t][p]
        d = mh["overall"][f"{t}|{p}"]
        h(f"H-T.overall.{t}.price_{p}", e["value"], d["value"])
        ci_h(f"H-T.overall.{t}.price_{p}", e["ci95"], d["ci95"], e["boot_sd"])
        for L in range(1, 7):
            e2 = ht["by_level"][str(L)][t][p]
            d2 = mh["by_level"][str(L)][f"{t}|{p}"]
            h(f"H-T.level{L}.{t}.price_{p}", e2["value"], d2["value"])
            ci_h(f"H-T.level{L}.{t}.price_{p}", e2["ci95"], d2["ci95"], e2["boot_sd"])
for p in ("0.1", "1.0"):
    h(f"H-T.transport_share_of_numerator.price_{p}", ht["overall"]["transport_share_of_T_numerator"][p]["value"],
      mh["overall"][f"share|{p}"]["value"])
h("H-T.cubic_over_neg_S_on_Tcr", ht["overall"]["cubic_over_neg_S_on_Tcr"]["value"], mh["overall"]["cubic_over_neg_S_on_Tcr"]["value"])
h("H-T.n_vertices", ht["n_vertices"], mh["n_vertices"])
h("H-T.unpaired_rows", ht["checks"]["unpaired_rows"], mh["unpaired_keys"])
tv = [ht["by_vertex"][v]["T"]["1.0"]["value"] for v in ht["by_vertex"]]
h("H-T.T_v_at_1GAE.min", min(tv), mh["T_v_at_1_range"][0])
h("H-T.T_v_at_1GAE.max", max(tv), mh["T_v_at_1_range"][1])
ex = J(f"{SCR}/extras.json")
for v, x in ht["by_vertex"].items():
    h(f"H-T.vertex{v}.velu_point_maps_M_v", x["velu_point_maps"], ex["M_v"][v]["M_v"])
    for t in ("T", "T0"):
        for p in ("0.0", "0.1", "1.0"):
            h(f"H-T.vertex{v}.{t}.price_{p}", x[t][p]["value"], ex["T_v"][v][f"{t}|{p}"])
# H-M
hm, mm = res["H-M"]["curves"], me["H-M"]["curves"]
for c in sorted(hm):
    for arm in ("ic", "rho-neg"):
        e = hm[c]["arms"][arm]
        d = mm[c][arm]
        h(f"H-M.{c}.{arm}.F", e["F"], d["F"])
        sdp = math.sqrt(max(e["p_perm"] * (1 - e["p_perm"]), 1e-4 * (1 - 1e-4)) / B)
        h(f"H-M.{c}.{arm}.p_perm", e["p_perm"], d["p_perm"], 4 * math.sqrt(2) * sdp / e["p_perm"], "permutation p (Monte Carlo)")
        h(f"H-M.{c}.{arm}.max_over_min", e["max_over_min_model_mean"]["value"], d["max_over_min"])
        cie = e["max_over_min_model_mean"]["ci95"]
        sdm = (cie[1] - cie[0]) / 3.92
        ci_h(f"H-M.{c}.{arm}.max_over_min", cie, d["max_over_min_ci95"], sdm)
        for i, (a, b) in enumerate(zip(e["model_mean_S"], d["model_mean_S"])):
            h(f"H-M.{c}.{arm}.model{i}.mean_S", a, b)
    h(f"H-M.{c}.bsgs.identical_ops_across_models", res["H-M"]["bsgs_same_instance_check_fraction_identical_ops_gae_across_models"][c],
      mm[c]["bsgs"]["identical_ops_across_models_fraction"])
for arm in ("rho-neg", "bsgs", "ic"):
    h(f"H-M.n_significant_holm.{arm}", res["H-M"]["summary"][arm]["n_significant_holm"], me["H-M"]["n_significant"][arm],
      1e-9, "verdict input (Holm count; Monte Carlo dependent)")
# IC vs matched rho
for V in ("V5", "V6"):
    for L, ld in res["ic_vs_rho"][V]["levels"].items():
        for p in ("0.0", "0.1", "1.0"):
            h(f"ic_over_rho-neg.{V}.L{L}.price_{p}", ld["prices"][p]["ratio_ic_over_rho"], me["ic_vs_rho"][f"{V}|{L}"][p])
for V, rk, mk in (("V5", "pooled_levels_ge1", "V5|pooled_L1plus"), ("V5", "pooled", "V5|pooled_all"), ("V6", "pooled", "V6|pooled_all")):
    for p in ("0.0", "0.1", "1.0"):
        e = res["ic_vs_rho"][V][rk][p]
        h(f"ic_over_rho-neg.{V}.{rk}.price_{p}", e["ratio_ic_over_rho"], me["ic_vs_rho"][mk][p])
        if f"ci95_{p}" in me["ic_vs_rho"][mk]:
            ci_h(f"ic_over_rho-neg.{V}.{rk}.price_{p}", e["ci95"], me["ic_vs_rho"][mk][f"ci95_{p}"], (e["ci95"][1] - e["ci95"][0]) / 3.92)
cr = res["ic_vs_rho_cubic_crater"]
for p in ("0.0", "0.1", "1.0"):
    e = cr["prices"][p]
    d = me["ic_vs_rho"]["V5|crater_rho_cubic"][p]
    h(f"ic_over_rho-cubic.V5_crater.D+N.price_{p}", e["ratio_ic_over_rho_cubic"], d["ic_over_cubic"])
    h(f"ic_over_rho-cubic.V5_crater.mean_priced_S_ic.price_{p}", e["mean_priced_S_ic"], d["ic"])
    h(f"ic_over_rho-cubic.V5_crater.mean_priced_S_rho_cubic.price_{p}", e["mean_priced_S_rho_cubic"], d["rho_cubic"])
    ci_h(f"ic_over_rho-cubic.V5_crater.price_{p}", e["ci95"], d["ci95_ic_over_cubic"], (e["ci95"][1] - e["ci95"][0]) / 3.92)
h("ic_over_rho-cubic.V5_crater.n_pairs", cr["n_pairs"], me["ic_vs_rho"]["V5|crater_rho_cubic"]["n_pairs"])
for c, x in res["ic_vs_rho_M_descriptive"].items():
    for p in ("0.0", "1.0"):
        h(f"ic_over_rho-neg.M_curve.{c}.price_{p}", x["prices"][p]["ratio_ic_over_rho_neg"], me["ic_vs_rho"]["M_descriptive"][c][p])
for s, x in res["crater_models_V5"]["ic"].items():
    h(f"crater_ic_by_model.V5.{s}.mean_S", x["mean_S"], me["ic_vs_rho"]["V5_crater_ic_by_model"][s])
# wall
for V in ("V5", "V6"):
    w = res["wall_admission"][V]
    h(f"wall.{V}.aa_ratio(isogenous families, as results.json)", w["aa_ratio"], ex["wall"][f"{V}|iso"]["aa_ratio"])
    for arm, x in w["arms"].items():
        h(f"wall.{V}.{arm}.frac_rows_L2(isogenous)", x["frac_rows_L2"], me["wall"][V][f"L2|{arm}|iso"])
    h(f"wall.{V}.isolation_fraction_L2_all_rows", res["isolation_fraction_L2"][V], me["wall"][V]["L2|all_rows"])
# mixed model (spot check)
for k, d in mix.items():
    e = res["mixed_model"][k]
    h(f"mixed_model.{k}.icc", e["icc"], d["icc"], 1e-6, "statsmodels MixedLM (same library)")
    for L, x in e["level_ratio"].items():
        h(f"mixed_model.{k}.level_ratio.L{L}", x["ratio"], d["level_ratio"][L], 1e-6, "statsmodels MixedLM (same library)")
# transport-then-solve signal cells and transfer cost
for k, x in ex["tts"].items():
    V, f, a, L = k.split("|")
    e = res["transport_then_solve"][f"{V}|{f}|{a}"]["levels"][L[1:]]
    h(f"transport_then_solve.{k}.ratio_p0", e["prices"]["0.0"]["ratio"], x["ratio_p0"])
    h(f"transport_then_solve.{k}.native_control", e["control_native_ratio"]["ratio"], x["control_native_ratio"])
tcr_ = res["transfer_cost"]["by_volcano_path_len"]
h("transfer_cost.V5.ns_per_edge_median.path_len_1", tcr_["V5"]["1"]["ns_per_edge_median_q1_q3"][0], me["transfer_cost"]["V5|ns_per_edge_median_path_len_1"])
h("transfer_cost.V6.ns_per_edge_median.path_len_1", tcr_["V6"]["1"]["ns_per_edge_median_q1_q3"][0], me["transfer_cost"]["V6|ns_per_edge_median_path_len_1"])

n_agree = sum(x["agrees"] for x in H)
disagree = [x for x in H if not x["agrees"]]
pts = [x for x in H if x["kind"] == "point"]
mcz = []
for x in H:
    if x["kind"] == "bootstrap endpoint" and x["rel_tol"] > 0:
        mcz.append(x["rel_diff"] / (x["rel_tol"] / 4))
summary = {"numbers": len(H), "agree": n_agree, "disagree": len(disagree),
           "disagreeing": [x["name"] for x in disagree],
           "max_rel_diff_point_estimates": max(x["rel_diff"] for x in pts),
           "bootstrap_endpoints_max_mc_z": max(mcz) if mcz else None,
           "tolerances": "point estimates and counts rel 1e-9; bootstrap endpoints 4 x sqrt(2) Monte Carlo SE of a percentile (10,000 replicates; independent generators); permutation p 4 x sqrt(2) binomial SE; MixedLM 1e-6"}

# ------------------------------------------------------------- replay cert
runs = []
for r in replay:
    sess_name = r["run"]
    rcpt = glob.glob(f"{SCR}/receipts/{sess_name}.replay-*.json.xz")
    runs.append({"run": sess_name, "session_id": r["session_id"],
                 "archive": f"{REL}/sessions/{sess_name}.tar.xz", "archive_sha256": r["archive_sha256"],
                 "archive_sha256_match": r["archive_sha256_match"], "mode": r["mode"], "fallback_reason": r["fallback_reason"],
                 "verify_rc": r["verify_rc"], "verify_seconds": r["verify_seconds"], "audit_ok": r["audit_ok"],
                 "session_status": r["session_status"], "problems": r["problems"], "checks": r["checks"],
                 "records": r["records"], "scalars_verified": r["verified_records"] == r["records"],
                 "verified_records": r["verified_records"], "replays": r["replays"], "replays_identical": r["replays_identical"],
                 "replay_details": r["replay_details"], "auditor_binary_sha256": r["auditor_binary_sha256"],
                 "auditor_env_class_id": r["auditor_env_class_id"], "auditor_hardware_class_id": r["auditor_hardware_class_id"],
                 "receipt": "replay-receipts/" + os.path.basename(rcpt[0]) if rcpt else None,
                 "receipt_sha256": r["receipt_sha256"], "files_match_phase_b_audit": r["files_match_phase_b_audit"]})
ok_runs = all(x["audit_ok"] and x["archive_sha256_match"] and x["verify_rc"] == 0 and x["replays"] == x["replays_identical"]
              and x["scalars_verified"] and x["files_match_phase_b_audit"] and not x["problems"] for x in runs)
strata = sorted({"|".join(s.split("-")[:2]) for s in sample})
all_strata = sorted({"|".join(d["spec"].split("-")[:2]) for d in prog if d.get("status") == "done"})
tot = {"sessions": len(runs), "records_audited": sum(x["records"] for x in runs),
       "records_scalar_verified": sum(x["verified_records"] for x in runs),
       "measured_runs_replayed": sum(x["replays"] for x in runs),
       "replays_identical": sum(x["replays_identical"] for x in runs),
       "sessions_replay_all": sum(x["mode"] == "replay-all" for x in runs),
       "sessions_replay_50_fallback": [x["run"] for x in runs if x["mode"] != "replay-all"],
       "all_archive_hashes_match": all(x["archive_sha256_match"] for x in runs),
       "all_receipt_file_maps_match_phase_b_audit": all(x["files_match_phase_b_audit"] for x in runs),
       "problems": sum(len(x["problems"]) for x in runs)}
env_ids = sorted({x["auditor_env_class_id"] for x in runs})
hw_ids = sorted({x["auditor_hardware_class_id"] for x in runs})
srs = {d["session"]: d for d in sess}
cert = {
    "schema": "validation-gate/replay-certificate/v1",
    "status": "verified" if (ok_runs and len(runs) == len(sample)) else "failed",
    "independent": True,
    "scalars_verified": bool(ok_runs and all(d["scalar_check_failures"] == 0 for d in sess)),
    "study": "research/volcano_ecdlp_census_followup_20261023",
    "validation_id": VAL_ID,
    "snapshot_commit": SNAP,
    "replayed_at_snapshot": SNAP,
    "replay_started_utc": started,
    "what_was_replayed": ("Each sampled session was extracted from its committed archive sessions/<session>.tar.xz (SHA-256 checked "
                          "against runs/progress.jsonl) into a scratch directory, one at a time, and audited with `ecbench verify --dir "
                          "<extracted> --replay-all`; the fallback `--replay 50` applies only if --replay-all exceeded 1800 s (field mode). "
                          "The audit re-derives the session from its own files (file hashes, spec and plan identity, seeds, execution order, "
                          "record seals and count), re-checks every recovered logarithm [k]G = Q on every record (warm-up included) and "
                          "re-executes every measured deterministic run, requiring identical answer, total, phases, counters and factor base. "
                          "The receipt's file map and session id were compared with the committed Phase B audit receipt "
                          "(sessions/<session>.audit.json). The extracted copy was deleted after each session (free disk about 0.5 GB)."),
    "independence": {
        "replayed_by": "independent validator (did not write the study, its specs, sessions, driver, analysis or report); started from committed archives only",
        "same_binary": True, "same_vm_type_cpu_and_isa": True,
        "class_ids": {"session_header_binary": header["binary_sha256"],
                      "phase_b_audit_env_classes": pba["env_classes"], "phase_b_audit_hardware_classes": pba["hw_classes"],
                      "replay_env_classes": env_ids, "replay_hardware_classes": hw_ids},
        "honest_scope": ("Same shared 4-vCPU cloud VM type and the same ecbench binary as the sessions; ecbench's env and hardware class "
                         "ids of the replay differ from those of the Phase B audits (which already span several). Operation counts do not "
                         "depend on the host, so this is an independent re-execution of the counts, not an independent implementation "
                         "of the solvers or independent hardware. The [k]G = Q check of every record was additionally redone with "
                         "validator-written affine arithmetic (validation/rederive.py), which shares no code with ecbench.")},
    "binary": {"path": "target/release/ecbench", "sha256": "ac4bc5b26f858a4d8a374208297157840af25fb1dc25ea1c19dd41360735fb35",
               "progress_header_binary_sha256": header["binary_sha256"],
               "matches_progress_header": header["binary_sha256"] == "ac4bc5b26f858a4d8a374208297157840af25fb1dc25ea1c19dd41360735fb35",
               "checked_before_each_replayed_session": all(r["binary_sha256_checked"] == header["binary_sha256"] for r in replay)},
    "sample": {"method": ("validation/draw_sample.sh: forced V5-D-L0-c0, V5-N-L0-c0, V5-ctl-L0-c0 (every V5 crater session with rho-cubic "
                          "outside Tcr, i.e. the positive-control sessions); one session per (volcano x family) stratum, index = "
                          "SHA-256(nonce|V|family) mod n over the stratum sorted by name; extra Tcr draws SHA-256(nonce|V5|Tcr|k) until "
                          "3 Tcr sessions; extra M draws until 2 M sessions; one of the three redone .r2 sessions, SHA-256(nonce|r2) mod 3"),
               "nonce": SNAP, "nonce_is": "branch HEAD at validation start, fixed before the draw",
               "sessions_in_campaign": len([d for d in prog if d.get("status") == "done"]), "sessions_replayed": len(runs),
               "drawn": sample, "strata_in_campaign": all_strata, "strata_covered": strata,
               "every_stratum_covered": strata == all_strata,
               "tcr_sessions": [s for s in sample if "-Tcr-" in s], "m_sessions": [s for s in sample if "-M-" in s],
               "v5_crater_rho_cubic_sessions": ["V5-D-L0-c0", "V5-N-L0-c0", "V5-ctl-L0-c0"],
               "redone_session": [s for s in sample if s.endswith(".r2")]},
    "runs": runs,
    "totals": tot,
    "raw_record_rederivation": {
        "what": ("For all 147 sessions (not only the sample), every record of records.jsonl was streamed out of the committed archive "
                 "(archive SHA-256 checked against runs/progress.jsonl) and parsed by validation/rederive.py: [recovered]G == Q, Q on "
                 "the curve and [r]G == O with validator-written affine arithmetic; S == total_gae / sqrt(r); the A2.1 uncharged count "
                 "re-derived from counters / phase natives; then validation/compare_extracts.py compared every row with "
                 "runs/extract/<session>.csv.xz and runs/uncharged/<session>.csv.xz."),
        "sessions": len(sess), "records": sum(d["records"] for d in sess), "measured": sum(d["measured"] for d in sess),
        "warmup": sum(d["warmup"] for d in sess),
        "archive_sha256_all_match": all(d["archive_sha256_match"] for d in sess),
        "records_equal_progress_runs": all(d["records_match_progress_runs"] for d in sess),
        "scalar_check_failures": sum(d["scalar_check_failures"] for d in sess),
        "s_recompute_failures": sum(d["s_recompute_failures"] for d in sess),
        "unverified_measured": sum(d["unverified_measured"] for d in sess),
        "binary_mismatch_records": sum(d["binary_mismatch_records"] for d in sess),
        "extract_and_uncharged_comparison": cmpx},
    "phase_b_audits_all_sessions": {"receipts": pba["audits"], "all_ok_complete_and_bound_to_session": not pba["bad"],
                                    "replays": pba["replays"], "replays_identical": pba["replays_identical"],
                                    "records": pba["records"]},
    "session_provenance": {"what": ("validation/session_provenance.py: session.json and spec.json of every archive; spec.json equals the "
                                    "committed specs/<spec>.json, binary sha256 frozen, status complete, records_written = progress runs; "
                                    "git_dirty is true for 146 of 147 sessions because the driver commits and pushes per session while the "
                                    "campaign files change; the measuring binary is pinned by sha256 and unchanged"),
                           "result": {k: v for k, v in prov.items() if k != "git_commits"}},
    "orphan_partial_sessions": {"what": "runs/orphans/*.orphan.tar.xz (container restarts); each partial record compared with the same seq of the redone .r2 session",
                                "result": orph},
    "receipts": {"directory": "replay-receipts/", "format": "ecbench.audit/v1 receipts, xz -9"},
}
os.makedirs(f"{BENCH}/replay-receipts", exist_ok=True)
for f in glob.glob(f"{SCR}/receipts/*.json.xz"):
    shutil.copy(f, f"{BENCH}/replay-receipts/")
json.dump(cert, open(f"{BENCH}/replay-certificate.json", "w"), indent=1)

# -------------------------------------------------------------- raw data
raw = []
for d in prog:
    if d.get("status") == "done":
        s = os.path.basename(d["out"])
        raw.append({"path": f"{REL}/sessions/{s}.tar.xz", "sha256": d["sha256_tar_xz"]})
for sub in ("runs/extract", "runs/uncharged"):
    for f in sorted(glob.glob(f"{STUDY}/{sub}/*.csv.xz")):
        raw.append({"path": f"{REL}/{sub}/{os.path.basename(f)}", "sha256": sha(f)})
for f in ["runs/progress.jsonl", "runs/transfer_cost.jsonl", "runs/manifest.json", "census/V5/vertices.jsonl",
          "census/V6/vertices.jsonl", "census/V5/M.json", "census/V6/M.json", "census/V6/sources.json", "census/V5/sources.json",
          "ANALYSIS_PLAN.md", "PLAN.md", "AMENDMENTS.md", "REPORT.md", "analysis/results.json"] + \
         sorted(os.path.relpath(p, STUDY) for p in glob.glob(f"{STUDY}/census/V5/tcr/*.json")):
    raw.append({"path": f"{REL}/{f}", "sha256": sha(f"{STUDY}/{f}")})
for f in ("draw_sample.sh", "replay_sample.sh", "rederive.py", "vfu_data.py", "compare_extracts.py", "recompute.py",
          "hm_precise.py", "mixed_check.py", "report_check.py", "extras.py", "phase_b_checks.py", "session_provenance.py", "build_validation.py", "write_report.py", "run_all.sh"):
    raw.append({"path": f"{REL}/validation/{f}", "sha256": sha(f"{STUDY}/validation/{f}"), "role": "validator code"})

json.dump({"headline": H, "summary": summary, "raw": raw, "cert_status": cert["status"]},
          open(f"{SCR}/build_parts.json", "w"), indent=1)
print(json.dumps(summary, indent=1))
print("cert", cert["status"], cert["scalars_verified"], tot)
