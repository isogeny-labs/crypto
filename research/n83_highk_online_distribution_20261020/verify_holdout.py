#!/usr/bin/env python3
"""Independently replay eight fresh one-target N83 IC/rho pairs."""

import argparse
import json
from datetime import datetime
from pathlib import Path

from research.n83_highk_online_distribution_20261020 import arms
from research.n83_highk_online_distribution_20261020 import holdout_select
from research.n83_highk_online_distribution_20261020 import run as runner
from research.n83_highk_online_distribution_20261020 import select as screen_selector
from research.n83_highk_online_distribution_20261020 import verify_common as common
from research.n83_highk_online_distribution_20261020 import verify_screen

HERE = Path(__file__).resolve().parent


def suffix(value, count):
    return Path(value).parts[-count:]


def launch_ok(folder, event, root, arm):
    try:
        launch = common.read(folder / "launch.json")
        command = launch["command"]
        index = event["point_index"]
        target = root / "holdout" / "points" / f"p{index:02d}" / "target.jsonl"
        if event["kind"] == "ic":
            flags = {key: value for key, value in
                     runner.ic_environment(folder, arm).items()
                     if key.startswith("KIC_")}
            observed = dict(launch["environment_flags"])
            dump = observed.pop("KIC_DUMP_BASE", "")
            expected_dump = flags.pop("KIC_DUMP_BASE")
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
        rho_target = runner.cold.TARGETS if event["kind"] == "calibration" else target
        seed = (int(runner.cold.RHO_SEED) if event["kind"] == "calibration"
                else runner.RHO_SEED_BASE + 192 + index)
        return (suffix(command[1], 3) == suffix(rho_target, 3) and
                command[2] == str(seed) and
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
    hashes, streams = runner.fixture()
    _, points, scalars = verify_screen.fixture_points()
    screen_validated = common.read(root / "screen_validated.json")
    screen_selection = common.read(root / "screen_selection.json")
    if (screen_selector.select(screen_validated) != screen_selection or
            screen_selection.get("status") != "SELECTED" or
            screen_selection.get("admission_status") != "PASS"):
        raise ValueError("screen selection does not admit a holdout")
    candidate = screen_selection["selected_candidate_id"]
    arm = next(item for item in arms.ARMS if item[3] == candidate)
    events = runner.holdout_schedule(candidate)
    source = common.read(root / "source.json")
    holdout_source = common.read(root / "holdout_source.json")
    host = common.read(root / "host.json")
    if (holdout_source.get("schema") !=
            "n83.online-distribution-holdout-source/v1" or
            holdout_source.get("candidate_id") != candidate or
            holdout_source.get("screen_selection_sha256") !=
            runner.parent.digest(root / "screen_selection.json") or
            holdout_source.get("events") != events or
            holdout_source.get("source_sha256") != runner.SOURCE_SHA256 or
            holdout_source.get("targets_sha256") != runner.HOLDOUT_TARGETS_SHA256 or
            holdout_source.get("answers_sha256") != runner.HOLDOUT_ANSWERS_SHA256 or
            source.get("hashes") != hashes or
            source.get("binary_sha256") !=
            runner.parent.digest(root / "executed_bins/koblitz_orbit_dlp_fast") or
            source.get("rho_binary_sha256") !=
            runner.parent.digest(root / "executed_bins/n83_rho_optimization_eval") or
            host.get("runpod_pod_id") is None or
            not (root / "holdout_host_telemetry_before.json").exists()):
        raise ValueError("holdout source or Pod identity differs")
    calibration_point = common.group.point_from_json(
        json.loads(runner.cold.TARGETS.read_text()))
    calibration_scalar = int(common.read(runner.cold.ANSWERS)["scalar"])
    if common.group.multiply(common.group.G, calibration_scalar) != calibration_point:
        raise ValueError("calibration point differs")
    rows = {index: {"point_index": index, "ic": None,
                    "rho": None, "calibration": None} for index in range(8)}
    children, cal_raw, telemetry, launch_checks, finished, snapshots, target_checks = (
        {}, {}, [], [], [], [], [])
    for event in events:
        index = event["point_index"]
        name = event["name"]
        folder = root / "holdout" / "runs" / name
        target = root / "holdout" / "points" / f"p{index:02d}" / "target.jsonl"
        target_checks.append(target.exists() and
                             target.read_text() == streams["holdout"][index])
        launch_checks.append(launch_ok(folder, event, root, arm))
        telemetry.append(common.telemetry(root / "holdout", name))
        usage_path = folder / "usage.json"
        if usage_path.exists():
            usage = common.read(usage_path)
            finished.append(datetime.fromisoformat(usage["finished_utc"]))
            snapshots.append(all(common.v1_snapshot_consistent(usage.get(side, {}))
                                 for side in ("cgroup_before", "cgroup_after")))
        else:
            snapshots.append(False)
        if event["kind"] == "ic":
            result = common.replay_ic(folder, arm,
                                      [points["holdout"][index]],
                                      [scalars["holdout"][index]])
            children[name] = result
            item = result.get("targets", [{}])[0]
            rows[index]["ic"] = {
                "candidate_id": candidate,
                "status": item.get("status", result["status"]),
                "resource_status": result.get("resources", {}).get("status"),
                "one_target_online_ms": item.get("one_target_online_ms"),
                "cold_process_ms": result.get("cold_process_ms"),
                "raw_replay": result,
            }
        elif event["kind"] == "rho":
            result = common.replay_rho(
                folder, points["holdout"][index], scalars["holdout"][index],
                runner.RHO_SEED_BASE + 192 + index, target, hashes)
            children[name] = result
            rows[index]["rho"] = {
                "status": result["status"],
                "resource_status": result.get("resources", {}).get("status"),
                "online_ms": result.get("online_ms"), "raw_replay": result,
            }
        else:
            before = "_cal_before_" in name
            result = common.replay_rho(
                folder, calibration_point, calibration_scalar,
                int(runner.cold.RHO_SEED), runner.cold.TARGETS, hashes)
            children[name] = result
            cal_raw.setdefault((index, before), []).append(result)
    for index in range(8):
        rows[index]["calibration"] = common.calibration(
            cal_raw.get((index, True), []), cal_raw.get((index, False), []))
    base_hashes = {result.get("base_hash") for name, result in children.items()
                   if name.endswith("_ic") and
                   result.get("status") == "PASS_INDEPENDENT_REPLAY"}
    same_base = len(base_hashes) == 1 and None not in base_hashes
    order_ok = len(finished) == len(events) and all(
        a < b for a, b in zip(finished, finished[1:]))
    telemetry_ok = (all(item["status"] == "PASS" and
                    item["steal_jiffies_delta"] == 0 for item in telemetry) and
                    (root / "holdout/host_telemetry_after.json").exists())
    replay_ok = (all(target_checks) and all(launch_checks) and
                 len(children) == len(events) and same_base and
                 all(item["status"] == "PASS_INDEPENDENT_REPLAY"
                     for item in children.values()))
    resource_ok = replay_ok and all(snapshots) and all(
        item.get("resources", {}).get("status") == "PASS"
        for item in children.values())
    aa_ok = all(row["calibration"]["status"] == "PASS_20_PERCENT"
                for row in rows.values())
    admitted = (replay_ok and resource_ok and aa_ok and order_ok and
                telemetry_ok and not (root / "producer_stop.json").exists())
    analysis = {
        "schema": "n83.online-distribution-holdout-analysis/v1",
        "candidate_id": candidate,
        "status": "HOLDOUT_ADMITTED" if admitted else "BOUNDED_UNKNOWN",
        "evidence_status": "PASS_INDEPENDENT_REPLAY" if replay_ok else
                           "VERIFICATION_FAILED",
        "resource_status": "PASS" if resource_ok else "FAILED_OR_INCOMPLETE",
        "aa_status": "PASS_20_PERCENT_PER_PAIR" if aa_ok else "NOISY_OR_INCOMPLETE",
        "tier_status": "PASS_TIER1" if admitted else "NOT_ADMITTED",
        "same_candidate_base": same_base,
        "order_status": "PASS" if order_ok else "INCOMPLETE_OR_OUT_OF_ORDER",
        "telemetry_status": "PASS" if telemetry_ok else "INCOMPLETE_OR_STEAL",
        "all_target_files_match": all(target_checks),
        "all_launch_commands_match": all(launch_checks),
        "children": children, "points": [rows[index] for index in range(8)],
    }
    validated = {
        "schema": "n83.online-distribution-holdout-validated/v1",
        "source_sha256": runner.SOURCE_SHA256,
        "targets_sha256": runner.HOLDOUT_TARGETS_SHA256,
        "candidate_id": candidate,
        "evidence_status": analysis["evidence_status"],
        "single_cloud_instance": bool(host.get("runpod_pod_id") and
                                      host.get("runpod_machine_id")),
        "same_candidate_base": same_base,
        "host_strata": [{"point_indices": list(range(8)),
                         "tier_status": analysis["tier_status"],
                         "aa_status": analysis["aa_status"],
                         "instance_id": host["instance_id"]}],
        "points": analysis["points"],
    }
    selection = holdout_select.select(screen_selection, validated)
    for name, value in (("holdout_analysis.json", analysis),
                        ("holdout_validated.json", validated),
                        ("holdout_selection.json", selection)):
        (root / name).write_text(json.dumps(value, indent=2,
                                            sort_keys=True) + "\n")
    print(json.dumps({"admission_status": selection["admission_status"],
                      "status": selection["status"],
                      "verified_speedup_candidate_id":
                      selection["verified_speedup_candidate_id"]},
                     sort_keys=True))
    return 0 if admitted else 2


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    return verify(args.output.resolve())


if __name__ == "__main__":
    raise SystemExit(main())
