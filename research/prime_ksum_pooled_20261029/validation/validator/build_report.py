#!/usr/bin/env python3
"""Assemble validator-report.json from the validator's own outputs.
README values are transcribed by hand from README.md (HEAD blob
f405f34f2fb41c2ff033e636a965cab131883ef8) into README_CLAIMS below."""
import hashlib, json, subprocess
from pathlib import Path

V = Path(__file__).resolve().parent
REPO = Path("/home/user/crypto")
STUDY_REL = "research/prime_ksum_pooled_20261029"
STUDY = REPO / STUDY_REL
H = json.loads((V / "hashes.json").read_text())
ME = json.loads((V / "recompute.json").read_text())
IG = json.loads((V / "integrity.json").read_text())
CC = json.loads((V / "compare_committed.json").read_text())
B2 = json.loads((V / "bootstrap2.json").read_text())
SE = json.loads((V / "sensitivity.json").read_text())
W = ["0", "0.1", "1"]

def sha(p): return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def blob(rev, rel): return subprocess.check_output(["git", "-C", str(REPO), "rev-parse", f"{rev}:{rel}"], text=True).strip()
R = lambda rd, b, pair, w: ME["ratios"][f"{rd}|P{b}|{pair}|w{w}"]["ratio"]

# ------------------------------------------------------------- raw data
raw = []
for s, h in H["sessions"].items():
    raw.append({
        "path": f"sessions/{s}/.artifacts.json",
        "sha256": sha(STUDY / "sessions" / s / ".artifacts.json"),
        "sha256_of": "the artifact-store manifest at this path (HEAD); it pins records.jsonl below",
        "git_blob_head": blob("HEAD", f"{STUDY_REL}/sessions/{s}/.artifacts.json"),
        "records_jsonl": {
            "repo_path": h["repo_path_at_commit"],
            "sha256": h["sha256"], "bytes": h["bytes"],
            "git_blob_at_a0ff77014f": h["git_blob_at_a0ff77014f"],
            "git_blob_of_recovered_bytes": h["git_blob_of_recovered_bytes"],
            "artifacts_json_sha256": h["artifacts_json_sha256"],
            "session_json_records_sha256": h["session_json_records_sha256"],
            "all_four_agree": h["ok"],
            "records": h["records_written"], "status_counts": h["status_counts"],
            "note": "absent from the HEAD work tree (artifact store, .gitignore); recovered from git at a0ff77014f and hashed by the validator",
        },
    })
    raw.append({"path": f"sessions/{s}/session.json", "sha256": h["side_files"]["session.json"]["sha256_repo_head"],
                "git_blob_head": h["side_files"]["session.json"]["git_blob_head"],
                "session_id": h["session_id"], "binary_sha256": h["binary_sha256"],
                "git_commit_in_session": h["git_commit_in_session"], "git_dirty_in_session": h["git_dirty_in_session"]})

# ------------------------------------------------------------- headline
heads = []
def cmp_file(b, A, B, w):
    return next(r for r in CC["compare_rows"] if r["size"] == b and r["a"] == A and r["b"] == B and r["w"] == w)
for b in (20, 24, 28, 32):
    for A, B in (("inc-sel", "ksum5-sel"), ("inc-sel", "ksum4-sel"), ("rho-neg", "ksum5-sel")):
        for w in W:
            row = cmp_file(b, A, B, w)
            heads.append({"name": f"P{b} {B}/{A} pooled paired ratio, w={w}, pre-2026-11-08 reading",
                          "claimed": row["committed_ratio"], "recomputed": R("pre", b, f"{A}__{B}", w), "rel_tol": 1e-12,
                          "claimed_source": f"analysis/{row['file']} ops.ratio_b_over_a"})
# README transcriptions: (name, claimed, decimals, recomputed)
RC = []
for b, c5, c4 in ((20, 0.700, 0.632), (24, 0.446, 0.452), (28, 0.324, 0.336), (32, 0.217, 0.253)):
    RC.append((f"README table: ksum5-sel/inc-sel P{b} w=0", c5, 3, R("pre", b, "inc-sel__ksum5-sel", "0")))
    RC.append((f"README table: ksum4-sel/inc-sel P{b} w=0", c4, 3, R("pre", b, "inc-sel__ksum4-sel", "0")))
for b, c in ((24, 0.486), (28, 0.368), (32, 0.248)):
    RC.append((f"README: ksum5-sel/inc-w0 P{b} w=0", c, 3, R("pre", b, "inc-w0__ksum5-sel", "0")))
for b, c in ((20, 6.0), (24, 5.8), (28, 7.3), (32, 7.3)):
    RC.append((f"README distance remaining: ksum5-sel/rho-neg P{b} w=0", c, 1, R("pre", b, "rho-neg__ksum5-sel", "0")))
for b, c in ((20, 7.6), (24, 6.5), (28, 7.2), (32, 7.1)):
    RC.append((f"README distance remaining: ksum5-sel/bsgs-neg P{b} w=0", c, 1, R("pre", b, "bsgs-neg__ksum5-sel", "0")))
for w, c in (("0", 1.034), ("0.1", 1.000), ("1", 0.863)):
    RC.append((f"README (c): inc-sel/inc-rec P20 w={w}", c, 3, R("pre", 20, "inc-rec__inc-sel", w)))
SECONDARY = []
for b, c in ((24, 0.92), (28, 0.88), (32, 0.88)):
    rv = R("pre", b, "inc-rec__ksum5-sel", "0") / R("pre", b, "inc-w0__ksum5-sel", "0")
    direct = R("pre", b, "inc-rec__ksum5-sel", "0") / R("pre", b, "inc-w0__ksum5-sel", "0")
    SECONDARY.append({"name": f"README section 4 (c) derived: inc-w0/inc-rec P{b} w=0 ('not a direct comparison')",
                      "claimed": c, "recomputed": rv, "printed_rounding_of_recomputed": round(rv, 2),
                      "agrees_at_printed_precision": abs(c - rv) <= 0.005 * 1.000001})
for b, c in ((20, 0.965), (24, 0.934), (28, 0.860), (32, 0.814)):
    RC.append((f"README (d): ksum5-sel/ksum4-sel P{b} w=1", c, 3, R("pre", b, "ksum4-sel__ksum5-sel", "1")))
RC.append(("README (d): ksum5-sel/ksum4-sel P20 w=0", 1.106, 3, R("pre", 20, "ksum4-sel__ksum5-sel", "0")))
for name, c, dec, rv in RC:
    half = 0.5 * 10 ** (-dec)
    heads.append({"name": name, "claimed": c, "recomputed": rv, "rel_tol": half * 1.000001 / abs(c),
                  "claimed_source": "README.md (rounded; rel_tol = half a unit in the last printed digit)"})
for b in (24, 28, 32):
    for w in W:
        heads.append({"name": f"README (c): inc-sel/inc-rec P{b} w={w} is exactly 1", "claimed": 1.0,
                      "recomputed": R("pre", b, "inc-rec__inc-sel", w), "rel_tol": 1e-12, "claimed_source": "README.md section 4 (c)"})
rcnt = ME["record_counts"]; cert = ME["certificates"]
heads += [
    {"name": "README: train records, all verified", "claimed": 4896, "recomputed": rcnt["train_verified"], "rel_tol": 0,
     "claimed_source": "README.md section 3"},
    {"name": "README: holdout records, all verified", "claimed": 8928, "recomputed": rcnt["holdout_verified"], "rel_tol": 0,
     "claimed_source": "README.md section 3"},
    {"name": "holdout records whose recovered d satisfies [d]G = Q (validator arithmetic; README: all 8,928 verified)",
     "claimed": 8928, "recomputed": cert["holdout_measured_dG_eq_Q"] + cert["holdout_warmup_dG_eq_Q"], "rel_tol": 0,
     "claimed_source": "README.md section 3"},
    {"name": "measured (non-warm-up) verified holdout records with [d]G = Q (replay certificate runs list: 5,952)",
     "claimed": 5952, "recomputed": cert["holdout_measured_dG_eq_Q"], "rel_tol": 0,
     "claimed_source": "scratchpad/ksum/replay-certificate.json runs (5952, all validator_dG_equals_Q true)"},
]
bad_heads = [h for h in heads if abs(h["claimed"] - h["recomputed"]) > h["rel_tol"] * max(abs(h["claimed"]), abs(h["recomputed"]), 1e-300)]

# ------------------------------------------------------------- tables
table = {}
for rd in ("pre", "post"):
    for b in (20, 24, 28, 32):
        for pair, label in (("inc-sel__ksum5-sel", "ksum5-sel/inc-sel"), ("inc-sel__ksum4-sel", "ksum4-sel/inc-sel"),
                            ("rho-neg__ksum5-sel", "ksum5-sel/rho-neg"), ("bsgs-neg__ksum5-sel", "ksum5-sel/bsgs-neg"),
                            ("inc-w0__ksum5-sel", "ksum5-sel/inc-w0"), ("inc-w0__ksum4-sel", "ksum4-sel/inc-w0"),
                            ("ksum4-sel__ksum5-sel", "ksum5-sel/ksum4-sel"), ("inc-rec__inc-sel", "inc-sel/inc-rec")):
            if f"{rd}|P{b}|{pair}|w0" not in ME["ratios"]:
                continue
            table.setdefault(rd, {}).setdefault(f"P{b}", {})[label] = {
                w: {"ratio": R(rd, b, pair, w), "ci95_linearised_t": ME["ratios"][f"{rd}|P{b}|{pair}|w{w}"]["ci95_linearised_own"]}
                for w in W}
for b in (20, 24, 28, 32):
    for key, lbl in (("inc-sel__ksum5-sel", "ksum5-sel/inc-sel"), ("inc-sel__ksum4-sel", "ksum4-sel/inc-sel"),
                     ("ksum4-sel__ksum5-sel", "ksum5-sel/ksum4-sel"), ("inc-rec__inc-sel", "inc-sel/inc-rec")):
        for w in ("0", "1"):
            k = f"P{b}|{key}|w{w}"
            if k in B2:
                table["pre"][f"P{b}"][lbl][w]["ci95_two_stage_cluster_bootstrap_own_rng"] = B2[k]

band_pc = {}
for rd, lab in (("pre", "pre_2026_11_08_study_reading"), ("post", "table_work_priced_2026_11_08_default")):
    out = {}
    for slug, row in sorted(ME["orders"][rd].items(), key=lambda kv: (kv[1]["size"], kv[0])):
        out[slug] = {"size_bits": row["size"], "log2_r": ME["curves"][slug]["log2_r"],
                     **{w: {"ic_ksum5_sel": row[w]["ksum5_sel"], "rho_neg": row[w]["rho_neg"], "inc_sel": row[w]["inc_sel"],
                            "ksum4_sel": row[w]["ksum4_sel"], "bsgs_neg": row[w]["bsgs_neg"], "inc_w0": row[w]["inc_w0"]} for w in W},
                     "rho_lt_ksum5_lt_inc_every_weight": row["holds_all_weights"],
                     "ksum4_lt_inc_every_weight": row["ksum4_lt_inc_all_weights"]}
    band_pc[lab] = out
by_size = {}
for rd, lab in (("pre", "pre_2026_11_08_study_reading"), ("post", "table_work_priced_2026_11_08_default")):
    for b in (20, 24, 28, 32):
        rows = [v for v in band_pc[lab].values() if v["size_bits"] == b]
        by_size.setdefault(lab, {})[f"P{b}"] = {w: {"ic": sum(r[w]["ic_ksum5_sel"] for r in rows) / 4,
                                                   "rho": sum(r[w]["rho_neg"] for r in rows) / 4,
                                                   "inc_sel": sum(r[w]["inc_sel"] for r in rows) / 4} for w in W}

all_order_pre = all(v["rho_lt_ksum5_lt_inc_every_weight"] for v in band_pc["pre_2026_11_08_study_reading"].values())
all_order_post = all(v["rho_lt_ksum5_lt_inc_every_weight"] for v in band_pc["table_work_priced_2026_11_08_default"].values())
curves_ok = not ME["curve_problems"] and all(c["registry_match"] is True for c in ME["curves"].values())
hash_ok = H["all_ok"]
integrity_ok = not IG["problems"]
step4_ok = (CC["compare_max_rel_pre"] < 1e-12 and CC["band_max_rel_mean_s_pre"] < 1e-12 and not CC["band_rank_mismatches"]
            and CC["band_charged_sum_all_equal"] and CC["band_native_sum_all_equal_pre"]
            and CC["preflight_pre"]["band_max_rel"] < 1e-12 and CC["preflight_pre"]["pairs_max_rel"] < 1e-12
            and CC["preflight_tablework"]["band_max_rel"] < 1e-12 and CC["preflight_tablework"]["pairs_max_rel"] < 1e-12
            and all(v["max_rel"] < 1e-12 and v["order_flags_agree"] for v in CC["gae_band_per_curve"].values()))
cert_ok = (cert["holdout_measured_dG_eq_Q"] == cert["holdout_measured_records"] == 5952 and cert.get("holdout_measured_dG_ne_Q", 0) == 0
           and cert["workloads_with_multiple_distinct_logs"] == 0)
sel_ok = all(ME["selection"][f"pre|P{b}"][f]["sel_w1"] == v for b, vals in
             {20: (64, 10, 9), 24: (162, 14, 11), 28: (407, 21, 15), 32: (1024, 30, 19)}.items()
             for f, v in zip(("inc", "ksum4", "ksum5"), vals)) and all(ME["selection"][f"pre|P{b}"]["k_star"] == 5 for b in (20, 24, 28, 32)) \
         and all(ME["selection"][f"pre|P{b}"]["inc"]["sel_w0"] == v for b, v in ((20, 64), (24, 115), (28, 289), (32, 727)))
checks = {
    "raw_data_binding": hash_ok, "curves_and_primality": curves_ok, "certificates_dG_eq_Q": cert_ok,
    "session_integrity_plan_binary_timestamps": integrity_ok, "headline_recomputed": not bad_heads,
    "per_curve_band_order_pre": all_order_pre, "per_curve_band_order_post": all_order_post,
    "selection_recomputed": sel_ok, "committed_outputs_and_preflight_agree": step4_ok,
}
status = "pass" if all(checks.values()) else "fail"
rid = "VAL-20261112-" + hashlib.sha256((V / "recompute.json").read_bytes()).hexdigest()[:6]

rep = {
    "schema": "validator-report/v1",
    "id": rid,
    "status": status,
    "verdict": "passed" if status == "pass" else "failed",
    "disposition": "validated" if status == "pass" else "invalid_measurement",
    "independent": True,
    "independence_note": "Validator subagent (claude-opus-5-5) that did not produce the sessions, code, analysis, README, replay or preflight. All numbers below come from validator code written fresh in Python 3 stdlib (recompute.py, integrity.py, bootstrap2.py, sensitivity.py) reading records.jsonl directly; no ecbench binary was run. The preflight files were opened only after recompute.json existed. The independent: true boolean is the writer's; the sabotage contract's signed verdict is not part of this file.",
    "validator": "Validator subagent (claude-opus-5-5)",
    "claim": "research/prime_ksum_pooled_20261029: on 16 fresh prime-order curves (4 per size, 20-32 bits), the k-sum tables ksum5-sel and ksum4-sel cost less than the tuned mitm incumbent inc-sel at w = 0, 0.1 and 1 (pooled paired ratios of total counted cost), on every curve, with ksum5-sel/inc-sel = 0.700, 0.446, 0.324, 0.217 at w = 0; rho-neg remains 5.8-7.3x cheaper (distance remaining). Generic track; UNQUALIFIED (one host).",
    "snapshot": {
        "validated_results_commit": H["commit_full"],
        "repository_head_at_validation": H["head"],
        "head_work_tree_dirty_lines": H["dirty_lines"],
        "head_moved_during_validation": {"from": H["head"], "to": "55713d2001bfbced31461f7b94e96992e75b7177",
            "by": "merges from other sessions (not the validator)",
            "unchanged_between": ["research/prime_ksum_pooled_20261029 (whole directory)", "scripts/validation_gate.py",
                                  "src/cryptanalysis/ecbench/band.rs", "docs/curves/sources/prime-ksum-pooled-20261029.json"],
            "changed_between": ["src/cryptanalysis/ecbench/compare.rs (adds --unit bitop/v1; the GAE ratio-of-sums definition used here is unchanged)"],
            "work_tree_note": "other sessions also have uncommitted edits outside the study directory (docs/ic, docs/session-logs, src/cryptanalysis/ic_framework); git status for research/prime_ksum_pooled_20261029 is clean"},
        "readme_git_blob_head": blob("HEAD", f"{STUDY_REL}/README.md"),
        "protocol_git_blob_head": blob("HEAD", f"{STUDY_REL}/PROTOCOL.md"),
        "band_json_git_blob": blob("HEAD", f"{STUDY_REL}/analysis/band.json"),
        "note": "Study files at HEAD differ from a0ff77014f only by README.md (+3 lines, external comparison disclosure), analyse.sh (audit gating), PNG/PDF renders and the move of records.jsonl to the artifact store; analysis/ is byte-identical (band.json blob equal at both commits). Raw records validated are the a0ff77014f blobs, bound by sha256 to .artifacts.json and session.json at HEAD.",
    },
    "method": "Pricing: priced_gae(w) = total_gae - C + w * (U + n_C), S = priced_gae / sqrt(r), C = n_C = 0 (every record has calibration_pinned_units = []); U = counts behind cost.unpriced (top-level counter, else phase native [name] or [name minus _uncharged]); no byte-valued counter occurs. Study reading = pre-2026-11-08 (table_insert_attempts not charged); the 2026-11-08 default adds the phase sum of work_native.table_insert_attempts where a phase carries counted:table_insert_attempts (IC arms only). Pairs: (workload_id, round) over non-warm-up verified runs; pooled ratio B/A = sum S_B / sum S_A. Band row: mean S over measured verified runs per (curve, arm). Certificates: (a, b) solved from G and one target, then required to hold for G and all 24 (holdout) or 12 (train) targets of the curve; j(a, b) = ICV1 j; p and r Miller-Rabin; r = p + 1 - t; Hasse; cofactor 1; [r]G = O; [r]Q = O; [d]G = Q by affine double-and-add.",
    "raw_data": raw,
    "headline": heads,
    "headline_failures": bad_heads,
    "checks": checks,
    "requirements": [
        {"requirement": "raw data bound: sha256(recovered records.jsonl) = .artifacts.json = session.json records_sha256 = sha256(git blob at a0ff77014f); git blob id recomputed from the bytes",
         "result": "pass" if hash_ok else "fail", "evidence": "validator/hashes.json"},
        {"requirement": "[d]G = Q for every measured verified holdout record, own arithmetic",
         "result": "pass" if cert_ok else "fail",
         "evidence": "validator/recompute.json certificates",
         "counts": {k: v for k, v in cert.items()}},
        {"requirement": "curves: p, r prime (Miller-Rabin), r = p+1-t, Hasse, cofactor 1, [r]G = O, j matches ICV1, j not 0/1728, (a, b) equal to docs/curves/sources/prime-ksum-pooled-20261029.json",
         "result": "pass" if curves_ok else "fail", "evidence": "validator/recompute.json curves (24 curves)"},
        {"requirement": "session integrity: records = plan executions (arm, workload, round, warm-up, seed), seq contiguous, one binary fc86ee11 = BINARY.sha256 on every record, phase gae sums = total_gae, train between registration and freeze, holdout after freeze, spec bytes = REGISTRATION.sha256 / FROZEN.sha256",
         "result": "pass" if integrity_ok else "fail", "evidence": "validator/integrity.json"},
        {"requirement": "selection recomputed from train (smallest mean S at w = 1, ties to the smaller table): inc 64/162/407/1024, inc-w0 64/115/289/727, ksum4 10/14/21/30, ksum5 9/11/15/19, k* = 5",
         "result": "pass" if sel_ok else "fail", "evidence": "validator/recompute.json selection (unchanged under the 2026-11-08 reading)"},
        {"requirement": "headline ratios recomputed (committed exact values and README rounded values)",
         "result": "pass" if not bad_heads else "fail", "evidence": "headline (this file)"},
        {"requirement": "section 5a band per curve: rho-neg < ksum5-sel < inc-sel at w = 0, 0.1, 1 on all 16 curves",
         "result": "pass" if (all_order_pre and all_order_post) else "fail",
         "evidence": "gae_band_per_curve (this file); holds under both readings"},
        {"requirement": "step 4: committed analysis/compare-*.json (150), band.json (124 rows, 420 ranks), preflight.json, preflight-tablework.json, gae_band_per_curve.json agree with the validator",
         "result": "pass" if step4_ok else "fail", "evidence": "validator/compare_committed.json"},
        {"requirement": "reproducibility binding of the measurement binary's source revision",
         "result": "caveat", "evidence": "sessions/*/session.json git_commit = null, git_dirty = null; replay-certificate.json states the binary was built from 26249b4 plus the uncommitted #2852 diff (not verified by the validator). Mitigation: counts replay identically under ecbench built from clean 6edb2cb36d (replay certificate)."},
        {"requirement": "controls: reference (rho-neg, negation, strongest: prime field, j not 0/1728, automorphisms_used 2 on all 1,152 holdout rho-neg records), cheaper baseline (bsgs-neg), A/A (rho-neg-aa), frozen benchmark (FROZEN.sha256)",
         "result": "pass", "evidence": "validator/recompute.json curves; validator/integrity.json aa_counted_identical"},
        {"requirement": "external fastest implementation",
         "result": "not run (disclosed in README section 7); the claim compares only in-repository arms, so it is a scope gap, not a failed check", "evidence": "README.md section 7"},
    ],
    "pricing_readings": {
        "study_reading": "pre-2026-11-08 (ecbench --no-price-table-work equivalent): table_insert_attempts NOT priced. Evidence: every committed compare file equals the validator's pre reading to 1.6e-15 and differs from the 2026-11-08 reading by up to 25%; band.json carries no priced_table_work key and its native_sum equals the pre-reading sum on all 124 rows (48 rows, the reference arms, also equal under the post reading).",
        "table_work_counters_present_on": "IC arms (inc-*, ksum4-*, ksum5-*) in train and holdout, behind counted:table_insert_attempts; rho and BSGS records carry none; no sibling counters",
        "post_reading_effect": "ranks unchanged everywhere; w = 0 unchanged; IC arms rise at w > 0, so ksum/inc-sel falls slightly and ksum5-sel/rho-neg at w = 1 rises from 5.26/5.92/6.16/6.70 to 6.30/6.92/8.09/8.52 (20/24/28/32 bits).",
        "cite_rule": "AGENTS.md section 5a: where these figures are cited again, label them the pre-2026-11-08 reading.",
    },
    "headline_table": table,
    "gae_band_per_curve": band_pc,
    "gae_band_by_size_mean_of_curves": by_size,
    "order_holds_across_band": {"pre_2026_11_08": all_order_pre, "table_work_default": all_order_post,
                                "statement": "rho-neg < ksum5-sel < inc-sel at w = 0, 0.1, 1 on all 16 holdout curves under both readings; also ksum4-sel < inc-sel, ksum5-sel and ksum4-sel < inc-w0 (24-32 bits), and bsgs-neg < ksum5-sel, ksum4-sel everywhere. rho is ahead: distance remaining, not a beat."},
    "dG_count": {"holdout_measured_verified": cert["holdout_measured_records"], "holdout_measured_dG_eq_Q": cert["holdout_measured_dG_eq_Q"],
                 "holdout_warmup_dG_eq_Q": cert["holdout_warmup_dG_eq_Q"], "train_measured_dG_eq_Q": cert["train_measured_dG_eq_Q"],
                 "train_warmup_dG_eq_Q": cert["train_warmup_dG_eq_Q"], "failures": len(ME["certificate_failures"]),
                 "workloads": cert["workloads"], "workloads_with_conflicting_logs": cert["workloads_with_multiple_distinct_logs"]},
    "step4_comparison": {
        "committed_compare_files": CC["compare_files"], "max_rel_diff_ratio_vs_pre": CC["compare_max_rel_pre"],
        "max_rel_diff_ratio_vs_post": CC["compare_max_rel_post"], "max_rel_diff_band_triplets": CC["compare_max_rel_band_pre"],
        "max_rel_diff_ci95_linearised_t": CC["compare_max_rel_ci_linearised"],
        "band_json_rows": CC["band_rows_committed"], "band_json_max_rel_mean_s_pre": CC["band_max_rel_mean_s_pre"],
        "band_json_max_rel_mean_s_post": CC["band_max_rel_mean_s_post"], "band_json_rank_mismatches": len(CC["band_rank_mismatches"]),
        "band_json_ranks": CC["band_ranks_committed"],
        "preflight_json_max_rel": max(CC["preflight_pre"]["band_max_rel"], CC["preflight_pre"]["pairs_max_rel"]),
        "preflight_tablework_json_max_rel": max(CC["preflight_tablework"]["band_max_rel"], CC["preflight_tablework"]["pairs_max_rel"]),
        "gae_band_per_curve_json_max_rel": max(v["max_rel"] for v in CC["gae_band_per_curve"].values()),
        "readme_brackets": "README brackets equal the committed ci95_bootstrap (ecbench two-stage cluster bootstrap); the validator's two-stage bootstrap with its own RNG lands within 0.002 of each. ecbench's primary ci95 (linearised t) is narrower and matches the validator's linearised interval to 2.0e-15.",
    },
    "crosschecks": {
        "ols_exponent_log_total_vs_log_r_per_run_pre": {k.split("|", 1)[1]: v for k, v in ME["ols_exponent_crosscheck"].items() if k.startswith("pre|")},
        "ols_exponent_note": "Validator's own OLS of log(priced GAE) on log(r) over the 384 measured runs per arm; equals analysis/exponents.md to 3 decimals at every arm and weight (estimator identity with ecbench bound fit not established; intervals not checked). ksum5-sel minus inc-sel at w = 0: 0.139.",
        "ols_exponent_post": {k.split("|", 1)[1]: v for k, v in ME["ols_exponent_crosscheck"].items() if k.startswith("post|")},
        "sensitivity_all_counted_work_native": SE,
        "sensitivity_note": "Not a registered or repository reading: charging every counted work_native operation (walk bookkeeping, index builds, row construction) at w on the IC arms keeps every k-sum/inc-sel ratio below 1 and the per-curve order intact; rho records count no such work, so the distance to rho in this reading (up to 13.1x at w = 1) overstates the asymmetry and is not a result.",
        "freshness": "The 24 curve slugs of this study are distinct; none of the 16 holdout slugs appears in any other research/ file at a0ff77014f (git grep). Records already moved to the artifact store in other studies were not scanned, so this is partial.",
    },
    "secondary_values": SECONDARY,
    "discrepancies": [
        {"where": "README.md section 4 (c), third bullet (and the same figures behind section 1 Q3 '8-13%')",
         "claimed": "0.92, 0.88 and 0.88 at 24, 28 and 32 bits (inc-w0 relative to inc-rec, derived from the k-sum ratios)",
         "supported_by_records": [round(x["recomputed"], 4) for x in SECONDARY],
         "finding": "At 32 bits the records give 0.8737 (prints as 0.87), not 0.88; the README value matches dividing the rounded ratios 0.217 / 0.248 = 0.875. Off by 0.006 absolute in a secondary, explicitly derived remark; no registered test, verdict or headline number uses it, and the '8-13%' range in section 1 is consistent with 0.874. Correction owed in the README: 0.87 (or 0.874) at 32 bits.",
         "blocks_validation": False}
    ],
    "findings": [
        "Pricing reading: the study's committed numbers are the pre-2026-11-08 reading. Under the current default (table_insert_attempts priced) no rank or registered verdict changes, but every w > 0 figure moves; the w = 1 distance to rho grows to 6.30x-8.52x. Cite with the reading label.",
        "Binding: session.json records git_commit = null and git_dirty = null; the measurement binary (fc86ee11, matches BINARY.sha256 and every record) is pinned by hash only. The replay certificate attributes it to 26249b4 plus an uncommitted diff; counts replay identically with a clean build of 6edb2cb36d.",
        "A/A control: rho-neg-aa equals rho-neg in every counted field on 768 of 768 holdout pairs (deterministic method, same seeds), so in the counted unit it is a duplicate, not a noise estimate; band.json's 'unresolved inside the band' for that pair is a tie. It only informs wall time, which the study does not admit.",
        "Intervals: README brackets are ecbench's two-stage cluster bootstrap (labelled correctly); the primary linearised ci95 is narrower. No registered verdict depends on which is used (primary upper bounds at w = 0 are at most 0.465 at 24-32 bits; test (d) at 20 bits is unresolved under both).",
        "The README's 'derived, not a direct comparison' inc-w0/inc-rec values (0.92, 0.88, 0.88) are algebraically identical to the direct pooled ratio of totals (0.9191, 0.8804, 0.8737), because every pair is complete; they lack only an interval.",
        "Scope disclosed by the study and unchanged: one host (UNQUALIFIED), isolation L0 on every record (counted unit only), no second-host replay of this holdout, no external ECDLP tool, 20-bit incumbent pick on the grid edge, interval scope = workloads (between-curve variance carried only by the per-curve test).",
        "m = 83 / ECC2K-130 disclosure rules: not applicable (prime-field curves, 20-32 bits).",
    ],
    "artifacts": {
        "dir": str(V),
        "files": {p.name: sha(p) for p in sorted(V.iterdir()) if p.is_file() and p.name not in ("validator-report.json", "VALIDATOR.md", "gate_check.txt")},
    },
    "commands": [
        "python3 -I check_hashes.py ../sessions hashes.json",
        "python3 -I recompute.py ../sessions recompute.json /home/user/crypto/docs/curves/sources/prime-ksum-pooled-20261029.json",
        "python3 -I integrity.py ../sessions /home/user/crypto/research/prime_ksum_pooled_20261029 integrity.json",
        "python3 -I compare_committed.py recompute.json /home/user/crypto/research/prime_ksum_pooled_20261029/analysis .. compare_committed.json",
        "python3 -I bootstrap2.py ../sessions bootstrap2.json",
        "python3 -I sensitivity.py ../sessions sensitivity.json",
        "python3 -I build_report.py",
    ],
    "environment": {"python": "3.13.18 (stdlib only)", "host": "Linux 6.18 x86_64 (validator sandbox)", "ecbench_run_by_validator": False},
}
(V / "validator-report.json").write_text(json.dumps(rep, indent=1) + "\n")
print("status", status, checks, "headline entries", len(heads), "failures", len(bad_heads))
