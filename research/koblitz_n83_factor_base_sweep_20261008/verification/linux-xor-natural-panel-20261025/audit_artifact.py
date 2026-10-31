#!/usr/bin/env python3
"""Independently replay a completed hosted N83 XOR-panel provider ZIP."""

import argparse
import hashlib
import json
from pathlib import Path
import zipfile


HERE = Path(__file__).resolve().parent
MANIFEST = HERE.parent / "linux-xor-complete-inputs-20261025/manifest.json"
WORKFLOW = HERE.parents[3] / ".github/workflows/n83-linux-xor-natural-panel.yml"
HEAD = "1b34b3ae6cd634356ab3c5e858ceba45a033ed22"
SOURCE = "acfcd8adfad8d45ddf9c5f799f9385120dce6cf4"


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def sha(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def exact_prefix(names: set[str], suffix: str) -> str:
    matches = [name[:-len(suffix)] for name in names if name.endswith(suffix)]
    require(len(matches) == 1, f"one {suffix} member")
    return matches[0]


def interpret(stdout: bytes, stderr: bytes, receipt: dict) -> tuple[str, int | None]:
    statuses = [line.split()[1].decode() for line in stdout.splitlines()
                if line.startswith(b"s ") and len(line.split()) == 2]
    conflicts = []
    for line in stdout.splitlines():
        if line.startswith(b"c conflicts ") and b":" in line:
            tail = line.rsplit(b":", 1)[1].strip()
            if tail.isdigit():
                conflicts.append(int(tail))
    count = conflicts[0] if len(conflicts) == 1 else None
    if receipt["watchdog"]:
        return "WATCHDOG", count
    if receipt["oom_kill_delta"] > 0:
        return "OOM", count
    if stderr or len(statuses) != 1 or count is None:
        return "MALFORMED", count
    status = statuses[0]
    if (status, receipt["exit_code"]) in (("SATISFIABLE", 10),
                                          ("UNSATISFIABLE", 20),
                                          ("INDETERMINATE", 15)):
        return status, count
    return "MALFORMED", count


def audit(archive: Path, expected_zip_sha256: str, metadata_path: Path,
          jobs_path: Path) -> dict:
    raw = archive.read_bytes()
    require(len(expected_zip_sha256) == 64 and sha(raw) == expected_zip_sha256,
            "exact provider ZIP SHA-256")
    metadata = json.loads(metadata_path.read_bytes())
    jobs = json.loads(jobs_path.read_bytes())
    protocol = json.loads((HERE / "protocol.json").read_bytes())
    manifest_raw = MANIFEST.read_bytes()
    manifest = json.loads(manifest_raw)
    run_id = metadata["workflow_run"]["id"]
    attempt = int(metadata["name"].rsplit("-", 1)[1])
    require(metadata["name"] == f"n83-linux-xor-natural-{run_id}-{attempt}"
            and metadata["workflow_run"]["head_sha"] == HEAD
            and metadata["size_in_bytes"] == len(raw)
            and not metadata["expired"], "provider artifact identity")
    require(jobs["total_count"] == 1 and len(jobs["jobs"]) == 1,
            "single provider job")
    job = jobs["jobs"][0]
    require(job["run_id"] == run_id and job["head_sha"] == HEAD
            and job["status"] == "completed" and job["conclusion"] == "success",
            "provider terminal success")
    stages = {step["name"]: step["conclusion"] for step in job["steps"]}
    for name in ("Record host and fetch exact preflight binaries",
                 "Install frozen Linux solver dependencies",
                 "Repeat positive wide-XOR solver smoke",
                 "Fresh-fetch and independently replay all S3 inputs",
                 "Qualify exact panel inputs and binaries",
                 "Execute frozen sequential natural-query panel",
                 "Preserve exact process streams and receipts"):
        require(stages.get(name) == "success", f"provider stage: {name}")
    with zipfile.ZipFile(archive) as bundle:
        names = bundle.namelist()
        require(len(names) == len(set(names))
                and all(not name.startswith("/") and ".." not in Path(name).parts
                        for name in names), "unique safe ZIP members")
        files = {name: bundle.read(name) for name in names if not name.endswith("/")}
    meta = exact_prefix(set(files), "protocol-commit.txt")
    result = exact_prefix(set(files), "host.json")
    require(meta != result and all(name.startswith(meta) or name.startswith(result)
                                   for name in files), "only panel metadata and results")

    def get(prefix: str, name: str) -> bytes:
        path = prefix + name
        require(path in files, f"artifact member {path}")
        return files[path]

    require(get(meta, "protocol-commit.txt").strip().decode() == HEAD
            and get(meta, "source-commit.txt").strip().decode() == SOURCE,
            "frozen source and protocol commits")
    require(b"preflight.zip: OK" in get(meta, "preflight.sha256-check")
            and protocol["preflight_artifact_sha256"] in WORKFLOW.read_text(),
            "exact preflight ZIP checked on host")
    binary_hashes = get(meta, "binaries.sha256").decode()
    require(protocol["solver_sha256"] in binary_hashes
            and protocol["worker_sha256"] in binary_hashes,
            "pinned solver and verifier bytes")
    smoke = json.loads(get(meta, "smoke.json"))
    require(smoke["schema"] == "n83.linux-xor-actions-preflight/v1"
            and smoke["solver_sha256"] == protocol["solver_sha256"]
            and smoke["smoke_exit_code"] == 10
            and smoke["smoke_sat_line"] is True
            and smoke["selected_best_total_runtime"] is None,
            "repeated positive wide-XOR smoke")
    for filename, field in (("smoke.stdout", "smoke_stdout_sha256"),
                            ("smoke.stderr", "smoke_stderr_sha256")):
        require(sha(get(meta, filename)) == smoke[field],
                f"raw positive-control stream {filename}")
    replay = json.loads(get(meta, "input-replay.json"))
    require(replay["status"] == "PASS_COMPLETE_INPUT_REPLAY"
            and replay["curve"] == protocol["curve"]
            and len(replay["objects"]) == 11
            and {tuple(row[key] for key in ("key", "bytes", "sha256"))
                 for row in replay["objects"]}
            == {tuple(row[key] for key in ("key", "bytes", "sha256"))
                for row in manifest["entries"]}
            and sha(manifest_raw) == protocol["input_manifest_sha256"],
            "complete eleven-object S3 input replay")
    admission = json.loads(get(meta, "panel-preflight.json"))
    require(admission["status"] == "PASS_EXACT_INPUTS_BINARIES_AND_ORDER"
            and admission["solver_sha256"] == protocol["solver_sha256"]
            and admission["worker_sha256"] == protocol["worker_sha256"]
            and admission["selected_best_total_runtime"] is None,
            "immediate pre-search gate")
    host = json.loads(get(result, "host.json"))
    require(host["schema"] == "n83.linux-xor-natural-host/v1"
            and host["github_run_id"] == str(run_id)
            and host["github_run_attempt"] == str(attempt)
            and host["github_sha"] == HEAD
            and host["uname"]["machine"] in ("aarch64", "arm64")
            and host["selected_best_total_runtime"] is None,
            "host and run identity")
    summary = json.loads(get(result, "summary.json"))
    planned = protocol["arms"]
    completed = summary["completed_arms"]
    require(completed and completed == [arm["id"] for arm in planned[:len(completed)]]
            and summary["selected_best_total_runtime"] is None,
            "frozen arm prefix and claim boundary")
    rows = []
    for arm in planned[:len(completed)]:
        prefix = result + arm["id"] + "/"
        receipt = json.loads(get(prefix, "process.json"))
        stdout = get(prefix, "solver.stdout")
        stderr = get(prefix, "solver.stderr")
        status, conflicts = interpret(stdout, stderr, receipt)
        require(receipt["schema"] == "n83.linux-xor-natural-arm/v1"
                and receipt["arm"] == arm
                and receipt["status"] == status
                and receipt["conflicts"] == conflicts
                and receipt["xcnf_sha256"] == protocol["inputs"][arm["input"]]
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
            verified = json.loads(get(prefix, "verify-process.json"))
            require(verified["status"] in (
                "PASS_ONE_EXTERNAL_PARTIAL", "GROUP_REJECTED_EXTERNAL_MODEL",
                "REJECTED_OR_INCOMPLETE_MODEL")
                and verified["stdout_sha256"] == sha(get(prefix, "verify.stdout"))
                and verified["stderr_sha256"] == sha(get(prefix, "verify.stderr"))
                and summary["stop_reason"] == verified["status"],
                "SAT model must be group-checked and stop")
        rows.append({"arm": arm["id"], "status": status,
                     "conflicts": conflicts,
                     "wall_seconds_L0": receipt["wall_seconds_L0"]})
    require(abs(summary["summed_solver_wall_seconds_L0"]
                - sum(row["wall_seconds_L0"] for row in rows)) < 1e-6,
            "summed solver wall")
    if summary["status"] == "COMPLETE_BOUNDED":
        require(len(rows) == len(planned)
                and summary["stop_reason"] is None
                and all(row["status"] in ("INDETERMINATE", "UNSATISFIABLE")
                        for row in rows), "complete censored panel")
    else:
        require(summary["status"] == "STOPPED" and summary["stop_reason"],
                "explicit stop reason")
    return {"schema": "n83.linux-xor-natural-panel-audit/v1",
            "status": "PASS_HOSTED_PANEL_REPLAY",
            "run_id": run_id, "attempt": attempt,
            "artifact_id": metadata["id"],
            "artifact_zip_sha256": expected_zip_sha256,
            "arms": rows,
            "selected_best_total_runtime": None}


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--artifact-zip", type=Path, required=True)
    parser.add_argument("--expected-zip-sha256", required=True)
    parser.add_argument("--metadata", type=Path, required=True)
    parser.add_argument("--jobs", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(audit(args.artifact_zip, args.expected_zip_sha256,
                           args.metadata, args.jobs), indent=2, sort_keys=True))
