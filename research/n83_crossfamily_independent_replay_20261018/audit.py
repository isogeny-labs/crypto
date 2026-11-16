#!/usr/bin/env python3
"""Bind four independent hosted replay artifacts to the frozen N83 panel."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import re
import zipfile

HERE = Path(__file__).resolve().parent
RUN_ID = 39096459511
HEAD_SHA = "7f3a0bf3c7feaa54ecb3c5e497dd2022de8e1ab2"
BRANCH = "codex/n83-hosted-regenerate-20261016"
MEMBERS = {
    "download.exit_status.txt", "download.stderr.txt",
    "replay.exit_status.txt", "replay.json", "replay.stderr.txt",
    "replay.stdout.json",
}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def canonical_hash(value: object) -> str:
    blob = json.dumps(value, sort_keys=True, separators=(",", ":"))
    return hashlib.sha256(blob.encode()).hexdigest()


def read(path: Path) -> dict:
    return json.loads(path.read_text())


def audit(hosted: Path, provider: Path) -> dict:
    dispatch_bytes = (HERE / "dispatch.json").read_bytes()
    dispatch = json.loads(dispatch_bytes)
    run = read(hosted / "run.json")
    jobs = read(hosted / "jobs.json")
    inventory = read(hosted / "artifacts.json")
    validated = read(provider / "audit" / "validated_rows.json")
    require(dispatch["schema"] ==
            "n83.crossfamily-independent-hosted-replay-dispatch/v1"
            and dispatch["selected_best_total_runtime"] is None
            and [row["stratum"] for row in dispatch["strata"]] ==
            [0, 1, 2, 3], "dispatch differs")
    require(run["id"] == RUN_ID and run["run_attempt"] == 1
            and run["head_sha"] == HEAD_SHA
            and run["head_branch"] == BRANCH
            and run["event"] == "push"
            and run["path"] ==
            ".github/workflows/n83-crossfamily-independent-replay.yml"
            and run["status"] == "completed"
            and run["conclusion"] == "success", "hosted run differs")
    require(jobs["total_count"] == len(jobs["jobs"]) == 4
            and inventory["total_count"] == len(inventory["artifacts"]) == 4,
            "job or artifact coverage differs")
    job_by_stratum = {}
    for job in jobs["jobs"]:
        match = re.fullmatch(r"replay-stratum \(([0-3]), ([0-9]+)\)",
                             job["name"])
        require(match is not None and job["run_id"] == RUN_ID
                and job["head_sha"] == HEAD_SHA
                and job["status"] == "completed"
                and job["conclusion"] == "success"
                and job["runner_id"] > 0 and job["runner_name"],
                "hosted job differs")
        stratum = int(match.group(1))
        require(stratum not in job_by_stratum
                and int(match.group(2)) ==
                dispatch["strata"][stratum]["artifact_id"],
                "job stratum or artifact differs")
        job_by_stratum[stratum] = job
    require(set(job_by_stratum) == set(range(4))
            and len({row["runner_id"] for row in job_by_stratum.values()}) == 4
            and len({row["runner_name"] for row in job_by_stratum.values()}) == 4,
            "independent hosted runners not distinct")
    provider_runners = {row["runner_name"]
                        for row in validated["host_strata"]}
    require(not provider_runners.intersection(
                {row["runner_name"] for row in job_by_stratum.values()}),
            "hosted audit reused a provider runner")
    artifacts = {row["name"]: row for row in inventory["artifacts"]}
    require(len(artifacts) == 4, "duplicate hosted artifact names")
    summaries = []
    for row in dispatch["strata"]:
        number = row["stratum"]
        name = ("n83-crossfamily-independent-replay-stratum-"
                f"{number}-{RUN_ID}-1")
        meta = artifacts[name]
        archive = hosted / f"stratum{number}.zip"
        zip_hash = sha256(archive)
        require(meta["expired"] is False
                and meta["size_in_bytes"] == archive.stat().st_size
                and meta["digest"] == "sha256:" + zip_hash
                and meta["workflow_run"]["id"] == RUN_ID
                and meta["workflow_run"]["head_sha"] == HEAD_SHA
                and meta["workflow_run"]["head_branch"] == BRANCH,
                f"hosted artifact metadata differs: {number}")
        with zipfile.ZipFile(archive) as packed:
            names = packed.namelist()
            require(len(names) == len(MEMBERS) and set(names) == MEMBERS
                    and all(not Path(member).is_absolute()
                            and ".." not in Path(member).parts
                            for member in names),
                    f"hosted artifact members differ: {number}")
            require(packed.read("download.exit_status.txt") == b"0\n"
                    and not packed.read("download.stderr.txt")
                    and packed.read("replay.exit_status.txt") == b"0\n"
                    and not packed.read("replay.stderr.txt"),
                    f"hosted download or replay failed: {number}")
            receipt = json.loads(packed.read("replay.json"))
            stdout = packed.read("replay.stdout.json")
        analysis_path = provider / f"stratum{number}" / "results" / "analysis.json"
        analysis_bytes = analysis_path.read_bytes()
        output = json.loads(stdout)
        analysis = json.loads(analysis_bytes)
        local_stdout = json.loads((provider / f"stratum{number}" /
                                   "local_replay.stdout.txt").read_text())
        require(output == analysis == local_stdout
                and sha256(analysis_path) == row["provider_analysis_sha256"]
                and canonical_hash(output) == receipt["canonical_analysis_sha256"],
                f"hosted, provider, and local analyses differ: {number}")
        expected = {
            "status": "PASS_INDEPENDENT_HOSTED_REPLAY",
            "stratum": number,
            "source_commit": dispatch["source_commit"],
            "provider_run_id": dispatch["provider_run_id"],
            "provider_run_attempt": dispatch["provider_run_attempt"],
            "provider_artifact_id": row["artifact_id"],
            "provider_zip_sha256": row["zip_sha256"],
            "provider_analysis_sha256": row["provider_analysis_sha256"],
            "dispatch_sha256": hashlib.sha256(dispatch_bytes).hexdigest(),
            "bound_result_members": row["result_members"],
            "runner_name": job_by_stratum[number]["runner_name"],
            "runner_os": "Linux",
            "independent_run_id": str(RUN_ID),
            "independent_run_attempt": "1",
            "independent_job": "replay-stratum",
            "selected_best_total_runtime": None,
        }
        require(receipt["schema"] ==
                "n83.crossfamily-independent-hosted-replay/v1"
                and all(receipt.get(key) == value
                        for key, value in expected.items()),
                f"hosted replay receipt differs: {number}")
        summaries.append({"stratum": number,
                          "job_id": job_by_stratum[number]["id"],
                          "runner_name": receipt["runner_name"],
                          "artifact_id": meta["id"],
                          "artifact_zip_sha256": zip_hash,
                          "provider_analysis_sha256": row["provider_analysis_sha256"],
                          "replay_stdout_sha256": hashlib.sha256(stdout).hexdigest(),
                          "canonical_analysis_sha256": canonical_hash(output)})
    require(validated["evidence_status"] == "PASS_INDEPENDENT_REPLAY"
            and len(validated["points"]) == 24,
            "prior local aggregate not admitted")
    return {"schema": "n83.crossfamily-independent-hosted-audit/v1",
            "status": "PASS_FOUR_INDEPENDENT_HOSTED_REPLAYS",
            "run_id": RUN_ID, "run_attempt": 1, "head_sha": HEAD_SHA,
            "dispatch_sha256": hashlib.sha256(dispatch_bytes).hexdigest(),
            "strata": summaries, "selected_best_total_runtime": None}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--hosted-root", type=Path, required=True)
    parser.add_argument("--provider-root", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = audit(args.hosted_root, args.provider_root)
    blob = json.dumps(result, indent=2, sort_keys=True) + "\n"
    if args.output.exists() and args.output.read_text() != blob:
        raise ValueError("sealed audit differs")
    args.output.write_text(blob)
    print(json.dumps({"status": result["status"],
                      "run_id": RUN_ID}, sort_keys=True))


if __name__ == "__main__":
    main()
