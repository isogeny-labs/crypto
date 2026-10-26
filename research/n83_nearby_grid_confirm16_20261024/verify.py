#!/usr/bin/env python3
"""Independently replay one N83 confirmation stratum and every child."""

import argparse
import json
from pathlib import Path

from research.n83_nearby_grid_confirm16_20261024 import protocol
from research.n83_nearby_grid_confirm16_20261024 import run as producer
from research.n83_nearby_grid_tier1_20261024 import protocol as screen
from research.n83_nearby_grid_tier1_20261024 import run as screen_run
from research.n83_nearby_grid_tier1_20261024 import verify as screen_replay

parent = producer.parent


def read(path):
    return json.loads(path.read_text())


def recorded_output_root(output, stratum, selected_ks):
    """Recover one frozen runner root before replaying an extracted artifact."""
    first = protocol.events(stratum, selected_ks)[0]
    command = read(output / "points" / f"p{first['point_index']:02d}" /
                   "runs" / first["name"] / "launch.json").get("command")
    if not isinstance(command, list) or len(command) != 5:
        raise ValueError("first confirmation launch is incomplete")
    root = Path(command[0]).parent.parent
    if (not root.is_absolute() or ".." in root.parts or
            root.name != output.name or
            command[0] != str(root / "executed_bins/koblitz_orbit_dlp_fast")):
        raise ValueError("first confirmation launch has no consistent run root")
    return root


def launch_gate(folder, event, recorded_root):
    launch = read(folder / "launch.json")
    command = launch.get("command")
    if (not isinstance(command, list) or len(command) != 5 or
            launch.get("rss_cap_bytes") != protocol.RSS_CAP_BYTES or
            recorded_root is None):
        return False
    recorded_folder = (recorded_root / "points" /
                       f"p{event['point_index']:02d}" / "runs" / event["name"])
    recorded_target = (recorded_root / "points" /
                       f"p{event['point_index']:02d}" / "target.jsonl")
    if event["kind"] == "rho":
        config = Path(command[3])
        relative_config = screen.RHO_CONFIG.relative_to(screen.ROOT)
        if (not config.is_absolute() or ".." in config.parts or
                config.parts[-len(relative_config.parts):] !=
                relative_config.parts):
            return False
        return (command == [str(recorded_root /
                                "executed_bins/n83_rho_optimization_eval"),
                            str(recorded_target), str(event["seed"]),
                            str(config), str(recorded_folder / "receipt.json")] and
                launch.get("environment_flags") == {} and
                launch.get("cap_seconds") == protocol.RHO_CAP_SECONDS)
    return (command == [str(recorded_root /
                            "executed_bins/koblitz_orbit_dlp_fast"),
                        f"construct:83:1:{event['k']}", str(recorded_target),
                        "29", str(recorded_folder / "targets_result.jsonl")] and
            launch.get("environment_flags") ==
            screen_run.ic_flags(recorded_folder, event["k"]) and
            launch.get("cap_seconds") == protocol.IC_CAP_SECONDS)


def resource_gate(folder, host):
    result = screen_replay.resource_gate(folder)
    delta = result.get("steal_jiffies_delta")
    wall = result.get("wall_seconds")
    hz = host.get("clock_ticks_per_second")
    visible = host.get("visible_affinity") or []
    fraction = (delta / (wall * hz * len(visible))
                if isinstance(delta, int) and delta >= 0 and
                isinstance(wall, (int, float)) and wall > 0 and
                isinstance(hz, int) and hz > 0 and visible else None)
    result["steal_fraction_of_visible_cpu_wall"] = fraction
    if fraction is None or fraction > protocol.STEAL_FRACTION_CAP:
        result["status"] = "NOISY_OR_INCOMPLETE"
    return result


def check_source(source, host, output, raw_screen, analysis, choice, stratum,
                 schedule, manifest):
    expected = {
        "schema": "n83.nearby-grid-confirm16-source/v1",
        "git_head": host.get("git_sha"),
        "runner_sha256": protocol.digest(Path(producer.__file__)),
        "protocol_sha256": protocol.digest(Path(protocol.__file__)),
        "selector_sha256": protocol.digest(Path(producer.select.__file__)),
        "screen_runner_sha256": protocol.digest(Path(screen_run.__file__)),
        "screen_verifier_sha256": protocol.digest(Path(screen_replay.__file__)),
        "screen_analysis_sha256": producer.canonical_digest(analysis),
        "screen_selection_sha256": producer.canonical_digest(choice),
        "screen_run_url": read(raw_screen / "host.json").get("run_url"),
        "screen_workload_id": screen.WORKLOAD_ID,
        "screen_catalog_sha256": screen.CATALOG_SHA256,
        "fixture_manifest_sha256": protocol.digest(protocol.HERE / "fixture_manifest.json"),
        "targets_sha256": protocol.digest(protocol.HERE / "targets.jsonl"),
        "answers_sha256": protocol.digest(protocol.HERE / "answers.jsonl"),
        "workloads_sha256": protocol.digest(protocol.HERE / "workloads.jsonl"),
        "exclusion_audit_sha256": protocol.digest(protocol.HERE / "exclusion_audit.json"),
        "ic_source_sha256": screen.SOURCE_SHA256,
        "rho_source_sha256": screen.RHO_SOURCE_SHA256,
        "cargo_lock_sha256": screen.LOCK_SHA256,
        "rho_config_sha256": screen.RHO_CONFIG_SHA256,
        "binary_sha256": protocol.digest(
            output / "executed_bins/koblitz_orbit_dlp_fast"),
        "rho_binary_sha256": protocol.digest(
            output / "executed_bins/n83_rho_optimization_eval"),
        "selected_ks": choice["selected_ks"],
        "candidate_ids": choice["selected_candidate_ids"],
        "candidate_manifest_sha256": {str(k): screen.CANDIDATES[k][1]
                                      for k in choice["selected_ks"]},
        "stratum": stratum, "events": schedule,
        "rho_seed_rule": "2026122500+point_index*selected_count+fixed_candidate_index",
        "caps": {"ic_seconds": protocol.IC_CAP_SECONDS,
                 "rho_seconds": protocol.RHO_CAP_SECONDS,
                 "global_seconds": protocol.GLOBAL_CAP_SECONDS,
                 "rss_bytes": protocol.RSS_CAP_BYTES},
        "aa_spread_limit": protocol.AA_SPREAD_LIMIT,
        "steal_fraction_cap": protocol.STEAL_FRACTION_CAP,
        "fixture_point_count": manifest["point_count"],
    }
    return (all(source.get(key) == value for key, value in expected.items()) and
            read(output / "screen_analysis.json") == analysis and
            read(output / "screen_selection.json") == choice)


def verify(output, raw_screen):
    manifest, points = protocol.fixture()
    analysis, choice = producer.screen_selection(raw_screen)
    if choice["status"] != "PASS_FROZEN_SCREEN_SELECTION":
        return {"schema": "n83.nearby-grid-confirm16-replay/v1",
                "status": "BOUNDED_UNKNOWN", "reason": "SCREEN_NOT_ADMITTED",
                "screen_status": analysis["status"], "rows": []}
    source = read(output / "source.json")
    host = read(output / "host.json")
    stratum = source.get("stratum")
    selected_ks = choice["selected_ks"]
    schedule = protocol.events(stratum, selected_ks)
    try:
        recorded_root = recorded_output_root(output, stratum, selected_ks)
    except (OSError, ValueError, KeyError, TypeError, IndexError):
        recorded_root = None
    source_ok = check_source(source, host, output, raw_screen, analysis, choice,
                             stratum, schedule, manifest)
    host_ok = (host.get("schema") == "n83.nearby-grid-confirm16-host/v1" and
               host.get("stratum") == stratum and
               host.get("instance_id") ==
               f"{host.get('parent_instance_id')}-confirm16-stratum{stratum}" and
               screen_run.host_gate(host, require_current_run=False) and
               read(output / "preflight.json").get("status") ==
               "PASS_HOST_ENVELOPE" and
               (output / "host_telemetry_after.json").exists())
    children = []
    by_name = {}
    for event in schedule:
        index = event["point_index"]
        point_dir = output / "points" / f"p{index:02d}"
        target_file = point_dir / "target.jsonl"
        folder = point_dir / "runs" / event["name"]
        row = {"event": event, "status": "BOUNDED_UNKNOWN", "reason": None,
               "resources": None, "online_ms": None,
               "cold_process_ms": None, "launch_verified": False}
        try:
            if not folder.exists():
                row["reason"] = "NOT_RUN"
            elif target_file.read_text() != points[index][0]:
                raise ValueError("per-point public input differs")
            else:
                row["launch_verified"] = launch_gate(folder, event,
                                                      recorded_root)
                row["resources"] = resource_gate(folder, host)
                if not row["launch_verified"]:
                    raise ValueError("launch command, flags or cap differs")
                if row["resources"]["child_status"] != "PASS_PRODUCER":
                    row["reason"] = row["resources"]["child_status"]
                else:
                    _, point, scalar, _ = points[index]
                    result = (screen_replay.replay_rho(folder, event, point,
                                                       scalar, target_file)
                              if event["kind"] == "rho" else
                              screen_replay.replay_ic(folder, event["k"],
                                                      point, scalar))
                    row.update(result)
                    row["cold_process_ms"] = (
                        result["cold_process_ms"] if event["kind"] != "rho"
                        else row["resources"]["wall_seconds"] * 1000)
        except (OSError, ValueError, KeyError, TypeError, IndexError) as error:
            row["status"] = "VERIFICATION_FAILED"
            row["reason"] = str(error)
        children.append(row)
        by_name[event["name"]] = row
    rows = []
    aa = []
    for index in range(stratum * protocol.POINTS_PER_STRATUM,
                       (stratum + 1) * protocol.POINTS_PER_STRATUM):
        before = by_name[f"p{index:02d}_aa_before"]
        after = by_name[f"p{index:02d}_aa_after"]
        aa_ok = (all(item["status"] == "PASS_INDEPENDENT_REPLAY" and
                     item["resources"] and item["resources"]["status"] == "PASS"
                     for item in (before, after)) and
                 min(before["online_ms"], after["online_ms"],
                     before["cold_process_ms"], after["cold_process_ms"]) > 0 and
                 max(before["online_ms"], after["online_ms"]) /
                 min(before["online_ms"], after["online_ms"]) - 1 <=
                 protocol.AA_SPREAD_LIMIT and
                 max(before["cold_process_ms"], after["cold_process_ms"]) /
                 min(before["cold_process_ms"], after["cold_process_ms"]) - 1 <=
                 protocol.AA_SPREAD_LIMIT)
        aa.append({"point_index": index, "status": "PASS" if aa_ok else
                   "NOISY_OR_INCOMPLETE", "online_before_ms": before["online_ms"],
                   "online_after_ms": after["online_ms"],
                   "cold_before_ms": before["cold_process_ms"],
                   "cold_after_ms": after["cold_process_ms"]})
        for k in selected_ks:
            ic = by_name[f"p{index:02d}_k{k}_ic"]
            rho = by_name[f"p{index:02d}_k{k}_rho"]
            candidate_id = screen.CANDIDATES[k][0]
            workload_id = points[index][3]
            admitted = (aa_ok and all(
                item["status"] == "PASS_INDEPENDENT_REPLAY" and
                item["resources"] and item["resources"]["status"] == "PASS"
                for item in (ic, rho)))
            row = {"candidate_id": candidate_id, "workload_id": workload_id,
                   "run_id": f"{candidate_id}W{workload_id}R1",
                   "point_index": index, "k": k,
                   "status": "PASS_INDEPENDENT_REPLAY" if admitted else
                   "BOUNDED_UNKNOWN", "ic_online_ms": None,
                   "rho_online_ms": None, "rho_over_ic_online": None,
                   "ic_cold_process_ms": None, "online_phases_ms": None,
                   "ic_event": ic["event"]["name"],
                   "rho_event": rho["event"]["name"]}
            if admitted:
                try:
                    ic_dir = (output / "points" / f"p{index:02d}" /
                              "runs" / ic["event"]["name"])
                    phases = read(ic_dir / "targets_result.jsonl")
                    summary_ic = read(ic_dir / "stdout.jsonl")
                    timings = summary_ic["timing_ms"]
                    rho_dir = (output / "points" / f"p{index:02d}" /
                               "runs" / rho["event"]["name"])
                    rho_receipt = read(rho_dir / "receipt.json")
                    row.update({"ic_online_ms": ic["online_ms"],
                                "rho_online_ms": rho["online_ms"],
                                "rho_over_ic_online": rho["online_ms"] /
                                ic["online_ms"],
                                "ic_cold_process_ms": ic["cold_process_ms"],
                                "ic_setup_ms": timings["setup_total"],
                                "ic_rank_stage_ms": timings["rank_stage"],
                                "ic_index_build_ms": timings["index_build"],
                                "ic_linear_algebra_ms": timings["linear_algebra"],
                                "ic_base_load_ms": timings["base_load"],
                                "ic_rank_relations": summary_ic["rank_relations"],
                                "ic_rank_attempts": summary_ic["rank_attempts"],
                                "root_table_entries": summary_ic["root_table_entries"],
                                "root_table_allocated_bytes":
                                summary_ic["root_table_allocated_bytes"],
                                "ic_peak_rss_bytes": ic["resources"]["peak_rss_bytes"],
                                "rho_peak_rss_bytes": rho["resources"]["peak_rss_bytes"],
                                "rho_walk_seed": rho_receipt["walk_seed"],
                                "online_phases_ms": {name: phases[name]
                                                     for name in screen_replay.PHASES}})
                except (OSError, ValueError, KeyError, TypeError, IndexError) as error:
                    row["status"] = "VERIFICATION_FAILED"
                    row["reason"] = f"stage accounting: {error}"
            rows.append(row)
    summary_path = output / "producer_summary.json"
    summary = read(summary_path) if summary_path.exists() else {}
    complete = (source_ok and host_ok and
                summary.get("schema") ==
                "n83.nearby-grid-confirm16-producer-summary/v1" and
                summary.get("completed_events") == len(schedule) and
                summary.get("scheduled_events") == len(schedule) and
                summary.get("status") == "ALL_CHILDREN_PASSED" and
                len({row["event"]["name"] for row in children}) == len(schedule) and
                all(item["status"] == "PASS_INDEPENDENT_REPLAY" and
                    item["resources"] and item["resources"]["status"] == "PASS"
                    for item in children) and
                all(item["status"] == "PASS" for item in aa) and
                all(row["status"] == "PASS_INDEPENDENT_REPLAY" for row in rows))
    result = {"schema": "n83.nearby-grid-confirm16-replay/v1",
              "status": "STRATUM_ADMITTED" if complete else "BOUNDED_UNKNOWN",
              "source_verified": source_ok, "host_verified": host_ok,
              "noise_gate_passed": all(item["status"] == "PASS" for item in aa),
              "stratum": stratum, "selected_ks": selected_ks,
              "git_head": source.get("git_head"),
              "host_instance_id": host.get("instance_id"),
              "runner_name": host.get("runner_name"),
              "screen_run_url": source.get("screen_run_url"),
              "children": children, "aa_controls": aa, "rows": rows}
    parent.write_json(output / "independent_replay.json", result)
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("output", type=Path)
    parser.add_argument("--screen-output", type=Path, required=True)
    args = parser.parse_args()
    result = verify(args.output.resolve(), args.screen_output.resolve())
    print(json.dumps({key: result.get(key) for key in
                      ("status", "source_verified", "host_verified",
                       "noise_gate_passed", "stratum")}, sort_keys=True))
    return 0 if result["status"] == "STRATUM_ADMITTED" else 2


if __name__ == "__main__":
    raise SystemExit(main())
