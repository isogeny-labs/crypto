#!/usr/bin/env python3
"""Independently replay every N83 one-target IC/rho pair on the frozen Pod."""

import argparse
from datetime import datetime
import json
from pathlib import Path

from research.n83_highk_beyond4405_20261021 import holdout, run, runpod_runner
from research.n83_highk_beyond4405_20261021 import select as selector
from research.n83_highk_beyond4405_20261021 import verify_screen as screen

HERE = Path(__file__).resolve().parent
common = screen.common


def read(path: Path) -> dict:
    return json.loads(path.read_text())


def suffix(value, length: int) -> tuple:
    return Path(value).parts[-length:]


def launch_ok(folder: Path, event: dict, output: Path, arm: dict) -> bool:
    try:
        launch = read(folder / "launch.json")
        command = launch["command"]
        kind, index = event["kind"], event["point_index"]
        point_target = output / "holdout" / "points" / f"p{index:02d}" / "target.jsonl"
        if kind == "ic":
            flags = {key: value for key, value in
                     run.ic_environment(folder, arm).items()
                     if key.startswith("KIC_")}
            actual = dict(launch["environment_flags"])
            dump = actual.pop("KIC_DUMP_BASE")
            expected_dump = flags.pop("KIC_DUMP_BASE")
            return (
                command[1] == f"construct:83:1:{arm['k']}" and
                suffix(command[2], 4) == suffix(point_target, 4) and
                command[3] == "29" and
                suffix(command[4], 5) ==
                suffix(folder / "targets_result.jsonl", 5) and
                actual == flags and
                suffix(dump, 5) == suffix(expected_dump, 5) and
                launch["cap_seconds"] == run.IC_CAP_SECONDS and
                launch["rss_cap_bytes"] == run.RSS_CAP_BYTES and
                suffix(command[0], 2) ==
                ("executed_bins", "koblitz_orbit_dlp_fast")
            )
        target = (point_target if kind == "rho" else run.cold.TARGETS)
        seed = (holdout.RHO_SEED_BASE + index if kind == "rho"
                else int(run.cold.RHO_SEED))
        return (
            kind in ("rho", "calibration") and
            suffix(command[1], 4 if kind == "rho" else 3) ==
            suffix(target, 4 if kind == "rho" else 3) and
            command[2] == str(seed) and
            suffix(command[3], 3) == suffix(run.cold.RHO_CONFIG, 3) and
            suffix(command[4], 5) == suffix(folder / "receipt.json", 5) and
            launch["environment_flags"] == {} and
            launch["cap_seconds"] == run.RHO_CAP_SECONDS and
            launch["rss_cap_bytes"] == run.RSS_CAP_BYTES and
            suffix(command[0], 2) ==
            ("executed_bins", "n83_rho_optimization_eval")
        )
    except (KeyError, IndexError, OSError, ValueError, TypeError):
        return False


def verify(output: Path) -> int:
    dispatch, streams = run.fixture()
    workloads = read(HERE / "workloads.json")
    allocation_path = output / "runpod_allocation.json"
    allocation = runpod_runner.read_allocation(allocation_path)
    source = read(output / "source.json")
    host = read(output / "host.json")
    selection_path = output / "screen_selection.json"
    selection = read(selection_path)
    validated_screen = read(output / "screen_validated.json")
    screen_analysis = read(output / "screen_analysis.json")
    holdout_dir = output / "holdout"
    holdout_source = read(holdout_dir / "source.json")
    candidate_id = selection.get("selected_candidate_id")
    arm = next((arm for arm in dispatch["arms"]
                if arm["candidate_id"] == candidate_id and arm["fixed31"]), None)
    if arm is None:
        raise ValueError("selected fixed31 candidate differs")
    schedule = holdout.events(candidate_id)
    source_ok = (
        selection.get("status") == "SELECTED" and
        selection.get("admission_status") == "PASS" and
        selection == selector.select(validated_screen, dispatch) and
        screen_analysis.get("status") == "SCREEN_ADMITTED" and
        host.get("provider") == "runpod" and
        host.get("runpod_pod_id") == allocation["pod_id"] and
        host.get("runpod_machine_id") == allocation["machine_id"] and
        host.get("visible_affinity") == allocation["cpu_affinity_ids"] and
        source.get("source_sha256") == dispatch["source_sha256"] and
        source.get("workload_manifest_sha256") ==
        dispatch["workload_manifest_sha256"] and
        source.get("runpod_allocation_sha256") == run.digest(allocation_path) and
        holdout_source.get("schema") == "n83.fixed31-one-target-source/v1" and
        holdout_source.get("runner_sha256") == run.digest(HERE / "holdout.py") and
        holdout_source.get("candidate_id") == candidate_id and
        holdout_source.get("selection_sha256") == run.digest(selection_path) and
        holdout_source.get("screen_validated_sha256") ==
        run.digest(output / "screen_validated.json") and
        holdout_source.get("screen_analysis_sha256") ==
        run.digest(output / "screen_analysis.json") and
        holdout_source.get("workload_manifest_sha256") ==
        dispatch["workload_manifest_sha256"] and
        holdout_source.get("source_sha256") == dispatch["source_sha256"] and
        holdout_source.get("binary_sha256") == source.get("binary_sha256") and
        holdout_source.get("rho_binary_sha256") ==
        source.get("rho_binary_sha256") and
        holdout_source.get("targets_sha256") ==
        dispatch["holdout_targets_sha256"] and
        holdout_source.get("answers_sha256") ==
        run.digest(HERE / "holdout_answers.jsonl") and
        holdout_source.get("runpod_allocation_sha256") ==
        run.digest(allocation_path) and
        holdout_source.get("events") == schedule and
        run.digest(output / "executed_bins/koblitz_orbit_dlp_fast") ==
        source.get("binary_sha256") and
        run.digest(output / "executed_bins/n83_rho_optimization_eval") ==
        source.get("rho_binary_sha256")
    )
    points = [common.group.point_from_json(json.loads(line))
              for line in streams["holdout"]]
    scalars = [int(json.loads(line)["scalar"]) for line in
               (HERE / "holdout_answers.jsonl").read_text().splitlines()]
    calibration_point = common.group.point_from_json(
        json.loads(run.cold.TARGETS.read_text()))
    calibration_scalar = int(read(run.cold.ANSWERS)["scalar"])
    if common.group.multiply(common.group.G, calibration_scalar) != calibration_point:
        raise ValueError("calibration scalar differs")
    rho_source = {"examples/n83_rho_optimization_eval.rs":
                  source["rho_source_sha256"]}
    children = {}
    calibration_rows = {}
    telemetry_rows = {}
    launch_rows = {}
    snapshots = {}
    finished = {}
    for event in schedule:
        name, index, kind = event["name"], event["point_index"], event["kind"]
        folder = holdout_dir / "runs" / name
        launch_rows[name] = launch_ok(folder, event, output, arm)
        telemetry_rows[name] = common.telemetry(holdout_dir, name)
        usage_path = folder / "usage.json"
        if usage_path.exists():
            usage = read(usage_path)
            finished[name] = datetime.fromisoformat(usage["finished_utc"])
            snapshots[name] = all(
                common.snapshot_consistent(usage.get(key, {}))
                for key in ("cgroup_before", "cgroup_after"))
        else:
            snapshots[name] = False
        target = holdout_dir / "points" / f"p{index:02d}" / "target.jsonl"
        if kind == "ic":
            result = common.replay_ic(folder, screen.arm_tuple(arm),
                                      [points[index]], [scalars[index]])
        elif kind == "rho":
            result = common.replay_rho(
                folder, points[index], scalars[index],
                holdout.RHO_SEED_BASE + index, target, rho_source)
        else:
            result = common.replay_rho(
                folder, calibration_point, calibration_scalar,
                int(run.cold.RHO_SEED), run.cold.TARGETS, rho_source)
            calibration_rows.setdefault((index, event["side"]), []).append(result)
        children[name] = result
    rows = []
    for index in range(16):
        workload_id = workloads["holdout_one_target"][index]["workload_id"]
        ic_name, rho_name = f"p{index:02d}_ic", f"p{index:02d}_rho"
        point_events = [event for event in schedule
                        if event["point_index"] == index]
        point_names = [event["name"] for event in point_events]
        calibration = common.calibration(
            calibration_rows.get((index, "before"), []),
            calibration_rows.get((index, "after"), []))
        ic, rho = children[ic_name], children[rho_name]
        target_path = holdout_dir / "points" / f"p{index:02d}" / "target.jsonl"
        target_ok = (target_path.exists() and
                     target_path.read_text() == streams["holdout"][index])
        times = [finished.get(name) for name in point_names]
        order_ok = all(t is not None for t in times) and all(
            a < b for a, b in zip(times, times[1:]))
        per_child_ok = all(
            launch_rows[name] and snapshots[name] and
            telemetry_rows[name]["status"] == "PASS" and
            telemetry_rows[name]["steal_jiffies_delta"] == 0 and
            children[name].get("status") == "PASS_INDEPENDENT_REPLAY" and
            children[name].get("resources", {}).get("status") == "PASS"
            for name in point_names)
        ic_targets = ic.get("targets", [])
        online_ic = (ic_targets[0]["one_target_online_ms"]
                     if len(ic_targets) == 1 else None)
        online_rho = rho.get("online_ms")
        passed = (source_ok and target_ok and order_ok and per_child_ok and
                  calibration["status"] == "PASS_20_PERCENT" and
                  isinstance(online_ic, (int, float)) and online_ic > 0 and
                  isinstance(online_rho, (int, float)) and online_rho > 0)
        rows.append({
            "point_index": index,
            "candidate_id": candidate_id,
            "workload_id": workload_id,
            "run_id": candidate_id + "W" + workload_id + "R1",
            "public_target": json.loads(streams["holdout"][index]),
            "verified_scalar": str(scalars[index]) if passed else None,
            "scalar_replay_verified": bool(passed),
            "status": "PASS_VERIFIED_ONE_TARGET_TIER1" if passed else
                      "BOUNDED_UNKNOWN",
            "ic_online_ms": online_ic if passed else None,
            "rho_online_ms": online_rho if passed else None,
            "online_speedup": online_rho / online_ic if passed else None,
            "ic_online_phases_ms": (ic_targets[0]["online_phases_ms"]
                                    if passed else None),
            "ic_cold_process_ms": ic.get("cold_process_ms"),
            "calibration": calibration,
            "target_match": target_ok,
            "event_order": order_ok,
            "child_gate": per_child_ok,
            "ic_replay_status": ic.get("status"),
            "rho_replay_status": rho.get("status"),
            "screen_child_names": point_names,
        })
    after_path = holdout_dir / "host_telemetry_after.json"
    chronology_ok = after_path.exists() and (
        runpod_runner.timestamp(allocation["created_utc"]) <=
        runpod_runner.timestamp(read(holdout_dir / "host_telemetry_before.json")
                                ["recorded_utc"]) <=
        runpod_runner.timestamp(read(after_path)["recorded_utc"]) <=
        runpod_runner.timestamp(allocation["stop_after_utc"]))
    if not chronology_ok:
        for row in rows:
            row["status"] = "BOUNDED_UNKNOWN"
            row["ic_online_ms"] = None
            row["rho_online_ms"] = None
            row["online_speedup"] = None
            row["ic_online_phases_ms"] = None
            row["verified_scalar"] = None
            row["scalar_replay_verified"] = False
    analysis = {
        "schema": "n83.fixed31-one-target-analysis/v1",
        "candidate_id": candidate_id,
        "source_identity_status": "PASS" if source_ok else "FAILED",
        "chronology_status": "PASS" if chronology_ok else "FAILED_OR_INCOMPLETE",
        "primary_status": rows[0]["status"],
        "verified_replication_count": sum(
            row["status"] == "PASS_VERIFIED_ONE_TARGET_TIER1" for row in rows[1:]),
        "producer_stop": (read(holdout_dir / "producer_stop.json")
                          if (holdout_dir / "producer_stop.json").exists() else None),
        "rows": rows,
        "children": children,
    }
    (holdout_dir / "analysis.json").write_text(
        json.dumps(analysis, indent=2, sort_keys=True) + "\n")
    (holdout_dir / "primary_one_target.json").write_text(
        json.dumps(rows[0], indent=2, sort_keys=True) + "\n")
    print(json.dumps({"primary_status": rows[0]["status"],
                      "verified_replication_count":
                      analysis["verified_replication_count"],
                      "online_speedup": rows[0]["online_speedup"]},
                     sort_keys=True))
    return 0 if rows[0]["status"] == "PASS_VERIFIED_ONE_TARGET_TIER1" else 2


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    output = args.output.resolve()
    try:
        return verify(output)
    except Exception as error:
        folder = output / "holdout"
        if folder.is_dir():
            (folder / "verify_error.json").write_text(json.dumps({
                "schema": "n83.fixed31-one-target-verification-error/v1",
                "status": "VERIFICATION_FAILED",
                "error_type": type(error).__name__,
                "message": str(error)[:500],
            }, indent=2, sort_keys=True) + "\n")
        print(json.dumps({"status": "VERIFICATION_FAILED",
                          "error_type": type(error).__name__}, sort_keys=True))
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
