#!/usr/bin/env python3
"""Independent provider/ZIP/stream replay for one frozen N83 pattern stratum."""

import argparse
import hashlib
import json
from pathlib import Path
import re
import zipfile


HERE = Path(__file__).resolve().parent
INPUT_MANIFEST = HERE.parent / "linux-xor-complete-inputs-20261025/manifest.json"
WORKFLOW = HERE.parents[3] / ".github/workflows/n83-natural-yield-pattern-panel.yml"
HEAD = "fd9121107c40b159b13497c475f848ea9a21aa96"


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def sha(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def zip_files(path: Path, expected_sha: str) -> dict[str, bytes]:
    raw = path.read_bytes()
    require(len(expected_sha) == 64 and sha(raw) == expected_sha,
            "exact provider ZIP SHA-256")
    with zipfile.ZipFile(path) as bundle:
        names = bundle.namelist()
        require(len(names) == len(set(names))
                and all(not name.startswith("/") and ".." not in Path(name).parts
                        for name in names), "unique safe ZIP members")
        return {name: bundle.read(name) for name in names if not name.endswith("/")}


def files_prefix(files: dict[str, bytes], suffix: str) -> str:
    matches = [name[:-len(suffix)] for name in files if name.endswith(suffix)]
    require(len(matches) == 1, f"one {suffix} member")
    return matches[0]


def get(files: dict[str, bytes], prefix: str, name: str) -> bytes:
    require(prefix + name in files, f"ZIP member {prefix + name}")
    return files[prefix + name]


def provider_identity(metadata: dict, raw_zip: Path, run_id: int,
                      attempt: int, name: str) -> None:
    require(metadata["name"] == name
            and metadata["workflow_run"]["id"] == run_id
            and metadata["workflow_run"]["head_sha"] == HEAD
            and metadata["size_in_bytes"] == raw_zip.stat().st_size
            and not metadata["expired"], "provider artifact identity")


def interpret(stdout: bytes, stderr: bytes, receipt: dict) -> tuple[str, int | None]:
    statuses = re.findall(rb"(?m)^s (SATISFIABLE|UNSATISFIABLE|INDETERMINATE)\s*$", stdout)
    counts = re.findall(rb"(?m)^c conflicts\s+:\s+(\d+)\s*$", stdout)
    conflicts = int(counts[0]) if len(counts) == 1 else None
    if receipt["watchdog"]:
        return "WATCHDOG", conflicts
    if receipt["oom_kill_delta"] > 0:
        return "OOM", conflicts
    if stderr or len(statuses) != 1 or conflicts is None:
        return "MALFORMED", conflicts
    status = statuses[0].decode()
    if (status, receipt["exit_code"]) in (("SATISFIABLE", 10),
                                          ("UNSATISFIABLE", 20),
                                          ("INDETERMINATE", 15)):
        return status, conflicts
    return "MALFORMED", conflicts


def audit(stratum_id: str, artifact: Path, artifact_sha: str,
          metadata_path: Path, build_artifact: Path, build_sha: str,
          build_metadata_path: Path, jobs_path: Path) -> dict:
    protocol = json.loads((HERE / "protocol.json").read_bytes())
    manifest_raw = INPUT_MANIFEST.read_bytes()
    manifest = json.loads(manifest_raw)
    require(sha(manifest_raw) == protocol["input_manifest_sha256"],
            "pinned eleven-object manifest")
    strata = [row for row in protocol["strata"] if row["id"] == stratum_id]
    require(len(strata) == 1, "one frozen stratum")
    stratum = strata[0]
    metadata = json.loads(metadata_path.read_bytes())
    provider_artifact_id = metadata["id"]
    build_metadata = json.loads(build_metadata_path.read_bytes())
    jobs = json.loads(jobs_path.read_bytes())
    run_id = metadata["workflow_run"]["id"]
    attempt = int(metadata["name"].rsplit("-", 1)[1])
    provider_identity(metadata, artifact, run_id, attempt,
                      f"n83-natural-pattern-{stratum_id}-{run_id}-{attempt}")
    provider_identity(build_metadata, build_artifact, run_id, attempt,
                      f"n83-pattern-worker-{run_id}-{attempt}")
    require(jobs["total_count"] == 5 and len(jobs["jobs"]) == 5,
            "one build and four stratum jobs")
    matching = [job for job in jobs["jobs"] if job["name"] == f"solve-stratum ({stratum_id})"]
    builders = [job for job in jobs["jobs"] if job["name"] == "build-worker"]
    require(len(matching) == len(builders) == 1,
            "provider build and selected stratum jobs")
    for job in (matching[0], builders[0]):
        require(job["run_id"] == run_id and job["head_sha"] == HEAD
                and job["status"] == "completed"
                and job["conclusion"] == "success", "provider terminal job success")
    steps = {row["name"]: row["conclusion"] for row in matching[0]["steps"]}
    for stage in ("Download exact built worker",
                  "Fetch pinned solver from qualified preflight artifact",
                  "Repeat positive wide-XOR solver smoke",
                  "Fresh-fetch and independently replay all S3 inputs",
                  "Export and solve the frozen stratum",
                  "Preserve exact process streams and receipts"):
        require(steps.get(stage) == "success", f"provider stage {stage}")
    files = zip_files(artifact, artifact_sha)
    build_files = zip_files(build_artifact, build_sha)
    meta_prefix = files_prefix(files, "input-replay.json")
    result_prefix = files_prefix(files, "host.json")
    require(meta_prefix != result_prefix
            and all(name.startswith(meta_prefix) or name.startswith(result_prefix)
                    for name in files), "only pattern metadata and results")
    build_prefix = files_prefix(build_files, "build.json")
    require(all(name.startswith(build_prefix) for name in build_files),
            "only one worker-build payload")
    build = json.loads(get(build_files, build_prefix, "build.json"))
    worker = get(build_files, build_prefix, "worker")
    require(build["schema"] == "n83.natural-pattern-build/v1"
            and build["source_commit"] == protocol["worker_source_commit"]
            and build["worker_sha256"] == sha(worker)
            and build["build_wall_seconds_L0"] > 0
            and build["selected_best_total_runtime"] is None,
            "exact source-pinned build artifact")
    require(b"preflight.zip: OK" in get(files, meta_prefix, "preflight.sha256-check")
            and protocol["preflight_artifact_sha256"] in WORKFLOW.read_text(),
            "preflight ZIP pin checked")
    smoke = json.loads(get(files, meta_prefix, "smoke.json"))
    require(smoke["smoke_exit_code"] == 10 and smoke["smoke_sat_line"] is True
            and smoke["solver_sha256"] == protocol["solver_sha256"]
            and sha(get(files, meta_prefix, "smoke.stdout"))
            == smoke["smoke_stdout_sha256"]
            and sha(get(files, meta_prefix, "smoke.stderr"))
            == smoke["smoke_stderr_sha256"], "positive wide-XOR solver control")
    replay = json.loads(get(files, meta_prefix, "input-replay.json"))
    require(replay["status"] == "PASS_COMPLETE_INPUT_REPLAY"
            and replay["curve"] == protocol["curve"]
            and len(replay["objects"]) == 11
            and {tuple(row[key] for key in ("key", "bytes", "sha256"))
                 for row in replay["objects"]}
            == {tuple(row[key] for key in ("key", "bytes", "sha256"))
                for row in manifest["entries"]}, "fresh eleven-object S3 replay")
    host = json.loads(get(files, result_prefix, "host.json"))
    preflight = json.loads(get(files, result_prefix, "preflight.json"))
    require(host["schema"] == "n83.natural-pattern-host/v1"
            and host["github_run_id"] == str(run_id)
            and host["github_run_attempt"] == str(attempt)
            and host["github_sha"] == HEAD
            and host["uname"]["machine"] in ("aarch64", "arm64")
            and preflight["status"] == "PASS_EXACT_INPUTS_BINARIES_AND_STRATUM"
            and preflight["stratum"] == stratum_id
            and preflight["source_commit"] == build["source_commit"]
            and preflight["worker_sha256"] == build["worker_sha256"]
            and preflight["solver_sha256"] == protocol["solver_sha256"]
            and host["selected_best_total_runtime"] is None
            and preflight["selected_best_total_runtime"] is None,
            "host and immediate search gate")
    export_wall = 0.0
    for name in dict.fromkeys(row["formula"] for row in stratum["arms"]):
        prefix = result_prefix + f"exports/{name}/"
        spec = protocol["formulas"][name]
        xcnf = get(files, prefix, "instance.xcnf")
        metadata = json.loads(get(files, prefix, "export.json"))
        receipt = json.loads(get(files, prefix, "export-process.json"))
        require(sha(xcnf) == spec["sha256"] and len(xcnf) == spec["bytes"]
                and metadata["schema"] == "n83.cross-stream-pattern-xcnf/v2"
                and metadata["trial"] == spec["trial"]
                and metadata["source_id"] == spec["trial"] + 1
                and metadata["identity_mask"] == spec["identity_mask"]
                and metadata["coef_a"] == spec["coef_a"]
                and metadata["coef_b"] == spec["coef_b"]
                and metadata["variables"] == spec["variables"]
                and metadata["original_clauses"] == spec["original_clauses"]
                and metadata["xor_rows"] == spec["xor_rows"]
                and receipt["xcnf_sha256"] == spec["sha256"]
                and receipt["metadata_sha256"]
                == sha(get(files, prefix, "export.json"))
                and receipt["worker_sha256"] == build["worker_sha256"]
                and receipt["stdout_sha256"]
                == sha(get(files, prefix, "export.stdout"))
                and receipt["stderr_sha256"]
                == sha(get(files, prefix, "export.stderr"))
                and receipt["wall_seconds_L0"] >= 0,
                f"{name}: exact exported XCNF and process")
        export_wall += receipt["wall_seconds_L0"]
    summary = json.loads(get(files, result_prefix, "summary.json"))
    planned = stratum["arms"]
    completed = summary["completed_arms"]
    require(completed and completed == [row["id"] for row in planned[:len(completed)]],
            "frozen completed arm prefix")
    rows = []
    pending_sat_replay = False
    candidate_hashes = set()
    for arm in planned[:len(completed)]:
        prefix = result_prefix + arm["id"] + "/"
        receipt = json.loads(get(files, prefix, "process.json"))
        stdout = get(files, prefix, "solver.stdout")
        stderr = get(files, prefix, "solver.stderr")
        status, conflicts = interpret(stdout, stderr, receipt)
        expected_arm = dict(arm, input=arm["formula"],
                            trial=stratum["trial"],
                            identity_mask=stratum["identity_mask"])
        require(receipt["arm"] == expected_arm
                and receipt["status"] == status
                and receipt["conflicts"] == conflicts
                and receipt["xcnf_sha256"] == protocol["formulas"][arm["formula"]]["sha256"]
                and receipt["solver_sha256"] == protocol["solver_sha256"]
                and receipt["stdout_sha256"] == sha(stdout)
                and receipt["stderr_sha256"] == sha(stderr)
                and receipt["stdout_bytes"] == len(stdout)
                and receipt["stderr_bytes"] == len(stderr)
                and receipt["wall_seconds_L0"] >= 0
                and receipt["user_cpu_seconds"] >= 0
                and receipt["system_cpu_seconds"] >= 0
                and receipt["peak_rss_bytes_linux"] > 0
                and receipt["selected_best_total_runtime"] is None,
                f"{arm['id']}: raw process replay")
        if status == "SATISFIABLE":
            verified = json.loads(get(files, prefix, "verify-process.json"))
            require(verified["stdout_sha256"] == sha(get(files, prefix, "verify.stdout"))
                    and verified["stderr_sha256"] == sha(get(files, prefix, "verify.stderr"))
                    and verified["status"] in ("PASS_ONE_EXTERNAL_PARTIAL",
                                               "GROUP_REJECTED_EXTERNAL_MODEL"),
                    "complete SAT requires exact verifier receipt")
            if verified["status"] == "PASS_ONE_EXTERNAL_PARTIAL":
                candidate = verified["result"]["candidate"]
                require(candidate["source_id"] == stratum["trial"] + 1
                        and candidate["a"] == protocol["formulas"][arm["formula"]]["coef_a"]
                        and candidate["b"] == protocol["formulas"][arm["formula"]]["coef_b"],
                        "candidate trial and equation")
                candidate_hashes.add(sha(json.dumps(candidate, sort_keys=True,
                                                    separators=(",", ":")).encode()))
                pending_sat_replay = True
            else:
                require(summary["stop_reason"] == verified["status"],
                        "group-rejected model must stop stratum")
        rows.append({"arm": arm["id"], "status": status,
                     "conflicts": conflicts,
                     "wall_seconds_L0": receipt["wall_seconds_L0"]})
    require(abs(summary["summed_export_wall_seconds_L0"] - export_wall) < 1e-6
            and abs(summary["summed_solver_wall_seconds_L0"]
                - sum(row["wall_seconds_L0"] for row in rows)) < 1e-6
            and sorted(candidate_hashes)
            == summary["unique_group_checked_candidate_hashes"]
            and summary["selected_best_total_runtime"] is None,
            "summary accounting and candidate deduplication")
    if summary["status"] == "COMPLETE_BOUNDED":
        require(len(rows) == len(planned) and summary["stop_reason"] is None
                and all(row["status"] in ("SATISFIABLE", "UNSATISFIABLE",
                                                "INDETERMINATE") for row in rows),
                "complete frozen stratum")
    else:
        require(summary["status"] == "STOPPED" and summary["stop_reason"],
                "explicit stopped stratum")
    status = ("PENDING_INDEPENDENT_SAT_GROUP_REPLAY" if pending_sat_replay
              else "PASS_STOPPED_STRATUM_REPLAY" if summary["status"] == "STOPPED"
              else "PASS_HOSTED_STRATUM_REPLAY")
    return {"schema": "n83.natural-pattern-provider-audit/v1",
            "status": status,
            "run_id": run_id, "attempt": attempt, "stratum": stratum_id,
            "artifact_id": provider_artifact_id,
            "artifact_zip_sha256": artifact_sha,
            "build_artifact_id": build_metadata["id"],
            "build_artifact_zip_sha256": build_sha,
            "arms": rows,
            "unique_candidate_hashes": sorted(candidate_hashes),
            "selected_best_total_runtime": None}


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--stratum", required=True)
    parser.add_argument("--artifact-zip", type=Path, required=True)
    parser.add_argument("--expected-zip-sha256", required=True)
    parser.add_argument("--metadata", type=Path, required=True)
    parser.add_argument("--build-artifact-zip", type=Path, required=True)
    parser.add_argument("--expected-build-zip-sha256", required=True)
    parser.add_argument("--build-metadata", type=Path, required=True)
    parser.add_argument("--jobs", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(audit(args.stratum, args.artifact_zip,
                           args.expected_zip_sha256, args.metadata,
                           args.build_artifact_zip, args.expected_build_zip_sha256,
                           args.build_metadata, args.jobs), indent=2, sort_keys=True))
