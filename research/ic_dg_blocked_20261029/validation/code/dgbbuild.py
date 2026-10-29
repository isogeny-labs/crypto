#!/usr/bin/env python3
"""Assemble the validator outputs for XP-20261029-dg-blocked (validator's own code).

Reads work/recompute.json (dgbval.py), work/seals.json (dgbseal.py), the
replay outputs under replay/ and the committed analysis files (as "claimed").
Writes validator-report.json, replay-certificate.json, VALIDATION.draft.json
and REPORT.md under /dev/shm/dgb-validation.
"""
import hashlib
import json
import os
import platform
import secrets

V = "/dev/shm/dgb-validation"
EV = "/dev/shm/wt-blk/research/ic_dg_blocked_20261029"
S3 = "/dev/shm/dgb-val/s3"
BIN = "88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c"
WT = {"w0": 0.0, "w0.1": 0.1, "w1": 1.0}
SIZES = ("P24", "P28", "P32")
JOBS = {("P24", "wall"): "ic-dgb-p24-20261029", ("P24", "counted"): "ic-dgb-rho-p24-20261029",
        ("P28", "wall"): "ic-dgb-p28-20261029", ("P28", "counted"): "ic-dgb-rho-p28-20261029",
        ("P32", "wall"): "ic-dgb-p32-20261029", ("P32", "counted"): "ic-dgb-rho-p32-20261029"}


def jload(p):
    with open(p) as f:
        return json.load(f)


def sha(p):
    h = hashlib.sha256()
    with open(p, "rb") as f:
        for c in iter(lambda: f.read(1 << 20), b""):
            h.update(c)
    return h.hexdigest()


def dump(obj, p):
    with open(p, "w") as f:
        json.dump(obj, f, indent=1, sort_keys=False)
        f.write("\n")


idp = f"{V}/work/id.txt"
if not os.path.exists(idp):
    with open(idp, "w") as f:
        f.write(f"VAL-20261029-{secrets.token_hex(3)}\n")
VID = open(idp).read().strip()

R = jload(f"{V}/work/recompute.json")
SEALS = jload(f"{V}/work/seals.json")
S = R["sizes"]

# ------------------------------------------------------------ headline (claimed = committed files)
head = []


def H(name, claimed, recomputed, tol=1e-9):
    head.append({"name": name, "claimed": claimed, "recomputed": recomputed, "rel_tol": tol})


for P in SIZES:
    b = P[1:]
    rc = S[P]["recompute"]
    dec = rc["decision_dg__dgb"]
    for wtag in WT:
        com = jload(f"{EV}/{P}/wall/analysis/compare-dg__dgb-{wtag}.json")["wall"]
        for k in ("ns_per_gae_a", "ns_per_gae_b", "ratio_b_over_a", "aa_ratio", "corrected_ratio",
                  "corrected_dof", "total_wall_ratio", "pairs", "pairs_below_level", "aa_pairs"):
            H(f"{P}/wall/dg__dgb/{wtag}/wall.{k}", com[k], dec[k])
        for k in ("ci95", "aa_ci95", "corrected_ci95"):
            H(f"{P}/wall/dg__dgb/{wtag}/wall.{k}.lo", com[k][0], dec[k][0])
            H(f"{P}/wall/dg__dgb/{wtag}/wall.{k}.hi", com[k][1], dec[k][1])
    for pair, d in rc["descriptive"].items():
        com = jload(f"{EV}/{P}/wall/analysis/compare-{pair}-w0.json")["wall"]
        H(f"{P}/wall/{pair}/w0/wall.ratio_b_over_a", com["ratio_b_over_a"], d["ratio_b_over_a"])
        H(f"{P}/wall/{pair}/w0/wall.ci95.lo", com["ci95"][0], d["ci95"][0])
        H(f"{P}/wall/{pair}/w0/wall.ci95.hi", com["ci95"][1], d["ci95"][1])
        H(f"{P}/wall/{pair}/w0/wall.pairs", com["pairs"], d["pairs"])
        H(f"{P}/wall/{pair}/w0/wall.pairs_below_level", com["pairs_below_level"], d["pairs_below_level"])
    wg = S[P]["wall_gate"]
    for wtag in WT:
        H(f"{P}/wall/dg__dgb/{wtag}/ops.ratio_b_over_a", wg["8_identical_work"]["committed_ops_ratio"][wtag],
          wg["8_identical_work"]["recomputed_pairwise_S"][wtag]["sum_ratio_b_over_a"])
        for a, bb in (("dg", "dg-aa"), ("dgb-edge", "dgb")):
            H(f"{P}/wall/{a}__{bb}/{wtag}/ops.ratio_b_over_a",
              wg["3_counted_aa"][f"{a}/{bb} {wtag} (committed ops.ratio_b_over_a)"],
              wg["3_counted_aa"][f"{a}/{bb} recomputed"][wtag]["sum_ratio_b_over_a"])
    cc = jload(f"{S3}/{P}-wall/control-comparison.json") if os.path.exists(f"{S3}/{P}-wall/control-comparison.json") else None
    if cc:
        H(f"{P}/wall/control-comparison/wall.aa_pairs", cc["wall"]["aa_pairs"], 288)
    for key, e in S[P]["band_vs_committed"].items():
        if key == "max_rel_diff":
            continue
        H(f"{P}/counted/{key}/ops.ratio_b_over_a", e["committed_ops_ratio"], e["recomputed"])
        for slug, c in e["per_curve"].items():
            H(f"{P}/counted/{key}/curves[{slug}].ratio_b_over_a", c["committed"], c["recomputed"])
    ps = jload(f"{EV}/{P}/wall/analysis/phase_sums.json")["by_arm"]
    for arm in ("dg", "dgb"):
        for k, sub in (("phases.oracle_setup.wall_ns", ("phase_wall_ms", "oracle_setup")),
                       ("online.phases_ns.target_PDP", ("online_phases_ms", "target_PDP")),
                       ("phases.relations.wall_ns", ("phase_wall_ms", "relations")),
                       ("time.solve_wall_ns", None)):
            mine = S[P]["phase_sums_wall"][arm]
            m = mine["solve_wall_ms"] * 1e6 if sub is None else mine[sub[0]][sub[1]] * 1e6
            H(f"{P}/wall/phase_sums/{arm}/{k}", ps[arm][k], m)

bad_head = [h for h in head if abs(h["claimed"] - h["recomputed"]) > h["rel_tol"] * max(abs(h["claimed"]), abs(h["recomputed"]), 1e-300)]

# main-session claims (rounded, as stated in the task), checked at their stated precision
p28 = S["P28"]
ms = [
    {"claim": "P28 corrected dgb/dg 0.9300", "stated": 0.9300, "recomputed": p28["recompute"]["decision_dg__dgb"]["corrected_ratio"], "precision": 5e-5},
    {"claim": "P28 corrected_ci95 lower 0.9266", "stated": 0.9266, "recomputed": p28["recompute"]["decision_dg__dgb"]["corrected_ci95"][0], "precision": 5e-5},
    {"claim": "P28 corrected_ci95 upper 0.9333", "stated": 0.9333, "recomputed": p28["recompute"]["decision_dg__dgb"]["corrected_ci95"][1], "precision": 5e-5},
    {"claim": "P28 set-up (oracle_setup) dg 1769 ms", "stated": 1769, "recomputed": p28["phase_sums_wall"]["dg"]["phase_wall_ms"]["oracle_setup"], "precision": 0.5},
    {"claim": "P28 set-up (oracle_setup) dgb 1000 ms", "stated": 1000, "recomputed": p28["phase_sums_wall"]["dgb"]["phase_wall_ms"]["oracle_setup"], "precision": 0.5},
    {"claim": "P28 target PDP dg 1062 ms", "stated": 1062, "recomputed": p28["phase_sums_wall"]["dg"]["online_phases_ms"]["target_PDP"], "precision": 0.5},
    {"claim": "P28 target PDP dgb 1428 ms", "stated": 1428, "recomputed": p28["phase_sums_wall"]["dgb"]["online_phases_ms"]["target_PDP"], "precision": 0.5},
]
for m in ms:
    m["agrees_at_stated_precision"] = abs(m["stated"] - m["recomputed"]) <= m["precision"]

# ------------------------------------------------------------ raw data list (paths relative to the evidence dir)
raw = []
for P in SIZES:
    for s in ("wall", "counted"):
        base = f"{S3}/{P}-{s}"
        for fn in ("session/records.jsonl", "session/session.json", "session/plan.json", "session/spec.json",
                   "session/host.json", "audit.json", "l2-gate.json", "job-exit.json", "input-spec.json",
                   "build-info.json", "SHA256SUMS", "control-comparison.json"):
            if os.path.exists(f"{base}/{fn}"):
                raw.append({"path": f"{P}/{s}/eks-results/{fn}", "sha256": sha(f"{base}/{fn}")})

# ------------------------------------------------------------ replay certificate
runs = []
for P in SIZES:
    for s in ("wall", "counted"):
        d = f"{V}/replay/{P}-{s}"
        a = jload(f"{d}/audit.json")
        inj = jload(f"{S3}/{P}-{s}/audit.json")
        sums = {}
        for line in open(f"{S3}/{P}-{s}/SHA256SUMS"):
            h, fn = line.split()
            sums[fn] = h
        before = {}
        for line in open(f"{d}/session-files.before.sha256"):
            h, fn = line.split()
            before[fn] = h
        after = open(f"{d}/session-files.after.sha256").read()
        reps = a["replays"]
        runs.append({
            "size": P, "session_kind": s, "job": JOBS[(P, s)],
            "pod_uid": jload(f"{S3}/{P}-{s}/job-exit.json")["pod_uid"],
            "session_id": a["session_id"], "session_status": a["session_status"],
            "records": a["records"], "verified_records": a["verified_records"],
            "measured_runs": sum(1 for _ in reps), "replays": len(reps),
            "replays_identical": sum(1 for x in reps if x["detail"] == "identical" and x["reproduced"]),
            "problems": len(a["problems"]), "ok": a["ok"], "checks": a["checks"],
            "auditor_binary_sha256": a["auditor_binary_sha256"],
            "auditor_env_class_id": a["auditor_env_class_id"],
            "auditor_hardware_class_id": a["auditor_hardware_class_id"],
            "audited_unix_ms": a["audited_unix_ms"],
            "replay_audit_path": f"replay/{P}-{s}/audit.json", "replay_audit_sha256": sha(f"{d}/audit.json"),
            "replayed_copy_equals_job_SHA256SUMS": all(sums.get(f"session/{fn}") == h for fn, h in before.items()),
            "session_files_unchanged_by_replay": open(f"{d}/session-files.before.sha256").read() == after,
            "replayed_records_jsonl_sha256": before["records.jsonl"],
            "same_replay_seq_set_as_in_job_audit": sorted(x["seq"] for x in reps) == sorted(x["seq"] for x in inj["replays"]),
            "in_job_audit": {"ok": inj["ok"], "replays": len(inj["replays"]),
                             "identical": sum(1 for x in inj["replays"] if x["detail"] == "identical"),
                             "auditor_env_class_id": inj["auditor_env_class_id"],
                             "auditor_hardware_class_id": inj["auditor_hardware_class_id"]},
            "verify_log": open(f"{d}/verify.log").read().splitlines(),
            "verify_stderr": open(f"{d}/verify.stderr").read().splitlines(),
        })
all_ok = all(r["ok"] and r["replays"] == r["replays_identical"] == 1152 and r["problems"] == 0
             and r["replayed_copy_equals_job_SHA256SUMS"] and r["session_files_unchanged_by_replay"]
             and r["auditor_binary_sha256"] == BIN for r in runs)
cert = {
    "schema": "replay-certificate/v1",
    "id": f"{VID}-replay",
    "experiment": "XP-20261029-dg-blocked",
    "snapshot": "claude/ic-dg-blocked-20261029 at bf899979a6a6304fbbfbbc27204e50acfc52f6b8 (verdict snapshot, pushed; equals origin)",
    "status": "verified" if all_ok else "failed",
    "independent": True,
    "scalars_verified": all_ok,
    "replayer": "Validator subagent (claude-opus-5-5): not the protocol author, director, executor or analyst of XP-20261029-dg-blocked",
    "method": ("ecbench verify --dir <copy> --replay-all --exit-code --out <audit.json> on a copy of each of the six sessions "
               "(host.json, plan.json, records.jsonl, session.json, spec.json) taken from the S3 pod original, whose every file is "
               "byte-equal to the evidence directory and to the job's SHA256SUMS. --replay-all re-executes every measured deterministic "
               "run and requires identical counted scalars (GAE, per-phase counts, native counters, recovered logarithm); the audit also "
               "re-checks record seals, plan/host/spec hashes and completeness. Script: code/replay.sh."),
    "binary": {"path": "/dev/shm/isolab-build/ecbench", "sha256": BIN,
               "note": "the image's binary (built from be2969ce); the replay host differs from the measuring pods"},
    "replay_host": {"kernel": platform.release(), "machine": platform.machine(), "cpus": os.cpu_count(),
                    "cpu_model": "Intel(R) Xeon(R) Processor @ 2.10GHz", "env_class_id": runs[0]["auditor_env_class_id"],
                    "hardware_class_id": runs[0]["auditor_hardware_class_id"]},
    "measuring_host_class": "EKS c8a.2xlarge pods (AMD EPYC 9R45, AuthenticAMD), env class ECBENV3hc1c5e5126853",
    "what_is_not_replayable": ("wall time. The replay verifies counted scalars only; wall admissibility rests on the in-job L2 grading, "
                               "the native A/A admission (control-comparison.json) and the validator's recomputation from records.jsonl."),
    "runs": runs,
    "totals": {"sessions": len(runs), "records": sum(r["records"] for r in runs),
               "replays": sum(r["replays"] for r in runs), "replays_identical": sum(r["replays_identical"] for r in runs),
               "problems": sum(r["problems"] for r in runs)},
    "record_seals": {"recomputed": SEALS["total_records"], "failures": SEALS["total_seal_failures"], "code": "code/dgbseal.py"},
    "answers_checked": "every record of all six sessions has outcome.status verified and matches_target true (validator recompute)",
}
dump(cert, f"{V}/replay-certificate.json")

# ------------------------------------------------------------ verdicts
def wfail(P):
    return S[P]["wall_gate"]["failed"]


size_verdict = {
    "P24": "fail",
    "P28": "pass_with_caveats",
    "P32": "fail",
}
size_reason = {
    "P24": ("fail: wall-session section 6.1 fails conditions 1, 5, 6, 7 and 9 (exit 42; l2-gate not accepted; no control-comparison.json; "
            "decision block pairs_below_level 1; identity rel diff 1.17e-5). The executions below L2 are seq 1144 (dgb-edge, 903.9 us) and "
            "seq 1145 (dgb, 913.0 us): one block (icv1-fp24-tm995-ee6c519b, target 22, round 5, reversed order, 28,031 GAE, the cheapest "
            "instance), graded L1 only because grading v3 does not admit wall time under 1 ms; dg and dg-aa in the same block took 1.270 and "
            "1.284 ms. The counted session also fails its route: counted dgb seq 1145 (911.2 us) is an ic.pipeline execution below L2, so the "
            "exit-42 clause does not hold. The main session's 'undecidable' reading is confirmed; no row is admissible."),
    "P28": ("pass with caveats: wall-session conditions 1-9 and A1 hold and the counted session holds under the exit-42 clause; corrected "
            "dgb/dg 0.929953 [0.926605, 0.933313], upper bound below 1, recomputed to 1e-16 of the committed block; 2,304 replays identical. "
            "Caveats are of scope and interpretation, not of the instrument: the two curves differ beyond their intervals (0.9379 vs 0.9212); "
            "one host class; the gain is a wall-clock component speed-up (counted S identical); the prediction (0.77-0.84) was missed."),
    "P32": ("fail: wall-session section 6.1 fails conditions 1, 5, 6, 7 and 9 (exit 42; l2-gate not accepted; no control-comparison.json; "
            "decision block aa_pairs 287; identity rel diff 5.9e-5). The one execution below L2 is seq 1313 (dg-aa, icv1-fp32-tm81759-3ad5e8c5, "
            "target 16, round 6), graded L1 for 'L2: 3 foreign task slices on the run CPU during the solve (0 us)'; its solve (195.09 ms) is "
            "within 0.07% of dg's in the same block. The counted session holds under the exit-42 clause, but that does not rescue the wall "
            "decision. The main session's 'undecidable' reading is confirmed; no row is admissible."),
}

# ------------------------------------------------------------ rho reference band (P28 rows)
b28 = S["P28"]["band_counted"]
rows = sorted(b28["per_curve"])
gae_band = {k: {"ic": b28["pooled_mean_S"]["dgb"][k], "rho": b28["pooled_mean_S"]["rho-neg"][k]} for k in ("0", "0.1", "1")}
gae_band_by_row = {slug: {k: {"ic": b28["per_curve"][slug]["mean_S"]["dgb"][k], "rho": b28["per_curve"][slug]["mean_S"]["rho-neg"][k]}
                          for k in ("0", "0.1", "1")} for slug in rows}
order_holds = all(b28["per_curve"][s]["ic_over_rho"][a]["order_holds_across_band"] for s in rows for a in ("dg", "dgb"))

curves_all = {P: S[P]["curves"] for P in SIZES}
basis = []
for P in SIZES:
    for slug, c in curves_all[P].items():
        basis.append(f"{slug}: p={c['p']} prime={c['p_prime']}, r={c['r']} prime={c['r_prime']}, r=p+1-t={c['r_eq_p_plus_1_minus_t']}, "
                     f"cofactor {c['cofactor']}, j={c['j_recomputed']} (recomputed from a,b solved on G and targets; equals ICV1: {c['j_matches']}), "
                     f"squarefree part of t^2-4p = {c['squarefree_part_of_D']}")

caveats = [
    "28 bits: wall-clock improvement of the packed-table oracle at N_inc 407: corrected dgb/dg ns per GAE 0.929953 [0.926605, 0.933313] (288 L2 pairs, both A/A admitted). It is a component speed-up of the oracle (whole-solve wall per charged GAE), outside tables A-C; counted S is identical record by record, so no counted ranking and no distance to rho changes (dg/rho-neg = dgb/rho-neg = 21.79, 20.66, 15.48 at w = 0, 0.1, 1).",
    "Per-curve corrected ratios at 28 bits differ beyond their intervals: icv1-fp28-t9805-a382b8a8 0.9379 [0.9333, 0.9425], icv1-fp28-tm3463-6d2f5699 0.9212 [0.9168, 0.9257]. Both are below 1; the size of the gain depends on the curve.",
    "Target-clustered intervals (48 targets) are a little wider than the pair interval and stay below 1: linearised joint [0.92610, 0.93382], independent combination [0.92597, 0.93396], stratified bootstrap [0.92703, 0.93276].",
    "ns per GAE is an upper bound per operation for both arms alike (unpriced native work: sqrt solves, lookups, row ops, Legendre symbols, target-guard probes). Measured on EKS c8a.2xlarge (AMD EPYC 9R45) only; wall time is not replayable, counted scalars are (6,912 of 6,912 replays identical on an Intel host).",
    "Packed layout only (RULING-P2 A1): p <= 2^32-1 and at most 2^16 signed points. The small-maps branch of the same oracle string is untested.",
    "The pre-registered prediction (corrected 0.77-0.84 at 28 bits) was missed: the set-up fell to 0.565 of the scalar build (predicted 0.25-0.5) and target PDP rose from 1062.3 to 1427.6 ms (+34%), consistent with a slower probe on the packed table, as the section 7 model-risk bullet allowed. The prediction decides nothing.",
    "24 and 32 bits are undecidable under the frozen rule (section 6.1 fails); their descriptive corrected ratios (0.9075, 0.8975) are reported but are not results.",
]
promotion_blockers = [
    "Section 10 outputs missing at bf899979a6: no README.md with the position table and the section 6.2 position-control figures, no figure and no PDF (research-visuals) under research/ic_dg_blocked_20261029/. They must be committed before the leaderboard or benchmarks/ change. (An untracked README.md and figures/ appeared in the working tree after the snapshot push; they are uncommitted and were not reviewed.)",
    "The launch/ folders (requests, launcher receipts, logs) are untracked at bf899979a6; commit them in the Coordinator's snapshot (they are byte-equal to the S3 launcher receipts this validation used) or cite the S3 receipts.",
    "The evidence review must record which leaderboard section receives the 28-bit row (pair_table_component, or a new oracle-named component section if that section holds set-up figures only), per PROTOCOL section 10.",
]

SB = R["snapshot_binding"]
_tr, _mf = set(SB["tracked"]), SB["manifest_files"]
RAW_BINDING = {"git_tracked_at_bf899979a6": sum(x["path"] in _tr for x in raw),
               "artifacts_manifest_at_bf899979a6": sum(x["path"] in _mf for x in raw),
               "unbound": [x["path"] for x in raw if x["path"] not in _tr and x["path"] not in _mf],
               "manifests_checked": len(SB["manifests"]), "manifest_files_checked": len(_mf),
               "manifest_mismatches": SB["manifest_mismatches"],
               "tracked_eks_results_blob_mismatches": SB["tracked_eks_results_blob_mismatches"],
               "launch_folders_tracked": any("/launch/" in x for x in _tr)}
assert not RAW_BINDING["unbound"] and not SB["manifest_mismatches"] and not SB["tracked_eks_results_blob_mismatches"]
assert not RAW_BINDING["launch_folders_tracked"]
assert all(_mf[x["path"]]["sha256"] == x["sha256"] for x in raw if x["path"] in _mf)

# ------------------------------------------------------------ validator report
gates = {P: {"wall": S[P]["wall_gate"], "counted": S[P]["counted_gate"]} for P in SIZES}
PROV_BOOLS = ("evidence_equals_s3", "sha256sums_verify", "original_request_equals_request", "receipt_request_spec_equals_frozen",
              "receipt_spec_equals_session_spec", "result_request_sha256_matches", "result_job_name_equals_job_exit",
              "result_job_uid_equals_status", "status_single_pod", "status_pod_uid_equals_job_exit", "status_node_equals_job_exit",
              "status_container_exit_equals_job_exit", "status_restart_count_zero", "launcher_state_consistent_with_exit",
              "working_tree_launch_copy_equals_s3")
prov_fail = [(k, f) for k, v in R["provenance"].items() for f in PROV_BOOLS if v.get(f) is not True]
assert not prov_fail, prov_fail
assert {k: (v["result_state"], v["result_exit_code"], v["job_exit_code"]) for k, v in R["provenance"].items()} == {
    "P24-wall": ("failed", 23, 42), "P24-counted": ("failed", 23, 42), "P28-wall": ("succeeded", 0, 0),
    "P28-counted": ("failed", 23, 42), "P32-wall": ("failed", 23, 42), "P32-counted": ("failed", 23, 42)}
rep = {
    "schema": "validator-report/v1",
    "id": VID,
    "id_note": "random 6-hex suffix chosen by the validator; the Coordinator's archive task confirms or reallocates it with tools/allocate_id.py --check",
    "status": "pass",
    "status_scope": "pass applies to the 28-bit rows icv1-fp28-t9805-a382b8a8 and icv1-fp28-tm3463-6d2f5699 only; P24 and P32 fail the frozen instrument and back no row (their undecidable readings are confirmed)",
    "verdict": "passed",
    "independent": True,
    "validator": "Validator subagent (claude-opus-5-5), independent of the protocol author, director, executor and analyst; own code under code/",
    "experiment": "XP-20261029-dg-blocked",
    "claim": "mitm-prime-blocked:m=2 (dgb, PackedPairTable) has lower wall time per charged GAE than mitm:negation_folded=1 (dg) at N_inc with targets=stride-renew-deferred-guard, per size b in {24, 28, 32} (PROTOCOL section 6.2)",
    "claim_verdict": {"P24": "undecidable (confirmed)", "P28": "improvement (confirmed)", "P32": "undecidable (confirmed)"},
    "size_verdict": size_verdict,
    "size_verdict_reasons": size_reason,
    "snapshot": {"branch": "claude/ic-dg-blocked-20261029", "commit": "bf899979a6a6304fbbfbbc27204e50acfc52f6b8",
                 "remote_tip_equals_snapshot": True,
                 "head_at_end_of_validation": "bf899979a6 (unchanged; remote tip unchanged)",
                 "working_tree_at_start": "clean",
                 "working_tree_at_end": ("not clean: during the validation another session (not this validator) modified .github/workflows/ic-leaderboard.yml, "
                                         ".github/workflows/validation-gate.yml and scripts/build_ic_leaderboard.py and added untracked "
                                         "research/ic_dg_blocked_20261029/P{24,28,32}/{wall,counted}/launch/, README.md and figures/ (created 13:49:07Z-13:51:39Z, "
                                         "after the 13:47:30Z snapshot push). None of these working-tree-only files is used as evidence here: launcher receipts "
                                         "are read from their durable S3 copies, and the tracked evidence files still verify against the pod SHA256SUMS and S3 originals."),
                 "freeze_commit": "cd04cfd2d1df985e34f59b6a19bb8354e0f9a5ae", "freeze_pushed": "2026-10-29T12:48:34Z",
                 "ruling_commit": "df274b2aa3 (RULING-P2.md, pushed 12:56:56Z; unchanged at HEAD)",
                 "cards_commit": "df75474da1 (pushed 13:06:41Z)", "first_execution": "2026-10-29T13:11:26Z (P24 counted)",
                 "last_execution": "2026-10-29T13:35:56Z (P32 wall)",
                 "backlog_unchanged_since_freeze": True},
    "method": {
        "code": {f: sha(f"{V}/code/{f}") for f in sorted(os.listdir(f"{V}/code")) if not f.startswith("__")},
        "description": ("Own Python code (no reuse of research/ic_dg_blocked_20261029/cards/tools). Wall statistic re-implemented from its "
                        "definition (PROTOCOL section 5): pairs keyed by (workload, round) over measured verified records, admitted when both "
                        "runs are L2 or L3; ns per GAE = sum(solve_wall)/sum(total_gae); ratio with a pair-influence standard error and a log-scale "
                        "Student t interval (t quantile from my own regularised incomplete beta, self-tested against the NIST table to "
                        f"{R['t975_self_test_max_abs_err']:.1e}); corrected = ratio / aa_ratio with independent log-scale combination and "
                        "Welch-Satterthwaite df. Priced S from the band.rs definition (G + w * U)/sqrt(r), U = sum of the record's unpriced counters "
                        "(no record carries pinned calibration). compare.rs, stats.rs and band.rs at be2969ce were read for definitions only."),
        "ecbench_binary_used_for": "replay only (ecbench verify --replay-all); no compare or band output of mine comes from the binary",
    },
    "raw_data": raw,
    "raw_data_note": "records.jsonl, plan.json, audit.json and the counted l2-gate.json are artifact-store files (gitignored, pinned by .artifacts.json committed at bf899979a6); hydrate with tools/artifacts/artifacts.py pull before running scripts/validation_gate.py evidence",
    "raw_data_binding": RAW_BINDING,
    "headline": head,
    "headline_count": len(head),
    "headline_mismatches": bad_head,
    "main_session_claims_checked": ms,
    "wall_decision": {P: S[P]["recompute"]["decision_dg__dgb"] for P in SIZES},
    "wall_decision_vs_committed_max_rel_diff": {P: S[P]["recompute"]["decision_vs_committed"]["max_rel_diff"] for P in SIZES},
    "wall_descriptive_pairs": {P: S[P]["recompute"]["descriptive"] for P in SIZES},
    "wall_identity_condition_9": {P: S[P]["recompute"]["identity_cond9_recomputed"] for P in SIZES},
    "wall_per_curve_descriptive": {P: S[P]["recompute"]["per_curve"] for P in SIZES},
    "wall_target_clustered_descriptive": {P: S[P]["recompute"]["clustered"] for P in SIZES},
    "position_control_edge": {P: S[P]["recompute"]["position_control"] for P in SIZES},
    "arm_positions_alternate_order": {P: S[P]["positions"] for P in SIZES},
    "instrument_6_1": gates,
    "packed_layout_A1": {
        "source": "be2969ce src/cryptanalysis/ic_boundary.rs 5754-5777: PairTable::build_negation_folded_prime_blocked takes PackedPairTable iff g.p <= u64::from(u32::MAX) && fb.points.len() <= 1 << 16, else build_negation_folded_prime_blocked_maps (same kind string, same work counters: the records cannot tell the branches apart, so A1 is decided from inputs). The record field factor_base.signed_points is fb.points.len() (methods.rs 4002). Routing: methods.rs 4195 'mitm-prime-blocked' -> PrimeBatchMitmOracle::blocked() -> plugins.rs 2267; 'mitm' + negation_folded -> plugins.rs 2090-2091 PairTable::build_negation_folded.",
        "per_size": {P: {"wall_dgb_and_dgb-edge": S[P]["wall_gate"]["A1_packed_layout"], "counted_dgb": S[P]["counted_gate"]["A1_counted_dgb"]} for P in SIZES},
        "holds_every_size": all(S[P]["wall_gate"]["A1_packed_layout"]["holds"] and S[P]["counted_gate"]["A1_counted_dgb"]["holds"] for P in SIZES),
        "not_used": "no wall field, RSS or set-up time was used to infer the layout (RULING-P2). Descriptively, mean max RSS at 32 bits is 80.3 MB (dg) vs 61.9 MB (dgb), consistent with the packed table's smaller footprint; at 24 and 28 bits RSS does not differ.",
    },
    "not_relabelling_or_accounting": {
        "record_by_record": {P: S[P]["wall_gate"]["8_identical_work"]["record_by_record"] for P in SIZES},
        "counted_session_equals_wall_session": {P: S[P]["counted_gate"]["counted_equals_wall_dg_dgb"] for P in SIZES},
        "fields_compared": "outcome.recovered, status, matches_target, cost.total_gae, every unpriced counter (band.rs lookup rule), each phase's adds/doubles/scalar_mults/gae and full native map, algorithm_seed, counters; additionally work_native, bitop/v1 prims and bitops, cost.s, automorphisms_used, fb_sha256 and points_sha256: no difference anywhere",
        "pairs": "each dgb and dgb-edge record (336 per size, 48 warm-ups included) against the dg record of the same curve, target and round",
        "conclusion": "identical charged work, identical answers and seeds; the only measured difference is wall time (and RSS at 32 bits). dgb runs different code (method_id ECM1h0214efbf2ff5 vs ECM1h06aa2f4e545c; oracle string mitm-prime-blocked:m=2) and its oracle_setup wall falls at every size. The ns/GAE ratio is a real executed-time change, not a relabelling or accounting change.",
    },
    "phase_sums_descriptive": {P: S[P]["phase_sums_wall"] for P in SIZES},
    "phase_sums_vs_committed": {P: S[P]["phase_sums_vs_committed"] for P in SIZES},
    "matched_rho_distance_remaining": {P: {"per_curve": S[P]["band_counted"]["per_curve"], "pooled_mean_S": S[P]["band_counted"]["pooled_mean_S"],
                                           "vs_committed": {"max_rel_diff": S[P]["band_vs_committed"]["max_rel_diff"]},
                                           "wall_session_IC_mean_S": S[P]["band_wall_ic"]} for P in SIZES},
    "curves_and_automorphisms": curves_all,
    "strongest_available_basis": basis,
    "automorphisms_used": {P: S[P]["automorphisms_used"] for P in SIZES},
    "provenance": R["provenance"],
    "record_integrity": SEALS,
    "replay_certificate": {"path": "replay-certificate.json", "sha256": sha(f"{V}/replay-certificate.json"), "status": cert["status"],
                           "replays": cert["totals"]["replays"], "identical": cert["totals"]["replays_identical"]},
    "predictions_vs_measured": {
        "P24": {"predicted": "0.81-0.87", "measured_descriptive": S["P24"]["recompute"]["decision_dg__dgb"]["corrected_ratio"], "setup_ratio_dgb_over_dg": S["P24"]["phase_sums_wall"]["dgb"]["phase_wall_ms"]["oracle_setup"] / S["P24"]["phase_sums_wall"]["dg"]["phase_wall_ms"]["oracle_setup"]},
        "P28": {"predicted": "0.77-0.84", "measured": S["P28"]["recompute"]["decision_dg__dgb"]["corrected_ratio"], "setup_ratio_dgb_over_dg": S["P28"]["phase_sums_wall"]["dgb"]["phase_wall_ms"]["oracle_setup"] / S["P28"]["phase_sums_wall"]["dg"]["phase_wall_ms"]["oracle_setup"]},
        "P32": {"predicted": "0.78-0.85", "measured_descriptive": S["P32"]["recompute"]["decision_dg__dgb"]["corrected_ratio"], "setup_ratio_dgb_over_dg": S["P32"]["phase_sums_wall"]["dgb"]["phase_wall_ms"]["oracle_setup"] / S["P32"]["phase_sums_wall"]["dg"]["phase_wall_ms"]["oracle_setup"]},
        "reading": "every size lies above its predicted range; the set-up model's 0.25-0.5 held only at 32 bits (0.451), and target PDP rose at every size (24: 228.4 -> 239.1 ms, 28: 1062.3 -> 1427.6 ms, 32: 10998.0 -> 13353.4 ms), consistent with a slower probe on the packed table: the 'probe cost taken as equal' assumption does not hold descriptively",
    },
    "null_object_controls": "Same-shape null objects are the two wall A/A pairs (dg-aa/dg, dgb/dgb-edge: same method, different arm): 1.000149 [0.99911, 1.00119] and 0.999743 [0.99831, 1.00118] at 28 bits, both covering 1, and the counted A/A (exactly 1 at every weight). The position control (edge arms) reproduces the oracle effect at other block positions (0.930331 [0.92703, 0.93365]), so the signal is the oracle swap, not position. A decay-with-parameter test does not apply to a constant-factor engineering change.",
    "caveats": caveats,
    "limitations": [
        "No review_plan was supplied, so this report gives per-size verdicts on the whole validation task rather than per-joint holds/breaks.",
        "Wall time cannot be replayed; the replay host (Intel, 4 vCPU) differs from the measuring pods (AMD EPYC 9R45).",
        "The 288 pairs per arm pair are 48 targets x 6 rounds; the decision interval treats pairs as independent (the target-clustered intervals are descriptive).",
        "The packed table's get() returns None for key 0; identical records rule out any effect in these runs, but the edge case was not examined further.",
        "PROTOCOL section 4 says each request embeds its spec 'byte for byte'; the launcher receipt request.json carries the spec as a JSON object equal to the frozen spec, and input-spec.json is JSON-equal (not byte-equal) to it. Condition 4 requires JSON equality, which holds.",
        "The RULING-P2 coordinator_decision ledger record and the P1-P5 pre-launch check outputs were not re-audited beyond reading RULING-P2.md.",
        "The evidence directory's launch/ folders, README.md and figures/ are untracked at bf899979a6 (working-tree only, written after the snapshot push by another session). They were not validated; the launch receipts are admitted from their S3 originals only. The Coordinator's snapshot must commit them before they can be cited.",
        "Working-tree modifications to .github/workflows/{ic-leaderboard,validation-gate}.yml and scripts/build_ic_leaderboard.py appeared during validation; they were not read. The gate check uses scripts/validation_gate.py from bf899979a6 (git show), which is unmodified.",
    ],
    "promotion_blockers": promotion_blockers,
    "promotion_note": "Evidence admissibility only. Promotion and placement are the Coordinator's decision; this report authorizes nothing and supports no ECDLP or counted-cost claim. Per AGENTS.md 'Curve identity and measured bounds', wall time is never a measured bound.",
    "review_attestation": {
        "joints_owned": ["PROTOCOL.md section 10 independent validation of XP-20261029-dg-blocked: task items 1-6 (wall recompute, section 6.1 + A1 for both sessions, relabelling/accounting and packed-regime check, independent replay, matched rho band, phase sums)"],
        "sources_read": [
            "/home/user/crypto-autoresearcher/docs/agent-runtime-core.md", "/home/user/crypto-autoresearcher/agents/validator.md",
            "/home/user/crypto-autoresearcher/AGENTS.md sections 'Review architecture' and 'Curve identity and measured bounds'",
            "backlog/XP-20261029-dg-blocked/{PROTOCOL.md, APPROVAL.md (status, pre-launch checks, reasons), spec-P{24,28,32}{,-rho}.json} at cd04cfd2d1",
            "backlog/XP-20261029-dg-wall/PROTOCOL.md section 6.1; backlog/XP-20261029-srenew-deferred-guard-v3/PROTOCOL.md section 6.1 (git show HEAD)",
            "research/ic_dg_blocked_20261029/cards/RULING-P2.md; VERDICT.md (producer claim); cards/tools listed only, not opened",
            "research/ic_dg_blocked_20261029/P{24,28,32}/{wall,counted}/eks-results (all files), analysis (compare-*.json, band.md, phase_sums.json, binary-path.txt, s3-prefixes.txt, pod-uid.txt, receipt.json head)",
            "S3 pod originals s3://isolab-results-104186811093-us-east-2/runs/ic-dgb-{,rho-}p{24,28,32}-20261029/<pod>/ (read-only copy)",
            "S3 launcher receipts s3://isolab-results-104186811093-us-east-2/runs/ic-dgb-{,rho-}p{24,28,32}-20261029/launcher/<launcher-id>/{request,original-request,result,status,submission}.json (read-only copy to /dev/shm/dgb-val/s3-launcher); the untracked working-tree launch/ folders were compared to them by sha256 only",
            "be2969ce sources: ic_boundary.rs 5482-5850, plugins.rs 2045-2290, methods.rs 3969-4010 and 4150-4200, compare.rs (wall_figures, choose_aa, wall_data, wall_compare, match_pairs, run_cost), stats.rs (ci95_log, influence_se, linearised_ratio, divide_independent, t_quantile), band.rs 1-152, record.rs (counts, seal, check_line_seal)",
            "precedent format: /dev/shm/dgw-validation (VALIDATION.draft.json, report and certificate key structure, replay.sh), /dev/shm/wt-wall/research/ic_dg_wall_20261029/VALIDATION.json",
            "docs/VALIDATION_GATE.md and scripts/validation_gate.py (git show HEAD)",
        ],
        "read_sibling_reports": False,
        "sealed_material": "PR #2844 (research/ic_table_floor_20261029) not opened",
        "verdict": "passed (P28 pass_with_caveats; P24 fail; P32 fail)",
    },
}
rep["validation_report"] = {
    "id": VID,
    "task_id": "not supplied by the caller (independent validation of XP-20261029-dg-blocked requested by the main session)",
    "run_ids": [f"{JOBS[(P, s)]}/{jload(f'{S3}/{P}-{s}/job-exit.json')['pod_uid']} ({SEALS[f'{P}-{s}']['session_id']})" for P in SIZES for s in ("wall", "counted")],
    "artifact_checks": [
        "snapshot bf899979a6 equals origin tip (start and end of validation); backlog unchanged since freeze cd04cfd2d1; working tree clean at start, later dirtied by another session (untracked launch/, README.md, figures/; modified workflows and build_ic_leaderboard.py), none of it used as evidence",
        "all SHA256SUMS of six pods verify; every evidence file byte-equal to the S3 pod original; one pod per job; no reruns",
        f"every raw_data file bound to bf899979a6: {RAW_BINDING['git_tracked_at_bf899979a6']} git-tracked (working-tree blob = committed blob), {RAW_BINDING['artifacts_manifest_at_bf899979a6']} artifact-store files whose sha256 and size equal the {RAW_BINDING['manifests_checked']} committed .artifacts.json manifests; 0 unbound; no working-tree-only file used",
        "launcher receipts read from S3 (launcher/<id>/), not from the untracked working-tree launch/ copies (byte-equal to S3 but not committed at bf899979a6): request.json job.spec JSON-equal to each frozen spec and to session/spec.json; original-request.json = request.json; result.request_sha256 = sha256(request.json); one pod per Job, restartCount 0, and status.json pod uid, node and container exit = job-exit.json; launcher state succeeded/0 for P28 wall (exit 0) and failed/23 with empty pod_uid for the five exit-42 sessions, as the v3 protocol anticipates; input-spec.json and session/spec.json JSON-equal to the frozen spec; frozen spec sha256 = freeze commit message",
        "build-info names be2969ce and 88070375; every record carries binary 88070375; image cf0dcc8b; c8a.2xlarge; 3600 s; harness claude-code-local",
        f"{SEALS['total_records']} record seals recomputed: {SEALS['total_seal_failures']} failures",
    ],
    "metric_recomputations": [
        f"{len(head)} headline numbers (decision blocks at 3 weights, 4 descriptive wall pairs, ops ratios, counted band per curve, phase sums): {len(bad_head)} disagree beyond 1e-9 (max rel diff ~1e-15)",
        "per-curve, target-clustered (linearised and stratified bootstrap) and edge-position intervals reported descriptively",
    ],
    "control_checks": [
        "wall A/A dg-aa/dg and dgb/dgb-edge cover 1 at every size; counted A/A exactly 1 at w=0, 0.1, 1 (dg/dg-aa, dgb-edge/dgb, rho-neg/rho-neg-aa)",
        "native A/A admission (control-comparison.json, dgb-edge vs dg-aa, aa_arms {dgb, dgb-edge}, includes arm A) present and admitted at 28 bits only",
        "arm positions: dgb and dg-aa each 144 blocks at position 1 and 144 at 2; 144 forward and 144 reversed measured blocks per size",
    ],
    "heuristic_validation_checks": ["not a heuristic-validation experiment; section 7 predictions recorded before data and compared descriptively (all missed, ratio higher than predicted)"],
    "cost_model_checks": ["unit ns per charged ecbench.gae (wall), S = GAE/sqrt(r) (counted) at w = 0, 0.1, 1; ns/GAE is an upper bound for both arms; memory reported (mean max RSS per arm); no total-cost extrapolation is claimed"],
    "proof_architecture_checks": [],
    "verdict": "passed",
    "limitations": rep["limitations"],
    "artifact_paths": ["validator-report.json", "replay-certificate.json", "VALIDATION.draft.json", "REPORT.md", "code/", "work/", "replay/", "frozen/", "SHA256SUMS"],
}
dump(rep, f"{V}/validator-report.json")

# ------------------------------------------------------------ VALIDATION.draft.json
valj = {
    "schema": "validation-gate/v1",
    "rows": rows,
    "replay_certificate": {"path": "validation/replay-certificate.json", "sha256": sha(f"{V}/replay-certificate.json")},
    "validator_report": {"path": "validation/validator-report.json", "sha256": sha(f"{V}/validator-report.json")},
    "rho_reference": {
        "kind": "negation",
        "protocol": "PROTOCOL.md section 2 counted session, arm rho-neg: rho.negation in job ic-dgb-rho-p28-20261029, on the same two curves, the same 24 public targets per curve, the same rounds and algorithm seeds as dg and dgb (the counted session's dg and dgb records equal the wall session's in answers, GAE, unpriced counts, per-phase counts and seeds, record by record)",
        "unit": "ecbench.gae / sqrt(r) (S), cold, whole pipeline; unpriced native operations priced at w GAE each (AGENTS.md section 5a): IC sqrt solves, lookups, row ops, Legendre symbols, target-guard probes; rho canonicalisations",
        "binary_or_process": f"ecbench sha256 {BIN} (isolab image cf0dcc8b, source be2969ce); rho and IC arms in one counted session; replayed by the same binary on another host",
        "matches_protocol": True,
        "strongest_available": True,
        "strongest_available_basis": "Both 28-bit curves are prime-order curves over prime fields (p and r prime, r = p + 1 - t, cofactor 1) with j not 0 or 1728 (j recomputed from a, b solved on G and the targets, equal to the ICV1 j), so Aut(E) = {+1, -1}: negation is the only automorphism fold, there is no subfield Frobenius fold over F_p, and the Frobenius discriminants (squarefree parts -69430827 and -598737787) give no small-norm endomorphism. Records show automorphisms_used 2.",
        "same_unit": True,
        "same_binary_or_process": True,
        "gae_band_row": "ic = dgb (candidate, N_inc 407) mean S over its 288 measured counted runs (identical to dg), rho = rho-neg mean S over the same 288 (target, round) pairs; per-curve values in gae_band_by_row",
        "gae_band": gae_band,
        "gae_band_by_row": gae_band_by_row,
        "order_holds_across_band": order_holds,
        "order_note": "rho-neg < dgb (= dg) at w = 0, 0.1 and 1 on both curves; distance remaining dgb/rho-neg 21.79, 20.66, 15.48 (pooled). The improvement this file admits is wall-clock (ns per charged GAE), not a counted ranking: counted S of dgb and dg are identical.",
    },
    "verdict": "passed",
    "size_verdict": size_verdict,
    "caveats": caveats,
    "promotion_blockers": promotion_blockers,
    "promotion_note": "Evidence admissibility only for the two 28-bit rows. 24 and 32 bits are undecidable and get no row. Promotion and placement are the Coordinator's decision.",
    "draft_note": "Draft written by the validator outside the repository. Place it as research/ic_dg_blocked_20261029/VALIDATION.json with the validator files under research/ic_dg_blocked_20261029/validation/ (replay-certificate.json, validator-report.json and the rest); the two pinned sha256 values hold only if those files are copied byte for byte.",
}
dump(valj, f"{V}/VALIDATION.draft.json")
print("headline", len(head), "mismatches", len(bad_head), "cert", cert["status"], "rows", rows, "order", order_holds)
print("main-session claims", [(m["claim"], round(m["recomputed"], 4), m["agrees_at_stated_precision"]) for m in ms])
