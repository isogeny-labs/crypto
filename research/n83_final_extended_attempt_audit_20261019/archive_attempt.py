#!/usr/bin/env python3
"""Bind the first measured extended N83 holdout's failed gate to durable ZIPs."""

import argparse
import hashlib
import json
import math
import statistics
import subprocess
import zipfile
from pathlib import Path, PurePosixPath

RUN_ID = 39253164581
RUN_ATTEMPT = 1
HEAD_SHA = "448a07d5e6554592425af1d089be527986de5737"
PROTOCOL_SHA256 = "5c6209d7e2e689a6c40bccf9710c6ca3c4c879c1499476ebc40c7626c4a6670c"
SOURCE_SHA256 = "4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507"
TARGETS_SHA256 = "10a0cb758b3dfcff8b79e06a2437c3786ee03049dba7e096e7858024cdd9e811"
PREFIX = "s3://crypto-autoresearcher/factor-bases/icv1/etc/n83_final_extended_attempt_20261019/a1/provider-artifacts"
EXPECTED = {
    0: {"job_id": 118061740677,
        "artifact_id": 12196822837,
        "artifact_bytes": 293340831,
        "artifact_sha256": "fa2252377835ec0db62a773001770ad01998a9b64fc2b1924c8e04e45bb49e20",
        "log_sha256": "43c461722d6bf24f590c728d1c94027909fd5d0d1b7d59859f1f94ceee6c7d5e",
        "analysis_sha256": "ef05b6b05999df78be96ab8ce55a9f18736feb5b0d3e37c6bbe295b1a5550049"},
    1: {"job_id": 118061741036,
        "artifact_id": 12194257314,
        "artifact_bytes": 293338727,
        "artifact_sha256": "56bf5856e9b86d03a5bafec50f62fb33fe35116347cbf24a68216f2ec3a8c8e4",
        "log_sha256": "d3692d3a5013f3e01ef6b24900fc6dc306d158baacdb59f010bd6d09546eff53",
        "analysis_sha256": "9ff32637d3c27ff715f3f3ce6f9b9b2330dd9654a5f2f0c772cf0e98e248d91a"},
}
K1024_ID = "IC1N83Ckb1a1fb169984PDP4rootRCguidedLAgaussTDdirectISO0h644bda8bfc36"
CANDIDATE_IDS = (
    "IC1N83Ckb1a1fb164672PDP4rootRCguidedLAgaussTDdirectISO0ha6a31a64b52b",
    K1024_ID,
    "IC1N83Ckb1a1fb547800PDP4rootRCguidedLAgaussTDdirectISO0h8d1e886f8586",
    "IC1N83Ckb1a1fb382464PDP4rootRCguidedLAgaussTDdirectISO0h00a0a52d463c",
    "IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0he423fcbc5668",
)


def file_sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def provider_analysis(log_path):
    rows = []
    for line in log_path.read_text(encoding="utf-8-sig").splitlines():
        marker = '{"aa_status":'
        if marker in line:
            rows.append(json.loads(line[line.index(marker):]))
    if len(rows) != 1:
        raise ValueError(f"{log_path}: expected one verifier analysis row")
    return rows[0]


def check_zip(path, expected, from_log):
    if path.stat().st_size != expected["artifact_bytes"] or file_sha256(path) != expected["artifact_sha256"]:
        raise ValueError(f"{path}: provider ZIP length or SHA-256 differs")
    with zipfile.ZipFile(path) as archive:
        names = archive.namelist()
        if len(names) != len(set(names)) or any(
                name.startswith("/") or ".." in PurePosixPath(name).parts
                for name in names):
            raise ValueError(f"{path}: duplicate or unsafe ZIP member")
        if archive.testzip() is not None:
            raise ValueError(f"{path}: ZIP CRC failure")
        analysis_bytes = archive.read("results/analysis.json")
        if hashlib.sha256(analysis_bytes).hexdigest() != expected["analysis_sha256"]:
            raise ValueError(f"{path}: archived analysis SHA-256 differs")
        analysis = json.loads(analysis_bytes)
        verification = json.loads(archive.read("verify.log"))
        if analysis != from_log or verification != analysis:
            raise ValueError(f"{path}: ZIP, verifier log and job log disagree")
        return len(names)


def check_analysis(stratum, analysis):
    if (analysis["schema"] != "n83.final-extended-stratum-analysis/v1" or
            analysis["stratum"] != stratum or
            analysis["point_indices"] != list(range(8 * stratum, 8 * stratum + 8)) or
            analysis["source_sha256"] != SOURCE_SHA256 or
            analysis["choice"]["ordered_candidate_ids"] != list(CANDIDATE_IDS) or
            analysis["targets_sha256"] != TARGETS_SHA256 or
            analysis["evidence_status"] != "PASS_INDEPENDENT_REPLAY" or
            analysis["base_status"] != "PASS" or
            analysis["resource_status"] != "PASS" or
            analysis["execution_order_status"] != "PASS" or
            analysis["topology_status"] != "PASS" or
            analysis["telemetry_status"] != "PASS" or
            analysis["all_launch_commands_match"] is not True or
            analysis["all_per_point_targets_match"] is not True or
            analysis["archived_executed_binaries_match"] is not True or
            analysis["producer_stop"] is not None):
        raise ValueError(f"stratum {stratum}: verifier evidence gate differs")
    points = analysis["points"]
    if len(points) != 8 or [p["point_index"] for p in points] != analysis["point_indices"]:
        raise ValueError(f"stratum {stratum}: point rows differ")
    noise = []
    for point in points:
        if (len(point["arms"]) != 5 or
                {arm["candidate_id"] for arm in point["arms"]} != set(CANDIDATE_IDS) or
                set(point["rho_by_candidate"]) != set(CANDIDATE_IDS) or
                set(point["calibration_by_candidate"]) != set(CANDIDATE_IDS) or
                any(arm["status"] != "PASS_INDEPENDENT_REPLAY"
                    for arm in point["arms"]) or
                any(rho["status"] != "PASS_INDEPENDENT_REPLAY"
                    for rho in point["rho_by_candidate"].values())):
            raise ValueError(f"stratum {stratum}: incomplete IC or rho replay")
        for candidate_id, calibration in point["calibration_by_candidate"].items():
            before = calibration["before_repetitions_ms"]
            after = calibration["after_repetitions_ms"]
            if (len(before) != 3 or len(after) != 3 or
                    any(not math.isfinite(value) or value <= 0
                        for value in (*before, *after))):
                raise ValueError("calibration repetitions missing or invalid")
            b, a = statistics.median(before), statistics.median(after)
            spread = abs(b - a) / ((b + a) / 2)
            if (abs(spread - calibration["relative_spread"]) > 1e-10 or
                    (calibration["status"] == "PASS_10_PERCENT") != (spread <= 0.10)):
                raise ValueError("calibration status disagrees with raw repetitions")
            if calibration["status"] != "PASS_10_PERCENT":
                noise.append({"point_index": point["point_index"],
                              "candidate_id": candidate_id,
                              "status": calibration["status"],
                              "relative_spread": calibration["relative_spread"],
                              "before_repetitions_ms": calibration["before_repetitions_ms"],
                              "after_repetitions_ms": calibration["after_repetitions_ms"]})
    if stratum == 0:
        if (noise or analysis["status"] != "STRATUM_ADMITTED" or
                analysis["aa_status"] != "PASS_10_PERCENT_PER_PAIR" or
                analysis["tier_status"] != "PASS_TIER1"):
            raise ValueError("stratum 0 expected all 40 calibrations admitted")
    else:
        if (len(noise) != 1 or noise[0]["point_index"] != 14 or
                noise[0]["candidate_id"] != K1024_ID or
                noise[0]["status"] != "NOISY_10_PERCENT" or
                not 0.10757 < noise[0]["relative_spread"] < 0.10758 or
                analysis["status"] != "BOUNDED_UNKNOWN" or
                analysis["aa_status"] != "NOISY_OR_INCOMPLETE" or
                analysis["tier_status"] != "NOT_ADMITTED"):
            raise ValueError("stratum 1 expected exact K1024 noise gate failure")
    return {"stratum": stratum, "status": analysis["status"],
            "evidence_status_reported_by_provider": analysis["evidence_status"],
            "aa_status": analysis["aa_status"], "tier_status": analysis["tier_status"],
            "point_count": len(points), "ic_replay_rows": 40,
            "rho_replay_rows": 40, "calibration_pairs": 40,
            "passing_calibration_pairs": 40 - len(noise),
            "noisy_pairs": noise}


def s3_head(uri):
    key = uri.removeprefix("s3://crypto-autoresearcher/")
    call = subprocess.run(("aws", "s3api", "head-object", "--bucket",
                           "crypto-autoresearcher", "--key", key,
                           "--output", "json"), capture_output=True, text=True)
    if call.returncode:
        if "Not Found" in call.stderr or "404" in call.stderr:
            return None
        raise RuntimeError(f"S3 HEAD failed: {call.stderr.strip()}")
    return json.loads(call.stdout)


def s3_upload_and_replay(path, stratum, expected):
    uri = f"{PREFIX}/{expected['artifact_sha256']}.zip"
    metadata = (f"github-artifact-id={expected['artifact_id']},"
                f"run-id={RUN_ID},stratum={stratum}")
    if s3_head(uri) is None:
        subprocess.run(("aws", "s3", "cp", str(path), uri, "--no-progress",
                        "--only-show-errors", "--metadata", metadata),
                       check=True, capture_output=True, text=True)
    observed = s3_head(uri)
    if (observed is None or observed["ContentLength"] != expected["artifact_bytes"] or
            observed.get("Metadata", {}).get("github-artifact-id") !=
            str(expected["artifact_id"])):
        raise ValueError(f"stratum {stratum}: S3 HEAD differs")
    process = subprocess.Popen(("aws", "s3", "cp", uri, "-", "--only-show-errors"),
                               stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    digest = hashlib.sha256()
    for chunk in iter(lambda: process.stdout.read(1024 * 1024), b""):
        digest.update(chunk)
    error = process.stderr.read().decode(errors="replace")
    if process.wait() != 0 or digest.hexdigest() != expected["artifact_sha256"]:
        raise ValueError(f"stratum {stratum}: S3 downloaded ZIP differs: {error}")
    return {"s3_uri": uri, "s3_etag": observed["ETag"],
            "s3_last_modified": observed["LastModified"],
            "download_sha256_status": "PASS"}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--artifact-dir", type=Path, required=True)
    parser.add_argument("--log-dir", type=Path, required=True)
    parser.add_argument("--receipt", type=Path, required=True)
    parser.add_argument("--provider-snapshot", type=Path, required=True)
    parser.add_argument("--upload", action="store_true")
    args = parser.parse_args()
    snapshot_bytes = args.provider_snapshot.read_bytes()
    snapshot = json.loads(snapshot_bytes)
    run = snapshot["run"]
    if (snapshot["schema"] != "n83.final-extended-provider-snapshot/v1" or
            run["id"] != RUN_ID or run["run_attempt"] != RUN_ATTEMPT or
            run["status"] != "completed" or run["conclusion"] != "failure" or
            run["head_sha"] != HEAD_SHA):
        raise ValueError("GitHub provider run snapshot differs")
    provider_artifacts = {item["id"]: item for item in snapshot["artifacts"]}
    if set(provider_artifacts) != {e["artifact_id"] for e in EXPECTED.values()}:
        raise ValueError("GitHub provider artifact set differs")
    rows = []
    for stratum, expected in EXPECTED.items():
        provider = provider_artifacts[expected["artifact_id"]]
        if (provider["digest"] != "sha256:" + expected["artifact_sha256"] or
                provider["size_in_bytes"] != expected["artifact_bytes"] or
                provider["name"] !=
                f"n83-final-extended-stratum-{stratum}-{RUN_ID}-{RUN_ATTEMPT}"):
            raise ValueError(f"stratum {stratum}: provider artifact metadata differs")
        path = args.artifact_dir / f"stratum{stratum}.zip"
        log = args.log_dir / f"n83-extended-stratum{stratum}-{expected['job_id']}.log"
        if file_sha256(log) != expected["log_sha256"]:
            raise ValueError(f"stratum {stratum}: GitHub job log SHA-256 differs")
        analysis = provider_analysis(log)
        member_count = check_zip(path, expected, analysis)
        row = check_analysis(stratum, analysis)
        row.update({"job_id": expected["job_id"],
                    "github_artifact_id": expected["artifact_id"],
                    "provider_artifact_sha256": expected["artifact_sha256"],
                    "provider_artifact_bytes": expected["artifact_bytes"],
                    "job_log_sha256": expected["log_sha256"],
                    "archived_analysis_sha256": expected["analysis_sha256"],
                    "zip_member_count": member_count,
                    "zip_crc_status": "PASS"})
        rows.append(row)
        print(f"stratum {stratum}: {row['status']}, {member_count} ZIP members",
              flush=True)
    if args.upload:
        for stratum, row in enumerate(rows):
            row.update(s3_upload_and_replay(
                args.artifact_dir / f"stratum{stratum}.zip", stratum,
                EXPECTED[stratum]))
            print(f"stratum {stratum}: S3 download SHA-256 PASS", flush=True)
    receipt = {"schema": "n83.final-extended-attempt-audit/v1",
               "audit_scope": "provider_ZIP_CRC_analysis_and_job_log_binding",
               "raw_arm_reexecution_performed_here": False,
               "run_id": RUN_ID, "run_attempt": RUN_ATTEMPT,
               "run_head_sha": HEAD_SHA,
               "frozen_protocol_sha256": PROTOCOL_SHA256,
               "provider_snapshot_sha256": hashlib.sha256(snapshot_bytes).hexdigest(),
               "status": "FROZEN_CALIBRATION_GATE_FAIL",
               "selection_admitted": False,
               "selected_candidate_id": None,
               "selected_best_total_runtime": None,
               "provider_run_conclusion": "failure",
               "strata": rows}
    encoded = json.dumps(receipt, indent=2, sort_keys=True) + "\n"
    if args.receipt.exists() and args.receipt.read_text() != encoded:
        raise ValueError("existing attempt receipt differs")
    args.receipt.write_text(encoded)
    print(json.dumps({"status": receipt["status"],
                      "s3_uploaded": args.upload}, sort_keys=True))


if __name__ == "__main__":
    main()
