#!/usr/bin/env python3
"""Independently replay every N83 one-target IC/rho pair on the frozen Pod."""

import argparse
from datetime import datetime
from decimal import Decimal
import json
import math
from pathlib import Path
import statistics

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
                     holdout.ic_environment(folder, arm).items()
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
        seed = (holdout.RHO_SEED_BASE + 2 * index +
                (0 if event["arm"] == "highk" else 1)
                if kind == "rho" else int(run.cold.RHO_SEED))
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


def cross_family_select(rows: list[dict], candidate_id: str) -> dict:
    result = {
        "schema": "n83.fixed31-cross-family-selector/v1",
        "status": "BOUNDED_UNKNOWN",
        "selected_candidate_id": None,
        "point_count": 16,
        "verified_pair_count": 0,
        "alpha": "0.05",
    }
    if len(rows) != 32:
        return result
    for index in range(16):
        highk, lower = rows[2 * index:2 * index + 2]
        if (highk.get("point_index") != index or
                lower.get("point_index") != index or
                highk.get("role") != "highk" or
                lower.get("role") != "k1107" or
                highk.get("candidate_id") != candidate_id or
                lower.get("candidate_id") != holdout.K1107_CANDIDATE_ID or
                highk.get("workload_id") != lower.get("workload_id")):
            return result
    paired = [rows[2 * index]["status"] == "PASS_VERIFIED_ONE_TARGET_TIER1" and
              rows[2 * index + 1]["status"] == "PASS_VERIFIED_ONE_TARGET_TIER1"
              for index in range(16)]
    result["verified_pair_count"] = sum(paired)
    if not all(paired):
        return result
    highk_times = [rows[2 * index]["ic_online_ms"] for index in range(16)]
    k1107_times = [rows[2 * index + 1]["ic_online_ms"]
                   for index in range(16)]
    if any(not isinstance(value, (int, float)) or isinstance(value, bool) or
           not math.isfinite(value) or value <= 0
           for value in highk_times + k1107_times):
        return result
    highk_wins = sum(a < b for a, b in zip(highk_times, k1107_times))
    k1107_wins = sum(b < a for a, b in zip(highk_times, k1107_times))
    p_value = selector.exact_two_sided_sign_p(highk_wins, k1107_wins)
    logs = [math.log(b) - math.log(a)
            for a, b in zip(highk_times, k1107_times)]
    mean_log = statistics.mean(logs)
    margin = 1.96 * statistics.stdev(logs) / math.sqrt(16)
    highk_geo = statistics.geometric_mean(highk_times)
    k1107_geo = statistics.geometric_mean(k1107_times)
    significant = p_value < Decimal("0.05")
    selected = (candidate_id if highk_wins > k1107_wins and
                highk_geo < k1107_geo else
                holdout.K1107_CANDIDATE_ID if k1107_wins > highk_wins and
                k1107_geo < highk_geo else None)
    descriptive = (candidate_id if highk_geo < k1107_geo else
                   holdout.K1107_CANDIDATE_ID if k1107_geo < highk_geo
                   else None)
    result.update({
        "status": "SELECTED" if significant and selected else "BOUNDED_UNKNOWN",
        "selected_candidate_id": selected if significant else None,
        "descriptive_fastest_candidate_id": descriptive,
        "highk_wins": highk_wins,
        "k1107_wins": k1107_wins,
        "ties": 16 - highk_wins - k1107_wins,
        "two_sided_exact_sign_p": str(p_value),
        "geometric_mean_online_ms": {
            candidate_id: highk_geo,
            holdout.K1107_CANDIDATE_ID: k1107_geo,
        },
        "mean_log_k1107_over_highk": mean_log,
        "normal_approx_95_mean_log_interval": [
            mean_log - margin, mean_log + margin],
        "paired_ic_online_ms": [list(pair) for pair in
                                zip(highk_times, k1107_times)],
    })
    return result


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
        holdout_source.get("schema") ==
        "n83.fixed31-cross-family-one-target-source/v2" and
        holdout_source.get("runner_sha256") == run.digest(HERE / "holdout.py") and
        holdout_source.get("candidate_id") == candidate_id and
        holdout_source.get("k1107_candidate_id") ==
        holdout.K1107_CANDIDATE_ID and
        holdout_source.get("k1107_manifest_sha256") ==
        holdout.K1107_MANIFEST_SHA256 and
        run.digest(HERE / "identities" /
                   f"candidate_{holdout.K1107_CANDIDATE_ID}.json") ==
        holdout.K1107_MANIFEST_SHA256 and
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
        holdout_source.get("rho_seed_rule") ==
        "2026120300+2*point_index+(0 highk,1 k1107)" and
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
        chosen = arm if event["arm"] == "highk" else holdout.K1107_ARM
        launch_rows[name] = launch_ok(folder, event, output, chosen)
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
            arm_tuple = (screen.arm_tuple(arm) if event["arm"] == "highk"
                         else ("k1107_root11", 1107, "root11_dynamic",
                               holdout.K1107_CANDIDATE_ID))
            result = common.replay_ic(folder, arm_tuple,
                                      [points[index]], [scalars[index]])
        elif kind == "rho":
            result = common.replay_rho(
                folder, points[index], scalars[index],
                holdout.RHO_SEED_BASE + 2 * index +
                (0 if event["arm"] == "highk" else 1),
                target, rho_source)
        else:
            result = common.replay_rho(
                folder, calibration_point, calibration_scalar,
                int(run.cold.RHO_SEED), run.cold.TARGETS, rho_source)
            calibration_rows.setdefault((index, event["arm"],
                                         event["side"]), []).append(result)
        children[name] = result
    rows = []
    for index in range(16):
        workload_id = workloads["holdout_one_target"][index]["workload_id"]
        point_events = [event for event in schedule
                        if event["point_index"] == index]
        point_names = [event["name"] for event in point_events]
        target_path = holdout_dir / "points" / f"p{index:02d}" / "target.jsonl"
        target_ok = (target_path.exists() and
                     target_path.read_text() == streams["holdout"][index])
        times = [finished.get(name) for name in point_names]
        order_ok = all(t is not None for t in times) and all(
            a < b for a, b in zip(times, times[1:]))
        for role, row_candidate_id in (("highk", candidate_id),
                                       ("k1107", holdout.K1107_CANDIDATE_ID)):
            arm_names = [event["name"] for event in point_events
                         if event["arm"] == role]
            ic = children[f"p{index:02d}_{role}_ic"]
            rho = children[f"p{index:02d}_{role}_rho"]
            calibration = common.calibration(
                calibration_rows.get((index, role, "before"), []),
                calibration_rows.get((index, role, "after"), []))
            per_child_ok = all(
                launch_rows[name] and snapshots[name] and
                telemetry_rows[name]["status"] == "PASS" and
                telemetry_rows[name]["steal_jiffies_delta"] == 0 and
                children[name].get("status") == "PASS_INDEPENDENT_REPLAY" and
                children[name].get("resources", {}).get("status") == "PASS"
                for name in arm_names)
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
                "role": role,
                "candidate_id": row_candidate_id,
                "workload_id": workload_id,
                "run_id": row_candidate_id + "W" + workload_id + "R1",
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
                "child_names": arm_names,
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
    paired = [rows[2 * index]["status"] == "PASS_VERIFIED_ONE_TARGET_TIER1" and
              rows[2 * index + 1]["status"] == "PASS_VERIFIED_ONE_TARGET_TIER1"
              for index in range(16)]
    primary = {
        "status": "PASS_VERIFIED_CROSS_FAMILY_TIER1" if paired[0]
                  else "BOUNDED_UNKNOWN",
        "point_index": 0,
        "public_target": rows[0]["public_target"],
        "workload_id": rows[0]["workload_id"],
        "highk": rows[0],
        "k1107": rows[1],
        "k1107_online_ms_over_highk_online_ms":
            rows[1]["ic_online_ms"] / rows[0]["ic_online_ms"]
            if paired[0] else None,
    }
    cross_family = cross_family_select(rows, candidate_id)
    analysis = {
        "schema": "n83.fixed31-cross-family-one-target-analysis/v2",
        "candidate_id": candidate_id,
        "k1107_candidate_id": holdout.K1107_CANDIDATE_ID,
        "source_identity_status": "PASS" if source_ok else "FAILED",
        "chronology_status": "PASS" if chronology_ok else "FAILED_OR_INCOMPLETE",
        "primary_status": primary["status"],
        "verified_replication_count": sum(paired[1:]),
        "cross_family_selector": cross_family,
        "producer_stop": (read(holdout_dir / "producer_stop.json")
                          if (holdout_dir / "producer_stop.json").exists() else None),
        "rows": rows,
        "children": children,
    }
    (holdout_dir / "analysis.json").write_text(
        json.dumps(analysis, indent=2, sort_keys=True) + "\n")
    (holdout_dir / "primary_one_target.json").write_text(
        json.dumps(rows[0], indent=2, sort_keys=True) + "\n")
    (holdout_dir / "cross_family_primary.json").write_text(
        json.dumps(primary, indent=2, sort_keys=True) + "\n")
    (holdout_dir / "cross_family_selection.json").write_text(
        json.dumps(cross_family, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"primary_status": primary["status"],
                      "verified_replication_count":
                      analysis["verified_replication_count"],
                      "k1107_online_ms_over_highk_online_ms":
                      primary["k1107_online_ms_over_highk_online_ms"]},
                     sort_keys=True))
    return 0 if paired[0] else 2


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
                "schema": "n83.fixed31-cross-family-verification-error/v2",
                "status": "VERIFICATION_FAILED",
                "error_type": type(error).__name__,
                "message": str(error)[:500],
            }, indent=2, sort_keys=True) + "\n")
        print(json.dumps({"status": "VERIFICATION_FAILED",
                          "error_type": type(error).__name__}, sort_keys=True))
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
