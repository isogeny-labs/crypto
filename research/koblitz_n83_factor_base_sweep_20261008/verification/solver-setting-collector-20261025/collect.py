#!/usr/bin/env python3
"""Probe and independently collect the frozen N83 solver-setting run."""

import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
import subprocess
import sys


HERE = Path(__file__).resolve().parent
PANEL = HERE.parent / "solver-setting-panel-20261025"
PROTOCOL = json.loads((PANEL / "protocol.json").read_bytes())
REPO = "repos/aburan28/crypto"
RUN_ID = 40193751812
ATTEMPT = 1
HEAD = "a5af8d5023a644e9e3b347af4d8495540c8dea99"
WORKFLOW = ".github/workflows/n83-solver-setting-panel.yml"


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def api(endpoint: str) -> bytes:
    return subprocess.run(["gh", "api", endpoint], capture_output=True,
                          check=True, timeout=180).stdout


def snapshots() -> tuple[bytes, bytes, bytes]:
    return (api(f"{REPO}/actions/runs/{RUN_ID}"),
            api(f"{REPO}/actions/runs/{RUN_ID}/jobs?per_page=100"),
            api(f"{REPO}/actions/runs/{RUN_ID}/artifacts?per_page=100"))


def assess(run: dict, jobs: dict, artifacts: dict) -> dict:
    ids = {row["id"] for row in PROTOCOL["strata"]}
    require(PROTOCOL["schema"] == "n83.solver-setting-screen-protocol/v1"
            and len(ids) == len(PROTOCOL["strata"]) == 8,
            "frozen eight-stratum protocol")
    require(run["id"] == RUN_ID and run["run_attempt"] == ATTEMPT
            and run["head_sha"] == HEAD and run["path"] == WORKFLOW
            and run["event"] == "push"
            and run["head_branch"] == "codex/n83-aws-handoff-20261025",
            "exact frozen provider run")
    expected_jobs = {f"solve-setting-stratum ({row})" for row in ids}
    entries = jobs["jobs"]
    require(jobs["total_count"] == len(entries) == 8
            and {job["name"] for job in entries} == expected_jobs
            and all(job["run_id"] == RUN_ID and job["head_sha"] == HEAD
                    for job in entries), "exact eight provider jobs")
    completed = [job for job in entries if job["status"] == "completed"]
    bad = [job["name"] for job in completed if job["conclusion"] != "success"]
    if bad:
        return {"status": "FAILED_PROVIDER_JOB", "bad_jobs": bad,
                "completed_jobs": len(completed), "expected_jobs": 8,
                "available_artifacts": len(artifacts.get("artifacts", []))}
    if len(completed) != 8 or run["status"] != "completed":
        return {"status": "WAITING_FOR_PROVIDER_RUN",
                "completed_jobs": len(completed), "expected_jobs": 8,
                "available_artifacts": len(artifacts.get("artifacts", []))}
    require(run["conclusion"] == "success", "completed provider run succeeded")
    items = artifacts["artifacts"]
    expected_artifacts = {
        f"n83-solver-setting-{row}-{RUN_ID}-{ATTEMPT}": row for row in ids}
    if len(items) != 8 or artifacts["total_count"] != 8:
        return {"status": "WAITING_FOR_EIGHT_ARTIFACTS",
                "completed_jobs": 8, "expected_jobs": 8,
                "available_artifacts": len(items)}
    require({item["name"] for item in items} == set(expected_artifacts)
            and len({item["id"] for item in items}) == 8
            and all(not item["expired"]
                    and item["workflow_run"]["id"] == RUN_ID
                    and item["workflow_run"]["head_sha"] == HEAD
                    for item in items), "eight exact unexpired provider artifacts")
    return {"status": "READY_TO_AUDIT", "completed_jobs": 8,
            "expected_jobs": 8, "available_artifacts": 8,
            "artifact_ids": {expected_artifacts[item["name"]]: item["id"]
                             for item in items}}


def fetch_artifact(artifact_id: int, destination: Path,
                   expected_sha256: str | None = None) -> tuple[Path, Path, str]:
    metadata_raw = api(f"{REPO}/actions/artifacts/{artifact_id}")
    metadata = json.loads(metadata_raw)
    require(metadata["id"] == artifact_id and not metadata["expired"],
            f"provider artifact {artifact_id} identity")
    metadata_path = destination.with_suffix(".metadata.json")
    metadata_path.write_bytes(metadata_raw)
    archive = destination.with_suffix(".zip")
    archive.write_bytes(api(f"{REPO}/actions/artifacts/{artifact_id}/zip"))
    digest = hashlib.sha256(archive.read_bytes()).hexdigest()
    require(metadata["size_in_bytes"] == archive.stat().st_size
            and metadata["digest"] == f"sha256:{digest}"
            and (expected_sha256 is None or expected_sha256 == digest),
            f"provider archive {artifact_id} digest and length")
    return archive, metadata_path, digest


def collect(out: Path, provider_raw: tuple[bytes, bytes, bytes],
            assessment: dict) -> dict:
    require(assessment["status"] == "READY_TO_AUDIT",
            "provider run and all eight artifacts terminal")
    require(not out.exists(), "new immutable collection directory")
    out.mkdir(parents=True)
    run_raw, jobs_raw, artifacts_raw = provider_raw
    (out / "provider-run.json").write_bytes(run_raw)
    (out / "provider-jobs.json").write_bytes(jobs_raw)
    (out / "provider-artifacts.json").write_bytes(artifacts_raw)
    sources = {}
    for trial in (1, 2):
        row = next(item for item in PROTOCOL["strata"]
                   if item["trial"] == trial)
        sources[trial] = fetch_artifact(
            row["source_artifact_id"], out / f"source-t{trial}",
            row["source_artifact_sha256"])
    solver = fetch_artifact(PROTOCOL["solver_artifact_id"], out / "solver",
                            PROTOCOL["solver_artifact_sha256"])
    audits = []
    for row in PROTOCOL["strata"]:
        stratum = row["id"]
        archive, metadata, digest = fetch_artifact(
            assessment["artifact_ids"][stratum], out / stratum)
        source_zip, source_meta, _ = sources[row["trial"]]
        command = [sys.executable, str(PANEL / "audit_artifact.py"),
                   "--stratum", stratum,
                   "--artifact-zip", str(archive),
                   "--expected-zip-sha256", digest,
                   "--metadata", str(metadata),
                   "--jobs", str(out / "provider-jobs.json"),
                   "--source-zip", str(source_zip),
                   "--source-metadata", str(source_meta),
                   "--solver-zip", str(solver[0])]
        result = subprocess.run(command, capture_output=True, check=True,
                                timeout=180)
        audit = json.loads(result.stdout)
        require(audit["run_id"] == RUN_ID and audit["attempt"] == ATTEMPT
                and audit["head_sha"] == HEAD
                and audit["stratum"] == stratum
                and audit["artifact_id"] == assessment["artifact_ids"][stratum]
                and audit["artifact_zip_sha256"] == digest
                and audit["selected_best_total_runtime"] is None,
                f"independent audit of {stratum}")
        (out / f"{stratum}.audit.json").write_bytes(result.stdout)
        audits.append(audit)
    statuses = Counter(audit["status"] for audit in audits)
    arms = Counter(arm["status"] for audit in audits for arm in audit["arms"])
    summary = {"schema": "n83.solver-setting-run-collection/v1",
               "status": "PASS_EIGHT_CENSORED_STRATA" if
               statuses == {"PASS_FIVE_CENSORED_SETTINGS_REPLAY": 8}
               else "AUDITED_WITH_PENDING_OR_STOPPED_ARMS",
               "run_id": RUN_ID, "attempt": ATTEMPT, "head_sha": HEAD,
               "provider_job_count": 8, "provider_artifact_count": 8,
               "audited_strata": len(audits), "audited_arms": sum(arms.values()),
               "audit_status_counts": dict(statuses),
               "solver_status_counts": dict(arms),
               "summed_solver_wall_seconds_L0": sum(
                   arm["wall_seconds_L0"] for audit in audits
                   for arm in audit["arms"]),
               "selected_best_total_runtime": None}
    (out / "collection.json").write_text(
        json.dumps(summary, indent=2, sort_keys=True) + "\n")
    return summary


def main() -> None:
    parser = argparse.ArgumentParser()
    actions = parser.add_mutually_exclusive_group(required=True)
    actions.add_argument("--probe", action="store_true")
    actions.add_argument("--collect", action="store_true")
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()
    require(not args.collect or args.out is not None,
            "--collect requires --out")
    provider_raw = snapshots()
    assessment = assess(*(json.loads(raw) for raw in provider_raw))
    if args.probe:
        print(json.dumps(assessment, indent=2, sort_keys=True))
    else:
        print(json.dumps(collect(args.out, provider_raw, assessment),
                         indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
