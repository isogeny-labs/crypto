#!/usr/bin/env python3
"""Execute one sealed eight-point N83 finalist confirmation stratum."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import time

from research.n83_highk_online_distribution_20261020 import run as library
from research.n83_nearby_grid_confirm16_20261024 import protocol
from research.n83_nearby_grid_confirm16_20261024 import select
from research.n83_nearby_grid_tier1_20261024 import protocol as screen
from research.n83_nearby_grid_tier1_20261024 import run as screen_run
from research.n83_nearby_grid_tier1_20261024 import verify as screen_replay

parent = library.parent
telemetry = library.holdout_parent
parent.RSS_CAP_BYTES = protocol.RSS_CAP_BYTES


def canonical_digest(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":"),
                                     ensure_ascii=False).encode()).hexdigest()


def screen_selection(raw_screen):
    analysis = screen_replay.verify(raw_screen)
    choice = select.choose(analysis)
    return analysis, choice


def execute(binary, rho_binary, raw_screen, output, stratum):
    manifest, points = protocol.fixture()
    screen_run.fixture()
    if output.exists():
        raise ValueError("confirmation output already exists")
    output.mkdir(parents=True)
    analysis, choice = screen_selection(raw_screen)
    parent.write_json(output / "screen_analysis.json", analysis)
    parent.write_json(output / "screen_selection.json", choice)
    if choice["status"] != "PASS_FROZEN_SCREEN_SELECTION":
        parent.write_json(output / "preflight.json", {
            "status": "REJECTED_SCREEN_SELECTION", "screen_status": analysis["status"]})
        return 2
    selected_ks = choice["selected_ks"]
    schedule = protocol.events(stratum, selected_ks)
    binary, rho_binary = binary.resolve(strict=True), rho_binary.resolve(strict=True)
    executed = output / "executed_bins"
    executed.mkdir()
    ic_copy = executed / "koblitz_orbit_dlp_fast"
    rho_copy = executed / "n83_rho_optimization_eval"
    for source, target in ((binary, ic_copy), (rho_binary, rho_copy)):
        shutil.copy2(source, target)
        target.chmod(0o755)
    group_path = parent.cgroup_root()
    host = parent.host_receipt(group_path)
    host["schema"] = "n83.nearby-grid-confirm16-host/v1"
    host["parent_instance_id"] = host["instance_id"]
    host["instance_id"] = f"{host['instance_id']}-confirm16-stratum{stratum}"
    host["stratum"] = stratum
    host["topology"] = telemetry.topology_receipt(host["visible_affinity"] or [])
    host["cpu_telemetry_before"] = telemetry.cpu_telemetry()
    host["clock_ticks_per_second"] = os.sysconf("SC_CLK_TCK")
    parent.write_json(output / "host.json", host)
    parent.write_json(output / "source.json", {
        "schema": "n83.nearby-grid-confirm16-source/v1",
        "git_head": subprocess.check_output(["git", "rev-parse", "HEAD"],
                                            cwd=screen.ROOT, text=True).strip(),
        "runner_sha256": protocol.digest(Path(__file__)),
        "protocol_sha256": protocol.digest(Path(protocol.__file__)),
        "selector_sha256": protocol.digest(Path(select.__file__)),
        "screen_runner_sha256": protocol.digest(Path(screen_run.__file__)),
        "screen_verifier_sha256": protocol.digest(Path(screen_replay.__file__)),
        "screen_analysis_sha256": canonical_digest(analysis),
        "screen_selection_sha256": canonical_digest(choice),
        "screen_run_url": json.loads((raw_screen / "host.json").read_text()).get("run_url"),
        "screen_workload_id": screen.WORKLOAD_ID,
        "screen_catalog_sha256": screen.CATALOG_SHA256,
        "fixture_manifest_sha256": protocol.digest(protocol.HERE / "fixture_manifest.json"),
        "targets_sha256": protocol.digest(protocol.HERE / "targets.jsonl"),
        "answers_sha256": protocol.digest(protocol.HERE / "answers.jsonl"),
        "workloads_sha256": protocol.digest(protocol.HERE / "workloads.jsonl"),
        "exclusion_audit_sha256": protocol.digest(protocol.HERE / "exclusion_audit.json"),
        "ic_source_sha256": screen.SOURCE_SHA256,
        "rho_source_sha256": screen.RHO_SOURCE_SHA256,
        "cargo_lock_sha256": protocol.digest(screen.ROOT / "Cargo.lock"),
        "rho_config_sha256": protocol.digest(screen.RHO_CONFIG),
        "binary_sha256": protocol.digest(ic_copy),
        "rho_binary_sha256": protocol.digest(rho_copy),
        "selected_ks": selected_ks,
        "candidate_ids": choice["selected_candidate_ids"],
        "candidate_manifest_sha256": {str(k): screen.CANDIDATES[k][1]
                                      for k in selected_ks},
        "stratum": stratum, "events": schedule,
        "rho_seed_rule": "2026122500+point_index*selected_count+fixed_candidate_index",
        "caps": {"ic_seconds": protocol.IC_CAP_SECONDS,
                 "rho_seconds": protocol.RHO_CAP_SECONDS,
                 "global_seconds": protocol.GLOBAL_CAP_SECONDS,
                 "rss_bytes": protocol.RSS_CAP_BYTES},
        "aa_spread_limit": protocol.AA_SPREAD_LIMIT,
        "steal_fraction_cap": protocol.STEAL_FRACTION_CAP,
        "fixture_point_count": manifest["point_count"],
    })
    if not screen_run.host_gate(host, require_current_run=False):
        parent.write_json(output / "preflight.json", {
            "status": "REJECTED_HOST_ENVELOPE",
            "disk_free_bytes": host["disk_free_bytes"],
            "visible_affinity": host["visible_affinity"],
            "memory_max": host["cgroup_before"]["memory.max"]})
        return 2
    parent.write_json(output / "preflight.json", {"status": "PASS_HOST_ENVELOPE"})
    started = time.monotonic()
    completed = 0
    all_passed = True
    try:
        for event in schedule:
            index = event["point_index"]
            point_dir = output / "points" / f"p{index:02d}"
            point_dir.mkdir(parents=True, exist_ok=True)
            target_file = point_dir / "target.jsonl"
            if target_file.exists():
                if target_file.read_text() != points[index][0]:
                    raise ValueError("per-point public input differs")
            else:
                target_file.write_text(points[index][0])
            folder = point_dir / "runs" / event["name"]
            if event["kind"] == "rho":
                command = [str(rho_copy), str(target_file), str(event["seed"]),
                           str(screen.RHO_CONFIG), str(folder / "receipt.json")]
                environment = {"PATH": os.environ.get("PATH", "/usr/bin:/bin")}
                cap = protocol.RHO_CAP_SECONDS
            else:
                command = [str(ic_copy), f"construct:83:1:{event['k']}",
                           str(target_file), "29", str(folder / "targets_result.jsonl")]
                environment = {"PATH": os.environ.get("PATH", "/usr/bin:/bin"),
                               **screen_run.ic_flags(folder, event["k"])}
                cap = protocol.IC_CAP_SECONDS
            elapsed = time.monotonic() - started
            if elapsed + cap + 60 >= protocol.GLOBAL_CAP_SECONDS:
                parent.write_json(output / "producer_stop.json", {
                    "status": "GLOBAL_CAP_BEFORE_NEXT_EVENT", "next_event": event,
                    "elapsed_seconds": elapsed})
                break
            print(f"starting {event['name']} at {parent.now()}", flush=True)
            usage = telemetry.run_with_telemetry(folder, command, environment,
                                                 cap, group_path)
            print(f"{event['name']}: {usage['status']} "
                  f"{usage['wall_seconds']:.3f}s", flush=True)
            completed += 1
            if usage["status"] != "PASS_PRODUCER":
                all_passed = False
    finally:
        parent.write_json(output / "host_telemetry_after.json",
                          telemetry.cpu_telemetry())
        parent.write_json(output / "producer_summary.json", {
            "schema": "n83.nearby-grid-confirm16-producer-summary/v1",
            "completed_events": completed, "scheduled_events": len(schedule),
            "status": "ALL_CHILDREN_PASSED" if all_passed and
                      completed == len(schedule) else "INCOMPLETE"})
    return 0 if all_passed and completed == len(schedule) else 2


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--fixture-preflight", action="store_true")
    parser.add_argument("--screen-output", type=Path)
    parser.add_argument("--stratum", type=int, choices=range(protocol.STRATA))
    parser.add_argument("--binary", type=Path)
    parser.add_argument("--rho-binary", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    manifest, _ = protocol.fixture()
    if args.fixture_preflight:
        print(json.dumps({"status": "PASS_FROZEN_FIXTURE",
                          "point_count": manifest["point_count"],
                          "workload_ids": manifest["workload_ids"]}, sort_keys=True))
        return 0
    if (args.screen_output is None or args.stratum is None or
            args.binary is None or args.rho_binary is None or args.output is None):
        parser.error("screen output, stratum, binaries and output are required")
    return execute(args.binary, args.rho_binary, args.screen_output.resolve(),
                   args.output.resolve(), args.stratum)


if __name__ == "__main__":
    raise SystemExit(main())
