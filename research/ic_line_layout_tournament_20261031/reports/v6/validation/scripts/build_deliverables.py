#!/usr/bin/env python3
"""Assemble validation.json, the gate draft VALIDATION.json, and the two files it pins
(replay certificate, validator report) from the validator's own outputs.
usage: build_deliverables.py VALIDATION_DIR UNTRUSTED_ROOT VAL_ID"""
import hashlib
import json
import sys
from pathlib import Path

V = Path(sys.argv[1])
B = Path(sys.argv[2])
VAL_ID = sys.argv[3]
rc = json.loads((V / "recompute.json").read_text())
vd = json.loads((V / "verdict-rederived.json").read_text())
desc = json.loads((V / "descriptive.json").read_text())
A = str(B / "analysis")


def sha(p):
    return hashlib.sha256(Path(p).read_bytes()).hexdigest()


def rows(key):
    return {r["field"]: r for r in rc["comparisons"][key]["rows"]}


PROD = {"s1": f"s1@{A}/run-i:1", "s2": f"s2@{A}/run-i:2", "s3": f"s3@{A}/run-ii:1",
        "s1_run_ii": f"s1@{A}/run-ii:2"}
CNT = f"counted@{A}/run-i"
SESS = ("s1", "s2", "s3", "counted")
records_sha = {}
sums_sha = {}
for s in SESS:
    line = next(l for l in (B / f"{s}-untrusted" / "SHA256SUMS").read_text().splitlines()
                if l.endswith("  session/records.jsonl"))
    records_sha[s] = line.split()[0]
    sums_sha[s] = sha(B / f"{s}-untrusted" / "SHA256SUMS")
    assert sha(B / f"{s}-untrusted" / "session" / "records.jsonl") == records_sha[s]

replay = {s: json.loads((V / "replay" / f"{s}-replay-all.json").read_text()) for s in SESS}
replay_exit = {s: int((V / "replay" / f"{s}-replay-all.exit").read_text()) for s in SESS}

# ------------------------------------------------------------- headline list
headline = []
for s in ("s1", "s2", "s3"):
    r = rows(PROD[s])
    for f in ("candidate.total_wall_ratio", "candidate.total_wall_ci95[0]", "candidate.total_wall_ci95[1]",
              "aa.ratio", "aa.ci95[0]", "aa.ci95[1]", "corrected.ratio", "corrected.ci95[0]",
              "corrected.ci95[1]", "candidate.ops_ratio", "control.aa_ci95[0]", "control.aa_ci95[1]",
              "positive.total_wall_ratio", "positive.total_wall_ci95[0]", "positive.ops_ratio"):
        headline.append({"name": f"{s}.{f}", "claimed": r[f]["producer"], "recomputed": r[f]["mine"],
                         "rel_tol": 1e-12})
rc_cnt = rows(CNT)
for w in ("0", "0.1", "1"):
    for pr, nm in (("rho-neg__incumbent", "counted.ic_over_rho_S"), ("incumbent__candidate", "counted.candidate_over_incumbent_S")):
        f = f"{pr}@w{w}.ratio"
        headline.append({"name": f"{nm}@w{w}", "claimed": rc_cnt[f]["producer"], "recomputed": rc_cnt[f]["mine"],
                         "rel_tol": 1e-12})
max_rel = max(abs(h["claimed"] - h["recomputed"]) / max(abs(h["claimed"]), abs(h["recomputed"]))
              for h in headline if h["claimed"] != h["recomputed"]) if any(
    h["claimed"] != h["recomputed"] for h in headline) else 0.0

# ------------------------------------------------------------- gate draft files
EVID = "research/ic_line_layout_tournament_20261031/reports/v6"
raw_rel = {s: f"../../eks-results-v6/{s}/session/records.jsonl" for s in SESS}
replay_cert = {
    "status": "verified",
    "independent": True,
    "scalars_verified": True,
    "validator_report_id": VAL_ID,
    "binary_sha256": "73362eed4165d6db20fe1d398baa8fabefa2497aff6e78dab9ed8f31af31dd01",
    "command": "ecbench verify --dir <copy of session/> --replay-all --exit-code --out <validator dir>",
    "independence_note": "replayed by the validator on its own copy of each S3-anchored session, with the pinned binary the protocol requires; same binary as the producer, so this certifies determinism and record binding, not an independent implementation",
    "runs": [{"session": s, "session_id": replay[s]["session_id"], "records": replay[s]["records"],
              "verified_records": replay[s]["verified_records"], "replays": len(replay[s]["replays"]),
              "reproduced": sum(1 for x in replay[s]["replays"] if x["reproduced"]),
              "exit_code": replay_exit[s], "ok": replay[s]["ok"],
              "records_jsonl_sha256": records_sha[s], "receipt_sha256": sha(V / "replay" / f"{s}-replay-all.json")}
             for s in SESS],
}
validator_report = {
    "status": "pass",
    "independent": True,
    "validator_report_id": VAL_ID,
    "scope": "recompute of the v6 headline numbers from raw records.jsonl (validator-owned Python, no shared code with analyze-v6.sh or ecbench)",
    "experiment_verdict_rederived": vd["combined_verdict"],
    "note": "status pass means the headline numbers recompute; it does not admit any improvement row. The combined v6 verdict is invalid_measurement and the candidate is ~12% slower in wall time.",
    "raw_data": [{"path": raw_rel[s], "sha256": records_sha[s]} for s in SESS],
    "headline": headline,
}
gd = V / "gate-draft"
gd.mkdir(exist_ok=True)
rc_path = gd / "VALIDATION-replay-certificate.json"
vr_path = gd / "VALIDATION-validator-report.json"
rc_path.write_text(json.dumps(replay_cert, indent=1, sort_keys=True) + "\n")
vr_path.write_text(json.dumps(validator_report, indent=1, sort_keys=True) + "\n")

ops = rc["counted"]["ops"]
band = {w: {"ic": ops[k]["rho-neg__incumbent"]["mean_S_b"], "rho": ops[k]["rho-neg__incumbent"]["mean_S_a"]}
        for w, k in (("0", "0.0"), ("0.1", "0.1"), ("1", "1.0"))}
orders = rc["counted"]["band_orders"]
order_holds = all(v == "rho-neg < incumbent" for k, v in orders.items() if k.endswith("incumbent_vs_rho-neg")) and \
    all(v == "rho-neg < candidate" for k, v in orders.items() if k.endswith("candidate_vs_rho-neg"))
VALIDATION = {
    "schema": "validation-gate/v1",
    "note": (f"DRAFT by validator {VAL_ID} for evidence dir {EVID}/. It covers the v6 evidence only; v6 has "
             "no improvement row to show (combined verdict invalid_measurement; candidate/incumbent wall ~1.12). "
             "raw_data paths do not exist in the repository yet: the four session/records.jsonl must be committed "
             "at the pinned sha256 before validation_gate.py evidence can pass. Not pcc-registered or signed."),
    "replay_certificate": {"path": rc_path.name, "sha256": sha(rc_path)},
    "validator_report": {"path": vr_path.name, "sha256": sha(vr_path)},
    "rho_reference": {
        "kind": "negation",
        "protocol": "rho-neg (rho.negation, cap_multiple 64) as named in spec-fp28-v6-counted.json, counted companion ic-line-v6-counted-20261106",
        "unit": "GAE (ecbench.gae, lower bound), reported as S = GAE / sqrt(r)",
        "binary_or_process": "ecbench 73362eed4165d6db20fe1d398baa8fabefa2497aff6e78dab9ed8f31af31dd01, one counted session (same session as the IC arms)",
        "matches_protocol": True, "strongest_available": True,
        "same_unit": True, "same_binary_or_process": True,
        "gae_band": band,
        "order_holds_across_band": order_holds,
    },
}
(V / "VALIDATION.json").write_text(json.dumps(VALIDATION, indent=1, sort_keys=True) + "\n")
(gd / "VALIDATION.json").write_text(json.dumps(VALIDATION, indent=1, sort_keys=True) + "\n")

# ------------------------------------------------------------- validation.json (per-check)
S = vd["sessions"]
W = rc["wall"]
C = rc["counted"]


def chk(cid, status, what, evidence):
    return {"id": cid, "status": status, "check": what, "evidence": evidence}


checks = [
    chk("A1-sha256sums", "pass", "SHA256SUMS of every session verifies; file set equals the list; SHA256SUMS and job-exit.json byte-equal to the S3 originals",
        {s: {"sha256sums_sha256": sums_sha[s], "records_jsonl_sha256": records_sha[s]} for s in SESS}),
    chk("A2-s3-anchor", "pass", "validator read S3 directly: each v6 job prefix has exactly one pod dir; only five v6 jobs exist (qual, s1, s2, s3, counted); no -r2/-nr",
        {"listing": "s3-anchor/v6-job-prefixes.txt"}),
    chk("A3-binary", "pass", "pinned binary sha256 equals 73362eed...; every record and every binary.sha256 carries it",
        {s: W[s]["binary_sha256_set"] for s in ("s1", "s2", "s3")}),
    chk("A4-image-extraction", "caveat", "binary extraction from image 3c4ad8bf... (layer 5) is executor-attested (analysis-step-E.out); the validator checked only the binary hash", {}),
    chk("A5-spec-and-build", "pass", "input-spec.json and session/spec.json JSON-equal (jq -S -c) to the frozen spec; request-embedded spec equal; build-info source ea8e9ef7 and ecbench 73362eed equal the request's job.build",
        {"wall_spec_sha256": "0116954ff2646a81fa4383ef87125d445f2f3d3d4315a4ed49db9d0845515a81",
         "counted_spec_sha256": "ac231ee6303942e3d491daa2c31e07216a72cf227e15015778aa7ffe829bb2a2"}),
    chk("A6-freeze", "pass", "protocol_sha256 2aedaaf8... and prior 12415faa... reproduce with experiment_protocol.py hash; validate 0 findings; amendment sha 6383bbfc and request-session-3 sha 9302e585 match the amendments[] entry",
        {}),
    chk("A7-timing", "pass", "original freeze 04:25:05Z before s2 start 04:51:55Z; PR #3531 merged 05:32:59Z before s3 start 06:23:19Z", {}),
    chk("A8-committed-snapshot", "caveat", "frozen contract is committed on main e87fa18a (study dir identical at 4a3461a7f); the session raw data and analysis outputs are NOT committed (untracked in the working tree; records.jsonl absent from the repo). Receipts are anchored to S3 instead", {}),
    chk("R1-replay", "pass", "ecbench verify --replay-all with the pinned binary into the validator's dir: exit 0, ok, 576/576 verified, 512/512 replays identical in every session",
        {s: {"exit": replay_exit[s], "replays": len(replay[s]["replays"]),
             "reproduced": sum(1 for x in replay[s]["replays"] if x["reproduced"])} for s in SESS}),
    chk("M1-total-wall", "pass", "candidate/incumbent total solve-wall ratio and paired linearised-t 95% CI (delta method on log R, t_127) recomputed; agrees with producer to <= 4e-16 relative",
        {s: {"ratio": W[s]["candidate"]["ratio"], "ci95": W[s]["candidate"]["ci95_log"]} for s in ("s1", "s2", "s3")}),
    chk("M2-aa", "pass", "incumbent-aa/incumbent A/A ratio and CI recomputed; agrees to <= 3e-16",
        {s: {"ratio": W[s]["aa"]["ratio"], "ci95": W[s]["aa"]["ci95_log"]} for s in ("s1", "s2", "s3")}),
    chk("M3-corrected", "pass", "corrected = R/AA, log-scale variance sum, Welch-Satterthwaite dof; agrees to <= 3e-16",
        {s: {"ratio": W[s]["candidate_corrected"]["ratio"], "ci95": W[s]["candidate_corrected"]["ci95_log"],
             "dof": W[s]["candidate_corrected"]["dof"]} for s in ("s1", "s2", "s3")}),
    chk("M4-ops", "pass", "counted ops ratio candidate/incumbent exactly 1 at w = 0, 0.1, 1 (every pair identical), CI [1,1]",
        {w: C["ops"][w]["incumbent__candidate"]["ratio_S"] for w in C["ops"]}),
    chk("M5-rho", "pass", "IC/rho S ratio at w = 0, 0.1, 1 (workload-cluster linearised t, t_15) and per-curve band recomputed; agrees to <= 7e-16; rho < IC on both curves at every w",
        {w: {"ratio": C["ops"][w]["rho-neg__incumbent"]["ratio_S"], "ci95": C["ops"][w]["rho-neg__incumbent"]["ci95_cluster_log"]}
         for w in C["ops"]}),
    chk("P1-record-parity", "pass", "candidate vs incumbent equal on the frozen projection in 144/144 cells per wall session and 144/144 in counted; counted IC records equal each wall session's in 288/288 cells; outside timing only detail.label, fbm_id and phases[].wall_ns differ",
        {"wall": {s: W[s]["record_parity"]["mismatched_cells"] for s in ("s1", "s2", "s3")},
         "counted": C["record_parity_in_counted"], "cross": C["cross_session_parity"]}),
    chk("C1-aa-gate", "fail_s1_pass_s2_s3", "A/A CI must be non-null, contain 1 and lie in [0.9,1.1] (both the candidate comparison and control-comparison.json read the same incumbent-aa/incumbent pair)",
        {s: S[s]["gates"]["aa_inside_0_9_1_1"] for s in ("s1", "s2", "s3")}),
    chk("C2-positive-control", "pass", "pc-fb2x/incumbent: ops S ratio 2.396 > 1, total wall ratio ~1.38 with CI lower bound > 1.34, all pairs L2",
        {s: {"ratio": W[s]["positive"]["ratio"], "ci95": W[s]["positive"]["ci95_log"]} for s in ("s1", "s2", "s3")}),
    chk("C3-null-object", "pass", "the null object is the same-session twin incumbent-aa, at the same block positions {1,2} as the candidate; A/A ratios 0.9984/0.9996/0.9989 against candidate ~1.12",
        {s: W[s]["block_positions"]["measured"] for s in ("s1",)}),
    chk("V1-per-session", "pass", "per-session verdicts re-derived from the frozen rule text: s1 invalid_measurement (A/A gate), s2 inconclusive, s3 inconclusive; gates identical to the producer's; unchanged under both readings of outside_noise",
        {s: S[s]["verdict"] for s in ("s1", "s2", "s3")}),
    chk("V2-host-epoch", "pass", "amendment section 3 command exits 0: s3 node ip-10-84-75-83 differs from H1 ip-10-84-67-216, pod UIDs distinct, s3 exit 0", vd["host_epoch"]),
    chk("V3-determinism", "pass", "s1 {verify_exit, verdict, gates} byte-equal in run (i) and run (ii); s1 compare files byte-identical across runs", vd["s1_determinism"]),
    chk("V4-combined", "pass", "three-session combination: any invalid_measurement gives invalid_measurement; host-epoch rule can only lower support",
        {"combined": vd["combined_verdict"]}),
    chk("K1-counted-acceptance", "pass", "counted: audit OK, 576 verified, 512/512 replays identical, invalid_outcomes [], spec/build equal to freeze; exit 42 accepted (97 below-L2 executions, all rho arms at L1)",
        {"below_l2": desc["counted_below_l2"]}),
    chk("X1-regression-label", "caveat", "candidate is slower by ~12% in all three sessions (CI wholly above 1); the frozen vocabulary has no regression label and maps it to inconclusive with the direction reported descriptively", {}),
    chk("X2-admission-noise-labels", "caveat", "ecbench's own wall.status 'admitted' and outside_noise labels are not re-implemented; the validator checked their inputs (128 pairs, all L2) and two readings of outside_noise, both true", {}),
    chk("X3-qualification", "caveat", "4 of 5 frozen qualification conditions re-checked from committed files; the min solve_wall_ns >= 5 ms condition needs the uncommitted qualification records.jsonl (attested true)", {}),
    chk("X4-card-procedure", "caveat", "pre-launch EC2 node check (0 and 0 at 06:20:27Z) and the budget gate line (USD 1.14 <= 5.00) are executor-attested card records", {}),
    chk("X5-output-paths", "caveat", "analysis written to scratch run-i/run-ii instead of reports/v6/analysis-a|b; the later reports/v6 copies are byte-identical (empty .stderr files omitted)", {}),
]
out = {
    "schema": "validator.validation/v1",
    "validation_report_id": VAL_ID,
    "protocol_id": "IC-LINE-LAYOUT-E2E-FP28-V6-20261106",
    "amendment": "V6-AMENDMENT-1 (PR #3531)",
    "repo_revision": {"named": "e87fa18a73340a95871c73c259eca0fa99ebf81e", "checked_out": "4a3461a7f2a7db7820e7c49d108e317455294100",
                      "study_dir_identical": True},
    "checks": checks,
    "per_session_verdicts": {s: S[s]["verdict"] for s in ("s1", "s2", "s3")},
    "combined_verdict": vd["combined_verdict"],
    "reported_outcome_confirmed": True,
    "headline_max_rel_diff": max_rel,
    "validation_verdict": "passed",
    "artifacts": {p: sha(V / p) for p in ("recompute.json", "verdict-rederived.json", "descriptive.json",
                                          "VALIDATION.json", "gate-draft/VALIDATION-replay-certificate.json",
                                          "gate-draft/VALIDATION-validator-report.json",
                                          "scripts/recompute.py", "scripts/derive_verdict.py",
                                          "scripts/descriptive.py")},
}
(V / "validation.json").write_text(json.dumps(out, indent=1, sort_keys=True) + "\n")
print(json.dumps({"headline_rows": len(headline), "max_rel": max_rel, "order_holds": order_holds,
                  "band": band, "combined": vd["combined_verdict"]}))
