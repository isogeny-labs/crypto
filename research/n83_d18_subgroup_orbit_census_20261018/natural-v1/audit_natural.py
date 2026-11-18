#!/usr/bin/env python3
"""Audit the frozen U1,D1,D2,U2 Spot receipts without promoting censored searches."""
from __future__ import annotations

import hashlib
import json
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parent
ARMS = (
    ("u1", "U", "1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d", "564c6bb72b346b73112240bb0bddc4ef68b7525f7ef2556d6046e52c32f976e0"),
    ("d1", "D", "e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682", "8c1c245cda15bdd0019b487a5535a3b7eac642c259c4dc0bae25875d87d345b0"),
    ("d2", "D", "e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682", "8c1c245cda15bdd0019b487a5535a3b7eac642c259c4dc0bae25875d87d345b0"),
    ("u2", "U", "1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d", "564c6bb72b346b73112240bb0bddc4ef68b7525f7ef2556d6046e52c32f976e0"),
)
SOURCE = "fdf36e4b471ec65ea0c77be81d1b266fb417e7b5"
BINARY = "67a6e46a1477091f5c8db81b5932b742f4035de6d7397fd15866ead227f50c02"
IMAGE = "sha256:8ed3c0a996841537f75607e7d1de2114d8150391f75792e8da9268738547e73f"


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def read_json(path: Path):
    return json.loads(path.read_text())


def read_jsonl(path: Path) -> list[dict]:
    lines = path.read_text().splitlines()
    require(bool(lines), f"empty {path}")
    return [json.loads(line) for line in lines]


def utc(path: Path) -> datetime:
    value = path.read_text().strip()
    return datetime.fromisoformat(value.replace("Z", "+00:00")).astimezone(timezone.utc)


def audit_one(run: str, arm: str, object_hash: str, point_hash: str) -> dict:
    p = ROOT / "spot-arms" / run
    require((p / "arm.txt").read_text().strip() == arm, f"{run}: arm")
    require((p / "mode.txt").read_text().strip() == "natural", f"{run}: mode")
    start, end = utc(p / "start.utc"), utc(p / "end.utc")
    require(end > start, f"{run}: UTC interval")
    exit_code = int((p / "host-exit.txt").read_text().strip())
    inspect = read_json(p / "container-inspect.json")
    require(len(inspect) == 1, f"{run}: container inspect")
    container = inspect[0]
    state, config, host = container["State"], container["Config"], container["HostConfig"]
    require(state["ExitCode"] == exit_code and not state["OOMKilled"], f"{run}: exit/OOM")
    require(host["Memory"] == 4 * 1024**3 and host["MemorySwap"] == 4 * 1024**3, f"{run}: memory/swap")
    require(host["NanoCpus"] == 1_000_000_000 and host["CpusetCpus"] == "7", f"{run}: CPU quota")
    require(host["NetworkMode"] == "none" and host["ReadonlyRootfs"], f"{run}: isolation")
    require(host["CapDrop"] == ["ALL"] and "no-new-privileges" in host["SecurityOpt"] and host["PidsLimit"] == 256, f"{run}: container privileges")
    mounts = {entry["Destination"]: entry["RW"] for entry in container["Mounts"]}
    require(mounts == {"/opt/worker": False, "/inputs/base": False, "/inputs/evidence": False,
                       "/inputs/fixture": False, "/snapshot": False, "/out": True}, f"{run}: mount modes")
    require(config["Image"].endswith(IMAGE), f"{run}: image")
    phases = read_jsonl(p / "worker.json.phases.jsonl")
    progress = read_jsonl(p / "worker.json.progress.jsonl")
    for kind, rows in (("phase", phases), ("progress", progress)):
        require([row["sequence"] for row in rows] == list(range(len(rows))), f"{run}: {kind} sequence")
        require(all(x["elapsed_wall_ms"] <= y["elapsed_wall_ms"] for x, y in zip(rows, rows[1:])), f"{run}: {kind} clock")
        require(all(row["source_commit"] == SOURCE and row["object_blake3"] == object_hash and row["point_set_blake3"] == point_hash for row in rows), f"{run}: {kind} identity")
    named = {row["phase"]: row for row in phases}
    for phase in ("inputs_checked", "model_domain_done", "root_propagation_entered", "root_propagation_returned", "sat_entered"):
        require(phase in named, f"{run}: missing {phase}")
    require(named["inputs_checked"]["details"]["binary_sha256"] == BINARY, f"{run}: binary")
    require(named["root_propagation_returned"]["details"]["result"] == "Fixpoint", f"{run}: root result")
    model = named["model_domain_done"]["details"]
    require(model["variables"] == 201598 and model["xor_rows"] == 3901, f"{run}: model shape")
    require(model["clauses"] == 5915238, f"{run}: natural clauses")
    sample = progress[-1]
    require(sample["elapsed_wall_ms"] >= named["sat_entered"]["elapsed_wall_ms"], f"{run}: progress after search")
    cap = p / "worker.json.cap.json"
    worker = p / "worker.json"
    if cap.exists():
        receipt = read_json(cap)
        require(exit_code == 124 and not worker.exists(), f"{run}: capped exit/output")
        require(receipt["schema"] == "n83.paired-orbit-cap/v1" and receipt["status"] == "UNKNOWN_budget", f"{run}: cap status")
        require(receipt["arm"] == arm and receipt["mode"] == "natural" and receipt["budget_seconds"] == 1790, f"{run}: cap identity")
        require(1789000 <= receipt["elapsed_process_wall_ms"] <= 1800000, f"{run}: cap clock")
        require(receipt["selected_best_total_runtime"] is None, f"{run}: premature winner")
        status = "UNKNOWN_budget"
        process_cpu_s = None
        true_peak_bytes = None
    else:
        require(exit_code == 0 and worker.exists(), f"{run}: missing successful worker")
        receipt = read_json(worker)
        require(receipt["schema"] == "n83.paired-orbit-worker/v1" and receipt["arm"] == arm and receipt["mode"] == "natural", f"{run}: worker identity")
        require(receipt["source_commit"] == SOURCE and receipt["binary_sha256"] == BINARY, f"{run}: worker binary")
        require(receipt["object_blake3"] == object_hash and receipt["point_set_blake3"] == point_hash, f"{run}: worker base")
        require(receipt["candidate_id"] is None and receipt["selected_best_total_runtime"] is None, f"{run}: premature winner")
        status = receipt["status"]
        require(status in ("UNKNOWN_search_cap", "PASS_NATURAL_RELATION_PENDING_REPLAY", "UNVERIFIED_MODEL_UNSAT", "INVALID_MODEL", "INVALID_GROUP_LIFT", "INVALID_RELATION"), f"{run}: unexpected status {status}")
        process_cpu_s = receipt.get("process_usage")
        true_peak_bytes = receipt.get("memory_cgroup_peak_bytes")
    return {
        "arm": run, "base": arm, "status": status, "host_exit": exit_code,
        "start_utc": start.isoformat(), "end_utc": end.isoformat(),
        "charged_host_wall_s": (end - start).total_seconds(),
        "base_import_ms": named["inputs_checked"]["details"]["base_import_ms"],
        "target_validation_ms": named["inputs_checked"]["details"]["target_validation_ms"],
        "model_domain_ms": model["model_ms"],
        "root_propagation_ms": named["root_propagation_returned"]["details"]["root_ms"],
        "search_entered_ms": named["sat_entered"]["elapsed_wall_ms"],
        "last_durable_progress_ms": sample["elapsed_wall_ms"],
        "completed_decisions_lower_bound": sample["completed_decisions"],
        "completed_conflicts_lower_bound": sample["completed_conflicts"],
        "completed_xor_passes_lower_bound": sample["completed_xor_passes"],
        "max_sampled_memory_bytes": max(row["memory_current_bytes"] for row in progress),
        "true_peak_memory_bytes": true_peak_bytes,
        "process_cpu": process_cpu_s,
        "progress_samples": len(progress),
        "sat_variables": model["variables"], "sat_clauses": model["clauses"], "sat_xor_rows": model["xor_rows"],
        "container_oom_killed": state["OOMKilled"],
        "candidate_id": None, "selected_best_total_runtime": None,
    }


def audit_interruption() -> dict:
    p = ROOT / "spot-arms" / "u2"
    interruption = read_json(p / "interruption.json")
    instance = read_json(p / "instance-state.json")
    request = read_json(p / "spot-request.json")
    cleanup = read_json(p / "cloud-cleanup.json")
    require(interruption["schema"] == "n83.paired-orbit-interruption/v1" and interruption["status"] == "UNKNOWN_spot_interruption", "u2: interruption status")
    require(interruption["arm"] == "U" and interruption["mode"] == "natural", "u2: intended arm")
    require(interruption["ssh_exit_code"] == 255 and not interruption["durable_worker_start_observed"] and not interruption["worker_receipt_available"], "u2: SSH/worker outcome")
    require(instance["InstanceId"] == request["InstanceId"] == interruption["ec2_instance_id"], "u2: instance identity")
    require(request["SpotInstanceRequestId"] == interruption["spot_request_id"], "u2: spot request identity")
    require(instance["State"] == "terminated" and instance["InstanceLifecycle"] == "spot", "u2: EC2 state")
    require(request["State"] == "closed" and request["Status"]["Code"] == "instance-terminated-no-capacity", "u2: spot status")
    require(interruption["spot_status_code"] == request["Status"]["Code"], "u2: status mismatch")
    require("2026-10-19 06:45:32" in instance["StateTransitionReason"] and interruption["ec2_termination_time_utc"] == "2026-10-19T06:45:32Z", "u2: termination time")
    require(not any(p.glob("worker.json*")) and not (p / "host-exit.txt").exists(), "u2: unexpected worker evidence")
    require(interruption["candidate_id"] is None and interruption["selected_best_total_runtime"] is None, "u2: premature winner")
    require(cleanup["schema"] == "n83.paired-spot-cleanup/v1" and cleanup["instance_id"] == instance["InstanceId"] and cleanup["instance_state"] == "terminated", "u2: cleanup instance")
    require(cleanup["delete_key_pair_return"] and cleanup["post_delete_matching_key_pair_count"] == 0, "u2: key cleanup")
    require(cleanup["matching_network_interface_count_before_delete"] == 0 and cleanup["delete_security_group_return"] and cleanup["post_delete_matching_security_group_count"] == 0, "u2: security group cleanup")
    return {"arm": "u2", "base": "U", "status": "UNKNOWN_spot_interruption", "host_exit": None,
            "start_utc": None, "end_utc": interruption["ec2_termination_time_utc"],
            "charged_host_wall_s": None, "search_entered_ms": None,
            "last_durable_progress_ms": None, "completed_decisions_lower_bound": None,
            "completed_conflicts_lower_bound": None, "max_sampled_memory_bytes": None,
            "true_peak_memory_bytes": None, "process_cpu": None,
            "progress_samples": 0, "container_oom_killed": None,
            "candidate_id": None, "selected_best_total_runtime": None}


def main() -> None:
    rows = [audit_one(*spec) for spec in ARMS[:3]] + [audit_interruption()]
    require(all(x["end_utc"] <= y["start_utc"] for x, y in zip(rows[:3], rows[1:3])), "arm overlap/order")
    require(rows[2]["end_utc"] <= rows[3]["end_utc"], "D2 ended after Spot termination")
    raw = sorted(p for p in (ROOT / "spot-arms").iterdir() if p.is_dir() and p.name in {spec[0] for spec in ARMS})
    require(len(raw) == 4, "arm directory count")
    entries = []
    for directory in raw:
        for p in sorted(directory.iterdir()):
            require(p.is_file() and not p.name.endswith(".incoming"), f"unexpected raw entry {p}")
            entries.append(f"{hashlib.sha256(p.read_bytes()).hexdigest()}  {p.relative_to(ROOT)}")
    (ROOT / "natural-raw-files.sha256").write_text("\n".join(entries) + "\n")
    report = {"schema": "n83.paired-orbit-natural-audit/v1", "status": "AUDITED_INTERRUPTED", "source_commit": SOURCE,
              "binary_sha256": BINARY, "arms": rows, "raw_file_count": len(entries),
              "replicate_axis": "fresh run with same point and no intentional SAT seed variation",
              "candidate_id": None, "selected_best_total_runtime": None}
    (ROOT / "natural-audit.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"status": "AUDITED_INTERRUPTED", "raw_file_count": len(entries), "arms": [(r["arm"], r["status"]) for r in rows]}))


if __name__ == "__main__":
    main()
