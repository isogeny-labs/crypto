#!/usr/bin/env python3
"""Bind four N83 bridge GitHub jobs and raw archives before paired selection."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path, PurePosixPath
import stat
import zipfile

HEAD = "77c8ebe307c047c78a7f66f6a7f41a0efef60584"
BRANCH = "codex/n83-small-intermediate-bridge-20261018"
WORKFLOW = ".github/workflows/n83-small-intermediate-bridge.yml"
RUN_ID = 39096257170
HERE = Path(__file__).resolve().parent
REPO = HERE.parents[1]
S3_BUCKET = "crypto-autoresearcher"
S3_PREFIX = "factor-bases/icv1/etc/n83_small_intermediate_bridge_20261018/a1/provider-artifacts/"


def require(ok: bool, message: str) -> None:
    if not ok:
        raise ValueError(message)


def read(path: Path) -> dict:
    return json.loads(path.read_text())


def file_digest(path: Path, algorithm: str) -> str:
    digest = hashlib.new(algorithm)
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def sha256(path: Path) -> str:
    return file_digest(path, "sha256")


def audit(run_path: Path, jobs_path: Path, artifacts_path: Path,
          archives: list[Path], manifests: list[Path],
          analyses: list[Path], s3_downloads: list[Path],
          s3_gets: list[Path], s3_puts: list[Path],
          s3_inventory: Path) -> dict:
    require(all(len(rows) == 4 for rows in (archives, manifests, analyses,
                                            s3_downloads, s3_gets, s3_puts)),
            "four raw archives, replays and S3 receipts required")
    protocol = read(HERE / "protocol.json")
    workflow_path = REPO / WORKFLOW
    workflow_bytes = workflow_path.read_bytes()
    require(protocol["host_strata"] == 4
            and protocol["producer_cap_seconds"] == 16_200
            and protocol["source_hashes"][WORKFLOW] == sha256(workflow_path)
            and b"timeout-minutes: 330" in workflow_bytes,
            "frozen workflow or resource cap differs")
    run, jobs, artifacts = read(run_path), read(jobs_path), read(artifacts_path)
    require(run["id"] == RUN_ID and run["status"] == "completed"
            and run["conclusion"] == "success" and run["run_attempt"] == 1
            and run["event"] == "push" and run["head_sha"] == HEAD
            and run["head_branch"] == BRANCH and run["path"] == WORKFLOW,
            "completed provider run differs")
    require(jobs["total_count"] == len(jobs["jobs"]) == 4
            and artifacts["total_count"] == len(artifacts["artifacts"]) == 4,
            "provider job or artifact coverage differs")
    by_job = {row["name"]: row for row in jobs["jobs"]}
    by_artifact = {row["name"]: row for row in artifacts["artifacts"]}
    require(len(by_job) == len(by_artifact) == 4,
            "duplicate provider job or artifact name")
    summaries = []
    expected_inventory = {}
    runner_ids, starts, ends = set(), [], []
    for stratum in range(4):
        job = by_job[f"bridge-stratum ({stratum})"]
        artifact = by_artifact[
            f"n83-native-bridge-stratum-{stratum}-{RUN_ID}-1"]
        archive_path = archives[stratum]
        manifest = read(manifests[stratum])
        analysis_path = analyses[stratum]
        require(job["id"] > 0 and job["run_id"] == RUN_ID
                and job["head_sha"] == HEAD and job["status"] == "completed"
                and job["conclusion"] == "success"
                and job["runner_id"] > 0 and job["runner_name"],
                f"stratum {stratum} provider job differs")
        require(job["runner_id"] not in runner_ids,
                "provider runners are not distinct")
        runner_ids.add(job["runner_id"])
        starts.append(job["started_at"])
        ends.append(job["completed_at"])
        digest = sha256(archive_path)
        require(artifact["id"] > 0 and artifact["expired"] is False
                and artifact["size_in_bytes"] == archive_path.stat().st_size
                and artifact["digest"] == "sha256:" + digest
                and artifact["workflow_run"]["id"] == RUN_ID
                and artifact["workflow_run"]["head_sha"] == HEAD
                and artifact["workflow_run"]["head_branch"] == BRANCH,
                f"stratum {stratum} GitHub artifact differs")
        fresh = s3_downloads[stratum]
        get = read(s3_gets[stratum])
        put = read(s3_puts[stratum])
        key = S3_PREFIX + digest + ".zip"
        expected_etag = '"' + file_digest(archive_path, "md5") + '"'
        require(fresh.stat().st_size == archive_path.stat().st_size
                and sha256(fresh) == digest
                and get["ContentLength"] == fresh.stat().st_size
                and get["ContentType"] == "application/zip"
                and get["ETag"] == put["ETag"] == expected_etag
                and get["Metadata"] == {
                    "run-id": str(RUN_ID), "stratum": str(stratum),
                    "head-sha": HEAD,
                }
                and put["ChecksumType"] == "FULL_OBJECT"
                and put["ServerSideEncryption"] == "AES256",
                f"stratum {stratum} fresh S3 provider ZIP differs")
        expected_inventory[key] = (fresh.stat().st_size, expected_etag)
        require(manifest["schema"] == "n83.native-bridge-archive/v1"
                and manifest["artifact_sha256"] == digest
                and manifest["artifact_bytes"] == archive_path.stat().st_size
                and manifest["member_count"] == len(manifest["members"])
                and manifest["member_count"] >= 100,
                f"stratum {stratum} extracted archive manifest differs")
        member = next((row for row in manifest["members"]
                       if row["path"] == "results/analysis.json"), None)
        require(member is not None and member["sha256"] == sha256(analysis_path),
                f"stratum {stratum} analysis byte binding differs")
        with zipfile.ZipFile(archive_path) as zipped:
            infos = zipped.infolist()
            names = [info.filename for info in infos]
            require(len(names) == len(set(names))
                    and len(names) == manifest["member_count"]
                    and zipped.testzip() is None,
                    f"stratum {stratum} ZIP integrity differs")
            manifest_members = {row["path"]: row for row in manifest["members"]}
            require(len(manifest_members) == len(infos)
                    and set(manifest_members) == set(names),
                    f"stratum {stratum} member inventory differs")
            for info in infos:
                path = PurePosixPath(info.filename)
                require(not path.is_absolute() and ".." not in path.parts
                        and "\\" not in info.filename
                        and not stat.S_ISLNK(info.external_attr >> 16),
                        f"stratum {stratum} unsafe ZIP member")
                member_bytes = zipped.read(info)
                expected = manifest_members[info.filename]
                require(len(member_bytes) == expected["bytes"]
                        and hashlib.sha256(member_bytes).hexdigest() ==
                        expected["sha256"],
                        f"stratum {stratum} member {info.filename} differs")
            require(hashlib.sha256(zipped.read("results/analysis.json")).hexdigest()
                    == member["sha256"],
                    f"stratum {stratum} archived analysis differs")
        analysis = read(analysis_path)
        require(analysis["schema"] == "n83.native-bridge-stratum-analysis/v1"
                and analysis["status"] == "PASS_TIER1"
                and analysis["stratum"] == stratum
                and analysis["host"]["git_sha"] == HEAD
                and analysis["host"]["github_run_id"] == str(RUN_ID)
                and analysis["host"]["runner_name"] == job["runner_name"]
                and len(analysis["rows"]) == 16,
                f"stratum {stratum} replay analysis differs")
        summaries.append({"stratum": stratum, "job_id": job["id"],
                          "runner_id": job["runner_id"],
                          "artifact_id": artifact["id"],
                          "artifact_bytes": archive_path.stat().st_size,
                          "artifact_sha256": digest,
                          "s3_uri": "s3://" + S3_BUCKET + "/" + key,
                          "s3_etag": expected_etag,
                          "analysis_sha256": member["sha256"],
                          "point_count": 8, "candidate_point_rows": 16})
    require(max(starts) < min(ends),
            "the four provider allocations did not overlap")
    inventory = read(s3_inventory)
    listed = inventory.get("Contents", [])
    require(inventory["IsTruncated"] is False
            and inventory["KeyCount"] == len(expected_inventory)
            and {row["Key"]: (row["Size"], row["ETag"])
                 for row in listed} == expected_inventory,
            "provider S3 prefix inventory differs")
    return {"schema": "n83.native-bridge-provider-audit/v1",
            "status": "PASS_FOUR_BOUND_PROVIDER_ARCHIVES",
            "run_id": RUN_ID, "head_sha": HEAD,
            "run_sha256": sha256(run_path),
            "jobs_sha256": sha256(jobs_path),
            "artifacts_sha256": sha256(artifacts_path),
            "conservative_full_job_charge_seconds": 4 * 330 * 60,
            "conservative_full_producer_charge_seconds":
            4 * protocol["producer_cap_seconds"],
            "strata": summaries, "selected_best_total_runtime": None}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run", type=Path, required=True)
    parser.add_argument("--jobs", type=Path, required=True)
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--archives", type=Path, nargs=4, required=True)
    parser.add_argument("--manifests", type=Path, nargs=4, required=True)
    parser.add_argument("--analyses", type=Path, nargs=4, required=True)
    parser.add_argument("--s3-downloads", type=Path, nargs=4, required=True)
    parser.add_argument("--s3-gets", type=Path, nargs=4, required=True)
    parser.add_argument("--s3-puts", type=Path, nargs=4, required=True)
    parser.add_argument("--s3-inventory", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = audit(args.run, args.jobs, args.artifacts,
                   args.archives, args.manifests, args.analyses,
                   args.s3_downloads, args.s3_gets, args.s3_puts,
                   args.s3_inventory)
    text = json.dumps(result, sort_keys=True, indent=2) + "\n"
    if args.output.exists():
        require(args.output.read_text() == text, "sealed provider audit differs")
    else:
        args.output.write_text(text)
    print(json.dumps({"status": result["status"], "run_id": RUN_ID}))


if __name__ == "__main__":
    main()
