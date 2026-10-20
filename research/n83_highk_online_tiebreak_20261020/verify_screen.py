#!/usr/bin/env python3
"""Independently replay the conditional 1,536-point N83 tie-break."""

import argparse
import json
from datetime import datetime
from pathlib import Path

from research.n83_highk_online_distribution_20261020 import arms
from research.n83_highk_online_distribution_20261020 import verify_common as common
from research.n83_highk_online_tiebreak_20261020 import run as runner
from research.n83_highk_online_tiebreak_20261020 import select as selector

HERE = Path(__file__).resolve().parent


def suffix(value, count):
    return Path(value).parts[-count:]


def fixture_points():
    _, lines = runner.fixture()
    answers = [json.loads(line) for line in
               (HERE / "screen_answers.jsonl").read_text().splitlines()]
    points = [common.group.point_from_json(json.loads(line)) for line in lines]
    scalars = [int(item["scalar"]) for item in answers]
    return lines, points, scalars


def launch_ok(folder, event, root):
    try:
        launch = common.read(folder / "launch.json")
        command = launch["command"]
        if event["kind"] == "ic":
            arm = next(item for item in arms.ARMS
                       if item[0] == event["candidate"])
            flags = {key: value for key, value in
                     runner.ic_environment(folder, arm).items()
                     if key.startswith("KIC_")}
            observed = dict(launch["environment_flags"])
            dump = observed.pop("KIC_DUMP_BASE", "")
            expected_dump = flags.pop("KIC_DUMP_BASE")
            target = root / "screen" / f"block{event['block']}" / "targets.jsonl"
            return (command[1] == f"construct:83:1:{arm[1]}" and
                    suffix(command[2], 3) == suffix(target, 3) and
                    command[3] == "29" and
                    suffix(command[4], 5) ==
                    suffix(folder / "targets_result.jsonl", 5) and
                    observed == flags and
                    suffix(dump, 5) == suffix(expected_dump, 5) and
                    launch["cap_seconds"] == runner.IC_CAP_SECONDS[arm[0]] and
                    launch.get("rss_cap_bytes") == runner.RSS_CAP_BYTES and
                    suffix(command[0], 2) ==
                    ("executed_bins", "koblitz_orbit_dlp_fast"))
        return (event["kind"] == "calibration" and
                suffix(command[1], 3) == suffix(runner.cold.TARGETS, 3) and
                command[2] == str(int(runner.cold.RHO_SEED)) and
                suffix(command[3], 3) == suffix(runner.cold.RHO_CONFIG, 3) and
                suffix(command[4], 5) == suffix(folder / "receipt.json", 5) and
                launch["environment_flags"] == {} and
                launch["cap_seconds"] == runner.RHO_CAP_SECONDS and
                launch.get("rss_cap_bytes") == runner.RSS_CAP_BYTES and
                suffix(command[0], 2) ==
                ("executed_bins", "n83_rho_optimization_eval"))
    except (KeyError, IndexError, OSError, ValueError, TypeError, StopIteration):
        return False


def verify(root):
    hashes, lines = runner.fixture()
    _, points, scalars = fixture_points()
    host, source, preflight = (common.read(root / name) for name in
                               ("host.json", "source.json", "preflight.json"))
    events = runner.screen_schedule()
    chosen, choice = arms.choose()
    trigger = runner.validate_trigger(root / "trigger.json")
    if (host.get("schema") != "n83.online-tiebreak-host/v1" or
            host.get("cgroup_interface") != "v1" or
            not common.v1_snapshot_consistent(host.get("cgroup_before", {})) or
            not host.get("git_sha") or
            preflight.get("status") != "PASS_CAPACITY_PREFLIGHT" or
            preflight.get("rss_cap_bytes") != runner.RSS_CAP_BYTES or
            source.get("schema") != "n83.online-tiebreak-source/v1" or
            source.get("hashes") != hashes or
            source.get("source_sha256") != runner.SOURCE_SHA256 or
            source.get("runner_sha256") != runner.parent.digest(HERE / "run.py") or
            source.get("screen_targets_sha256") != runner.SCREEN_TARGETS_SHA256 or
            source.get("screen_answers_sha256") != runner.SCREEN_ANSWERS_SHA256 or
            source.get("fixture_manifest_sha256") !=
            runner.parent.digest(HERE / "fresh_manifest.json") or
            source.get("exclusion_audit_sha256") !=
            runner.parent.digest(HERE / "fresh_exclusion_audit.json") or
            source.get("trigger_receipt_sha256") !=
            runner.parent.digest(root / "trigger.json") or
            source.get("trigger_aggregate_sha256") !=
            trigger["trigger_aggregate_sha256"] or
            source.get("trigger_archive_sha256") !=
            trigger["trigger_archive_sha256"] or
            source.get("arms") != [list(arm) for arm in chosen] or
            source.get("choice") != choice or
            source.get("screen_events") != events or
            source.get("calibration_seed") != int(runner.cold.RHO_SEED) or
            source.get("calibration_repetitions_per_side") != 3 or
            source.get("calibration_comparison") !=
            "median_of_three_before_vs_after_20_percent" or
            source.get("calibration_target_sha256") !=
            runner.parent.digest(runner.cold.TARGETS) or
            source.get("calibration_answer_sha256") !=
            runner.parent.digest(runner.cold.ANSWERS) or
            source.get("global_producer_cap_seconds") !=
            runner.GLOBAL_PRODUCER_CAP_SECONDS or
            source.get("ic_cap_seconds") != runner.IC_CAP_SECONDS or
            source.get("screen_mode") !=
            "secondary_shared_setup_per_point_intervals_no_rho"):
        raise ValueError("host, source, fixture or schedule differs")
    binaries_match = (
        runner.parent.digest(root / "executed_bins/koblitz_orbit_dlp_fast") ==
        source["binary_sha256"] and
        runner.parent.digest(root / "executed_bins/n83_rho_optimization_eval") ==
        source["rho_binary_sha256"])
    target_files_match = []
    for block in range(2):
        path = root / "screen" / f"block{block}" / "targets.jsonl"
        target_files_match.append(
            path.read_text() == "".join(lines[block*768:(block+1)*768]))
    point_rows = {index: {"point_index": index, "arms": []}
                  for index in range(selector.POINT_COUNT)}
    children, cal_raw, telemetry, launch_checks, finished, snapshots = {}, {}, [], [], [], []
    calibration_point = common.group.point_from_json(
        json.loads(runner.cold.TARGETS.read_text()))
    calibration_scalar = int(common.read(runner.cold.ANSWERS)["scalar"])
    if common.group.multiply(common.group.G, calibration_scalar) != calibration_point:
        raise ValueError("calibration fixture scalar differs")
    for event in events:
        folder = root / "screen" / "runs" / event["name"]
        launch_checks.append(launch_ok(folder, event, root))
        telemetry.append(common.telemetry(root / "screen", event["name"]))
        usage_path = folder / "usage.json"
        if usage_path.exists():
            usage = common.read(usage_path)
            finished.append(datetime.fromisoformat(usage["finished_utc"]))
            snapshots.append(all(common.v1_snapshot_consistent(usage.get(name, {}))
                                 for name in ("cgroup_before", "cgroup_after")))
        else:
            snapshots.append(False)
        kind = event["kind"]
        if kind == "ic":
            arm = next(item for item in arms.ARMS
                       if item[0] == event["candidate"])
            block = event["block"]
            start = block * 768
            result = common.replay_ic(folder, arm,
                                      points[start:start+768],
                                      scalars[start:start+768])
            children[event["name"]] = result
            for local in range(768):
                item = (result.get("targets", [{}] * 768)[local] if
                        len(result.get("targets", [])) == 768 else {})
                point_rows[start+local]["arms"].append({
                    "candidate_id": arm[3],
                    "status": item.get("status", result["status"]),
                    "resource_status": result.get("resources", {}).get("status"),
                    "one_target_online_ms": item.get("one_target_online_ms"),
                    "cold_process_ms": result.get("cold_process_ms"),
                    "screen_child": event["name"],
                })
        elif kind == "calibration":
            before = "_cal_before_" in event["name"]
            key = (event["block"], event["candidate"], before)
            result = common.replay_rho(
                folder, calibration_point, calibration_scalar,
                int(runner.cold.RHO_SEED), runner.cold.TARGETS, hashes)
            children[event["name"]] = result
            cal_raw.setdefault(key, []).append(result)
        else:
            raise ValueError("tie-break schedule contains an unknown event")
    calibrations = {}
    for block in range(2):
        for arm in arms.ARMS:
            key = f"b{block}_{arm[0]}"
            calibrations[key] = common.calibration(
                cal_raw.get((block, arm[0], True), []),
                cal_raw.get((block, arm[0], False), []))
    base_hashes = {arm[3]: {children[f"b{block}_{arm[0]}"].get("base_hash")
                            for block in range(2)} for arm in arms.ARMS}
    same_bases = all(len(values) == 1 and None not in values
                     for values in base_hashes.values())
    order_ok = len(finished) == len(events) and all(
        first < second for first, second in zip(finished, finished[1:]))
    after_path = root / "screen" / "host_telemetry_after.json"
    topology = host.get("topology", {})
    topology_ok = bool(topology.get("lscpu_json") and
                       len(topology.get("visible_cpu_topology", {})) >= 4 and
                       after_path.exists())
    telemetry_ok = (topology_ok and all(item["status"] == "PASS" and
                    item["steal_jiffies_delta"] == 0 for item in telemetry))
    replay_ok = (binaries_match and all(target_files_match) and
                 all(launch_checks) and same_bases and
                 len(children) == len(events) and
                 all(item["status"] == "PASS_INDEPENDENT_REPLAY"
                     for item in children.values()))
    resource_ok = replay_ok and all(snapshots) and all(
        item.get("resources", {}).get("status") == "PASS"
        for item in children.values())
    aa_ok = all(item["status"] == "PASS_20_PERCENT"
                for item in calibrations.values())
    admitted = (replay_ok and resource_ok and aa_ok and order_ok and
                telemetry_ok and not (root / "producer_stop.json").exists())
    analysis = {
        "schema": "n83.online-tiebreak-screen-analysis/v1",
        "source_sha256": runner.SOURCE_SHA256,
        "targets_sha256": runner.SCREEN_TARGETS_SHA256,
        "status": "SCREEN_ADMITTED" if admitted else "BOUNDED_UNKNOWN",
        "evidence_status": "PASS_INDEPENDENT_REPLAY" if replay_ok else
                           "VERIFICATION_FAILED",
        "resource_status": "PASS" if resource_ok else "FAILED_OR_INCOMPLETE",
        "aa_status": "PASS_20_PERCENT_PER_CHILD" if aa_ok else "NOISY_OR_INCOMPLETE",
        "tier_status": "PASS_TIER1" if admitted else "NOT_ADMITTED",
        "order_status": "PASS" if order_ok else "INCOMPLETE_OR_OUT_OF_ORDER",
        "telemetry_status": "PASS" if telemetry_ok else "INCOMPLETE_OR_STEAL",
        "same_candidate_bases": same_bases,
        "all_target_files_match": all(target_files_match),
        "all_launch_commands_match": all(launch_checks),
        "archived_executed_binaries_match": binaries_match,
        "children": children, "calibration_by_child": calibrations,
        "points": [point_rows[index] for index in range(selector.POINT_COUNT)],
    }
    validated = {
        "schema": "n83.online-tiebreak-screen-validated/v1",
        "source_sha256": runner.SOURCE_SHA256,
        "targets_sha256": runner.SCREEN_TARGETS_SHA256,
        "candidate_ids": [arm[3] for arm in arms.ARMS],
        "screen_mode": "secondary_shared_setup_per_point_intervals_no_rho",
        "trigger_status": trigger["status"],
        "trigger_aggregate_sha256": trigger["trigger_aggregate_sha256"],
        "evidence_status": analysis["evidence_status"],
        "single_cloud_instance": bool(host.get("runpod_pod_id") and
                                      host.get("runpod_machine_id")),
        "same_candidate_bases": same_bases,
        "block_point_indices": [list(block) for block in selector.BLOCKS],
        "block_candidate_orders": [[arm[3] for arm in arms.ARMS],
                                   [arm[3] for arm in reversed(arms.ARMS)]],
        "host_strata": [{"point_indices": list(range(selector.POINT_COUNT)),
                         "tier_status": analysis["tier_status"],
                         "aa_status": analysis["aa_status"],
                         "instance_id": host["instance_id"]}],
        "points": analysis["points"],
    }
    selection = selector.select(validated)
    for name, value in (("screen_analysis.json", analysis),
                        ("screen_validated.json", validated),
                        ("screen_selection.json", selection)):
        (root / name).write_text(json.dumps(value, indent=2,
                                            sort_keys=True) + "\n")
    print(json.dumps({"admission_status": selection["admission_status"],
                      "status": selection["status"],
                      "selected_candidate_id": selection["selected_candidate_id"]},
                     sort_keys=True))
    return 0 if admitted else 2


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    return verify(args.output.resolve())


if __name__ == "__main__":
    raise SystemExit(main())
