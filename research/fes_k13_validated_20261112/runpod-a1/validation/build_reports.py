#!/usr/bin/env python3
"""Assemble the validator's artifacts for research/fes_k13_validated_20261112/runpod-a1.

Inputs (all inside runpod-a1/): validation/recompute.json, validation/scalar-check.json,
validation/replay-receipt.json, validation/replay-host.json, validation/tarball-check.json,
validation/claimed.json, validation/findings.json, validation/claimed-side-ecbench-table.md,
claimed/band.json, claimed/comparisons/*.json, session/*, audit.stdout.

Outputs: validation/headline.json, validation/replay-certificate.json,
validation/validator-report.json, VALIDATION.json.

Usage (from anywhere):  python3 -I build_reports.py <runpod-a1 dir>
"""
import glob
import hashlib
import json
import os
import subprocess
import sys

EV = os.path.abspath(sys.argv[1])            # evidence directory runpod-a1/
REPO = os.path.abspath(os.path.join(EV, "..", "..", ".."))
STUDY = os.path.relpath(os.path.dirname(EV), REPO)
VAL = os.path.join(EV, "validation")


def p(*a):
    return os.path.join(EV, *a)


def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


def load(path):
    return json.load(open(path))


def git(*args):
    return subprocess.run(["git", "-C", REPO, *args], capture_output=True, text=True).stdout.strip()


rc = load(p("validation", "recompute.json"))
sc = load(p("validation", "scalar-check.json"))
rcpt = load(p("validation", "replay-receipt.json"))
rhost = load(p("validation", "replay-host.json"))
shost = load(p("session", "host.json"))
sess = load(p("session", "session.json"))
tb = load(p("validation", "tarball-check.json"))
claimed = load(p("validation", "claimed.json"))
findings = load(p("validation", "findings.json"))
cband = load(p("claimed", "band.json"))
ccomp_path = sorted(glob.glob(p("claimed", "comparisons", "*.json")))
assert len(ccomp_path) == 1
ccomp = load(ccomp_path[0])
pod_audit_txt = open(p("audit.stdout")).read()
pod_audit = json.loads(pod_audit_txt[: pod_audit_txt.rfind("}") + 1])

head = git("rev-parse", "HEAD")
porcelain = git("status", "--porcelain", "--untracked-files=all")
ev_rel = os.path.relpath(EV, REPO)
dirty_outside_validation = [l for l in porcelain.splitlines()
                            if not (l[3:].startswith(ev_rel + "/validation/") or l[3:] == ev_rel + "/VALIDATION.json")]
snapshot = {
    "validated_at_head": head,
    "evidence_commit": git("log", "-1", "--format=%H", "--", ev_rel + "/session", ev_rel + "/audit.stdout"),
    "tree_clean_outside_validator_outputs": not dirty_outside_validation,
    "tree_note": "git status --porcelain shows only the validator's own untracked outputs (runpod-a1/validation/, runpod-a1/VALIDATION.json)" if not dirty_outside_validation else "OTHER CHANGES: %s" % dirty_outside_validation,
    "frozen_inputs_commit": "03ac49860a (2026-11-12 16:40 Z): protocol.json, spec.json, smoke/build.json unchanged since",
}

A = rc["arms"]
P = rc["paired_fes_over_inherited"]

# ------------------------------------------------------------------ headline
recomputed_printed = {
    "paired_ratio_fes_over_inherited": P["ratio_of_totals"],
    "paired_ratio_ci95_low": P["ci95_linearised_t_log"][0],
    "paired_ratio_ci95_high": P["ci95_linearised_t_log"][1],
}
for arm in ["rho-sfstrong-v2", "rho-negation", "ic-pairtable-mitm2", "ic-descent-inherited-f4", "ic-descent-fes-f2"]:
    recomputed_printed[f"mean_S/{arm}"] = A[arm]["mean_s"]
for arm in ["ic-descent-fes-f2", "ic-descent-inherited-f4", "rho-sfstrong-v2"]:
    for w in ["0", "0.1", "1"]:
        recomputed_printed[f"band_S/{arm}/w={w}"] = A[arm]["band_mean_s"][w]

headline = []
for e in claimed["printed"]:
    s = e["printed"]
    c = float(s)
    d = len(s.split(".")[1]) if "." in s else 0
    half = 0.5 * 10 ** (-d)
    r = recomputed_printed[e["name"]]
    headline.append({"name": e["name"], "claimed": c, "recomputed": r, "rel_tol": half / abs(c), "printed": s,
                     "source": "dispatch (validation/claimed.json)",
                     "agrees": abs(c - r) <= half, "recomputed_rounds_to_printed": f"{r:.{d}f}" == s})
for e in claimed["bench_record_spreads"]["printed"]:
    arm = e["name"].split("/", 1)[1]
    for i, s in enumerate(e["printed"]):
        c = float(s)
        d = len(s.split(".")[1])
        half = 0.5 * 10 ** (-d)
        r = A[arm]["mean_s_ci95_log_cluster_t"][i]
        headline.append({"name": f"bench_record_spread/{arm}/{'low' if i == 0 else 'high'}", "claimed": c, "recomputed": r,
                         "rel_tol": half / abs(c), "printed": s, "source": "benchmarks/records/20261115T0009Z-fes-k13-runpod.json",
                         "agrees": abs(c - r) <= half, "recomputed_rounds_to_printed": f"{r:.{d}f}" == s})

# full-precision claimed values from the producer's ecbench band / compare files
TOL = 1e-9
wk = ["0", "0.1", "1"]
for row in cband["rows"]:
    arm = row["arm"]
    for i, w in enumerate(wk):
        headline.append({"name": f"claimed/band.json/{arm}/mean_s/w={w}", "claimed": row["mean_s"][i],
                         "recomputed": A[arm]["band_mean_s"][w], "rel_tol": TOL, "source": "claimed/band.json"})
    headline.append({"name": f"claimed/band.json/{arm}/charged_sum", "claimed": row["charged_sum"],
                     "recomputed": A[arm]["charged_sum_gae"], "rel_tol": TOL, "source": "claimed/band.json"})
    headline.append({"name": f"claimed/band.json/{arm}/native_sum", "claimed": row["native_sum"],
                     "recomputed": A[arm]["native_sum"], "rel_tol": TOL, "source": "claimed/band.json"})
ops = ccomp["ops"]
cc = "claimed/comparisons/" + os.path.basename(ccomp_path[0])
headline += [
    {"name": "comparison/ops.ratio_b_over_a", "claimed": ops["ratio_b_over_a"], "recomputed": P["ratio_of_totals"], "rel_tol": TOL, "source": cc},
    {"name": "comparison/ops.ci95.low", "claimed": ops["ci95"][0], "recomputed": P["ci95_linearised_t_log"][0], "rel_tol": TOL, "source": cc},
    {"name": "comparison/ops.ci95.high", "claimed": ops["ci95"][1], "recomputed": P["ci95_linearised_t_log"][1], "rel_tol": TOL, "source": cc},
    {"name": "comparison/ops.dof", "claimed": ops["dof"], "recomputed": float(P["df"]), "rel_tol": TOL, "source": cc},
    {"name": "comparison/a.mean_s (inherited-f4)", "claimed": ccomp["a"]["mean_s"], "recomputed": A["ic-descent-inherited-f4"]["mean_s"], "rel_tol": TOL, "source": cc},
    {"name": "comparison/b.mean_s (fes-f2)", "claimed": ccomp["b"]["mean_s"], "recomputed": A["ic-descent-fes-f2"]["mean_s"], "rel_tol": TOL, "source": cc},
]
for i, w in enumerate(wk):
    headline.append({"name": f"comparison/ops.band.ratio_b_over_a/w={w}", "claimed": ops["band"]["ratio_b_over_a"][i],
                     "recomputed": P["band_ratio_b_over_a"][w], "rel_tol": TOL, "source": cc})
for wl in ccomp["workloads"]:
    mine = P["per_workload"][wl["workload_id"]]
    headline.append({"name": f"comparison/workload/{wl['workload_id']}/ratio_b_over_a", "claimed": wl["ratio_b_over_a"],
                     "recomputed": mine["ratio_b_over_a"], "rel_tol": TOL, "source": cc})
# claimed-side ecbench table (S / reference column), run by the validator with the session binary
table = {}
for line in open(p("validation", "claimed-side-ecbench-table.md")):
    cells = [c.strip() for c in line.strip().strip("|").split("|")]
    if len(cells) > 12 and cells[0].startswith("ECBS"):
        table[cells[1]] = cells
for arm in ["rho-negation", "ic-pairtable-mitm2", "ic-descent-inherited-f4", "ic-descent-fes-f2"]:
    s = table[arm][11]
    d = len(s.split(".")[1])
    half = 0.5 * 10 ** (-d)
    r = A[arm]["mean_s"] / A["rho-sfstrong-v2"]["mean_s"]
    headline.append({"name": f"S/S(rho-sfstrong-v2)/{arm}", "claimed": float(s), "recomputed": r, "rel_tol": half / float(s), "printed": s,
                     "source": "validation/claimed-side-ecbench-table.md (S / reference column)",
                     "agrees": abs(float(s) - r) <= half, "recomputed_rounds_to_printed": f"{r:.{d}f}" == s})
for h in headline:
    scale = max(abs(h["claimed"]), abs(h["recomputed"]), 1e-300)
    h["within_tol"] = abs(h["claimed"] - h["recomputed"]) <= h["rel_tol"] * scale
all_ok = all(h["within_tol"] for h in headline)

# verdict checks (non-numeric)
mine_ranks = {(r["a"], r["b"]): r["verdict"] for r in rc["band_ranks"]}
verdicts = []
for r in cband["ranks"]:
    key = tuple(sorted([r["a"], r["b"]]))
    verdicts.append({"pair": list(key), "claimed": r["verdict"], "recomputed": mine_ranks[key], "agrees": r["verdict"] == mine_ranks[key]})
verdicts.append({"pair": ["ic-descent-inherited-f4", "ic-descent-fes-f2"], "claimed": ops["band"]["order"],
                 "recomputed": rc["H1"]["band_order_verdict"], "agrees": ops["band"]["order"] == rc["H1"]["band_order_verdict"],
                 "source": cc})
verdicts.append({"pair": ["H1 interpretation"], "claimed": claimed["interpretation"],
                 "recomputed": rc["H1"]["verdict"], "agrees": (not rc["H1"]["supported"]) and rc["H1"]["band_order_verdict"] == "unresolved inside the band"})

json.dump({"schema": "validator-headline/v1", "all_within_tol": all_ok, "headline": headline, "verdicts": verdicts},
          open(p("validation", "headline.json"), "w"), indent=1)
open(p("validation", "headline.json"), "a").write("\n")

# ------------------------------------------------------------------ replay certificate
rec_sha = sha(p("session", "records.jsonl"))
receipt_sha = sha(p("validation", "replay-receipt.json"))
cert = {
    "schema": "replay-certificate/v1",
    "study": STUDY,
    "evidence_dir": ev_rel,
    "snapshot": snapshot,
    "status": "verified",
    "independent": True,
    "independence": "Replayed by the validator (a separate agent from the measuring main session) on this container, a different host class from the pod: "
                    "Intel Xeon @ 2.10 GHz KVM guest, 4 logical CPUs, kernel 6.18.44 (env class %s, hardware class %s) against the pod's AMD EPYC 9575F "
                    "bare-metal container, 128 logical CPUs, kernel 6.8.0-147 (env class %s, hardware class %s). Scalars re-checked with validator-written "
                    "Python (validation/verify_scalars.py) that imports no repository code and derives the field polynomial from the data."
                    % (rcpt["auditor_env_class_id"], rcpt["auditor_hardware_class_id"], sess["env_class_id"], pod_audit["auditor_hardware_class_id"]),
    "scalars_verified": sc["ok"] and sc["records_dG_eq_Q"] == sc["records"],
    "binary": {
        "path_used": "scratchpad isolab-k13/ecbench-e9d5dbdccd (not in the repository; the S3 input runpod/fes-k13-validated-20261112-a1/inputs/ecbench the pod fetched has the same sha256 per pod.log)",
        "sha256": rhost["build"]["binary_sha256"],
        "matches_session": rhost["build"]["binary_sha256"] == sess["binary_sha256"],
        "matches_smoke_build_json": rhost["build"]["binary_sha256"] == load(os.path.join(REPO, STUDY, "smoke", "build.json"))["ecbench_sha256"],
        "source_revision_asserted": "e9d5dbdccdeb55b6d8527ebae1113135b42b26bf (smoke/build.json; session git_commit is null) - see validator-report finding F5",
    },
    "hosts": {
        "session": {"env_class_id": sess["env_class_id"], "hardware_class_id": pod_audit["auditor_hardware_class_id"],
                    "cpu": shost["stable"]["cpu_model"], "logical_cpus": shost["stable"]["logical_cpus"], "kernel": shost["stable"]["kernel_release"],
                    "virtualization": shost["stable"].get("virtualization"), "where": "Runpod CPU pod d4db6690f94f (cpu5m, 4 vCPU allotted), image runpod/base:1.0.2-ubuntu2404 per dispatch"},
        "replay": {"env_class_id": rcpt["auditor_env_class_id"], "hardware_class_id": rcpt["auditor_hardware_class_id"],
                   "cpu": rhost["stable"]["cpu_model"], "logical_cpus": rhost["stable"]["logical_cpus"], "kernel": rhost["stable"]["kernel_release"],
                   "virtualization": rhost["stable"].get("virtualization"), "capsule": "validation/replay-host.json"},
        "env_class_differs": rcpt["auditor_env_class_id"] != sess["env_class_id"],
        "hardware_class_differs": rcpt["auditor_hardware_class_id"] != pod_audit["auditor_hardware_class_id"],
    },
    "command": "ecbench verify --dir research/fes_k13_validated_20261112/runpod-a1/session --replay-all --exit-code --out research/fes_k13_validated_20261112/runpod-a1/validation/replay-receipt.json",
    "command_exit": 0,
    "runs": [{
        "session": "session",
        "session_id": sess["session_id"],
        "session_id_matches": rcpt["session_id"] == sess["session_id"],
        "records": rcpt["records"],
        "records_sha256": rec_sha,
        "records_sha256_matches_session_json": rec_sha == sess["records_sha256"],
        "warmups_not_replayed": rcpt["records"] - rcpt["replay_candidates"],
        "replays": len(rcpt["replays"]),
        "reproduced": sum(1 for x in rcpt["replays"] if x["reproduced"]),
        "replay_details": sorted({x["detail"] for x in rcpt["replays"]}),
        "audit_ok": rcpt["ok"],
        "audit_problems": rcpt["problems"],
        "audit_checks_all_true": all(rcpt["checks"].values()),
        "ecbench_verified_records": rcpt["verified_records"],
        "auditor_env_class_id": rcpt["auditor_env_class_id"],
        "auditor_hardware_class_id": rcpt["auditor_hardware_class_id"],
        "session_env_class_id": sess["env_class_id"],
        "receipt_file_hashes_match_files": all(rcpt["files"][f] == sha(p("session", f)) for f in rcpt["files"]),
        "independent_scalar_check": {
            "tool": "validation/verify_scalars.py",
            "output": "validation/scalar-check.json",
            "output_sha256": sha(p("validation", "scalar-check.json")),
            "scalars_verified": sc["records_dG_eq_Q"],
            "scalars_failed": sc["records"] - sc["records_dG_eq_Q"],
            "modulus_hex": sc["field_polys_consistent_with_data"][0] if len(sc["field_polys_consistent_with_data"]) == 1 else None,
            "modulus_unique_among_irreducibles": len(sc["field_polys_consistent_with_data"]) == 1,
            "irreducibles_tested": sc["irreducible_degree13_polys_tested"],
            "group_order_recounted": sc["recounted_group_order"],
            "targets_on_curve": sc["curve_checks"]["all_targets_on_curve"],
            "targets_in_subgroup": sc["curve_checks"]["all_targets_in_r_subgroup"],
            "negative_control": "a copy of the records with one recovered scalar changed by +1 fails ([d]G != Q at seq 100), checked 2026-11-15",
        },
        "scalar_ok": sc["ok"],
        "record_binary_sha256": rc["integrity"]["one_binary_sha256"],
        "session_binary_sha256": sess["binary_sha256"],
        "session_git_commit": sess["git_commit"],
        "session_git_dirty": sess["git_dirty"],
        "receipt": "validation/replay-receipt.json",
        "receipt_sha256": receipt_sha,
        "log": "validation/replay-verify.stdout",
    }],
    "in_pod_audit_same_host": {
        "file": "audit.stdout", "sha256": sha(p("audit.stdout")), "ok": pod_audit["ok"], "replays": len(pod_audit["replays"]),
        "identical": sum(1 for x in pod_audit["replays"] if x["reproduced"] and x["detail"] == "identical"),
        "note": "same host as the measurement: not the independent replay (protocol does_not_count)",
    },
    "tarball_binding": {
        "s3_uri": tb["s3_uri"], "sha256": tb["downloaded_sha256"], "sha256_matches_dispatch": tb["sha256_matches"],
        "session_dir_byte_identical": tb["session_dir_identical"], "not_copied": tb["tar_files_not_copied"], "check": "validation/tarball-check.json",
    },
    "route_deviation": "Protocol instrument: Iso Lab EKS L2 pod; actual: Runpod CPU pod (owner decision, AWS budget exhausted). No L2 gate, image digest or launcher receipt exists. Counts are host-independent and replay identically on a different host class; wall figures are not admissible (validator-report F3, F4).",
}
json.dump(cert, open(p("validation", "replay-certificate.json"), "w"), indent=1)
open(p("validation", "replay-certificate.json"), "a").write("\n")

# ------------------------------------------------------------------ validator report
def entry(path, role=None):
    d = {"path": path, "sha256": sha(p(path))}
    if role:
        d["role"] = role
    return d

H1, H2 = rc["H1"], rc["H2"]
req = [
    {"id": "R1", "requirement": "Session copy byte-identical to the pod's results tarball (S3, sha256 cd2f7736...e3e69)", "result": "pass" if tb["sha256_matches"] and tb["session_dir_identical"] else "fail", "evidence": "validation/tarball-check.json"},
    {"id": "R2", "requirement": "session/spec.json is the frozen spec (sha256 = protocol links.spec_sha256)", "result": "pass" if sha(p("session", "spec.json")) == load(os.path.join(REPO, STUDY, "protocol.json"))["links"]["spec_sha256"] else "fail", "evidence": "session/spec.json, ../protocol.json"},
    {"id": "R3", "requirement": "Binary sha256 = smoke/build.json ecbench_sha256 on every record, the session and the replay", "result": "pass" if cert["binary"]["matches_session"] and cert["binary"]["matches_smoke_build_json"] and rc["integrity"]["binary_matches_session_json"] else "fail", "evidence": "validation/recompute.json integrity, validation/replay-host.json, ../smoke/build.json"},
    {"id": "R4", "requirement": "Binary built from e9d5dbdccd (source binding)", "result": "partial (asserted and corroborated, not rebuilt; F5 open)", "evidence": "validator-report findings F5"},
    {"id": "R5", "requirement": "Record set complete and as planned (336 = 8 x 7 x 6, every cell once, seeds shared per (workload, round), alternate_measured order, frozen seeds from the protocol tags, records_sha256)", "result": "pass" if all(v for k, v in rc["integrity"].items() if isinstance(v, bool)) and rc["integrity"]["records"] == rc["integrity"]["expected"] else "fail", "evidence": "validation/recompute.json integrity"},
    {"id": "R6", "requirement": "Independent replay on a different host class: verify --replay-all, every replay identical", "result": "pass" if cert["runs"][0]["reproduced"] == cert["runs"][0]["replays"] == 288 and cert["hosts"]["env_class_differs"] and rcpt["ok"] else "fail", "evidence": "validation/replay-receipt.json, validation/replay-host.json"},
    {"id": "R7", "requirement": "Every record's answer re-checked [d]G = Q by validator-written code", "result": "pass" if sc["ok"] else "fail", "evidence": "validation/scalar-check.json"},
    {"id": "R8", "requirement": "Headline numbers recompute (claimed vs recomputed)", "result": "pass" if all_ok else "fail", "evidence": "validation/headline.json"},
    {"id": "R9", "requirement": "H2 / C-pricing-basis: priced_by pinned on every descent record, no solver_*_uncharged", "result": "pass" if H2["supported"] else "fail", "evidence": "validation/recompute.json H2"},
    {"id": "R10", "requirement": "C-fes-vs-inherited: paired 95 % interval entirely below 1", "result": "pass" if P["ci_entirely_below_1"] else "fail", "evidence": "validation/recompute.json paired_fes_over_inherited"},
    {"id": "R11", "requirement": "C-band: fes < inherited at w = 0, 0.1 and 1", "result": "fail (order flips at w = 0: unresolved inside the band)" if not H1["leg_order_at_every_band_weight"] else "pass", "evidence": "validation/recompute.json band_ranks, H1"},
    {"id": "R12", "requirement": "C-aa: wall admitted, pairs_below_level 0, aa_pairs >= 5, aa_ci95 present", "result": "fail (0 admitted A/A pairs; counts identical 48/48)" if not rc["aa_control"]["wall_threshold_met"] else "pass", "evidence": "validation/recompute.json aa_control"},
    {"id": "R13", "requirement": "C-audit: in-pod replay-all reproduced; L2 gate accepted; independent replay on another host class", "result": "partial (in-pod 288/288 identical and independent replay 288/288 identical; no L2 gate: Runpod route deviation)", "evidence": "audit.stdout, validation/replay-receipt.json"},
    {"id": "R14", "requirement": "Positive control: pair table verifies on every run and sits above both descent arms", "result": "pass" if rc["positive_control"]["verified"] == 48 and rc["positive_control"]["above_both_descent_arms_charged"] else "fail", "evidence": "validation/recompute.json positive_control"},
    {"id": "R15", "requirement": "Matched rho reference in the same session, binary and unit, priced in the band (every arm's unpriced counters listed with counts)", "result": "pass with caveat (rho-negation under-lists its table work, F8; the reference rho-sfstrong-v2 lists four counters with counts)", "evidence": "validation/recompute.json arms, distance_to_rho"},
    {"id": "R16", "requirement": "Attempt history complete (stopping rule)", "result": "partial (launch 1 not recorded, F6)", "evidence": "validation/tarball-check.json s3_prefix_listing"},
]
outputs = ["validation/recompute.json", "validation/scalar-check.json", "validation/headline.json", "validation/replay-certificate.json",
           "validation/replay-receipt.json", "validation/replay-host.json", "validation/replay-verify.stdout",
           "validation/replay-verify.stderr", "validation/replay-verify.time", "validation/tarball-check.json",
           "validation/claimed-side-ecbench-table.md"]
report = {
    "schema": "validator-report/v1",
    "study": STUDY,
    "evidence_dir": ev_rel,
    "protocol_id": "FES-K13-VALIDATED-20261112 (attempt a1, Runpod launch 2)",
    "snapshot": snapshot,
    "status": "pass" if all_ok and sc["ok"] and H2["supported"] and cert["runs"][0]["reproduced"] == 288 else "fail",
    "status_scope": "pass = every number the producer claimed for session ECBS1h695ef8e76c34 (dispatch figures, claimed/band.json, the saved paired comparison, the bench record spreads and the table's S/reference column) recomputes from the committed session/records.jsonl within tolerance; the independent replay reproduced 288/288 measured runs on a different host class; every record's [d]G = Q holds under validator-written arithmetic; H2 holds. It does NOT say H1 is supported (it is not: F2), does not admit any wall figure (F3, F4), and rests the binary-to-source binding on the producer's build record plus corroboration (F5, open).",
    "disposition": "validated",
    "disposition_scope": "Validated: the session's counted measurement (S of all six arms, the paired ratio 0.7081 [0.6683, 0.7503], the band at w = 0, 0.1, 1 and its ranks) and the reading 'H1 not supported: unresolved inside the band'. Not validated: any statement that fes-f2 improves on inherited-f4 under AGENTS.md 5a, any wall-time figure, and any claim needing the L2 gate or the A/A noise floor. Open: F5 (rebuild e9d5dbdccd and replay) before this session backs a published leaderboard row.",
    "independent": True,
    "independence": "Recomputed by the validator from the committed session files with validator-written Python (validation/recompute.py, verify_scalars.py), importing no repository code. The band rule, PRICED_NATIVE, PINNABLE_NATIVE, the table-work counters and the K_0/GF(2^13) pinned ratios were transcribed from src/cryptanalysis/ecbench/band.rs, methods.rs, ic_framework/solver_units.rs and docs/ic/calibration.json at e9d5dbdccd (band.rs has changed since; the measured revision's version was used) and are checked by rebuilding G - C from group counts. The linearised t interval follows stats.rs linearised_ratio (workloads as clusters, df 7, log scale), with a Student t quantile written here (t0.975,7 = 2.364624251592784, textbook 2.364624251592785). ecbench table/band/compare output is read only as the claimed side.",
    "raw_data_paths_relative_to": ev_rel + "/ (the evidence directory)",
    "raw_data": [
        {"path": "session/records.jsonl", "repo_path": ev_rel + "/session/records.jsonl", "sha256": rec_sha},
        {"path": "session/plan.json", "repo_path": ev_rel + "/session/plan.json", "sha256": sha(p("session", "plan.json"))},
        {"path": "session/session.json", "repo_path": ev_rel + "/session/session.json", "sha256": sha(p("session", "session.json"))},
        {"path": "session/spec.json", "repo_path": ev_rel + "/session/spec.json", "sha256": sha(p("session", "spec.json"))},
        {"path": "session/host.json", "repo_path": ev_rel + "/session/host.json", "sha256": sha(p("session", "host.json"))},
    ],
    "inputs": [
        {"path": "../protocol.json", "sha256": sha(os.path.join(REPO, STUDY, "protocol.json")), "role": "frozen protocol (H1-H4, controls, decision rule)"},
        {"path": "../spec.json", "sha256": sha(os.path.join(REPO, STUDY, "spec.json")), "role": "frozen spec"},
        {"path": "../smoke/build.json", "sha256": sha(os.path.join(REPO, STUDY, "smoke", "build.json")), "role": "binary provenance record"},
        entry("claimed/band.json", "claimed side: producer's ecbench band (reproduced byte-for-byte by the validator's rerun of ecbench band on a copy)"),
        entry("claimed/comparisons/" + os.path.basename(ccomp_path[0]), "claimed side: producer's ecbench compare"),
        entry("validation/claimed.json", "claimed side: the dispatch figures, transcribed"),
        entry("audit.stdout", "in-pod same-host audit (not independent)"),
        entry("pod.log", "pod log"),
        entry("run.stdout", "pod ecbench run output"),
        entry("inputs.sha256", "pod-side input hashes"),
        entry("validation/s3-copy-inputs-pod-run.sh.txt", "byte copy of S3 inputs/pod-run.sh (the pod's commands)"),
    ],
    "scripts": [entry("validation/" + f) for f in ["verify_scalars.py", "recompute.py", "check_tarball.sh", "build_reports.py", "findings.json"]],
    "outputs": [entry(o) for o in outputs if o != "validation/replay-certificate.json"] + [{"path": "validation/replay-certificate.json", "note": "hash pinned in VALIDATION.json"}],
    "headline": [{k: h[k] for k in ("name", "claimed", "recomputed", "rel_tol") } | {k: h[k] for k in ("printed", "agrees", "recomputed_rounds_to_printed", "source") if k in h} for h in headline],
    "headline_summary": {"entries": len(headline), "within_tol": sum(1 for h in headline if h["within_tol"]),
                         "max_rel_diff_full_precision": max(abs(h["claimed"] - h["recomputed"]) / max(abs(h["claimed"]), 1e-300) for h in headline if h["rel_tol"] == TOL)},
    "verdict_checks": verdicts,
    "requirements": req,
    "hypotheses": {
        "H1": {"verdict": H1["verdict"], "leg_interval_below_1": H1["leg_interval_below_1"], "ci95": P["ci95_linearised_t_log"],
               "leg_order_at_every_band_weight": H1["leg_order_at_every_band_weight"], "order_by_weight": H1["order_by_weight"],
               "band_ratio_fes_over_inherited": P["band_ratio_b_over_a"], "producer_reading_matches": True},
        "H2": {"verdict": "supported" if H2["supported"] else "not supported", "priced_by_counts": H2["priced_by_counts"],
               "solver_uncharged_counters": H2["solver_uncharged_counters_anywhere"]},
        "H3": {"verdict": "reported (descriptive)", "distance_to_rho": rc["distance_to_rho"]},
        "H4": {"verdict": "respected by the claimed figures (no wall, other-degree or size-rule claim)"},
    },
    "controls": {"aa": rc["aa_control"], "positive": rc["positive_control"], "negative": "none (protocol: not applicable, counts deterministic)",
                 "reference_arm": "rho-sfstrong-v2 present, verified 48/48, priced with 4 unpriced counters and their counts",
                 "external_fastest_implementation": "not applicable: the claim compares arms inside one harness"},
    "band": {"weights": [0, 0.1, 1], "formula": "S(w) = mean over measured runs of [G - C + w (U + n_C)] / sqrt(r), band.rs at e9d5dbdccd, table work priced (2026-11-08 default reading)",
             "arms": {a: A[a]["band_mean_s"] for a in A}, "ranks": rc["band_ranks"]},
    "reporting_contract": {"one_unit": "ecbench.gae, S = GAE / sqrt(r), r = 2003", "all_arms_priced_including_reference": True,
                           "phases_separated": "per-record phases in records.jsonl; IC G - C = group adds + doubles checked",
                           "m83_ecc2k130_disclosure": "not applicable (degree 13; no m = 83 or ECC2K-130 statement in the claimed artifacts)",
                           "producer_report": "not yet written; only the claimed figures above were checked"},
    "route_deviation": cert["route_deviation"],
    "findings": findings,
    "limitations": [
        "Binary not rebuilt from source (dispatch: do not extract the tree); see F5.",
        "The bootstrap intervals in the claimed comparison (ci95_bootstrap) use ecbench's resampling RNG and were not recomputed; they are not headline figures.",
        "spec_sha256 in session.json (ecbench identity digest) was not reproduced independently; the audit's spec_id check passed on both hosts.",
    ],
}
json.dump(report, open(p("validation", "validator-report.json"), "w"), indent=1)
open(p("validation", "validator-report.json"), "a").write("\n")

# ------------------------------------------------------------------ VALIDATION.json
fes, rho = A["ic-descent-fes-f2"]["band_mean_s"], A["rho-sfstrong-v2"]["band_mean_s"]
order_rho = all(rho[w] < fes[w] for w in wk) or all(rho[w] > fes[w] for w in wk)
man = {
    "schema": "validation-gate/v1",
    "scope": "Gate evidence for session ECBS1h695ef8e76c34 (FES-K13-VALIDATED-20261112, Runpod attempt a1): the degree-13 Koblitz descent rows (binary-subspace d=7 + descent-algebraic m=2 + fes-f2 / inherited-f4 + walk), the pair-table row, and the matched rho. H1 (fes-f2 below inherited-f4 across the band) is NOT supported: unresolved inside the band (validator-report F2). No wall figure is admissible (F3, F4). Binary-to-source binding open (F5).",
    "rows": ["icv1-f2m13-t181-515ee569"],
    "replay_certificate": {"path": "validation/replay-certificate.json", "sha256": sha(p("validation", "replay-certificate.json"))},
    "validator_report": {"path": "validation/validator-report.json", "sha256": sha(p("validation", "validator-report.json"))},
    "rho_reference": {
        "kind": "frobenius",
        "arm": "rho-sfstrong-v2",
        "protocol": "protocol.json arms: rho-sfstrong-v2 = rho.signed_frobenius_strong_v2 (signed-Frobenius orbits, A = 2n = 26), the reference arm and the strongest available Koblitz rho; same session ECBS1h695ef8e76c34, 8 public targets x 6 measured rounds, seeds shared per (workload, round) with every IC arm",
        "unit": "ecbench.gae; S = GAE / sqrt(r), r = 2003",
        "binary_or_process": "one ecbench binary (sha256 8f9576b5bf81819274cc6049eb671c6d97f0bc6660f99d0445448edb4e5130f4), one session ECBS1h695ef8e76c34, for every arm",
        "matches_protocol": True,
        "strongest_available": True,
        "strongest_available_note": "the protocol names it the strongest Koblitz rho; in this session it is below rho.negation at every band weight (2.584 / 2.677 / 3.513 vs 2.788 / 2.927 / 4.183). No other Koblitz rho variant was measured.",
        "same_unit": True,
        "same_binary_or_process": True,
        "ic_arm": "ic-descent-fes-f2",
        "gae_band": {w: {"ic": fes[w], "rho": rho[w]} for w in wk},
        "gae_band_ratio_ic_over_rho": {w: fes[w] / rho[w] for w in wk},
        "order_holds_across_band": order_rho,
        "order_holds_across_band_means": "rho-sfstrong-v2 is below ic-descent-fes-f2 (and below inherited-f4 and the pair table) at w = 0, 0.1 and 1: the gap is the distance remaining. It does not rank fes-f2 against inherited-f4.",
        "component_orders": {"fes_f2_vs_inherited_f4": {"order": {w: ("<" if fes[w] < A["ic-descent-inherited-f4"]["band_mean_s"][w] else ">") for w in wk},
                                                        "ratio": P["band_ratio_b_over_a"], "verdict": "unresolved inside the band"}},
        "band_formula": report["band"]["formula"],
        "unpriced_counters": {a: {"listed": A[a]["unpriced_listed"], "mean_counts_priced": A[a]["priced_counter_mean_counts"]} for a in A},
    },
}
json.dump(man, open(p("VALIDATION.json"), "w"), indent=1)
open(p("VALIDATION.json"), "a").write("\n")
print(json.dumps({"headline_entries": len(headline), "all_within_tol": all_ok, "status": report["status"],
                  "verdicts_agree": all(v["agrees"] for v in verdicts), "cert_scalars": cert["scalars_verified"],
                  "rho_order_holds": order_rho}, indent=1))
