#!/usr/bin/env python3
"""Independent provider and raw-stream replay for one N83 setting stratum."""

import argparse
import hashlib
import json
from pathlib import Path
import re
import zipfile


HERE = Path(__file__).resolve().parent
PRIOR = HERE.parent / "natural-yield-panel-20261025"
ORDER = ("base", "vsids", "matrix48", "vsids_matrix48", "base_repeat")


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def sha(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def members(path: Path, expected_sha: str) -> dict[str, bytes]:
    raw = path.read_bytes()
    require(sha(raw) == expected_sha and len(expected_sha) == 64,
            "exact provider ZIP SHA-256")
    with zipfile.ZipFile(path) as bundle:
        names = bundle.namelist()
        require(len(names) == len(set(names))
                and all(not name.startswith("/")
                        and ".." not in Path(name).parts for name in names),
                "safe unique ZIP members")
        return {name: bundle.read(name) for name in names
                if not name.endswith("/")}


def one_prefix(files: dict[str, bytes], suffix: str) -> str:
    matches = [name[:-len(suffix)] for name in files if name.endswith(suffix)]
    require(len(matches) == 1, f"one {suffix} member")
    return matches[0]


def get(files: dict[str, bytes], prefix: str, name: str) -> bytes:
    require(prefix + name in files, f"ZIP member {prefix + name}")
    return files[prefix + name]


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


def flags(protocol: dict, setting: dict) -> list[str]:
    return ["--threads", "1", "--random", "0",
            "--branchstr", setting["branchstr"],
            "--autodisablegauss", "0", "--maxmatrixcols", "25000",
            "--maxnummatrices", str(setting["maxnummatrices"]),
            "--verb", "1", "--verbstat", "1", "--printsol", "1",
            "--maxconfl", str(protocol["conflict_cap"]),
            "--maxtime", str(protocol["solver_time_cap_seconds"])]


def audit(stratum: str, artifact: Path, artifact_sha: str,
          metadata_path: Path, jobs_path: Path, source_zip: Path,
          source_metadata_path: Path, solver_zip: Path) -> dict:
    protocol_raw = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_raw)
    rows = [row for row in protocol["strata"] if row["id"] == stratum]
    require(protocol["schema"] == "n83.solver-setting-screen-protocol/v1"
            and len(rows) == 1 and protocol["selected_best_total_runtime"] is None,
            "frozen stratum")
    row = rows[0]
    spec = protocol["formulas"][row["formula"]]
    current = json.loads(metadata_path.read_bytes())
    source_metadata = json.loads(source_metadata_path.read_bytes())
    jobs = json.loads(jobs_path.read_bytes())
    run_id = current["workflow_run"]["id"]
    attempt = int(current["name"].rsplit("-", 1)[1])
    head = current["workflow_run"]["head_sha"]
    require(current["name"] == f"n83-solver-setting-{stratum}-{run_id}-{attempt}"
            and current["size_in_bytes"] == artifact.stat().st_size
            and not current["expired"], "current provider artifact identity")
    require(jobs["total_count"] == 8 and len(jobs["jobs"]) == 8,
            "eight provider stratum jobs")
    selected = [job for job in jobs["jobs"]
                if job["name"] == f"solve-setting-stratum ({stratum})"]
    require(len(selected) == 1 and selected[0]["run_id"] == run_id
            and selected[0]["head_sha"] == head
            and selected[0]["status"] == "completed"
            and selected[0]["conclusion"] == "success",
            "selected terminal provider job")
    steps = {step["name"]: step["conclusion"] for step in selected[0]["steps"]}
    for name in ("Fetch exact prior XCNF artifact and pinned solver",
                 "Install qualified Linux solver dependencies",
                 "Repeat positive wide-XOR solver control",
                 "Bind one exact S3-backed formula from prior provider ZIP",
                 "Run paired bounded solver settings",
                 "Preserve exact streams and process receipts"):
        require(steps.get(name) == "success", f"provider stage {name}")
    files = members(artifact, artifact_sha)
    source_files = members(source_zip, row["source_artifact_sha256"])
    solver_files = members(solver_zip, protocol["solver_artifact_sha256"])
    meta_prefix = one_prefix(files, "input-provenance.json")
    result_prefix = one_prefix(files, "host.json")
    require(meta_prefix != result_prefix
            and all(name.startswith(meta_prefix) or name.startswith(result_prefix)
                    for name in files), "only setting metadata and results")
    require(source_metadata["id"] == row["source_artifact_id"]
            and source_metadata["workflow_run"]["id"] == protocol["source_run_id"]
            and source_metadata["workflow_run"]["head_sha"]
            == protocol["source_head_sha"]
            and source_metadata["size_in_bytes"] == source_zip.stat().st_size,
            "source provider artifact identity")
    require(sha(get(solver_files, "", "cryptominisat5"))
            == protocol["solver_sha256"], "exact solver bytes")
    audit_path = PRIOR / "run-40185426122-attempt-1" / f"audit-t{row['trial']}_m0.json"
    require(sha(audit_path.read_bytes()) == row["source_audit_json_sha256"]
            and sha((PRIOR / "protocol.json").read_bytes())
            == protocol["source_protocol_sha256"],
            "prior S3-backed producer audit and protocol")
    prior_audit = json.loads(audit_path.read_bytes())
    require(prior_audit["status"] == "PASS_HOSTED_STRATUM_REPLAY"
            and prior_audit["artifact_id"] == row["source_artifact_id"]
            and prior_audit["artifact_zip_sha256"]
            == row["source_artifact_sha256"], "source producer replay")
    member = f"n83-pattern-results/exports/{row['formula']}/instance.xcnf"
    require(sha(get(source_files, "", member)) == spec["sha256"]
            and len(get(source_files, "", member)) == spec["bytes"],
            "exact source-backed formula")
    provenance = json.loads(get(files, meta_prefix, "input-provenance.json"))
    require(provenance["status"] == "PASS_EXACT_PRIOR_S3_BACKED_XCNF"
            and provenance["source_artifact_id"] == row["source_artifact_id"]
            and provenance["source_artifact_zip_sha256"]
            == row["source_artifact_sha256"]
            and provenance["xcnf_sha256"] == spec["sha256"]
            and provenance["xcnf_bytes"] == spec["bytes"]
            and provenance["source_artifact_metadata_sha256"]
            == sha(get(files, meta_prefix, "source-artifact-metadata.json"))
            and json.loads(get(files, meta_prefix,
                               "source-artifact-metadata.json"))
            == source_metadata,
            "exact hosted input provenance")
    require(b"preflight.zip: OK" in get(files, meta_prefix,
                                         "preflight.sha256-check")
            and b"cryptominisat5: OK" in get(files, meta_prefix,
                                             "solver.sha256-check"),
            "exact solver ZIP and binary checksum stage")
    smoke = json.loads(get(files, meta_prefix, "smoke.json"))
    require(smoke["smoke_exit_code"] == 10
            and smoke["smoke_sat_line"] is True
            and smoke["solver_sha256"] == protocol["solver_sha256"]
            and sha(get(files, meta_prefix, "smoke.stdout"))
            == smoke["smoke_stdout_sha256"]
            and sha(get(files, meta_prefix, "smoke.stderr"))
            == smoke["smoke_stderr_sha256"],
            "positive solver control")
    host = json.loads(get(files, result_prefix, "host.json"))
    preflight = json.loads(get(files, result_prefix, "preflight.json"))
    require(host["github_run_id"] == str(run_id)
            and host["github_run_attempt"] == str(attempt)
            and host["github_sha"] == head
            and host["uname"]["machine"] in ("aarch64", "arm64")
            and preflight["status"] == "PASS_EXACT_INPUT_AND_SOLVER"
            and preflight["stratum"] == stratum
            and preflight["protocol_sha256"] == sha(protocol_raw)
            and preflight["xcnf_sha256"] == spec["sha256"]
            and preflight["solver_sha256"] == protocol["solver_sha256"]
            and host["selected_best_total_runtime"] is None
            and preflight["selected_best_total_runtime"] is None,
            "host and immediate search admission")
    summary = json.loads(get(files, result_prefix, "summary.json"))
    completed = summary["completed_arms"]
    require(completed and completed == list(ORDER[:len(completed)]),
            "frozen completed configuration prefix")
    seen = []
    for config_id in completed:
        prefix = result_prefix + config_id + "/"
        receipt = json.loads(get(files, prefix, "process.json"))
        stdout = get(files, prefix, "solver.stdout")
        stderr = get(files, prefix, "solver.stderr")
        status, conflicts = interpret(stdout, stderr, receipt)
        setting = protocol["configurations"][config_id]
        require(receipt["arm"] == {"id": config_id,
                                  "input": row["formula"],
                                  "configuration": setting,
                                  "stratum": stratum}
                and receipt["command"][1:-1] == flags(protocol, setting)
                and receipt["status"] == status
                and receipt["conflicts"] == conflicts
                and receipt["xcnf_sha256"] == spec["sha256"]
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
                f"{config_id}: raw solver process replay")
        seen.append({"configuration": config_id, "status": status,
                     "conflicts": conflicts,
                     "wall_seconds_L0": receipt["wall_seconds_L0"]})
    expected_stop = (seen[-1]["status"] if seen[-1]["status"] not in
                     ("SATISFIABLE", "UNSATISFIABLE", "INDETERMINATE")
                     else None)
    candidates = [{"configuration": item["configuration"],
                   "solver_status": item["status"]} for item in seen
                  if item["status"] in ("SATISFIABLE", "UNSATISFIABLE")]
    require(abs(summary["summed_solver_wall_seconds_L0"]
                - sum(item["wall_seconds_L0"] for item in seen)) < 1e-6
            and summary["selected_best_total_runtime"] is None
            and summary["stop_reason"] == expected_stop
            and summary["candidate_statuses"] == candidates,
            "bounded summary and process accounting")
    if expected_stop is None:
        require(len(seen) == len(ORDER)
                and summary["status"] == "COMPLETE_BOUNDED",
                "five completed bounded settings")
    else:
        require(summary["status"] == "STOPPED", "explicit stopped setting")
    status = ("PENDING_INDEPENDENT_GROUP_REPLAY" if any(
        item["status"] == "SATISFIABLE" for item in seen)
              else "PENDING_UNSAT_CERTIFICATE" if any(
                  item["status"] == "UNSATISFIABLE" for item in seen)
              else expected_stop or "PASS_FIVE_CENSORED_SETTINGS_REPLAY")
    return {"schema": "n83.solver-setting-provider-audit/v1",
            "status": status,
            "run_id": run_id, "attempt": attempt, "head_sha": head,
            "stratum": stratum, "artifact_id": current["id"],
            "artifact_zip_sha256": artifact_sha,
            "source_artifact_id": row["source_artifact_id"],
            "source_artifact_zip_sha256": row["source_artifact_sha256"],
            "arms": seen, "selected_best_total_runtime": None}


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--stratum", required=True)
    parser.add_argument("--artifact-zip", type=Path, required=True)
    parser.add_argument("--expected-zip-sha256", required=True)
    parser.add_argument("--metadata", type=Path, required=True)
    parser.add_argument("--jobs", type=Path, required=True)
    parser.add_argument("--source-zip", type=Path, required=True)
    parser.add_argument("--source-metadata", type=Path, required=True)
    parser.add_argument("--solver-zip", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(audit(args.stratum, args.artifact_zip,
                           args.expected_zip_sha256, args.metadata,
                           args.jobs, args.source_zip, args.source_metadata,
                           args.solver_zip), indent=2, sort_keys=True))
