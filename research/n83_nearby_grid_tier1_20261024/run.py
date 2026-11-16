#!/usr/bin/env python3
"""Run two reverse-order, cold one-target IC/rho grid sweeps."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import time

from research.n83_highk_online_distribution_20261020 import run as library
from research.n83_indexed5_20261013 import analyze as group
from research.n83_online_finalists_20261018.run import ic_environment
from research.n83_nearby_grid_tier1_20261024 import protocol as spec

parent = library.parent
telemetry = library.holdout_parent
parent.RSS_CAP_BYTES = spec.RSS_CAP_BYTES


def manifest(k):
    candidate_id, expected_sha = spec.CANDIDATES[k]
    path = spec.GRID / "identities" / f"candidate_{candidate_id}.json"
    if parent.digest(path) != expected_sha:
        raise ValueError(f"K{k} candidate manifest changed")
    record = json.loads(path.read_text())
    if (record["candidate_id"] != candidate_id or
            record["implementation"]["source_sha256"]["ic"] !=
            spec.SOURCE_SHA256 or
            record["factor_base"]["actual_usable_points"] != 166 * k or
            record["factor_base"]["effective_columns"] != k):
        raise ValueError(f"K{k} manifest no longer describes frozen base")
    return record


def fixture():
    expected = ((spec.SOURCE, spec.SOURCE_SHA256),
                (spec.RHO_SOURCE, spec.RHO_SOURCE_SHA256),
                (spec.ROOT / "Cargo.lock", spec.LOCK_SHA256),
                (spec.TARGETS, spec.TARGETS_SHA256),
                (spec.ANSWERS, spec.ANSWERS_SHA256),
                (spec.FIXTURE_MANIFEST, spec.FIXTURE_MANIFEST_SHA256),
                (spec.EXCLUSION_AUDIT, spec.EXCLUSION_AUDIT_SHA256),
                (spec.WORKLOAD, spec.WORKLOAD_SHA256),
                (spec.GENERATOR, spec.GENERATOR_SHA256),
                (spec.RHO_CONFIG, spec.RHO_CONFIG_SHA256))
    for path, sha in expected:
        if parent.digest(path) != sha:
            raise ValueError(f"frozen input changed: {path}")
    point_line = spec.TARGETS.read_text()
    workload = json.loads(spec.WORKLOAD.read_text())
    frozen = json.loads(spec.FIXTURE_MANIFEST.read_text())
    audit = json.loads(spec.EXCLUSION_AUDIT.read_text())
    if (workload["workload_id"] != spec.WORKLOAD_ID or
            workload["target_stream_sha256"] != spec.ONE_TARGET_SHA256 or
            workload["target_domain"] != spec.TARGET_DOMAIN or
            workload["target_count"] != 1 or
            workload["curve_id"] != "EC1N83Ckb1a1haacb69ee9f37" or
            hashlib.sha256(json.dumps({key: value for key, value in workload.items()
                if key != "workload_id"}, sort_keys=True, separators=(",", ":"),
                ensure_ascii=False).encode()).hexdigest()[:12] != spec.WORKLOAD_ID or
            frozen["status"] != "SEALED_UNMEASURED" or
            frozen["domain"] != spec.TARGET_DOMAIN or
            frozen["target_sha256"] != spec.TARGETS_SHA256 or
            frozen["answer_sha256"] != spec.ANSWERS_SHA256 or
            frozen["exclusion_audit_sha256"] != spec.EXCLUSION_AUDIT_SHA256 or
            frozen["workload_sha256"] != spec.WORKLOAD_SHA256 or
            frozen["generator_sha256"] != spec.GENERATOR_SHA256 or
            audit["status"] != "PASS_ZERO_OVERLAP" or
            audit["overlap_count"] != 0 or
            audit["new_point"] != json.loads(point_line) or
            hashlib.sha256(point_line.encode()).hexdigest() !=
            spec.ONE_TARGET_SHA256):
        raise ValueError("independent one-target workload differs")
    answer = json.loads(spec.ANSWERS.read_text().splitlines()[0])
    point = group.point_from_json(json.loads(point_line))
    scalar = int(answer["scalar"])
    seed = hashlib.sha256(spec.TARGET_DOMAIN.encode() + (0).to_bytes(
        4, "little")).digest()
    expected_scalar = 1 + int.from_bytes(seed, "little") % (group.R - 1)
    if (answer["fixture_index"] != 0 or answer["seed_sha256"] != seed.hex() or
            scalar != expected_scalar or
            workload["target_generation_seed_sha256"] != seed.hex() or
            workload["target_point"] != json.loads(point_line) or
            not 0 < scalar < group.R or
            group.multiply(group.G, scalar) != point):
        raise ValueError("independent point fails scalar replay")
    for k in spec.CANDIDATES:
        manifest(k)
    return point_line, point, scalar


def ic_flags(folder, k):
    flags = {key: value for key, value in
             ic_environment(folder, "root11_dynamic", "k1107_root11").items()
             if key.startswith("KIC_")}
    published = json.loads((spec.ROOT /
        "research/n83_nearby_k_current_source_20261021/single/k1107/launch.json"
        ).read_text())["flags"]
    expected = {key: value for key, value in published.items()
                if key != "KIC_DUMP_BASE"}
    if ({key: value for key, value in flags.items()
         if key != "KIC_DUMP_BASE"} != expected or
            flags["KIC_DUMP_BASE"] != str(folder / "base_dump.jsonl") or
            manifest(k)["implementation"]["algorithm_flags"] !=
            manifest(spec.BASELINE_K)["implementation"]["algorithm_flags"]):
        raise ValueError(f"K{k} execution flags differ from current-source candidate")
    return flags


def host_gate(host, *, require_current_run=True):
    raw = (host["cgroup_before"]["memory.max"] or "").strip()
    limit = int(raw) if raw.isdigit() else host["mem_total_kib"] * 1024
    cpu = host["visible_affinity"] or []
    return (host["provider"] == "github_actions" and
            isinstance(host.get("git_sha"), str) and
            len(host["git_sha"]) == 40 and
            (not require_current_run or
             host["git_sha"] == os.environ.get("GITHUB_SHA")) and
            len(cpu) >= 2 and limit >= spec.MIN_MEMORY_BYTES and
            host["disk_free_bytes"] >= spec.MIN_DISK_BYTES and
            bool(host["cgroup_before"]["cpu.stat"]) and
            bool(host["cgroup_before"]["memory.events"]))


def execute(binary, rho_binary, output):
    point_line, _point, _scalar = fixture()
    binary, rho_binary = binary.resolve(strict=True), rho_binary.resolve(strict=True)
    if output.exists():
        raise ValueError("output already exists")
    output.mkdir(parents=True)
    bins = output / "executed_bins"
    bins.mkdir()
    ic_copy, rho_copy = bins / "koblitz_orbit_dlp_fast", bins / "n83_rho_optimization_eval"
    shutil.copy2(binary, ic_copy)
    shutil.copy2(rho_binary, rho_copy)
    for path in (ic_copy, rho_copy):
        path.chmod(0o755)
    target = output / "target.jsonl"
    target.write_text(point_line)
    group_path = parent.cgroup_root()
    host = parent.host_receipt(group_path)
    host["schema"] = "n83.nearby-grid-tier1-host/v1"
    host["topology"] = telemetry.topology_receipt(host["visible_affinity"] or [])
    host["cpu_telemetry_before"] = telemetry.cpu_telemetry()
    parent.write_json(output / "host.json", host)
    schedule = spec.events()
    parent.write_json(output / "source.json", {
        "schema": "n83.nearby-grid-tier1-source/v1",
        "git_head": subprocess.check_output(["git", "rev-parse", "HEAD"],
                                            cwd=spec.ROOT, text=True).strip(),
        "runner_sha256": parent.digest(Path(__file__)),
        "protocol_sha256": parent.digest(Path(spec.__file__)),
        "ic_source_sha256": spec.SOURCE_SHA256,
        "rho_source_sha256": spec.RHO_SOURCE_SHA256,
        "binary_sha256": parent.digest(ic_copy),
        "rho_binary_sha256": parent.digest(rho_copy),
        "target_sha256": parent.digest(target),
        "workload_id": spec.WORKLOAD_ID,
        "fixture_target_sha256": spec.TARGETS_SHA256,
        "answer_sidecar_sha256": spec.ANSWERS_SHA256,
        "fixture_manifest_sha256": spec.FIXTURE_MANIFEST_SHA256,
        "exclusion_audit_sha256": spec.EXCLUSION_AUDIT_SHA256,
        "workload_sha256": spec.WORKLOAD_SHA256,
        "generator_sha256": spec.GENERATOR_SHA256,
        "catalog_sha256": spec.CATALOG_SHA256,
        "rho_config_sha256": spec.RHO_CONFIG_SHA256,
        "candidate_ids": {str(k): value[0] for k, value in spec.CANDIDATES.items()},
        "candidate_manifest_sha256": {str(k): value[1] for k, value in spec.CANDIDATES.items()},
        "events": schedule,
        "caps": {"ic_seconds": spec.IC_CAP_SECONDS,
                 "rho_seconds": spec.RHO_CAP_SECONDS,
                 "global_seconds": spec.GLOBAL_CAP_SECONDS,
                 "rss_bytes": spec.RSS_CAP_BYTES},
    })
    if not host_gate(host):
        parent.write_json(output / "preflight.json", {
            "status": "REJECTED_HOST_ENVELOPE", "memory_max":
            host["cgroup_before"]["memory.max"],
            "visible_affinity": host["visible_affinity"],
            "disk_free_bytes": host["disk_free_bytes"]})
        return 2
    parent.write_json(output / "preflight.json", {"status": "PASS_HOST_ENVELOPE"})
    began = time.monotonic()
    completed = 0
    all_passed = True
    try:
        for event in schedule:
            cap = (spec.IC_CAP_SECONDS if event["kind"] == "ic"
                   else spec.RHO_CAP_SECONDS)
            if time.monotonic() - began + cap + 60 >= spec.GLOBAL_CAP_SECONDS:
                parent.write_json(output / "producer_stop.json", {
                    "status": "GLOBAL_CAP_BEFORE_NEXT_EVENT", "next_event": event})
                break
            folder = output / "runs" / event["name"]
            if event["kind"] == "ic":
                command = [str(ic_copy), f"construct:83:1:{event['k']}",
                           str(target), "29", str(folder / "targets_result.jsonl")]
                env = {"PATH": os.environ.get("PATH", "/usr/bin:/bin"),
                       **ic_flags(folder, event["k"])}
            else:
                command = [str(rho_copy), str(target), str(event["seed"]),
                           str(spec.RHO_CONFIG), str(folder / "receipt.json")]
                env = {"PATH": os.environ.get("PATH", "/usr/bin:/bin")}
            print(f"starting {event['name']} at {parent.now()}", flush=True)
            usage = telemetry.run_with_telemetry(folder, command, env, cap,
                                                 group_path)
            print(f"{event['name']}: {usage['status']} "
                  f"{usage['wall_seconds']:.3f}s", flush=True)
            completed += 1
            if usage["status"] != "PASS_PRODUCER":
                all_passed = False
                parent.write_json(output / "producer_stop.json", {
                    "status": "CHILD_FAILED", "event": event,
                    "child_status": usage["status"]})
                break
    finally:
        parent.write_json(output / "host_telemetry_after.json",
                          telemetry.cpu_telemetry())
        parent.write_json(output / "producer_summary.json", {
            "schema": "n83.nearby-grid-tier1-producer-summary/v1",
            "completed_events": completed, "scheduled_events": len(schedule),
            "status": "ALL_CHILDREN_PASSED" if all_passed and
                      completed == len(schedule)
                      else "INCOMPLETE"})
    return 0 if all_passed and completed == len(schedule) else 2


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--preflight", action="store_true")
    parser.add_argument("--binary", type=Path)
    parser.add_argument("--rho-binary", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    fixture()
    if args.preflight:
        print(json.dumps({"status": "PASS_FROZEN_INPUTS", "events":
                          len(spec.events()), "primary_point_index": 0,
                          "candidate_ids": [v[0] for v in spec.CANDIDATES.values()]},
                         sort_keys=True))
        return 0
    if not args.binary or not args.rho_binary or not args.output:
        parser.error("--binary, --rho-binary and --output are required")
    return execute(args.binary, args.rho_binary, args.output.resolve())


if __name__ == "__main__":
    raise SystemExit(main())
