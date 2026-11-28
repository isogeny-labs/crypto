#!/usr/bin/env python3
"""Validator: write replay-certificate.json, validator-report.json and VALIDATION.json
(docs/VALIDATION_GATE.md, validation-gate/v1) for research/quartic_diem_p0lines_20261128.

Adapted from the parent validation's build_gate_artifacts.py (quartic_diem_nofb_20261126).

Usage: python3 -I build_gate_artifacts.py REPO SHA VALDIR

Reads from VALDIR: certificates.json (verify_certificates.py), recompute.json (recompute.py),
replay_compare.json (compare_replay.py), checks_extra.json (checks_extra.py),
docs_check.json (docs_check.py) and diag_check.json (diag_check.py); analysis.json,
explore.json and the raw rows from Git at SHA.  Every pass/fail flag is derived here from
those files; nothing is set by hand.  Paths in the three JSON outputs are relative to
VALDIR (the gate resolves them from the directory that holds VALIDATION.json).
"""
import hashlib
import json
import math
import os
import subprocess
import sys

STUDY = "research/quartic_diem_p0lines_20261128"
FILES = ["raw/run.jsonl", "raw/run131101.jsonl"]
NOFB, P0, P0I3 = "Q_diem_2lp_sfb_sge_nofb", "Q_diem_2lp_sfb_sge_p0", "Q_diem_2lp_sfb_sge_p0i3"
LAB = {NOFB: "nofb", P0: "p0", P0I3: "p0i3"}
NATIVE = ["rel_mul", "rel_add", "la_mul", "la_add", "idx_lookups", "idx_inserts"]
BINARY_SHA = "283c1510a9da8d735495a44aafe3685b174029987f10dc10adb0ecffb23723cd"
PRIMES = ["4099", "16411", "65537", "131101"]
P5, PA = "primary_seeds_5_6", "all_seeds_1_6"


def git(repo, *args):
    return subprocess.run(["git", "-C", repo, *args], check=True, capture_output=True).stdout


def sha256_bytes(b):
    return hashlib.sha256(b).hexdigest()


def gm(v):
    v = list(v)
    return math.exp(sum(math.log(x) for x in v) / len(v))


def main():
    repo, sha, vd = sys.argv[1:4]
    full_sha = git(repo, "rev-parse", sha).decode().strip()

    def load(n):
        with open(os.path.join(vd, n)) as fh:
            return json.load(fh)
    certs, rec, rep, ext, docs, dg = (load("certificates.json"), load("recompute.json"), load("replay_compare.json"),
                                      load("checks_extra.json"), load("docs_check.json"), load("diag_check.json"))
    ana = json.loads(git(repo, "show", f"{sha}:{STUDY}/analysis.json"))
    exj = json.loads(git(repo, "show", f"{sha}:{STUDY}/explore.json"))
    rows = [json.loads(l) for fn in FILES
            for l in git(repo, "show", f"{sha}:{STUDY}/{fn}").decode().splitlines() if l.strip()]

    # ---- replay certificate ----
    runs = rep["rows"]
    s = rep["summary"]
    replay_ok = (bool(runs) and all(e["found"] and e["bytes_equal"] and e["fields_equal"] for e in runs)
                 and s["runs_replayed"] == s["runs_certified_and_correct"] == 360
                 and s["rows_replayed"] == s["rows_bytes_equal"] == s["rows_fields_equal"]
                 and s["duplicate_replayed_keys"] == 0 and s["configs_byte_identical"]
                 and s["committed_rows"] == s["committed_rows_covered"] == s["rows_replayed"])
    bb = ext["binary_binding"]
    li, bi = bb["dep_info"]["lib"], bb["dep_info"]["bin"]
    cert = {
        "schema": "replay-certificate/v1",
        "study": STUDY,
        "status": "verified" if replay_ok else "failed",
        "independent": True,
        "scalars_verified": replay_ok,
        "validated_commit": full_sha,
        "binary": {"name": "genus_ladder.p0", "sha256": BINARY_SHA,
                   "note": ("hashed and copied at dispatch; equal to binary.sha256, the scratch path the frozen run "
                            "scripts call and target/release/genus_ladder. Not rebuilt (dispatch: no builds). Binding "
                            "to the freeze commit 7f67f384ece through cargo's own records: the bin fingerprint names "
                            f"the crypto_lib fingerprint {bb['lib_fingerprint_dirs_matching'][0]}, whose compile was "
                            f"invoked {bb['lib_compile']['invoked_utc']} (22 s after the freeze commit) and wrote its "
                            f"dep-info at {bb['lib_compile']['dep_info_written_utc']}; of the {li['n_source_files']} + "
                            f"{bi['n_source_files']} files rustc read, "
                            f"{li['unmodified_since_before_invocation_and_equal_freeze'] + bi['unmodified_since_before_invocation_and_equal_freeze']} "
                            "are unmodified since before the compile and byte-equal to 7f67f384ece, "
                            f"{li['rewritten_later_by_merge_changed_in_git'] + bi['rewritten_later_by_merge_changed_in_git']} were rewritten "
                            "after the build by the 03:45:46 / 04:00:32 fast-forwards (changed in Git since the freeze), "
                            "none was modified inside the build window; the reflog keeps HEAD at 7f67f384ece from "
                            "03:28:06 to 03:45:46 and the bench record says commit 7f67f384ece, commit_dirty false at "
                            "03:45:36. mtime and record evidence, weaker than a rebuild (checks_extra.json binary_binding)")},
        "command": ("validation/replay.sh SNAP OUT: runs the frozen run_main.sh (--threads 2) then run_131101.sh "
                    "(--threads 1), byte-identical to frozen.sha256, verbatim, with OUT (validator scratch) as the "
                    "working directory, so their --out raw/... lands in OUT/raw/, never in the study. The full grid "
                    "(all 360 runs) was replayed, not a subset"),
        "comparison": ("every replayed instance, cell, skip and run row equals the committed row with the same "
                       "(kind, family, arm, p, curve_seed, seed) in the same file: byte-identical JSON lines after "
                       "masking only wall_ns / search_wall_s / wall_s, and equal parsed values on every other key; "
                       "the two config rows are byte-identical"),
        "summary": s,
        "runs": runs,
    }
    cpath = os.path.join(vd, "replay-certificate.json")
    with open(cpath, "w") as f:
        json.dump(cert, f, indent=1)
        f.write("\n")

    # ---- validator report ----
    raw = [{"path": f"../{fn}", "sha256": sha256_bytes(git(repo, "show", f"{sha}:{STUDY}/{fn}"))} for fn in FILES]
    ap, rp = ana["paired"], rec["paired"]
    head = []
    for cname, lab in (("p0_over_nofb", "p0/nofb"), ("p0i3_over_nofb", "p0i3/nofb")):
        for pan in (P5, PA):
            for w in ("1.0", "0.1", "0.0"):
                a, r = ap[cname][pan][f"gae_{w}"]["pooled"], rp[cname][pan][f"gae_{w}"]
                head.append({"name": f"pooled paired GAE_{w} {lab}, {pan} (gm, equal weight per prime)",
                             "claimed": a["gm"], "recomputed": r["pooled"]["gm"]})
                for i, end in ((0, "lower"), (1, "upper")):
                    head.append({"name": f"pooled paired GAE_{w} {lab}, {pan}, boot95 {end} (default_rng(20261128), "
                                         "validator's own replication of the draw order)",
                                 "claimed": a["ci95"][i], "recomputed": r["mc_producer_stream"]["pooled"][i]})
    for pan in (P5, PA):
        for p in PRIMES:
            a, r = ap["p0_over_nofb"][pan]["gae_1.0"]["per_prime"][p], rp["p0_over_nofb"][pan]["gae_1.0"]
            head.append({"name": f"paired GAE_1 p0/nofb p={p}, {pan} (gm over 5 curves)", "claimed": a["gm"],
                         "recomputed": r["per_prime"][p]["gm"]})
            for i, end in ((0, "lower"), (1, "upper")):
                head.append({"name": f"paired GAE_1 p0/nofb p={p}, {pan}, boot95 {end}", "claimed": a["ci95"][i],
                             "recomputed": r["mc_producer_stream"]["per_prime"][p][i]})
    for arm in (NOFB, P0, P0I3):
        for w in ("0.0", "0.1", "1.0"):
            for p in PRIMES:
                head.append({"name": f"IC/ideal neg. rho {LAB[arm]} p={p} w={w} (all seeds)",
                             "claimed": ana["ic_over_rho"][arm][f"all_{w}"]["per_prime"][p]["gm"],
                             "recomputed": rec["ic_over_rho"][arm][f"all_{w}"]["per_prime"][p]["gm"]})
        for i, end in ((0, "lower"), (1, "upper")):
            head.append({"name": f"IC/ideal neg. rho {LAB[arm]} p=131101 w=1.0, boot95 {end}",
                         "claimed": ana["ic_over_rho"][arm]["all_1.0"]["per_prime"]["131101"]["ci95"][i],
                         "recomputed": rec["ic_over_rho"][arm]["all_1.0"]["mc_producer_stream"]["per_prime"]["131101"][i]})
    att_a, att_r = ana["attempts"], rec["attempts"]
    for pan in (P5, PA):
        for arm in (NOFB, P0, P0I3):
            head.append({"name": f"runs with more than one attempt, {LAB[arm]}, {pan}",
                         "claimed": att_a[pan][arm]["multi_attempt"], "recomputed": att_r[pan][arm]["multi"]})
    for arm in (NOFB, P0, P0I3):
        for p in PRIMES:
            head.append({"name": f"runs with more than one attempt, {LAB[arm]}, p={p}, all seeds",
                         "claimed": att_a[PA][arm]["per_prime"][p][0], "recomputed": att_r[PA][arm]["per_prime"][p][0]})
    pm = rec["integrity"]["p0_means_p0_arm_all_seeds"]
    for p in PRIMES:
        head.append({"name": f"P0 lines per p0 run, p={p} (explore.json)", "claimed": exj[p]["p0_lines"],
                     "recomputed": pm[p]["p0_lines_mean"]})
        head.append({"name": f"P0-line relations per p0 run, mean, p={p} (explore.json)",
                     "claimed": exj[p]["p0_line_relations"], "recomputed": pm[p]["p0_line_relations_mean"]})
    ex = rec["explore"]
    for p in PRIMES:
        for k in sorted(ex[p]):
            head.append({"name": f"explore class {k} p={p}: pairs (explore.json n)", "claimed": exj[p][k]["n"],
                         "recomputed": ex[p][k]["n"]})
    integ = rec["integrity"]
    head += [
        {"name": "runs certified with recovered == t_secret, re-verified by validator Picard arithmetic (RESULTS: 360)",
         "claimed": 360, "recomputed": certs["summary"]["rows_cert_ok"]},
        {"name": "(curve, seed) pairs with one log across the three arms (RESULTS: all 120)", "claimed": 120,
         "recomputed": integ["same_log_all_arms_pairs"]},
        {"name": "C0: nofb seeds 1-4 rows equal to the parent's on the 24 fields (analysis.json identical)",
         "claimed": ana["C0_reproduction"]["identical"], "recomputed": integ["C0"]["identical_24"]},
        {"name": "p0/p0i3 rows with P0-line relations > 0 (RESULTS: all 240)", "claimed": 240,
         "recomputed": integ["p0_rows_with_p0_line_relations_gt0"]},
        {"name": "nofb rows with p0_lines == 0 (key present; RESULTS: all 120)", "claimed": 120,
         "recomputed": integ["nofb_rows_p0_lines_eq0_and_rel_eq0"]},
    ]
    heads_ok = all(abs(h["claimed"] - h["recomputed"])
                   <= 1e-9 * max(abs(h["claimed"]), abs(h["recomputed"]), 1e-300) for h in head)
    cmp_ = rec["compare"]["summary"]
    ci_ok = cmp_["producer_ci_inside_mc_noise"] and cmp_["max_rel_ci_vs_producer_stream"] < 1e-12
    certs_ok = (certs["summary"]["curves_all_ok"] == certs["summary"]["curves"] == 20
                and certs["summary"]["rows_cert_ok"] == certs["summary"]["rows_checked"] == 360)
    st_ = integ["p0_structure"]
    ci_ = integ["counter_identities"]
    integ_ok = (not rec["completeness"]["missing"] and not rec["completeness"]["extra"]
                and integ["certified_ok_recovered_eq_t"] == 360 and integ["exhausted_any"] == 0
                and integ["same_log_all_arms_pairs"] == 120
                and integ["p0_rows_with_p0_line_relations_gt0"] == 240
                and integ["nofb_rows_p0_lines_eq0_and_rel_eq0"] == 120
                and integ["C0"]["compared"] == integ["C0"]["identical_24"] == integ["C0"]["identical_all_common_keys"] == 80
                and not integ["C0"]["mismatch"]
                and st_["p0_lines_eq_fb_cols_minus_1"] == st_["p0_lines_eq_ceil_3sqrtp"] == st_["rel_le_lines"] == 240
                and st_["p0_counts_constant_per_curve"] == st_["curves"] == 20
                and ci_["la_mul_eq_splits"] == ci_["la_add_eq_splits"] == ci_["rel_ge_parts"] == 360
                and cmp_["analysis_integrity_fields"] and cmp_["analysis_attempts_equal"] and cmp_["explore_json_equal"])
    dec = rec["decision"]
    dec_ok = (dec["p0_over_nofb"]["exact_law"]["verdict"] == dec["p0_over_nofb"]["producer_stream"]["verdict"]
              == ana["decision"]["verdict_p0"] == "INCONCLUSIVE"
              and dec["p0i3_over_nofb"]["exact_law"]["verdict"] == dec["p0i3_over_nofb"]["producer_stream"]["verdict"]
              == ana["decision"]["verdict_p0i3_secondary"] == "INCONCLUSIVE"
              and ana["decision"]["integrity"] is True
              and not any(v["improvement_possible"] or v["kill_possible"] for v in dec["mc_noise_can_flip"].values()))
    ver = rec["ic_over_rho_verdict"]
    verd_ok = (all(ver[a][p][src]["verdict"] == "UNRESOLVED" for a in ver for p in ver[a] for src in ("exact95", "mc_producer_stream"))
               and all(v.startswith("UNRESOLVED") for d in cmp_["analysis_ic_verdicts"].values() for v in d.values()))
    m = ext["manifests"]
    bind_ok = m["frozen.sha256"]["ok"] and m["data.sha256"]["ok"] and m["binary"]["ok"] and m["source.sha256"]["ok"]
    time_ok = ext["freeze_order"]["ok"]
    repro = ext["reproduction"]
    repro_ok = bool(repro["analysis_json_from_committed_raw"] and repro["explore_json_from_committed_raw"]
                    and repro.get("analysis_json_from_replayed_raw") and repro.get("explore_json_from_replayed_raw"))
    parts = dg["parts"]
    diag_ok = all(v["present"] is False or (v["D1_rows_equal_committed"] == v["runs"] and v["core_dumps"] == v["core_dumps_matched"])
                  for v in parts.values()) and any(v["present"] for v in parts.values())
    status = "pass" if all((heads_ok, ci_ok, certs_ok, integ_ok, dec_ok, verd_ok, bind_ok, time_ok,
                            repro_ok, replay_ok, diag_ok, bb["ok"])) else "fail"
    icp = ana["ic_over_rho"][P0]["all_1.0"]["per_prime"]["131101"]
    dsum = dg["summary"]
    stmts = {tuple(x["where"]): x for x in docs["statements"]}
    sh = stmts[("RESULTS.md:76", "report.typ:43")]["unrescued_of_nofb_multi_attempt"]
    un = stmts[("report.typ:40",)]
    derr = {(e["file"], e["line"]) for e in docs["errors"]}
    errata = [
        {"id": "E1", "where": ["RESULTS.md:89", "reports/run-20261128T0345Z/report.typ:49 (and report.pdf)"],
         "displayed": "1.91 [1.82, 2.00]", "correct": f"{icp['gm']:.2f} [{icp['ci95'][0]:.2f}, {icp['ci95'][1]:.2f}]",
         "value": [icp["gm"]] + icp["ci95"],
         "text": ("p0 IC / ideal negation rho at p = 131101, w = 1: the upper boot95 end is 1.9945176; rounded once it "
                  "is 1.99. 2.00 is a double rounding (1.99452 -> 1.995 -> 2.00). No verdict changes (the lower end "
                  "1.82 is above 1 either way)."),
         "confirmed_by_check": ("RESULTS.md", 89) in derr and ("report.typ", 49) in derr},
        {"id": "E2", "where": ["RESULTS.md:76", "reports/run-20261128T0345Z/report.typ:43"],
         "displayed": "The share of unrescued failures grows with p (5 of 9 at 131101)",
         "correct": ("unrescued / nofb multi-attempt runs: 1/9 at 16411, 4/7 at 65537, 5/9 at 131101 (none at 4099): "
                     "higher at 65537 and 131101 than at 16411, not growing from 65537 (57%) to 131101 (56%); "
                     "n <= 9 per prime"),
         "value": sh,
         "confirmed_by_check": stmts[("RESULTS.md:76", "report.typ:43")]["monotone_increasing_16411_65537_131101"] is False},
        {"id": "E3", "where": ["reports/run-20261128T0345Z/report.typ:40"],
         "displayed": "unrescued failures (10): about 1.0",
         "correct": (f"geometric mean {un['pooled_gm_of_the_10']:.2f} over the 10 pairs; per prime 1.002 (n = 1), "
                     "1.004 (n = 4), 0.933 (n = 5); single pairs "
                     f"{un['individual_range'][0]:.2f}-{un['individual_range'][1]:.2f} (RESULTS.md:70-72 show the "
                     "per-prime values correctly)"),
         "value": un["per_prime_gm_n"], "text": "minor: a loose summary, no number in RESULTS.md is affected"},
    ]
    process = [
        {"id": "P1", "text": ("binary.sha256 and source.sha256 were committed at 04:00:54 (d6a31141e4e), 15 min after "
                              "run_main.sh started (03:45:43); the protocol does not require a pre-run commit, and the "
                              "binary-to-freeze binding was established instead from cargo's records (mtime evidence, "
                              "weaker than a rebuild; no rebuild was allowed)")},
        {"id": "P2", "text": ("the worktree fast-forwarded to 0d042b03db3 (main merged, 35 src files) at 03:45:46, 3 s "
                              "after run_main.sh started, and to e4787052663 at 04:00:32 during run_131101.sh; the "
                              "running binary (linked 03:45:08) and the frozen scripts (study directory untouched by "
                              "the merges) were unaffected, but the run-time tree state is recoverable only from the "
                              "reflog and the bench record. Do not merge into the worktree during a run")},
        {"id": "P3", "text": ("section 13 artifact store: raw/run.jsonl (553,191 B) and raw/run131101.jsonl (192,624 B) "
                              "are .jsonl over 64 KiB committed to Git, no .artifacts.json; at the validated commit the "
                              "bench record is open with raw [] and metrics []; the binary lives only at a scratch "
                              "path and in target/release")},
        {"id": "P4", "text": ("frozen analyze.py reads the C0 fields with .get and checks nofb rows with "
                              ".get('p0_lines', 0) == 0, so a missing key would pass silently (lesson L-99ee952350b2); "
                              "it does not check completeness against seeds.json. No effect here: 360 of 360 runs "
                              "present, the 24 C0 fields and all 86 common keys present and equal on 80 of 80 rows, "
                              "p0_lines present (0) on all 120 nofb rows")},
        {"id": "P5", "text": ("seeds.json: master_seed null; seeded streams on the study path not listed: the descent "
                              "RNG (0xDEC0_5EED_0000_0001 ^ q ^ seed.rotate_left(17), quartic_ladder.rs run_diem_inner), "
                              "the sampled factor base (0x5AFB_0000_0000_0000 ^ q and 0x5EED_5AFB_0000_0001 ^ q, "
                              "sampled_base) and the BSGS order computation (inherited, parent P5). Pairing and arm "
                              "exclusion are correct and the full replay is byte-identical")},
        {"id": "P6", "text": ("section 5a w_cal is not recorded; on the primary panel every pooled interval straddles 1 "
                              "at w = 0, 0.1 and 1, so the pair is not ranked at any band point and w_cal cannot change "
                              "the verdict. On the secondary all-seeds panel p0i3/nofb lies below 1 at all three band "
                              "points (upper 0.837, 0.976, 0.976) and p0/nofb only at w = 0.1 and 1; the protocol "
                              "assigns that panel no decision and RESULTS.md claims nothing from it")},
        {"id": "P7", "text": ("mechanism statements in RESULTS.md 'What this settles' (the P0 lines remove the "
                              "base-point-shift cause; the unrescued failures fit the frontier-column mechanism) rest on "
                              "the dev diagnosis and the rescue counts; the grid rows hold no diagnostic. The validator "
                              "replayed the binary's uncounted diagnosis and core dumps on grid curves (diag.sh, "
                              "diag_check.json; arms nofb and p0, seeds 1-6; 4099 curve 205, all 16411 curves, 65537 "
                              "curves 57/156/166/369, 131101 curves 53/177/288). Every p0 attempt core holds a line row "
                              f"not orthogonal to v_deg ({dsum['e_p0']['all_attempts'] - dsum['e_p0']['all_attempts_vdeg_free']} "
                              f"of {dsum['e_p0']['all_attempts']}), so v_deg is never free with the P0 lines (supports the "
                              f"first statement). nofb failed attempts: {dsum['e_nofb']['failed_attempts']}, v_deg free in "
                              f"{dsum['e_nofb']['failed_vdeg_free']}, z columns in {dsum['e_nofb']['failed_with_z_cols']}. "
                              f"p0 failed attempts: {dsum['e_p0']['failed_attempts']}, all with t undetermined, v_deg free "
                              f"in {dsum['e_p0']['failed_vdeg_free']}, z columns (columns only in decomposition rows) in "
                              f"{dsum['e_p0']['failed_with_z_cols']}, and {len(dsum['e_p0']['failed_neither_vdeg_nor_z'])} "
                              "with neither, all of them ab_rank 1 (one or two decomposition rows reach the core): "
                              + "; ".join(f"p={a['p']} curve {a['curve_seed']} seed {a['seed']} (dec rows {a['dec_rows']})"
                                          for a in dsum['e_p0']['failed_neither_vdeg_nor_z'])
                              + ". So the frontier-column reading fits the p0 failures that have z columns, not those "
                              "with ab_rank 1, which is a third cause the RESULTS text does not name")},
        {"id": "P8", "text": ("the branch moved during validation (b2c9a5d319c session log, b8fe85e9088 bench-record "
                              "launcher commands); neither touches the study directory; this validation covers "
                              "c271f320e40 only")},
        {"id": "P9", "text": ("PROTOCOL section 7: the runs used the local container, not the default EKS route "
                              "(disclosed: AWS key rejected); counted-operation metrics are host-independent and the "
                              "replay on this host is byte-identical")},
    ]
    report = {
        "schema": "validator-report/v1",
        "study": STUDY,
        "status": status,
        "independent": True,
        "disposition": "validated" if status == "pass" else "invalid_measurement",
        "verdict": "validated-with-errata" if status == "pass" else "not-validated",
        "validated_commit": full_sha,
        "study_tree": git(repo, "rev-parse", f"{sha}:{STUDY}").decode().strip(),
        "report": ("the prose report was returned to the dispatching session (subagent hand-back); this file "
                   "carries its checks, errata and process findings as data"),
        "raw_data": raw,
        "headline": head,
        "decision": {"frozen_rule": "PROTOCOL.md section 5",
                     "p0_over_nofb_primary": dec["p0_over_nofb"], "p0i3_over_nofb_primary": dec["p0i3_over_nofb"],
                     "mc_noise_can_flip": dec["mc_noise_can_flip"],
                     "predictions": {
                         "1_second_attempts_p0_at_most_half_nofb_primary": {
                             "p0": att_r[P5][P0]["multi"], "nofb": att_r[P5][NOFB]["multi"], "runs": 40,
                             "held": att_r[P5][P0]["multi"] * 2 <= att_r[P5][NOFB]["multi"]},
                         "2_p0_over_nofb_gae1_in_0.75_0.97": {"value": rp["p0_over_nofb"][P5]["gae_1.0"]["pooled"]["gm"],
                                                             "held": 0.75 <= rp["p0_over_nofb"][P5]["gae_1.0"]["pooled"]["gm"] <= 0.97},
                         "3_p0i3_over_nofb_in_0.75_1.02_and_more_second_attempts_than_p0": {
                             "value": rp["p0i3_over_nofb"][P5]["gae_1.0"]["pooled"]["gm"],
                             "p0i3_multi": att_r[P5][P0I3]["multi"], "p0_multi": att_r[P5][P0]["multi"],
                             "held": (0.75 <= rp["p0i3_over_nofb"][P5]["gae_1.0"]["pooled"]["gm"] <= 1.02
                                      and att_r[P5][P0I3]["multi"] > att_r[P5][P0]["multi"])}}},
        "checks": {
            "headline_agreement_1e-9": heads_ok,
            "bootstrap_ci_reproduced_by_independent_draw_order_max_rel": cmp_["max_rel_ci_vs_producer_stream"],
            "bootstrap_ci_producer_endpoints_inside_mc_noise": cmp_["producer_ci_inside_mc_noise"],
            "bootstrap_ci_max_rel_diff_vs_exact_law": cmp_["max_rel_ci_vs_exact"],
            "point_estimates_max_rel_diff": cmp_["max_rel_point"],
            "certificates_independent": certs["summary"],
            "replay": s,
            "integrity_completeness_C0_p0_structure_counter_identities": integ_ok,
            "C0": {k: integ["C0"][k] for k in ("compared", "identical_24", "identical_all_common_keys", "key_sets", "parent_seeds")},
            "p0_structure": integ["p0_structure"],
            "frozen_decision_INCONCLUSIVE_reproduced_stream_and_exact_law": dec_ok,
            "ic_over_rho_every_cell_unresolved_below_at_w0_w01_above_at_w1": verd_ok,
            "bindings_manifests_binary_source": bind_ok,
            "binary_to_freeze_commit_by_cargo_records": bb["ok"],
            "freeze_before_grid_data": time_ok,
            "analysis_and_explore_json_byte_reproduced_from_committed_and_replayed_raw": repro_ok,
            "diag_flag_leaves_committed_fields_unchanged_and_cores_matched": diag_ok,
            "mechanism_diag_summary": dsum,
            "documents_displayed_numbers_checked": docs["summary"]["numbers_checked"],
            "documents_errors": docs["errors"],
            "rho_reference_5a": ("ideal negation rho sqrt(pi r / 4), a closed form with no native work, so it is priced "
                                 "0 at every w while every IC arm carries all six counted native counters; ecbench band "
                                 "(lesson L-546d37ca7750) does not apply: there is no ecbench session and no rho "
                                 "counter to omit"),
        },
        "errata": errata,
        "process_findings": process,
        "scripts": ["replay.sh", "compare_replay.py", "verify_certificates.py", "picarith.py", "recompute.py",
                    "checks_extra.py", "docs_check.py", "diag.sh", "diag_check.py", "build_gate_artifacts.py"],
        "label": "[pcc:unverified] (no pcc claim was created by the validator)",
    }
    rpath = os.path.join(vd, "validator-report.json")
    with open(rpath, "w") as f:
        json.dump(report, f, indent=1)
        f.write("\n")

    # ---- VALIDATION.json: band of the p0 arm at p = 131101 against the ideal negation rho ----
    by = {}
    for r in rows:
        if r["kind"] == "run" and r["arm"] == P0 and r["p"] == 131101:
            by.setdefault(r["curve_seed"], []).append(r)
    curves = sorted(by)
    assert len(curves) == 5 and all(len(by[c]) == 6 for c in curves)
    rho = gm(math.sqrt(math.pi * by[c][0]["r"] / 4) for c in curves)
    band = {}
    for key, w in (("0", 0.0), ("0.1", 0.1), ("1", 1.0)):
        ic = gm(gm(x["group_ops"] + w * sum(x[k] for k in NATIVE) for x in by[c]) for c in curves)
        band[key] = {"ic": ic, "rho": rho}
    # the band ratio equals analysis.json's IC/rho for p0 at 131101
    for key, w in (("0", "0.0"), ("0.1", "0.1"), ("1", "1.0")):
        a = ana["ic_over_rho"][P0][f"all_{w}"]["per_prime"]["131101"]["gm"]
        assert abs(band[key]["ic"] / band[key]["rho"] - a) <= 1e-12 * a, (key, band[key], a)
    signs = {(v["ic"] > v["rho"]) - (v["ic"] < v["rho"]) for v in band.values()}
    psha = sha256_bytes(git(repo, "show", f"{sha}:{STUDY}/PROTOCOL.md"))
    man = {
        "schema": "validation-gate/v1",
        "note": ("Written by the validator. The study's claim is a paired component comparison (2lp-sfb-sge-p0 vs "
                 "2lp-sfb-sge-nofb, held-out seeds 5-6) whose frozen verdict is INCONCLUSIVE: no improvement is "
                 "validated, and the pair is not ranked at any band point (every pooled interval straddles 1). The "
                 "band below is the primary arm p0 against the ideal negation rho at p = 131101 (all seeds) and does "
                 "not order across the band (UNRESOLVED, no rho claim). Displayed numbers carry one rounding error "
                 "(validator-report.json E1: cite p0 IC/rho at 131101, w = 1 as 1.91 [1.82, 1.99]). Paths are "
                 "relative to this directory (validation/); if the manifest is moved to the study root, prefix them "
                 "with validation/ and change ../raw/ to raw/ in validator-report.json."),
        "replay_certificate": {"path": "replay-certificate.json", "sha256": sha256_bytes(open(cpath, "rb").read())},
        "validator_report": {"path": "validator-report.json", "sha256": sha256_bytes(open(rpath, "rb").read())},
        "rho_reference": {
            "kind": "negation",
            "protocol": (f"PROTOCOL.md (sha256 {psha[:8]}...{psha[-4:]}) section 5: IC / ideal negation rho "
                         "sqrt(pi r / 4) group additions, nothing charged to rho; no measured rho is run"),
            "unit": ("GAE_w per solve, p = 131101, 2lp-sfb-sge-p0: geometric mean over the 5 curves of the per-curve "
                     "geometric mean over solver seeds 1-6; one counted Pic^0 operation = 1; each counted native "
                     "operation (rel_mul, rel_add, la_mul, la_add, idx_lookups, idx_inserts) = w; the ideal negation "
                     "rho charges no native work at any w"),
            "binary_or_process": (f"IC arms: genus_ladder.p0 sha256 {BINARY_SHA} (replayed by the validator, all 360 "
                                  "runs byte-identical). The negation reference is a closed form of each instance's r "
                                  "as emitted by that binary and certified by the validator's own arithmetic (r prime, "
                                  "h = 1, Hasse-Weil, [r]G = 0)"),
            "matches_protocol": True,
            "strongest_available": True,
            "same_unit": True,
            "same_binary_or_process": True,
            "gae_band": band,
            "order_holds_across_band": signs in ({1}, {-1}),
        },
    }
    with open(os.path.join(vd, "VALIDATION.json"), "w") as f:
        json.dump(man, f, indent=1)
        f.write("\n")
    print(json.dumps({"replay_ok": replay_ok, "heads_ok": heads_ok, "ci_ok": ci_ok, "certs_ok": certs_ok,
                      "integ_ok": integ_ok, "dec_ok": dec_ok, "verd_ok": verd_ok, "bind_ok": bind_ok,
                      "binary_binding_ok": bb["ok"], "time_ok": time_ok, "repro_ok": repro_ok, "diag_ok": diag_ok,
                      "status": status, "n_headline": len(head), "band": band,
                      "order_holds_across_band": man["rho_reference"]["order_holds_across_band"]}, indent=1))


if __name__ == "__main__":
    main()
