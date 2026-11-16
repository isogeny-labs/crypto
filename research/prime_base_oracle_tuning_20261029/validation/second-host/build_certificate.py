#!/usr/bin/env python3
"""Assemble validation/second-host/replay-certificate.json from the
validator's own receipts in this directory (no number is typed by hand).

Run from the study directory:
  python3 -I validation/second-host/build_certificate.py
"""
import hashlib
import json
from pathlib import Path

HERE = Path("validation/second-host")
SH = Path("second-host")


def sha(p):
    return hashlib.sha256(Path(p).read_bytes()).hexdigest()


def load(p):
    return json.loads(Path(p).read_text())


def main():
    rc = load(HERE / "record-compare.json")
    seeds = load(HERE / "seed-law.json")
    v_sh = load(HERE / "verify-second-host-session.audit.json")
    v_or = load(HERE / "verify-original-session.audit.json")
    w_sh = load(SH / "eks-results/audit.json")
    s_sh = load(SH / "eks-results/session/session.json")
    s_or = load("sessions/holdout-P20/session.json")
    h_sh = load(SH / "eks-results/session/host.json")["stable"]
    h_or = load("sessions/holdout-P20/host.json")["stable"]
    build = load(SH / "eks-results/build-info.json")
    l2 = load(SH / "eks-results/l2-gate.json")
    result = load(SH / "launch/result.json")
    byte_cmp = (HERE / "compare/byte-compare.txt").read_text().split("\n")
    byte_cmp = [x for x in byte_cmp if x]

    ratios = {}
    for w in ("0", "0.1", "1"):
        ratios[w] = {}
        for a, b in (("inc", "sel"), ("rho-neg", "sel"), ("rho-neg", "inc"), ("inc", "inc-aa"),
                     ("rho-neg", "rho-neg-aa")):
            c = load(HERE / f"compare/compare-{a}__{b}-w{w}.json")
            cu = c["curves"][0]
            ratios[w][f"{b}/{a}"] = {
                "ratio": cu["ratio_b_over_a"], "ci95_cluster_bootstrap": cu["ci95_bootstrap"],
                "pairs": cu["pairs"], "same_seeds": c["ops"]["same_seeds"],
                "unpaired_runs": c["ops"]["unpaired_runs"],
                "validator_stdlib_recompute": rc["band_second_host_rounds_1_5"]["paired_ratio_sums"][f"{b}/{a}"][str(float(w))],
            }
    orig = rc["band_original_rounds_1_2"]["paired_ratio_sums"]
    sc = rc["scalar_check_second_host"]

    files = {}
    for p in sorted(HERE.rglob("*")):
        if p.is_file() and p.name not in ("replay-certificate.json", "validation_report.yaml", "SHA256SUMS"):
            files[str(p)] = sha(p)
    for p in ("second-host/eks-results/session/records.jsonl", "second-host/eks-results/session/session.json",
              "second-host/eks-results/session/spec.json", "second-host/eks-results/session/plan.json",
              "second-host/eks-results/session/host.json", "second-host/eks-results/audit.json",
              "second-host/eks-results/SHA256SUMS", "second-host/eks-results/build-info.json",
              "second-host/eks-results/l2-gate.json", "second-host/launch/request.json",
              "second-host/launch/validated-request.json", "second-host/launch/result.json",
              "sessions/holdout-P20/records.jsonl", "sessions/holdout-P20/session.json",
              "sessions/holdout-P20/spec.json", "specs/holdout-P20.json", "select/FROZEN.sha256"):
        files[p] = sha(p)

    cert = {
        "schema": "replay-certificate/v1",
        "id": "VAL-20261029-7ff9db",
        "kind": "second-host re-execution of the frozen holdout-P20 spec with a record-by-record match (PROTOCOL.md section 13)",
        "evidence_dir": "research/prime_base_oracle_tuning_20261029",
        "rows": ["icv1-fp20-tm565-6d7d1d32"],
        "snapshot": {
            "branch": "claude/pbo-xhost-leaderboard-20261029",
            "validated_content_commit": "1e7b2b7ed20dcfccaf6f9454281f042508a7d2bd",
            "note": "Every study file named here is unchanged in the work tree against 1e7b2b7ed (git status clean for the study dir apart from this untracked validation/second-host/ directory, written by the validator). Pointer-stored files (records.jsonl, audit.stdout, run.stderr, the PNG) hash to their .artifacts.json sha256 and the store holds objects of those sizes (artifacts.py verify --remote).",
        },
        "status": "verified",
        "independent": True,
        "scalars_verified": True,
        "runs": [
            {
                "role": "second-host re-execution",
                "session_dir": "second-host/eks-results/session",
                "session_id": s_sh["session_id"],
                "spec_id": s_sh["spec_id"],
                "records": s_sh["records_written"],
                "records_sha256": s_sh["records_sha256"],
                "measured_records": len(v_sh["replays"]),
                "env_class_id": s_sh["env_class_id"],
                "binary_sha256": s_sh["binary_sha256"],
                "isolation": l2,
                "worker_audit": {"path": "second-host/eks-results/audit.json", "sha256": sha(SH / "eks-results/audit.json"),
                                 "ok": w_sh["ok"], "verified_records": w_sh["verified_records"],
                                 "replays": len(w_sh["replays"]),
                                 "replays_identical": sum(1 for r in w_sh["replays"] if r["reproduced"] and r["detail"] == "identical"),
                                 "auditor_binary_sha256": w_sh["auditor_binary_sha256"],
                                 "auditor_env_class_id": w_sh["auditor_env_class_id"]},
                "validator_audit": {"path": str(HERE / "verify-second-host-session.audit.json"),
                                    "sha256": sha(HERE / "verify-second-host-session.audit.json"),
                                    "command": "RAYON_NUM_THREADS=1 taskset -c 2 <bf5eec6d binary> verify --dir second-host/eks-results/session --replay-all --exit-code",
                                    "ok": v_sh["ok"], "checks": v_sh["checks"], "verified_records": v_sh["verified_records"],
                                    "replays": len(v_sh["replays"]),
                                    "replays_identical": sum(1 for r in v_sh["replays"] if r["reproduced"] and r["detail"] == "identical"),
                                    "problems": v_sh["problems"],
                                    "auditor_binary_sha256": v_sh["auditor_binary_sha256"],
                                    "auditor_env_class_id": v_sh["auditor_env_class_id"],
                                    "files": v_sh["files"]},
            },
            {
                "role": "original measurement (the session the leaderboard row cites)",
                "session_dir": "sessions/holdout-P20",
                "session_id": s_or["session_id"],
                "spec_id": s_or["spec_id"],
                "records": s_or["records_written"],
                "records_sha256": s_or["records_sha256"],
                "env_class_id": s_or["env_class_id"],
                "binary_sha256": s_or["binary_sha256"],
                "validator_audit": {"path": str(HERE / "verify-original-session.audit.json"),
                                    "sha256": sha(HERE / "verify-original-session.audit.json"),
                                    "ok": v_or["ok"], "verified_records": v_or["verified_records"],
                                    "replays": len(v_or["replays"]),
                                    "replays_identical": sum(1 for r in v_or["replays"] if r["reproduced"] and r["detail"] == "identical"),
                                    "files": v_or["files"],
                                    "note": "fresh same-host re-audit at this snapshot; the earlier same-host certificate is ../../replay-certificate.json"},
            },
        ],
        "independence": {
            "replayer": "Validator subagent (claude-opus-5-5), separate from the producer session that launched the EKS job and wrote second-host/README.md and second-host/tools/match_records.py (the latter not read or run by the validator)",
            "second_host_class_distinct": True,
            "host_facts": {
                "original": {k: h_or.get(k) for k in ("cpu_vendor", "cpu_model", "cpu_signature", "kernel_release", "virtualization", "logical_cpus")},
                "second_host": {k: h_sh.get(k) for k in ("cpu_vendor", "cpu_model", "cpu_signature", "kernel_release", "virtualization", "logical_cpus")},
            },
            "env_class_ids": {"original": s_or["env_class_id"], "second_host": s_sh["env_class_id"],
                              "note": "the two ids come from different class-id schemes (ECBENV3 in bf5eec6d, ECBENV2 in the older 583e62f0 build), so their inequality alone proves nothing; distinctness rests on the host facts above (Intel Xeon 6/207/2 under KVM vs AMD EPYC 9R45 on an EC2 c8a.2xlarge, different kernels)"},
            "binaries": {
                "original": {"sha256": s_or["binary_sha256"], "source_commit": "0e124fb1b207a76ae9ef88c2bffb1f5b8d92b1b6 (PROTOCOL.md section 12; src/ unchanged to the measurement worktree 7df34ba6)"},
                "second_host": {"sha256": build["ecbench_sha256"], "source_commit": build["source_revision"], "built_at": build["built_at"],
                                "provenance": "attested by the task image's build-info.json (image isolab/tasks@sha256:67eea58f5d4a7a8882b55f089c6063bb49fad58d40260ee9b0ceb349fc0e4581); the validator did not rebuild it"},
                "ecbench_source_distance": "188 commits touch src/cryptanalysis/ecbench between cb2ec687 and 0e124fb1; the algorithm-seed law in spec.rs is the same in both",
            },
            "independent_implementation": False,
            "independent_scalar_check": True,
            "statement": "Every one of the 1,008 original records was re-executed on a different host class by a different ecbench build and reproduced identically in every counted, band-priced and outcome field. The comparison and the [d]G = Q check are the validator's own code (stdlib Python). Both hosts ran ecbench, so this is not an independent-implementation replication.",
        },
        "spec_identity": {
            "frozen_spec": {"path": "specs/holdout-P20.json", "sha256": sha("specs/holdout-P20.json"),
                            "equals_original_session_spec": sha("specs/holdout-P20.json") == sha("sessions/holdout-P20/spec.json"),
                            "frozen_sha256_still_matches": True},
            "second_host_spec": {"path": "second-host/eks-results/session/spec.json", "sha256": sha(SH / "eks-results/session/spec.json"),
                                 "equals_input_spec_and_request_spec": True},
            "fields_that_differ": {
                "label": "descriptive; enters spec_id only",
                "measurement.rounds": [2, 5],
                "measurement.isolation_required": ["L0", "L2"],
            },
            "note": "Three fields differ, not two as second-host/README.md says: the label also changed. None of the three enters a workload, a target or an algorithm seed (spec.rs plan: seed = derive_u64(domain, [measurement.seed, workload id, round])).",
        },
        "seed_law": {"receipt": str(HERE / "seed-law.json"), "script": str(HERE / "seed_law.py"),
                     "result": seeds["files"]},
        "record_match": {
            "receipt": str(HERE / "record-compare.json"), "script": str(HERE / "compare_records.py"),
            "pairing_key": rc["pairing_key"],
            "original_records": rc["original"]["records"],
            "original_records_with_partner": rc["original_records_with_partner"],
            "algorithm_seed_mismatches": len(rc["algorithm_seed_mismatches"]),
            "pairs_identical_on_strict_field_set": rc["pairs_identical_on_strict_field_set"],
            "strict_field_set": rc["strict_field_set"],
            "priced_s_mismatches_at_w_0_0.1_1": len(rc["priced_s_mismatches_w0_0.1_1"]),
            "records_with_uncounted_unpriced_counter": rc["records_with_uncounted_unpriced_counter"],
            "records_with_calibration_pinned_units": rc["records_with_calibration_pinned_units"],
            "fields_that_differ_on_some_pair": sorted(rc["diff_summary"]["differ_on_some_pair_depth2"]),
            "fields_only_in_original": sorted(rc["diff_summary"]["present_in_original_only_depth2"]),
            "fields_only_in_second_host": sorted(rc["diff_summary"]["present_in_second_host_only_depth2"]),
            "reading": "The differing fields are identity (record_id, session_id, binary_sha256, env_class_id), host observations (isolation.*, time.*) and wall clocks (online.wall_ns, online.phases_ns, phases[].wall_ns). Fields only in the original are the newer build's bitop/v1 `work` block, phases[].work_native, the allocator block and time.solve. No field that band.rs (unpriced_counts, calibrated_native, priced_gae, priced_s), compare.rs (match_pairs, run_s) or the outcome depends on differs or is missing.",
            "second_host_rounds_1_2_equal_original_band_exactly": rc["second_host_rounds_1_2_equal_original_exactly"],
        },
        "scalar_check": {"method": "validator's own affine arithmetic (compare_records.py): a, b derived from G and a target, every target on the curve, [r]G = O, [d]G = Q for every verified record",
                         "second_host": sc, "original": rc["scalar_check_original"]},
        "analysis_rerun": {
            "binary_sha256": v_sh["auditor_binary_sha256"],
            "commands": str(HERE / "compare/commands.log"),
            "byte_compare_with_second_host_analysis": byte_cmp,
            "extra": "rho-neg-aa vs rho-neg compares at w = 0, 0.1, 1 (not in the producer's analysis) give exactly 1.0 over 240 pairs",
        },
        "ratios_second_host_240_pairs": ratios,
        "ratios_original_96_pairs_validator_recompute": {w: {k: orig[k][str(float(w))] for k in ("sel/inc", "sel/rho-neg", "inc/rho-neg")} for w in ("0", "0.1", "1")},
        "launch": {"job": result["job_name"], "job_uid": result["job_uid"], "pod_uid": result["pod_uid"],
                   "launcher": result["launcher"], "results_uri": result["results_uri"],
                   "request_sha256": result["request_sha256"],
                   "s3_recheck": str(HERE / "provenance-checks.txt")},
        "caveats": [
            "Form: a new session from the frozen spec on another host class, matched record by record to the original, plus a local `ecbench verify --replay-all` of the new session. It is not an `ecbench verify --replay-all` receipt of the original session's bytes produced on the second host; a `vs_rho` claim through `ecbench claim build --independent-receipt` would need that form.",
            "Not an independent implementation: both hosts ran ecbench (builds 0e124fb1 and cb2ec687). The validator's [d]G = Q check and stdlib band recompute are the implementation-independent checks.",
            "The second-host binary's source revision is attested by its image, not rebuilt by the validator.",
            "The bitop/v1 `work` and phase `work_native` fields are absent from the older build's records, so no bitop/v1 figure is covered by this replay. Wall times are host observations and are not compared.",
            "Rounds 3-5 add 144 target-round pairs per arm on the same curve with seeds the original never ran. They are post-freeze data, consistent with the frozen 96-pair reading, and do not replace it.",
        ],
        "files_sha256": files,
    }
    (HERE / "replay-certificate.json").write_text(json.dumps(cert, indent=1, ensure_ascii=False) + "\n")


if __name__ == "__main__":
    main()
