#!/usr/bin/env python3
"""Validator: write replay-certificate.json, validator-report.json and VALIDATION.json
(docs/VALIDATION_GATE.md, validation-gate/v1) for research/quartic_diem_nofb_20261126.

Adapted from the parent validation's build_gate_artifacts.py (quartic_diem_la_20261123).

Usage: python3 -I build_gate_artifacts.py REPO SHA VALDIR

Reads from VALDIR: certificates.json (verify_certificates.py), recompute.json (recompute.py),
replay_compare.json (compare_replay.py), checks_extra.json (checks_extra.py) and
diag_check.json (diag_check.py); analysis.json and the raw rows from Git at SHA.  Every
pass/fail flag is derived here from those files; nothing is set by hand.  Paths in the three
JSON outputs are relative to VALDIR (the gate resolves them from the directory that holds
VALIDATION.json).
"""
import hashlib
import json
import math
import os
import subprocess
import sys

STUDY = "research/quartic_diem_nofb_20261126"
FILES = ["raw/run.jsonl", "raw/run131101.jsonl"]
SGE, NOFB = "Q_diem_2lp_sfb_sge", "Q_diem_2lp_sfb_sge_nofb"
NATIVE = ["rel_mul", "rel_add", "la_mul", "la_add", "idx_lookups", "idx_inserts"]
BINARY_SHA = "55ebb4404a070356bbd35612737aa31f00a9f0edfa8e72ab26b4a02287631602"
PRIMES = ["4099", "16411", "65537", "131101"]


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
    certs, rec, rep, ext, dg = (load("certificates.json"), load("recompute.json"), load("replay_compare.json"),
                                load("checks_extra.json"), load("diag_check.json"))
    dgs = dg["summary"]
    ana = json.loads(git(repo, "show", f"{sha}:{STUDY}/analysis.json"))
    rows = [json.loads(l) for fn in FILES
            for l in git(repo, "show", f"{sha}:{STUDY}/{fn}").decode().splitlines() if l.strip()]

    # ---- replay certificate ----
    runs = rep["rows"]
    s = rep["summary"]
    replay_ok = (bool(runs) and all(e["found"] and e["bytes_equal"] and e["fields_equal"] for e in runs)
                 and s["runs_replayed"] == s["runs_certified_and_correct"] == 160
                 and s["rows_replayed"] == s["rows_bytes_equal"] == s["rows_fields_equal"]
                 and s["duplicate_replayed_keys"] == 0 and s["configs_byte_identical"]
                 and s["committed_rows"] == s["committed_rows_covered"] == s["rows_replayed"])
    bb = ext["binary_binding"]
    cert = {
        "schema": "replay-certificate/v1",
        "study": STUDY,
        "status": "verified" if replay_ok else "failed",
        "independent": True,
        "scalars_verified": replay_ok,
        "validated_commit": full_sha,
        "binary": {"name": "genus_ladder.nofb", "sha256": BINARY_SHA,
                   "note": ("hashed and copied at dispatch; equal to binary.sha256, dev/binary.sha256 and "
                            "target/release/genus_ladder. Not rebuilt (no builds allowed). Binding to "
                            "31a1e2f53ef through cargo's own build records: every one of the "
                            f"{bb['dep_info']['lib']['n_source_files']} + {bb['dep_info']['bin']['n_source_files']} "
                            "source files rustc read (dep-info) is tracked, unmodified since before the compile "
                            "began, and equal to 31a1e2f53ef (checks_extra.json binary_binding)")},
        "command": ("validation/replay.sh SNAP OUT: runs the frozen run_main.sh (--threads 2) then run_131101.sh "
                    "(--threads 1), byte-identical to frozen.sha256, verbatim, with OUT (validator scratch) as the "
                    "working directory, so their --out raw/... lands in OUT/raw/, never in the study"),
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
    raw = []
    for fn in FILES:
        raw.append({"path": f"../{fn}", "sha256": sha256_bytes(git(repo, "show", f"{sha}:{STUDY}/{fn}"))})
    ap, rp = ana["paired_nofb_over_sge"], rec["paired_nofb_over_sge"]
    head = []
    for pan in ("primary_seeds_3_4", "all_seeds_1_4"):
        for w in ("1.0", "0.1", "0.0"):
            head.append({"name": f"pooled paired GAE_{w} nofb/sge, {pan} (gm, equal weight per prime)",
                         "claimed": ap[pan][f"gae_{w}"]["pooled"]["gm"],
                         "recomputed": rp[pan][f"gae_{w}"]["pooled"]["gm"]})
        for m in ("la_total", "native_all"):
            head.append({"name": f"pooled paired {m} nofb/sge, {pan}", "claimed": ap[pan][m]["pooled"]["gm"],
                         "recomputed": rp[pan][m]["pooled"]["gm"]})
    for p in PRIMES:
        for m in ("gae_1.0", "la_total"):
            head.append({"name": f"paired {m} nofb/sge p={p}, all seeds (gm over 5 curves)",
                         "claimed": ap["all_seeds_1_4"][m]["per_prime"][p]["gm"],
                         "recomputed": rp["all_seeds_1_4"][m]["per_prime"][p]["gm"]})
    for arm, lab in ((SGE, "sge"), (NOFB, "nofb")):
        for w in ("0.0", "0.1", "1.0"):
            for p in PRIMES:
                head.append({"name": f"IC/ideal neg. rho {lab} p={p} w={w}",
                             "claimed": ana["ic_over_rho"][arm][f"all_{w}"]["per_prime"][p]["gm"],
                             "recomputed": rec["ic_over_rho"][arm][f"all_{w}"]["per_prime"][p]["gm"]})
    pr = rec["pairs"]
    head += [
        {"name": "runs certified, re-verified with validator arithmetic (RESULTS: 160)", "claimed": 160,
         "recomputed": certs["summary"]["rows_cert_ok"]},
        {"name": "C0: baseline seeds 1-2 rows identical to the parent's 2lp-sfb-sge rows (RESULTS: 40)",
         "claimed": ana["C0_reproduction"]["identical"], "recomputed": rec["C0"]["identical_all_shared"]},
        {"name": "baseline fallbacks (RESULTS: 16)", "claimed": ana["rescues"]["baseline_fallbacks"],
         "recomputed": pr["fallbacks_total"]},
        {"name": "fallback pairs (RESULTS: 15)", "claimed": 15, "recomputed": pr["fallback_pairs"]},
        {"name": "rescues (RESULTS: 0), by the counter", "claimed": ana["rescues"]["baseline_rescues"],
         "recomputed": pr["rescues_total_counter"]},
        {"name": "pairs whose attempts differ (0 implies no rescue, independently of the counter)",
         "claimed": len(ana["rescues"]["attempts_differ"]), "recomputed": len(pr["attempts_differ"])},
        {"name": "nofb skipped fallbacks (RESULTS: 16)", "claimed": ana["rescues"]["nofb_skipped"],
         "recomputed": pr["skipped_total"]},
        {"name": "fallback-pair la ratio nofb/sge, minimum (RESULTS: 20.2%)", "claimed": 0.20229603154112658,
         "recomputed": pr["la_total_ratio_range"][0]},
        {"name": "fallback-pair la ratio nofb/sge, maximum (RESULTS: 27.3%)", "claimed": 0.2728250218412682,
         "recomputed": pr["la_total_ratio_range"][1]},
    ]
    heads_ok = all(abs(h["claimed"] - h["recomputed"])
                   <= 1e-9 * max(abs(h["claimed"]), abs(h["recomputed"]), 1e-300) for h in head)
    cmp = rec["compare_with_analysis_json"]
    ci_ok = all(cmp["producer_endpoints_inside_mc_range"].values())
    certs_ok = (certs["summary"]["curves_all_ok"] == certs["summary"]["curves"] == 20
                and certs["summary"]["rows_cert_ok"] == certs["summary"]["rows_checked"] == 160)
    integ_ok = (not rec["completeness"]["missing"] and not rec["completeness"]["unexpected"]
                and rec["recorded_certified_all"]["ok"] and rec["la_split_sums"]["ok"]
                and not pr["identity_violations"] and not pr["dominance_violations"]
                and rec["C0"]["identical_all_shared"] == rec["C0"]["compared"] == 40
                and rec["C0"]["analyze_c0_fields_present_everywhere"]
                and rec["instances_equal_parent"]["ok"] and rec["rho_skip_rows"]["expected_steps_eq_sqrt_pi_r_over_4"]
                and pr["zero_rescues_by_attempt_equality"] and pr["rescues_total_counter"] == 0)
    dec = rec["decision"]
    dec_ok = dec["verdict"] == "IMPROVEMENT" and ana["decision"]["verdict"].startswith("IMPROVEMENT") \
        and all(v < 1 for v in dec["upper95_exact"].values())
    verd_ok = cmp["verdicts_equal"] and all(v.startswith("UNRESOLVED") for d in rec["ic_over_rho_verdict"].values()
                                            for v in d.values())
    m = ext["manifests"]
    bind_ok = m["frozen.sha256"]["ok"] and m["data.sha256"]["ok"] and m["binary"]["ok"] and m["source.sha256"]["ok"]
    time_ok = ext["freeze_order"]["ok"]
    repro = ext["analyze_py_reproduction"]
    repro_ok = bool(repro["committed_raw"] and repro["committed_raw"]["byte_identical"]
                    and repro["replay_raw"] and repro["replay_raw"]["byte_identical"])
    struct_ok = ext["structural_identities"]["ok"] and ext["section5a_fields"]["ok"]
    diag_ok = dgs["committed_fields_equal"] == dgs["diag_runs"] > 0 and not dg.get("missing_files")
    status = "pass" if all((heads_ok, ci_ok, certs_ok, integ_ok, dec_ok, verd_ok, bind_ok, time_ok,
                            repro_ok, replay_ok, struct_ok, diag_ok)) else "fail"
    docs = ext["documents"]
    fails = {(d["file"], d["line"]): d for d in docs["failures"]}
    pg = ana["decision"]["pooled_gae_1.0"]["gm"]
    icn = ana["ic_over_rho"][NOFB]["all_0.1"]["per_prime"]
    multi = ext["documents"]["multi_attempt_cost_claim"]
    errata = [
        {"id": "E1", "where": ["RESULTS.md:32", "RESULTS.md:52", "reports/run-20261126T0333Z/report.typ:20 (and report.pdf)",
                               "benchmarks/records/20261126T0333Z-quartic-diem-nofb.json summary",
                               "subject of commit cc7e5c5488a"],
         "displayed": "0.833", "correct": f"{pg:.3f}", "value": pg,
         "text": ("The held-out pooled paired GAE_1 ratio nofb/sge is 0.8324606; rounded once it is 0.832. "
                  "0.833 is a double rounding (0.83246 -> 0.8325 -> 0.833). Cite 0.832 [0.758, 0.907]. "
                  "The bench record's metric value is correct."),
         "confirmed_by_check": bool(fails.get(("RESULTS.md", 32)) and fails.get(("RESULTS.md", 52))
                                    and fails.get(("report.typ", 20)))},
        {"id": "E2", "where": ["RESULTS.md:60"], "displayed": "[0.264, 0.275]",
         "correct": f"[{icn['65537']['ci95'][0]:.3f}, {icn['65537']['ci95'][1]:.3f}]",
         "value": icn["65537"]["ci95"], "text": "nofb IC/rho w = 0.1 at p = 65537: lower end 0.2634980 rounds to 0.263",
         "confirmed_by_check": bool(fails.get(("RESULTS.md", 60)))},
        {"id": "E3", "where": ["RESULTS.md:61"], "displayed": "[0.184, 0.217]",
         "correct": f"[{icn['131101']['ci95'][0]:.3f}, {icn['131101']['ci95'][1]:.3f}]",
         "value": icn["131101"]["ci95"], "text": "nofb IC/rho w = 0.1 at p = 131101: upper end 0.2164865 rounds to 0.216",
         "confirmed_by_check": bool(fails.get(("RESULTS.md", 61)))},
        {"id": "E4", "where": ["RESULTS.md:69", "reports/run-20261126T0333Z/report.typ:36"],
         "displayed": "about twice / about double",
         "correct": "1.50-1.90x (two-attempt runs); 2.25x (the one three-attempt run)",
         "value": {"two_attempt_range": multi["measured_nofb_gae1_two_attempt_over_same_curve_single_attempt_gm"],
                   "three_attempt": multi["measured_three_attempt"]},
         "text": ("GAE_1 of each multi-attempt nofb run over the geometric mean of the same curve's single-attempt "
                  "nofb runs")},
        {"id": "E5", "where": ["analysis.json:835", "analyze.py:218", "analyze.py:201"],
         "displayed": ext["documents"]["analysis_json_rule_text"], "correct": "PROTOCOL.md section 5",
         "text": "the decision rule is PROTOCOL.md section 5 (section 6 is Outputs); RESULTS.md:26 cites section 5"},
    ]
    process = [
        {"id": "P1", "text": ("section 13 artifact store: raw/run.jsonl (254,691 B) and raw/run131101.jsonl (89,012 B) are "
                              ".jsonl over 64 KiB committed to Git, no .artifacts.json, bench record raw uploaded=false; "
                              "the binary lives only at a scratch path (binary.sha256)")},
        {"id": "P2", "text": ("section 5a w_cal is not recorded; the pair's order does not depend on it: nofb is no costlier "
                              "than sge in every counted component on all 80 pairs, so the order holds at every w >= 0, "
                              "including w_cal and 10 w_cal")},
        {"id": "P3", "text": ("section 3a cost chain not written in RESULTS.md; its premise holds: the only differing fields "
                              "on the 15 fallback pairs are the fallback's Wiedemann splits (spmv, bm, vec), its failed "
                              "certificate (group_ops, cantor_mul, cantor_inv) and the two fallback counters")},
        {"id": "P4", "text": ("frozen analyze.py: C0 reads with .get; completeness against the seeds.json panel is not "
                              "checked; the pair-identity check compares a subset of fields; IC/rho uses runs.get. No "
                              "effect here: 160 of 160 runs present, every key exists, all fields equal on the 65 "
                              "no-fallback pairs")},
        {"id": "P5", "text": ("seeds.json against docs/experiment-seeds.md: master_seed is null; two seeded streams on the "
                              "study path are not listed (BSGS order computation, quartic_ladder.rs:1241, curve_seed ^ "
                              "0xB5B5_0DD5_0DD5_B5B5; sampled factor base, quartic_ladder.rs:1888, 0x5AFB_0000_0000_0000 ^ "
                              "q); the solver stream StdRng::seed_from_u64(solver_seed) is not keyed by instance or "
                              "domain-separated (declared as inherited); derived seeds are not retained. Pairing and "
                              "arm exclusion are correct and the replay is byte-identical")},
        {"id": "P6", "text": ("bench record: commit 8bc5537a844 with commit_dirty true (the record was opened 9 s before the "
                              "freeze commit); the driver run_all.sh ran uncommitted and was committed 51 s after the run "
                              "started (a8fa56ebb34); it is not frozen and only calls the frozen scripts")},
        {"id": "P7", "text": ("unit test sfb_sge_nofb_pairs_with_sge met no fallback (producer's log: 24 cases, 0 "
                              "fallbacks), so its fallback branch is untested by the test; PROTOCOL section 5 discloses "
                              "this and the grid check covers 15 fallback pairs. The validator ran no cargo test (no builds)")},
    ]
    report = {
        "schema": "validator-report/v1",
        "study": STUDY,
        "status": status,
        "independent": True,
        "disposition": "validated" if status == "pass" else "see validator-report.md",
        "verdict": "validated-with-errata" if status == "pass" else "failed",
        "validated_commit": full_sha,
        "study_tree": git(repo, "rev-parse", f"{sha}:{STUDY}").decode().strip(),
        "report": ("the prose report was returned to the dispatching session (subagent hand-back); this file "
                   "carries its errata and process findings as data"),
        "raw_data": raw,
        "headline": head,
        "checks": {
            "headline_agreement_1e-9": heads_ok,
            "bootstrap_ci_producer_endpoints_inside_mc_noise": ci_ok,
            "bootstrap_ci_max_rel_diff_vs_exact": cmp["max_rel_interval"],
            "certificates_independent": certs["summary"],
            "replay": s,
            "integrity_pairs_C0_rescues": integ_ok,
            "pair_identities_all_fields": {"pairs": pr["n_pairs"], "violations": pr["identity_violations"],
                                           "nofb_componentwise_dominates_sge": not pr["dominance_violations"]},
            "frozen_decision_IMPROVEMENT_reproduced_exact_law": dec_ok,
            "decision_exact_upper95": dec["upper95_exact"],
            "ic_over_rho_every_cell_unresolved": verd_ok,
            "bindings_manifests_binary": bind_ok,
            "binary_to_commit_by_cargo_dep_info": bb.get("ok"),
            "freeze_before_grid_data": time_ok,
            "analysis_json_byte_reproduced_by_frozen_script_committed_and_replay_raw": repro_ok,
            "structural_counter_identities_and_5a_fields": struct_ok,
            "diag_flag_leaves_committed_fields_unchanged": diag_ok,
            "rescue_mechanism": {k: dgs[k] for k in ("failed_attempts", "failed_attempts_t_determined",
                                                     "rescue_possible_in_any_failed_attempt", "by_seed_panel")},
            "documents_displayed_numbers_checked": docs["checked"],
            "documents_rounding_errors": [(d["file"], d["line"], d["displayed"], d["correct_rounding"])
                                          for d in docs["failures"]],
        },
        "errata": errata,
        "process_findings": process,
        "scripts": ["replay.sh", "compare_replay.py", "verify_certificates.py", "picarith.py",
                    "recompute.py", "checks_extra.py", "diag.sh", "diag_check.py", "build_gate_artifacts.py"],
        "label": "[pcc:unverified] (no pcc claim was created by the validator)",
    }
    rpath = os.path.join(vd, "validator-report.json")
    with open(rpath, "w") as f:
        json.dump(report, f, indent=1)
        f.write("\n")

    # ---- VALIDATION.json: band of the nofb arm at p = 131101 against the ideal negation rho ----
    by = {}
    for r in rows:
        if r["kind"] == "run" and r["arm"] == NOFB and r["p"] == 131101:
            by.setdefault(r["curve_seed"], []).append(r)
    curves = sorted(by)
    assert len(curves) == 5 and all(len(by[c]) == 4 for c in curves)
    rho = gm(math.sqrt(math.pi * by[c][0]["r"] / 4) for c in curves)
    band = {}
    for key, w in (("0", 0.0), ("0.1", 0.1), ("1", 1.0)):
        ic = gm(gm(x["group_ops"] + w * sum(x[k] for k in NATIVE) for x in by[c]) for c in curves)
        band[key] = {"ic": ic, "rho": rho}
    signs = {(v["ic"] > v["rho"]) - (v["ic"] < v["rho"]) for v in band.values()}
    man = {
        "schema": "validation-gate/v1",
        "note": ("Written by the validator. The study's claim is a paired component comparison "
                 "(2lp-sfb-sge-nofb vs 2lp-sfb-sge, frozen rule IMPROVEMENT, an order that holds across the band: "
                 "nofb is no costlier than sge in every counted component on all 80 pairs). The band below is the "
                 "nofb arm against the ideal negation rho at p = 131101 and does not order across the band "
                 "(UNRESOLVED, no rho claim). Headline numbers display with rounding errors (ERRATA.md E1-E3): "
                 "cite the held-out pooled GAE_1 ratio as 0.832 [0.758, 0.907]. Paths are relative to this "
                 "directory (validation/); if the manifest is moved to the study root, prefix them with "
                 "validation/ and change ../raw/ to raw/ in validator-report.json."),
        "replay_certificate": {"path": "replay-certificate.json", "sha256": sha256_bytes(open(cpath, "rb").read())},
        "validator_report": {"path": "validator-report.json", "sha256": sha256_bytes(open(rpath, "rb").read())},
        "rho_reference": {
            "kind": "negation",
            "protocol": ("PROTOCOL.md (sha256 7caf53a0...bb88) section 5: the ideal negation rho R = sqrt(pi r / 4) "
                         "group additions, nothing unpriced charged to rho; no measured rho is run"),
            "unit": ("GAE_w per solve, p = 131101, 2lp-sfb-sge-nofb: geometric mean over the 5 curves of the "
                     "per-curve geometric mean over solver seeds 1-4; one counted Pic^0 operation = 1; each counted "
                     "native operation (rel_mul, rel_add, la_mul, la_add, idx_lookups, idx_inserts) = w; the ideal "
                     "negation rho charges no native work at any w"),
            "binary_or_process": (f"IC arms: genus_ladder.nofb sha256 {BINARY_SHA} (replayed by the validator, all "
                                  "160 runs byte-identical). The negation reference is a closed form of each "
                                  "instance's r as emitted by that binary and certified by the validator's own "
                                  "arithmetic (r prime, h = 1, Hasse-Weil, [r]G = 0)"),
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
                      "time_ok": time_ok, "repro_ok": repro_ok, "struct_ok": struct_ok, "diag_ok": diag_ok,
                      "status": status, "n_headline": len(head), "band": band,
                      "order_holds_across_band": man["rho_reference"]["order_holds_across_band"]}, indent=1))


if __name__ == "__main__":
    main()
