#!/usr/bin/env python3
"""Independently replay the fixed31 screen and apply its Tier 1 gate."""

import argparse
from datetime import datetime
import json
from pathlib import Path

from research.n83_highk_beyond4405_20261021 import run, runpod_runner, select
from research.n83_highk_online_distribution_20261020 import verify_common as common

HERE = Path(__file__).resolve().parent
common.RSS_CAP_BYTES = run.RSS_CAP_BYTES


def read(path: Path) -> dict:
    return json.loads(path.read_text())


def safe_digest(path: Path):
    try:
        return run.digest(path)
    except OSError:
        return None


def arm_tuple(arm: dict) -> tuple:
    return ("k{}_{}".format(arm["k"], "fixed31" if arm["fixed31"] else "load75"),
            arm["k"], "indexed5_id30", arm["candidate_id"])


def manifest(arm: tuple) -> tuple:
    path = HERE / "identities" / f"candidate_{arm[3]}.json"
    return path, read(path)


# The shared replay checks generic group arithmetic and output phases. Bind
# only its immutable manifest lookup to the four new source-bound candidates.
common.arms.manifest = manifest
common.arms.digest = run.digest


def suffix(path, length: int) -> tuple:
    return Path(path).parts[-length:]


def launch_ok(folder: Path, event: dict, output: Path, arm: dict) -> bool:
    try:
        launch = read(folder / "launch.json")
        command = launch["command"]
        if event["kind"] == "ic":
            expected = {key: value for key, value in
                        run.ic_environment(folder, arm).items()
                        if key.startswith("KIC_")}
            actual = dict(launch["environment_flags"])
            expected_dump = expected.pop("KIC_DUMP_BASE")
            actual_dump = actual.pop("KIC_DUMP_BASE")
            target = output / "screen" / f"block{event['block']}" / "targets.jsonl"
            return (
                command[1] == f"construct:83:1:{arm['k']}" and
                suffix(command[2], 3) == suffix(target, 3) and
                command[3] == "29" and
                suffix(command[4], 5) ==
                suffix(folder / "targets_result.jsonl", 5) and
                actual == expected and
                suffix(actual_dump, 5) == suffix(expected_dump, 5) and
                launch["cap_seconds"] == run.IC_CAP_SECONDS and
                launch["rss_cap_bytes"] == run.RSS_CAP_BYTES and
                suffix(command[0], 2) ==
                ("executed_bins", "koblitz_orbit_dlp_fast")
            )
        return (
            event["kind"] == "calibration" and
            suffix(command[1], 3) == suffix(run.cold.TARGETS, 3) and
            command[2] == str(run.cold.RHO_SEED) and
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


def layout_control(children: dict, output: Path) -> str:
    for block in range(2):
        names = [f"b{block}_k4405_load75", f"b{block}_k4405_fixed31"]
        a, b = (children.get(name, {}) for name in names)
        if (a.get("status") != "PASS_INDEPENDENT_REPLAY" or
                b.get("status") != "PASS_INDEPENDENT_REPLAY" or
                a.get("base_hash") != b.get("base_hash") or
                a.get("k") != b.get("k") or
                a.get("k") != 4405):
            return "FAILED_OR_INCOMPLETE"
        dirs = [output / "screen" / "runs" / name for name in names]
        if (safe_digest(dirs[0] / "base_dump.jsonl") is None or
                safe_digest(dirs[0] / "base_dump.jsonl") !=
                safe_digest(dirs[1] / "base_dump.jsonl")):
            return "BASE_MISMATCH"
        rows = [[json.loads(line) for line in
                 (folder / "targets_result.jsonl").read_text().splitlines()]
                for folder in dirs]
        if len(rows[0]) != 768 or len(rows[1]) != 768:
            return "TARGET_COVERAGE"
        for first, second in zip(*rows):
            if (first.get("point_indices") != second.get("point_indices") or
                    first.get("recovered_scalar") !=
                    second.get("recovered_scalar")):
                return "RELATION_OR_ANSWER_MISMATCH"
    return "PASS_IDENTICAL_BASE_ROWS_RANK_ANSWERS"


def verify(output: Path) -> int:
    dispatch, streams = run.fixture()
    arms = dispatch["arms"]
    ids = [arm["candidate_id"] for arm in arms]
    events = run.events(dispatch)
    host, source, preflight = (
        read(output / name) for name in ("host.json", "source.json", "preflight.json"))
    allocation_path = output / "runpod_allocation.json"
    allocation = runpod_runner.read_allocation(allocation_path)
    binary_match = (
        isinstance(source.get("binary_sha256"), str) and
        isinstance(source.get("rho_binary_sha256"), str) and
        safe_digest(output / "executed_bins/koblitz_orbit_dlp_fast") ==
        source.get("binary_sha256") and
        safe_digest(output / "executed_bins/n83_rho_optimization_eval") ==
        source.get("rho_binary_sha256"))
    source_ok = (
        host.get("schema") == "n83.fixed31-tier1-host/v1" and
        host.get("provider") == "runpod" and
        host.get("runpod_pod_id") == allocation["pod_id"] and
        host.get("runpod_machine_id") == allocation["machine_id"] and
        host.get("git_sha") == allocation["source_head_sha"] and
        host.get("visible_affinity") == allocation["cpu_affinity_ids"] and
        host.get("runpod_allocation_sha256") == run.digest(allocation_path) and
        source.get("schema") == "n83.fixed31-tier1-source/v1" and
        source.get("source_sha256") == dispatch["source_sha256"] and
        source.get("rho_source_sha256") ==
        run.digest(run.ROOT / "examples/n83_rho_optimization_eval.rs") and
        source.get("cargo_lock_sha256") == dispatch["cargo_lock_sha256"] and
        source.get("runner_sha256") == run.digest(HERE / "run.py") and
        source.get("dispatch_sha256") == run.digest(HERE / "dispatch.json") and
        source.get("runpod_adapter_sha256") ==
        run.digest(HERE / "runpod_runner.py") and
        source.get("runpod_allocation_sha256") ==
        run.digest(allocation_path) and
        source.get("screen_targets_sha256") ==
        dispatch["screen_targets_sha256"] and
        source.get("screen_events") == events and
        preflight.get("status") == "PASS_TIER1_CAPACITY" and
        preflight.get("rss_cap_bytes") == run.RSS_CAP_BYTES and
        common.snapshot_consistent(host.get("cgroup_before", {})) and
        binary_match and
        all((output / "screen" / f"block{block}" / "targets.jsonl").read_text() ==
            "".join(streams["screen"][block*768:(block+1)*768])
            for block in range(2))
    )
    points = [common.group.point_from_json(json.loads(line))
              for line in streams["screen"]]
    answers = [int(json.loads(line)["scalar"]) for line in
               (HERE / "screen_answers.jsonl").read_text().splitlines()]
    point_rows = {index: {"point_index": index, "arms": []}
                  for index in range(1536)}
    calibration_point = common.group.point_from_json(
        json.loads(run.cold.TARGETS.read_text()))
    calibration_scalar = int(read(run.cold.ANSWERS)["scalar"])
    if common.group.multiply(common.group.G, calibration_scalar) != calibration_point:
        raise ValueError("rho calibration fixture differs")
    rho_source = {"examples/n83_rho_optimization_eval.rs":
                  source["rho_source_sha256"]}
    children, cal_raw, telemetry_rows, launch_rows, finished = {}, {}, [], [], []
    snapshots = []
    for event in events:
        folder = output / "screen" / "runs" / event["name"]
        arm = next(arm for arm in arms
                   if arm["candidate_id"] == event["candidate_id"])
        launch_rows.append(launch_ok(folder, event, output, arm))
        telemetry_rows.append(common.telemetry(output / "screen", event["name"]))
        usage_path = folder / "usage.json"
        if usage_path.exists():
            usage = read(usage_path)
            finished.append(datetime.fromisoformat(usage["finished_utc"]))
            snapshots.append(all(common.snapshot_consistent(usage.get(name, {}))
                                 for name in ("cgroup_before", "cgroup_after")))
        else:
            snapshots.append(False)
        if event["kind"] == "ic":
            start = event["block"] * 768
            result = common.replay_ic(folder, arm_tuple(arm),
                                      points[start:start+768],
                                      answers[start:start+768])
            children[event["name"]] = result
            target_rows = result.get("targets", [])
            for local in range(768):
                row = target_rows[local] if len(target_rows) == 768 else {}
                point_rows[start+local]["arms"].append({
                    "candidate_id": arm["candidate_id"],
                    "status": row.get("status", result["status"]),
                    "resource_status": result.get("resources", {}).get("status"),
                    "one_target_online_ms": row.get("one_target_online_ms"),
                    "cold_process_ms": result.get("cold_process_ms"),
                    "screen_child": event["name"],
                })
        else:
            result = common.replay_rho(
                folder, calibration_point, calibration_scalar,
                int(run.cold.RHO_SEED), run.cold.TARGETS, rho_source)
            children[event["name"]] = result
            side = "before" if "_cal_before_" in event["name"] else "after"
            cal_raw.setdefault((event["block"], arm["candidate_id"], side), []).append(result)
    calibrations = {}
    for block in range(2):
        for arm in arms:
            candidate_id = arm["candidate_id"]
            calibrations[f"b{block}_{candidate_id}"] = common.calibration(
                cal_raw.get((block, candidate_id, "before"), []),
                cal_raw.get((block, candidate_id, "after"), []))
    bases = {arm["candidate_id"]: {
        children[f"b{block}_k{arm['k']}_{'fixed31' if arm['fixed31'] else 'load75'}"]
        .get("base_hash") for block in range(2)} for arm in arms}
    same_bases = all(len(values) == 1 and None not in values
                     for values in bases.values())
    order_ok = len(finished) == len(events) and all(
        first < second for first, second in zip(finished, finished[1:]))
    after_path = output / "screen" / "host_telemetry_after.json"
    chronology_ok = after_path.exists() and (
        runpod_runner.timestamp(allocation["created_utc"]) <=
        runpod_runner.timestamp(host["recorded_utc"]) <=
        runpod_runner.timestamp(read(after_path)["recorded_utc"]) <=
        runpod_runner.timestamp(allocation["stop_after_utc"])
    )
    telemetry_ok = (after_path.exists() and
                    bool(host.get("topology", {}).get("lscpu_json")) and
                    len(host.get("topology", {}).get("visible_cpu_topology", {})) == 4 and
                    all(row["status"] == "PASS" and
                        row["steal_jiffies_delta"] == 0
                        for row in telemetry_rows))
    replay_ok = source_ok and all(launch_rows) and same_bases and all(
        child.get("status") == "PASS_INDEPENDENT_REPLAY"
        for child in children.values())
    resource_ok = replay_ok and all(snapshots) and all(
        child.get("resources", {}).get("status") == "PASS"
        for child in children.values())
    aa_ok = all(row["status"] == "PASS_20_PERCENT"
                for row in calibrations.values())
    layout = layout_control(children, output)
    admitted = (resource_ok and aa_ok and order_ok and telemetry_ok and
                chronology_ok and
                layout == "PASS_IDENTICAL_BASE_ROWS_RANK_ANSWERS" and
                not (output / "producer_stop.json").exists())
    analysis = {
        "schema": "n83.fixed31-screen-analysis/v1",
        "status": "SCREEN_ADMITTED" if admitted else "BOUNDED_UNKNOWN",
        "source_identity_status": "PASS" if source_ok else "FAILED",
        "evidence_status": "PASS_INDEPENDENT_REPLAY" if replay_ok else
                           "VERIFICATION_FAILED",
        "resource_status": "PASS" if resource_ok else "FAILED_OR_INCOMPLETE",
        "aa_status": "PASS_20_PERCENT_PER_CHILD" if aa_ok else
                     "NOISY_OR_INCOMPLETE",
        "tier_status": "PASS_TIER1" if admitted else "NOT_ADMITTED",
        "order_status": "PASS" if order_ok else "INCOMPLETE_OR_OUT_OF_ORDER",
        "telemetry_status": "PASS" if telemetry_ok else "INCOMPLETE_OR_STEAL",
        "chronology_status": "PASS" if chronology_ok else "FAILED_OR_INCOMPLETE",
        "layout_control_status": layout,
        "children": children,
        "calibration_by_child": calibrations,
        "points": [point_rows[index] for index in range(1536)],
    }
    validated = {
        "schema": "n83.fixed31-screen-validated/v1",
        "source_sha256": dispatch["source_sha256"],
        "screen_targets_sha256": dispatch["screen_targets_sha256"],
        "candidate_ids": ids,
        "evidence_status": analysis["evidence_status"],
        "layout_control_status": layout,
        "block_candidate_orders": [ids, list(reversed(ids))],
        "host": {"tier_status": analysis["tier_status"],
                 "aa_status": analysis["aa_status"],
                 "point_indices": list(range(1536)),
                 "instance_id": host.get("instance_id")},
        "points": analysis["points"],
    }
    selection = select.select(validated, dispatch)
    for name, value in (("screen_analysis.json", analysis),
                        ("screen_validated.json", validated),
                        ("screen_selection.json", selection)):
        (output / name).write_text(json.dumps(value, indent=2,
                                              sort_keys=True) + "\n")
    print(json.dumps({"admission_status": selection["admission_status"],
                      "status": selection["status"],
                      "selected_candidate_id": selection["selected_candidate_id"]},
                     sort_keys=True))
    return 0 if admitted else 2


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    output = args.output.resolve()
    try:
        return verify(output)
    except Exception as error:
        if output.is_dir():
            (output / "screen_verify_error.json").write_text(json.dumps({
                "schema": "n83.fixed31-screen-verification-error/v1",
                "status": "VERIFICATION_FAILED",
                "error_type": type(error).__name__,
                "message": str(error)[:500],
            }, indent=2, sort_keys=True) + "\n")
        print(json.dumps({"status": "VERIFICATION_FAILED",
                          "error_type": type(error).__name__}, sort_keys=True))
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
