#!/usr/bin/env python3
"""Independently bind two hosted N83 SAT-cap jobs to archived source bytes."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import re
import stat
import zipfile

HERE = Path(__file__).resolve().parent
HEAD_SHA = "206adfd27888b4a8bc7d39a3f30695f5f21cfcac"
BRANCH = "codex/n83-hosted-regenerate-20261016"
WORKFLOW = ".github/workflows/n83-hosted-m6-highcap.yml"


def require(condition: bool, reason: str) -> None:
    if not condition:
        raise ValueError(reason)


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def read(path: Path) -> dict:
    return json.loads(path.read_text())


def members(archive: Path) -> dict[str, bytes]:
    with zipfile.ZipFile(archive) as packed:
        rows = packed.infolist()
        names = [row.filename for row in rows]
        require(len(names) == len(set(names))
                and all(not Path(name).is_absolute()
                        and ".." not in Path(name).parts
                        and not stat.S_ISLNK(row.external_attr >> 16)
                        for name, row in zip(names, rows)),
                "unsafe or duplicate hosted artifact member")
        return {name: packed.read(name) for name in names}


def check_stage(files: dict[str, bytes], stage: str, arm: dict,
                dispatch: dict) -> dict:
    prefix = f"results/{stage}"
    receipt = json.loads(files[f"{prefix}.stage.json"])
    stdout = files[f"{prefix}.stdout"]
    stderr = files[f"{prefix}.stderr"]
    require(receipt["schema"] == "n83.hosted-m6-paired-stage/v1"
            and receipt["role"] == arm["role"]
            and receipt["stage"] == stage
            and receipt["stdout_sha256"] == sha256(stdout)
            and receipt["stderr_sha256"] == sha256(stderr)
            and receipt["inspect_exit_code"] == 0
            and receipt["cleanup_exit_code"] == 0
            and receipt["wall_cap_seconds"] ==
            dispatch["import_cap_seconds" if stage == "import"
                     else "outer_search_cap_seconds"],
            f"{stage} stage receipt differs")
    command = receipt["command"]
    require(command[:4] == ["docker", "run", "--pull", "never"]
            and "--read-only" in command and "--cap-drop" in command
            and "--security-opt" in command,
            f"{stage} container command differs")
    for flag, value in (("--network", "none"), ("--cpus", "1"),
                        ("--memory", "4096m"), ("--memory-swap", "4096m"),
                        ("--pids-limit", "64"), ("--cap-drop", "ALL"),
                        ("--security-opt", "no-new-privileges")):
        require(command.count(flag) == 1
                and command[command.index(flag) + 1] == value,
                f"{stage} {flag} differs")
    expected = list(arm["worker_verify_argv" if stage == "import"
                        else "worker_search_argv"])
    if stage == "search":
        require(expected[1] == "run-cross-pilot"
                and expected[9:14] == ["2300", "20000", "64", "4", "2"],
                "frozen search source command differs")
        expected[1] = "run-cross-pilot-canonical"
        expected[9] = str(dispatch["worker_wall_cap_seconds"])
        expected[10] = str(dispatch["conflict_cap"])
    require(command[-len(expected):] == expected,
            f"{stage} native argv differs")
    inspected = json.loads(files[f"{prefix}.inspect.json"])
    require(len(inspected) == 1, f"{stage} Docker inspect differs")
    host = inspected[0]["HostConfig"]
    require(host["NanoCpus"] == 1_000_000_000
            and host["Memory"] == 4 * (1 << 30)
            and host["MemorySwap"] == 4 * (1 << 30)
            and host["PidsLimit"] == 64
            and host["NetworkMode"] == "none"
            and host["ReadonlyRootfs"] is True
            and host["CapDrop"] == ["ALL"]
            and host["SecurityOpt"] == ["no-new-privileges"],
            f"{stage} Docker resource envelope differs")
    return receipt


def audit(hosted: Path, provider_zip: Path) -> dict:
    dispatch_bytes = (HERE / "dispatch.json").read_bytes()
    dispatch = json.loads(dispatch_bytes)
    require(dispatch["schema"] == "n83.hosted-m6-highcap-dispatch/v1"
            and dispatch["selected_best_total_runtime"] is None,
            "frozen high-cap dispatch differs")
    provider = provider_zip.read_bytes()
    require(len(provider) == dispatch["provider_zip_bytes"]
            and sha256(provider) == dispatch["provider_zip_sha256"],
            "provider ZIP differs")
    source_plan = read(HERE.parent / "hosted-m6-paired-protocol-20261017/plan.json")
    require(sha256((HERE.parent / "hosted-m6-paired-protocol-20261017/plan.json").read_bytes())
            == dispatch["frozen_plan_sha256"],
            "source plan differs")
    arms = {row["role"]: row for row in source_plan["arms"]}
    require(set(arms) == set(dispatch["arms"]), "source arms differ")
    run = read(hosted / "run.json")
    jobs = read(hosted / "jobs.json")
    artifacts = read(hosted / "artifacts.json")
    run_id = run["id"]
    require(run["status"] == "completed"
            and run["run_attempt"] == 1
            and run["event"] == "push"
            and run["head_sha"] == HEAD_SHA
            and run["head_branch"] == BRANCH
            and run["path"] == WORKFLOW,
            "hosted run identity or completion differs")
    require(jobs["total_count"] == len(jobs["jobs"]) == 2
            and artifacts["total_count"] == len(artifacts["artifacts"]) == 2,
            "hosted job or artifact coverage differs")
    job_by_arm = {}
    for job in jobs["jobs"]:
        match = re.fullmatch(r"exposed-target-cap \((gray_pool|hash_pool)\)",
                             job["name"])
        require(match is not None and job["run_id"] == run_id
                and job["head_sha"] == HEAD_SHA
                and job["status"] == "completed"
                and job["runner_id"] > 0 and job["runner_name"],
                "hosted job identity differs")
        arm = match.group(1)
        require(arm not in job_by_arm, "duplicate hosted arm")
        job_by_arm[arm] = job
    require(set(job_by_arm) == set(dispatch["arms"])
            and len({job["runner_id"] for job in job_by_arm.values()}) == 2,
            "hosted arms not on distinct runners")
    artifact_by_name = {row["name"]: row for row in artifacts["artifacts"]}
    require(len(artifact_by_name) == 2, "duplicate hosted artifact name")
    summaries = []
    for arm_name in dispatch["arms"]:
        expected_name = f"n83-hosted-m6-highcap-{arm_name}-{run_id}-1"
        meta = artifact_by_name[expected_name]
        archive = hosted / f"{arm_name}.zip"
        archive_bytes = archive.read_bytes()
        require(meta["expired"] is False
                and meta["size_in_bytes"] == len(archive_bytes)
                and meta["digest"] == "sha256:" + sha256(archive_bytes)
                and meta["workflow_run"]["id"] == run_id
                and meta["workflow_run"]["head_sha"] == HEAD_SHA
                and meta["workflow_run"]["head_branch"] == BRANCH,
                "hosted artifact identity differs")
        files = members(archive)
        require(files["provider.zip"] == provider
                and files["source-commit.txt"].decode().strip() == HEAD_SHA
                and not files["source-status.txt"].strip(),
                "hosted provider or source bytes differ")
        inputs = json.loads(files["inputs/input.json"])
        require(inputs["schema"] == "n83.hosted-m6-highcap-input/v1"
                and inputs["status"] == "PASS_EXACT_ARCHIVED_INPUTS"
                and inputs["provider_artifact_id"] == dispatch["provider_artifact_id"]
                and inputs["provider_zip_sha256"] == dispatch["provider_zip_sha256"]
                and inputs["dispatch_sha256"] == sha256(dispatch_bytes)
                and json.loads(files["prepare.stdout.json"]) == inputs
                and not files["prepare.stderr.txt"],
                "hosted input preparation differs")
        require(set(inputs["files"]) == {"worker"} | {
            f"{key}.jsonl.gz" for key in dispatch["source_archive_sha256"]},
            "hosted extracted input coverage differs")
        for name, receipt in inputs["files"].items():
            blob = files[f"inputs/{name}"]
            require(len(blob) == receipt["bytes"]
                    and sha256(blob) == receipt["sha256"],
                    "hosted extracted worker or base differs")
        require(inputs["files"]["worker"]["sha256"] == dispatch["worker_sha256"]
                and all(inputs["files"][f"{key}.jsonl.gz"]["sha256"] == digest
                        for key, digest in dispatch["source_archive_sha256"].items()),
                "hosted source archive hash differs")
        preflight = json.loads(files["preflight.stdout.json"])
        require(preflight["status"] == "PASS_HIGHCAP_PREFLIGHT"
                and preflight["arm"] == arm_name
                and preflight["source_commit"] == HEAD_SHA
                and not files["preflight.stderr.txt"],
                "hosted source preflight differs")
        config = json.loads(files["results/config.json"])
        require(config["schema"] == "n83.hosted-m6-highcap-config/v1"
                and config["source_commit"] == HEAD_SHA
                and config["arm"] == arm_name
                and config["worker_sha256"] == dispatch["worker_sha256"]
                and config["dispatch_sha256"] == sha256(dispatch_bytes)
                and config["runner_name"] == job_by_arm[arm_name]["runner_name"]
                and config["run_id"] == str(run_id)
                and config["run_attempt"] == "1"
                and config["conservative_full_charge_seconds"] == dispatch["job_cap_seconds"]
                and config["selected_best_total_runtime"] is None,
                "hosted run config differs")
        arm = arms[arm_name]
        imported = check_stage(files, "import", arm, dispatch)
        require(imported["exit_code"] == 0 and imported["timed_out"] is False,
                "hosted import failed")
        import_result = json.loads(files["results/import.stdout"])
        require(import_result["status"] == "PASS"
                and import_result["small_object_blake3"] == arm["base_object_blake3"]
                and import_result["pool_object_blake3"] == arm["pool_object_blake3"],
                "hosted exact import differs")
        searched = check_stage(files, "search", arm, dispatch)
        outer = json.loads(files["results/outer.json"])
        require(outer["schema"] == "n83.hosted-m6-highcap-outer/v1"
                and outer["arm"] == arm_name
                and outer["source_commit"] == HEAD_SHA
                and outer["run_id"] == str(run_id)
                and outer["import"] == imported
                and outer["search"] == searched
                and outer["conservative_full_charge_seconds"] == dispatch["job_cap_seconds"]
                and outer["natural_partial_yield"] is None
                and outer["matrix_rank"] is None
                and outer["total_index_calculus_runtime_ms"] is None
                and outer["selected_best_total_runtime"] is None,
                "hosted outer receipt differs")
        if searched["timed_out"]:
            require(outer["status"] == "UNKNOWN_outer_wall_cap", "outer wall classification differs")
        elif searched["exit_code"] == 124:
            require(outer["status"] == "UNKNOWN_worker_wall_cap", "worker wall classification differs")
        else:
            require(searched["exit_code"] == 0 and searched["timed_out"] is False,
                    "hosted search failed")
            result = json.loads(files["results/search/result.json"])
            require(result["schema"] == "n83.cross-stream-pilot-partial-result/v2"
                    and result["ordering"] == "equal_domain_runs"
                    and result["base_object_blake3"] == arm["base_object_blake3"]
                    and result["pool_object_blake3"] == arm["pool_object_blake3"]
                    and result["search"]["conflicts"] <= dispatch["conflict_cap"]
                    and result["search"]["domain_clauses"] ==
                    arm["source_capacity"]["domain_clauses"]
                    and result["search"]["peak_xors"] ==
                    arm["source_capacity"]["xor_rows"]
                    and result["selected_best_total_runtime"] is None,
                    "hosted worker result differs")
            if result["status"] == "UNKNOWN_search_cap":
                require(result["search"]["conflicts"] == dispatch["conflict_cap"]
                        and 1 <= result["search"]["solver_calls"] <= 16
                        and 1 <= result["search"]["patterns_attempted"] <= 16
                        and result["search"]["models"] == 0
                        and result["candidate"] is None,
                        "capped search counters differ")
            expected_status = ("OBSERVED_MODEL_REQUIRES_INDEPENDENT_REPLAY"
                               if result["status"] == "PASS_one_bounded_partial"
                               else result["status"])
            require(outer["worker_status"] == result["status"]
                    and outer["status"] == expected_status,
                    "hosted worker status differs")
        require(job_by_arm[arm_name]["conclusion"] == "success",
                "hosted job failed")
        summaries.append({"arm": arm_name, "status": outer["status"],
                          "worker_status": outer["worker_status"],
                          "job_id": job_by_arm[arm_name]["id"],
                          "runner_name": job_by_arm[arm_name]["runner_name"],
                          "artifact_id": meta["id"],
                          "artifact_zip_sha256": sha256(archive_bytes)})
    require(run["conclusion"] == "success", "hosted run failed")
    status = ("BOUND_RECEIPTS_MODEL_REPLAY_REQUIRED"
              if any(row["status"] == "OBSERVED_MODEL_REQUIRES_INDEPENDENT_REPLAY"
                     for row in summaries)
              else "PASS_BOUND_TWO_HOSTED_SOLVER_RECEIPTS")
    return {"schema": "n83.hosted-m6-highcap-audit/v1",
            "status": status,
            "run_id": run_id, "run_attempt": 1, "head_sha": HEAD_SHA,
            "dispatch_sha256": sha256(dispatch_bytes),
            "arms": summaries,
            "conservative_full_charge_seconds": 2 * dispatch["job_cap_seconds"],
            "selected_best_total_runtime": None}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--hosted-root", type=Path, required=True)
    parser.add_argument("--provider-zip", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = audit(args.hosted_root, args.provider_zip)
    text = json.dumps(result, sort_keys=True, indent=2) + "\n"
    if args.output.exists():
        require(args.output.read_text() == text, "sealed audit output differs")
    else:
        args.output.write_text(text)
    print(json.dumps({"status": result["status"],
                      "run_id": result["run_id"]}, sort_keys=True))


if __name__ == "__main__":
    main()
