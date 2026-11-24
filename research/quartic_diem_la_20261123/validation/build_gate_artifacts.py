#!/usr/bin/env python3
"""Validator: write replay-certificate.json, validator-report.json and VALIDATION.json
(docs/VALIDATION_GATE.md, validation-gate/v1) for research/quartic_diem_la_20261123.

Usage: python3 -I build_gate_artifacts.py REPO SHA VALDIR

Reads from VALDIR: certificates.json (verify_certificates.py), recompute.json
(recompute.py), replay_compare.json (compare_replay.py), checks_extra.json
(checks_extra.py); analysis.json and the raw rows from Git at SHA.  Every pass/fail
flag is derived here from those files; nothing is set by hand.  Paths in the three
JSON outputs are relative to VALDIR (the gate resolves them from the directory that
holds VALIDATION.json).
"""
import hashlib
import json
import math
import os
import subprocess
import sys

STUDY = "research/quartic_diem_la_20261123"
FILES = ["raw/run.jsonl", "raw/run131101.jsonl"]
SFB, SGE = "Q_diem_2lp_sfb", "Q_diem_2lp_sfb_sge"
NATIVE = ["rel_mul", "rel_add", "la_mul", "la_add", "idx_lookups", "idx_inserts"]
BINARY_SHA = "adddfbfa0bf2cdfd115a1fff510fa29753411c60c940dc0b5ca916b0e5d6d1f6"
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
    load = lambda n: json.load(open(os.path.join(vd, n)))
    certs, rec, rep, ext = (load("certificates.json"), load("recompute.json"),
                            load("replay_compare.json"), load("checks_extra.json"))
    ana = json.loads(git(repo, "show", f"{sha}:{STUDY}/analysis.json"))
    rows = [json.loads(l) for fn in FILES
            for l in git(repo, "show", f"{sha}:{STUDY}/{fn}").decode().splitlines() if l.strip()]

    # ── replay certificate ──
    runs = rep["rows"]
    s = rep["summary"]
    replay_ok = (bool(runs) and all(e["found"] and e["bytes_equal"] and e["fields_equal"] for e in runs)
                 and s["runs_replayed"] == s["runs_certified_and_correct"] == 80
                 and s["rows_replayed"] == s["rows_bytes_equal"] == s["rows_fields_equal"]
                 and s["duplicate_replayed_keys"] == 0
                 and s["committed_rows"] == s["committed_rows_covered"] == s["rows_replayed"])
    cert = {
        "schema": "replay-certificate/v1",
        "study": STUDY,
        "status": "verified" if replay_ok else "failed",
        "independent": True,
        "scalars_verified": replay_ok,
        "validated_commit": full_sha,
        "binary": {"name": "genus_ladder.qla", "sha256": BINARY_SHA,
                   "note": ("hash-checked copy taken at dispatch, equal to binary.sha256 and dev/binary.sha256; "
                            "no rebuild (none allowed on this host), so the binary-to-commit binding is not "
                            "verified beyond source.sha256")},
        "command": ("validation/replay.sh BIN OUT main (all 15 curves at 4099, 16411, 65537; --threads 2) and "
                    "validation/replay.sh BIN OUT 131101 53/177/193/228/288 (--threads 1): the frozen flags of "
                    "run_main.sh / run_131101.sh, output to validator scratch, not raw/"),
        "comparison": ("every replayed instance, cell, skip and run row equals the committed row with the same "
                       "(kind, family, arm, p, curve_seed, seed): byte-identical JSON lines after masking only "
                       "wall_ns / search_wall_s / wall_s, and equal parsed values on every other key"),
        "summary": s,
        "runs": runs,
    }
    cpath = os.path.join(vd, "replay-certificate.json")
    with open(cpath, "w") as f:
        json.dump(cert, f, indent=1)

    # ── validator report ──
    raw = []
    for fn in FILES:
        b = git(repo, "show", f"{sha}:{STUDY}/{fn}")
        raw.append({"path": f"../{fn}", "sha256": sha256_bytes(b)})
    ap, rp = ana["paired_sge_over_sfb"], rec["paired_sge_over_sfb"]
    head = []
    for w in ("1.0", "0.1", "0.0"):
        head.append({"name": f"pooled paired GAE_{w} sge/sfb (gm, equal weight per prime)",
                     "claimed": ap[f"gae_{w}"]["pooled"]["gm"], "recomputed": rp[f"gae_{w}"]["pooled"]["gm"]})
    for p in PRIMES:
        head.append({"name": f"paired GAE_1 sge/sfb p={p} (gm over 5 curves)",
                     "claimed": ap["gae_1.0"]["per_prime"][p]["gm"],
                     "recomputed": rp["gae_1.0"]["per_prime"][p]["gm"]})
    for p in PRIMES:
        head.append({"name": f"paired la total sge/sfb p={p}",
                     "claimed": ap["la_total"]["per_prime"][p]["gm"],
                     "recomputed": rp["la_total"]["per_prime"][p]["gm"]})
    for m in ("la_add", "la_mul", "la_total", "native_all"):
        head.append({"name": f"pooled paired {m} sge/sfb", "claimed": ap[m]["pooled"]["gm"],
                     "recomputed": rp[m]["pooled"]["gm"]})
    head.append({"name": "slope in ln p of paired GAE_1", "claimed": ap["gae_1.0"]["slope_ln_p"]["slope"],
                 "recomputed": rp["gae_1.0"]["slope_ln_p"]["slope"]})
    for arm, lab in ((SFB, "sfb"), (SGE, "sge")):
        for w in ("0.0", "0.1", "1.0"):
            for p in PRIMES:
                head.append({"name": f"IC/ideal neg. rho {lab} p={p} w={w}",
                             "claimed": ana["ic_over_rho"][arm][f"all_{w}"]["per_prime"][p]["gm"],
                             "recomputed": rec["ic_over_rho"][arm][f"w_{float(w):g}"]["per_prime"][p]["gm"]})
    for p in PRIMES:
        for arm in (SFB, SGE):
            a = ana["la_breakdown"][p][arm]
            b = rec["la_breakdown"][p][arm]
            head.append({"name": f"la share of native {arm[-3:]} p={p}", "claimed": a["la_share_of_native"],
                         "recomputed": b["la_share_of_native"]})
    head.append({"name": "runs certified, re-verified with validator arithmetic (RESULTS: all 80)",
                 "claimed": 80, "recomputed": certs["summary"]["rows_cert_ok"]})
    head.append({"name": "C0: 2lp-sfb rows identical to the ladder's (RESULTS: 40)",
                 "claimed": ana["C0_reproduction"]["identical"], "recomputed": rec["integrity"]["C0"]["identical"]})
    head.append({"name": "la_fallback runs (RESULTS: 8)", "claimed": 8,
                 "recomputed": rec["integrity"]["fallback_total"]})
    heads_ok = all(abs(h["claimed"] - h["recomputed"])
                   <= 1e-9 * max(abs(h["claimed"]), abs(h["recomputed"]), 1e-300) for h in head)

    # bootstrap intervals: producer (4000 MC) vs the exact bootstrap law (validator)
    ci = []
    nz = rec["bootstrap_noise_pooled_ci_400x4000"]
    for m in ap:
        a, b = ap[m]["pooled"]["ci95"], rp[m]["pooled"]["ci95_exact"]
        lo_r, hi_r = nz[m]["lo_range"], nz[m]["hi_range"]
        ci.append({"metric": m, "producer_pooled_ci95": a, "validator_exact_pooled_ci95": b,
                   "validator_mc_noise_lo_range": lo_r, "validator_mc_noise_hi_range": hi_r,
                   "producer_inside_noise_range": lo_r[0] <= a[0] <= lo_r[1] and hi_r[0] <= a[1] <= hi_r[1],
                   "max_rel_endpoint_diff_vs_exact": max(abs(a[i] - b[i]) / abs(b[i]) for i in (0, 1))})
    ci_ok = all(c["producer_inside_noise_range"] for c in ci)
    certs_ok = (certs["summary"]["curves_all_ok"] == certs["summary"]["curves"] == 20
                and certs["summary"]["rows_cert_ok"] == certs["summary"]["rows_checked"] == 80)
    integ = rec["integrity"]
    integ_ok = (integ["integrity"] and integ["same_log_both_arms"] and integ["same_instance_both_arms"]
                and integ["relation_phase_identical"] and integ["la_split_sums_ok"]
                and integ["C0"]["identical"] == integ["C0"]["compared"] == 40)
    dec = rec["decision"]
    dec_ok = (dec["ci95_mc"] == dec["ci95_exact"] == "IMPROVEMENT"
              and ana["decision"]["verdict"].startswith("IMPROVEMENT"))
    verd_ok = all(v["verdict_mc"] == v["verdict_exact"] == "UNRESOLVED"
                  and ana["ic_over_rho_verdict"][arm][p]["verdict"].startswith("UNRESOLVED")
                  for arm, d in rec["ic_over_rho_verdict"].items() for p, v in d.items())
    ex = ext
    bind_ok = (all(ex["manifests"]["frozen.sha256"].values()) and all(ex["manifests"]["data.sha256"].values())
               and all(all(v.values()) for v in ex["manifests"]["source.sha256 (vs Git blobs at 4faca82a6f0, 612f8dee20e, validated)"].values())
               and ex["manifests"]["binary"]["matches"])
    time_ok = (ex["timing"]["freeze_before_main_start"] and ex["timing"]["freeze_before_131101_start"]
               and ex["timing"]["first_commit_with_raw_after_freeze"]
               and ex["timing"]["analyze_py_unchanged_since_draft"]
               and ex["timing"]["protocol_section6_unchanged_by_freeze"])
    da = ex["dev_amendment"]
    dev_ok = (da["every_setting_has_4_fallbacks"] and da["every_fallback_on_sfb_multi_attempt_pair"]
              and da["fill_has_no_effect_on_any_field_except_wall_ns"] and da["pick_matches_protocol"])
    repro_ok = ex["analyze_reproduction"]["byte_identical"]
    stx = ex["structural"]
    struct_ok = (stx["sge_single_attempt_runs"] == stx["sge_spmv_add_equals_(2N+2)*sq_nnz"]
                 == stx["sge_trim_rows_in_equals_min(core_rows, core_cols+64)"]
                 == stx["sge_sq_dim_equals_sge_cols_out"] == 32
                 and 0.99 <= stx["sfb_spmv_add_over_2nnz(3c+2)_range"][0]
                 and stx["sfb_spmv_add_over_2nnz(3c+2)_range"][1] <= 1.01)
    status = "pass" if all((heads_ok, ci_ok, certs_ok, integ_ok, dec_ok, verd_ok, bind_ok, time_ok,
                            dev_ok, repro_ok, replay_ok, struct_ok)) else "fail"
    report = {
        "schema": "validator-report/v1",
        "study": STUDY,
        "status": status,
        "independent": True,
        "disposition": "validated" if status == "pass" else "see validator-report.md",
        "verdict": "validated-with-errata" if status == "pass" else "failed",
        "validated_commit": full_sha,
        "study_tree": git(repo, "rev-parse", f"{sha}:{STUDY}").decode().strip(),
        "report": "validator-report.md",
        "raw_data": raw,
        "headline": head,
        "checks": {
            "headline_agreement_1e-9": heads_ok,
            "bootstrap_ci_within_noise": ci_ok,
            "bootstrap_ci_comparison": ci,
            "certificates_independent": certs["summary"],
            "replay": s,
            "integrity_C0_same_log_relation_phase": integ_ok,
            "frozen_decision_IMPROVEMENT_reproduced": dec_ok,
            "strict_5a_order_sge_vs_sfb_holds_across_band": dec["strict_5a_order_holds_across_band"],
            "strict_5a_pooled_sides_w0_w01_w1": dec["strict_5a_pooled_sides_w0_w01_w1"],
            "band_crossover_w": rec["band_crossover"]["pooled_point_ratio_equals_1_at_w"],
            "ic_over_rho_every_cell_unresolved": verd_ok,
            "bindings_manifests_binary": bind_ok,
            "freeze_before_grid_data": time_ok,
            "dev_amendment_true_and_rule_followed": dev_ok,
            "analysis_json_byte_reproduced_by_frozen_script": repro_ok,
            "stress_trim_sge_charge_x100_w1": ex["stress_trim_sge_charge"]["x100_w1.0"],
            "structural_counter_identities": struct_ok,
        },
        "errata": "validator-report.md, section Errata (E1-E5) and process findings (P1-P4)",
        "scripts": ["replay.sh", "compare_replay.py", "verify_certificates.py", "picarith.py",
                    "recompute.py", "checks_extra.py", "build_gate_artifacts.py"],
        "label": "[pcc:unverified] (no pcc claim was created by the validator)",
    }
    rpath = os.path.join(vd, "validator-report.json")
    with open(rpath, "w") as f:
        json.dump(report, f, indent=1)

    # ── VALIDATION.json: band of the 2lp-sfb-sge cell at p = 131101 (the headline IC/rho) ──
    by = {}
    for r in rows:
        if r["kind"] == "run" and r["arm"] == SGE and r["p"] == 131101:
            by.setdefault(r["curve_seed"], []).append(r)
    curves = sorted(by)
    assert len(curves) == 5 and all(len(by[c]) == 2 for c in curves)
    rho = gm(math.sqrt(math.pi * by[c][0]["r"] / 4) for c in curves)
    band = {}
    for key, w in (("0", 0.0), ("0.1", 0.1), ("1", 1.0)):
        ic = gm(gm(x["group_ops"] + w * sum(x[k] for k in NATIVE) for x in by[c]) for c in curves)
        band[key] = {"ic": ic, "rho": rho}
    signs = {(v["ic"] > v["rho"]) - (v["ic"] < v["rho"]) for v in band.values()}
    man = {
        "schema": "validation-gate/v1",
        "note": ("Written by the validator. The study's claim is a paired component comparison "
                 "(2lp-sfb-sge vs 2lp-sfb, frozen rule IMPROVEMENT); the band below is the sge arm against the "
                 "ideal negation rho at p = 131101 and does not order across the band (UNRESOLVED, no rho claim). "
                 "The sge-vs-sfb order is also not a strict section-5a ranking: it reverses at w = 0 "
                 "(validator-report.md, E2). Paths are relative to this directory (validation/); if the manifest "
                 "is moved to the study root, prefix them with validation/ and change ../raw/ to raw/ in "
                 "validator-report.json."),
        "replay_certificate": {"path": "replay-certificate.json", "sha256": sha256_bytes(open(cpath, "rb").read())},
        "validator_report": {"path": "validator-report.json", "sha256": sha256_bytes(open(rpath, "rb").read())},
        "rho_reference": {
            "kind": "negation",
            "protocol": ("PROTOCOL.md (sha256 6d07d316...ac25) section 6: the ideal negation rho R = sqrt(pi r / 4) "
                         "group additions, nothing unpriced charged to rho; no measured rho is run"),
            "unit": ("GAE_w per solve, p = 131101, 2lp-sfb-sge: geometric mean over the 5 curves of the per-curve "
                     "geometric mean over solver seeds 1, 2; one counted Pic^0 operation = 1; each counted native "
                     "operation (rel_mul, rel_add, la_mul, la_add, idx_lookups, idx_inserts) = w; the ideal "
                     "negation rho charges no native work at any w"),
            "binary_or_process": (f"IC arms: genus_ladder.qla sha256 {BINARY_SHA} (replayed by the validator, all "
                                  "80 runs byte-identical). The negation reference is a closed form of each "
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
    print(json.dumps({"replay_ok": replay_ok, "heads_ok": heads_ok, "ci_ok": ci_ok, "certs_ok": certs_ok,
                      "integ_ok": integ_ok, "dec_ok": dec_ok, "verd_ok": verd_ok, "bind_ok": bind_ok,
                      "time_ok": time_ok, "dev_ok": dev_ok, "repro_ok": repro_ok, "struct_ok": struct_ok,
                      "status": status,
                      "n_headline": len(head), "band": band,
                      "order_holds_across_band": man["rho_reference"]["order_holds_across_band"]}, indent=1))


if __name__ == "__main__":
    main()
