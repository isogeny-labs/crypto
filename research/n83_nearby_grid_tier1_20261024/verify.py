#!/usr/bin/env python3
"""Replay the 19-base current-source N83 grid and Tier 1 resource gates."""

import argparse
import json
import math
from pathlib import Path
import statistics

from research.n83_highk_online_distribution_20261020 import run as library
from research.n83_indexed5_20261013 import analyze as group
from research.n83_nearby_grid_tier1_20261024 import protocol as spec
from research.n83_nearby_grid_tier1_20261024 import run as producer

PHASES = ("target_query_ms", "target_pdp_ms", "target_relation_check_ms",
          "target_descent_ms", "target_recovery_check_ms")
parent = library.parent


def read(path):
    return json.loads(path.read_text())


def count_delta(usage, section, name):
    def extract(side):
        lines = (usage.get(side, {}).get(section) or "").splitlines()
        for line in lines:
            fields = line.split()
            if len(fields) == 2 and fields[0] == name:
                return int(fields[1])
        return None
    before, after = extract("cgroup_before"), extract("cgroup_after")
    return after - before if before is not None and after is not None else None


def resource_gate(folder):
    usage = read(folder / "usage.json")
    telemetry = read(folder / "host_telemetry.json")
    before, after = telemetry["before"], telemetry["after"]
    throttle = count_delta(usage, "cpu.stat", "nr_throttled")
    oom = count_delta(usage, "memory.events", "oom")
    oom_kill = count_delta(usage, "memory.events", "oom_kill")
    limits_stable = all(
        usage.get("cgroup_before", {}).get(key) ==
        usage.get("cgroup_after", {}).get(key) and
        bool(usage.get("cgroup_before", {}).get(key))
        for key in ("cpu.max", "memory.max"))
    peak = usage.get("peak_rss_bytes")
    valid = (usage.get("status") == "PASS_PRODUCER" and
             isinstance(peak, int) and 0 < peak <= spec.RSS_CAP_BYTES and
             throttle == oom == oom_kill == 0 and limits_stable and
             before.get("steal_jiffies") is not None and
             after.get("steal_jiffies") is not None and
             after["steal_jiffies"] >= before["steal_jiffies"] and
             before.get("frequency_khz_by_visible_cpu") and
             set(before["frequency_khz_by_visible_cpu"]) ==
             set(after.get("frequency_khz_by_visible_cpu", {})))
    return {"status": "PASS" if valid else "NOISY_OR_INCOMPLETE",
            "peak_rss_bytes": peak, "throttled_periods_delta": throttle,
            "cgroup_limits_stable": limits_stable,
            "oom_events_delta": oom, "oom_kills_delta": oom_kill,
            "steal_jiffies_delta": (after["steal_jiffies"] -
                                    before["steal_jiffies"]
                                    if isinstance(after.get("steal_jiffies"), int)
                                    and isinstance(before.get("steal_jiffies"), int)
                                    else None),
            "wall_seconds": usage.get("wall_seconds"),
            "child_status": usage.get("status")}


def recorded_output_root(output):
    """Recover the original run root from its first frozen child receipt.

    An artifact is replayed after extraction at a different location. All
    recorded launch paths must still agree on one absolute original root.
    """
    first = spec.events()[0]["name"]
    command = read(output / "runs" / first / "launch.json")["command"]
    root = Path(command[0]).parent.parent
    if (not root.is_absolute() or ".." in root.parts or
            root.name != output.name or
            command[0] != str(root / "executed_bins/n83_rho_optimization_eval")):
        raise ValueError("first child has no consistent recorded run root")
    return root


def launch_gate(folder, event, output):
    launch = read(folder / "launch.json")
    command = launch["command"]
    recorded_root = recorded_output_root(output)
    recorded_folder = recorded_root / "runs" / event["name"]
    target = recorded_root / "target.jsonl"
    if launch.get("rss_cap_bytes") != spec.RSS_CAP_BYTES:
        return False
    if event["kind"] == "ic":
        expected = producer.ic_flags(recorded_folder, event["k"])
        return (command == [str(recorded_root /
                               "executed_bins/koblitz_orbit_dlp_fast"),
                            f"construct:83:1:{event['k']}", str(target), "29",
                            str(recorded_folder / "targets_result.jsonl")] and
                launch["environment_flags"] == expected and
                launch["cap_seconds"] == spec.IC_CAP_SECONDS)
    if len(command) != 5:
        return False
    config = Path(command[3])
    relative_config = spec.RHO_CONFIG.relative_to(spec.ROOT)
    if (not config.is_absolute() or ".." in config.parts or
            config.parts[-len(relative_config.parts):] != relative_config.parts):
        return False
    return (command == [str(recorded_root /
                            "executed_bins/n83_rho_optimization_eval"),
                        str(target), str(event["seed"]), str(config),
                        str(recorded_folder / "receipt.json")] and
            launch["environment_flags"] == {} and
            launch["cap_seconds"] == spec.RHO_CAP_SECONDS)


def replay_ic(folder, k, target, scalar):
    record = producer.manifest(k)
    summary = read(folder / "stdout.jsonl")
    rows = [json.loads(line) for line in
            (folder / "targets_result.jsonl").read_text().splitlines() if line]
    dump_path = folder / "base_dump.jsonl"
    dump = read(dump_path)
    if len(rows) != 1:
        raise ValueError("IC child did not emit exactly one target row")
    row = rows[0]
    if (parent.digest(dump_path) !=
            record["factor_base"]["ordered_base_dump_sha256"] or
            summary.get("n") != 83 or summary.get("a") != 1 or
            summary.get("orbit_columns") != k or
            summary.get("factor_base_points") != 166 * k or
            summary.get("base_hash") !=
            record["factor_base"]["source_base_hash_blake3"] or
            summary.get("rank") != k or summary.get("targets") != 1 or
            summary.get("targets_solved") != 1 or
            summary.get("targets_failed") != 0 or
            dump.get("base_hash") != summary.get("base_hash") or
            len(dump["factor_base_point_coordinates"]) != 166 * k or
            int(row["recovered_scalar"]) != scalar or
            row.get("group_verified") is not True or
            group.multiply(group.G, int(row["recovered_scalar"])) != target):
        raise ValueError("IC base, rank, point or scalar replay failed")
    indices = row["point_indices"]
    if (len(indices) != 4 or
            any(not isinstance(i, int) or isinstance(i, bool) or
                i < 0 or i >= 166 * k for i in indices)):
        raise ValueError("IC four-point relation indices differ")
    total = None
    for index in indices:
        point = group.point_from_json(
            dump["factor_base_point_coordinates"][index])
        if not group.on_curve(point) or group.multiply(point, group.R) is not None:
            raise ValueError("relation factor outside subgroup")
        total = group.add(total, point)
    if total != target:
        raise ValueError("four-point relation misses target")
    online = row["target_ms"]
    phases = [row[key] for key in PHASES]
    timing = summary["timing_ms"]
    if (not all(isinstance(value, (int, float)) and
                not isinstance(value, bool) and math.isfinite(value) and
                value >= 0 for value in [online, *phases,
                    timing["process_total"], timing["setup_total"],
                    timing["rank_stage"], timing["targets_total"]]) or
            online <= 0 or abs(math.fsum(phases) - online) > 1 or
            abs(timing["targets_total"] - online) > 2 or
            math.fsum([timing["setup_total"], timing["rank_stage"],
                       timing["targets_total"]]) > timing["process_total"] + 1):
        raise ValueError("IC phase accounting differs")
    return {"status": "PASS_INDEPENDENT_REPLAY", "online_ms": online,
            "cold_process_ms": timing["process_total"],
            "probes": row.get("target_probes"), "rank": k,
            "usable_points": 166 * k,
            "base_dump_sha256": parent.digest(dump_path),
            "recovered_scalar": scalar}


def replay_rho(folder, event, target, scalar, target_file):
    receipt = read(folder / "receipt.json")
    online = receipt["online_ms"]
    if (receipt.get("status") != "PASS_VERIFIED_PUBLIC_TARGET" or
            group.point_from_json(receipt["public_target_q"]) != target or
            int(receipt["recovered_scalar"]) != scalar or
            receipt.get("walk_seed") != event["seed"] or
            receipt.get("target_count") != 1 or
            receipt.get("known_answer_input") is not False or
            receipt.get("validation_sidecar_opened") is not False or
            receipt.get("group_verified") is not True or
            receipt.get("generic_scalar_replay_verified") is not True or
            group.multiply(group.G, int(receipt["recovered_scalar"])) != target or
            receipt.get("public_input_sha256") != parent.digest(target_file) or
            receipt["compiled_source_sha256"]["examples/n83_rho_optimization_eval.rs"] !=
            spec.RHO_SOURCE_SHA256 or
            not isinstance(online, (int, float)) or
            not math.isfinite(online) or online <= 0):
        raise ValueError("same-point rho source, scalar or online replay failed")
    return {"status": "PASS_INDEPENDENT_REPLAY", "online_ms": online,
            "recovered_scalar": scalar}



def summarize_blocks(children, schedule, complete):
    blocks = []
    block_size = spec.BLOCK_SIZE + 4
    block_count = len(schedule) // block_size
    for index in range(block_count):
        part = children[block_size * index:block_size * (index + 1)]
        entry = {"round": index // (block_count // spec.ROUNDS),
                 "block": index % (block_count // spec.ROUNDS),
                 "status": "BOUNDED_UNKNOWN"}
        if (len(part) != block_size or
                not all(row["status"] == "PASS_INDEPENDENT_REPLAY" and
                        row["resources"] and row["resources"]["status"] == "PASS"
                        for row in part)):
            entry["reason"] = "INCOMPLETE_OR_RESOURCE_FAILED"
            blocks.append(entry)
            continue
        rho_before, k_before = part[:2]
        challenger_rows = part[2:-2]
        k_after, rho_after = part[-2:]
        if (k_before["event"].get("k") != spec.BASELINE_K or
                k_after["event"].get("k") != spec.BASELINE_K or
                len(challenger_rows) != spec.BLOCK_SIZE or
                len({row["event"].get("k") for row in challenger_rows}) !=
                spec.BLOCK_SIZE):
            raise ValueError("grid block coverage differs")
        baseline_online = statistics.mean([k_before["online_ms"],
                                           k_after["online_ms"]])
        baseline_cold = statistics.mean([k_before["cold_process_ms"],
                                         k_after["cold_process_ms"]])
        rho_online = statistics.mean([rho_before["online_ms"],
                                      rho_after["online_ms"]])
        aa = abs(k_before["online_ms"] - k_after["online_ms"]) / baseline_online
        cold_aa = (abs(k_before["cold_process_ms"] -
                       k_after["cold_process_ms"]) / baseline_cold)
        rho_aa = (abs(rho_before["online_ms"] - rho_after["online_ms"]) /
                  rho_online)
        entry.update({
            "status": ("PASS_TIER1_BLOCK" if aa <= spec.AA_SPREAD_LIMIT and
                       cold_aa <= spec.AA_SPREAD_LIMIT and
                       rho_aa <= spec.AA_SPREAD_LIMIT else "NOISY_BLOCK"),
            "baseline_aa_relative_spread": aa,
            "baseline_cold_aa_relative_spread": cold_aa,
            "rho_aa_relative_spread": rho_aa,
            "rho_online_ms": rho_online,
            "baseline_online_ms": baseline_online,
            "baseline_cold_process_ms": baseline_cold,
            "online_ms": {str(row["event"]["k"]): row["online_ms"]
                          for row in challenger_rows},
            "cold_process_ms": {str(row["event"]["k"]): row["cold_process_ms"]
                                for row in challenger_rows},
            "paired_online_ratio_to_baseline": {
                str(row["event"]["k"]): row["online_ms"] / baseline_online
                for row in challenger_rows},
            "paired_cold_ratio_to_baseline": {
                str(row["event"]["k"]): row["cold_process_ms"] / baseline_cold
                for row in challenger_rows},
            "rho_over_ic_online": {
                str(row["event"]["k"]): rho_online / row["online_ms"]
                for row in challenger_rows},
        })
        blocks.append(entry)
    noise_ok = (len(blocks) == block_count and
                all(row["status"] == "PASS_TIER1_BLOCK" for row in blocks))
    passed = complete and noise_ok
    medians = None
    cold_medians = None
    online_ratios = None
    cold_ratios = None
    if passed:
        online_ratios = {str(spec.BASELINE_K): 1.0}
        cold_ratios = {str(spec.BASELINE_K): 1.0}
        medians = {str(spec.BASELINE_K): statistics.median(
            row["baseline_online_ms"] for row in blocks)}
        cold_medians = {str(spec.BASELINE_K): statistics.median(
            row["baseline_cold_process_ms"] for row in blocks)}
        for k in spec.CANDIDATES:
            if k == spec.BASELINE_K:
                continue
            key = str(k)
            selected = [row for row in blocks if key in row["online_ms"]]
            if len(selected) != spec.ROUNDS:
                raise ValueError(f"K{k} missing paired blocks")
            medians[key] = statistics.median(row["online_ms"][key]
                                             for row in selected)
            cold_medians[key] = statistics.median(row["cold_process_ms"][key]
                                                  for row in selected)
            online_ratios[key] = statistics.median(
                row["paired_online_ratio_to_baseline"][key] for row in selected)
            cold_ratios[key] = statistics.median(
                row["paired_cold_ratio_to_baseline"][key] for row in selected)
    status = "PASS_TIER1_GRID_SCREEN" if passed else "BOUNDED_UNKNOWN"
    return (blocks, noise_ok, medians, cold_medians, online_ratios,
            cold_ratios, status)

def verify(output):
    point_line, point, scalar = producer.fixture()
    source = read(output / "source.json")
    host = read(output / "host.json")
    target_file = output / "target.jsonl"
    schedule = spec.events()
    source_ok = (source.get("schema") == "n83.nearby-grid-tier1-source/v1" and
                 source.get("events") == schedule and
                 source.get("git_head") == host.get("git_sha") and
                 source.get("ic_source_sha256") == spec.SOURCE_SHA256 and
                 source.get("rho_source_sha256") == spec.RHO_SOURCE_SHA256 and
                 source.get("binary_sha256") ==
                 parent.digest(output / "executed_bins/koblitz_orbit_dlp_fast") and
                 source.get("rho_binary_sha256") ==
                 parent.digest(output / "executed_bins/n83_rho_optimization_eval") and
                 source.get("target_sha256") == parent.digest(target_file) and
                 source.get("workload_id") == spec.WORKLOAD_ID and
                 source.get("fixture_target_sha256") == spec.TARGETS_SHA256 and
                 source.get("answer_sidecar_sha256") == spec.ANSWERS_SHA256 and
                 source.get("fixture_manifest_sha256") ==
                 spec.FIXTURE_MANIFEST_SHA256 and
                 source.get("exclusion_audit_sha256") ==
                 spec.EXCLUSION_AUDIT_SHA256 and
                 source.get("workload_sha256") == spec.WORKLOAD_SHA256 and
                 source.get("generator_sha256") == spec.GENERATOR_SHA256 and
                 source.get("catalog_sha256") == spec.CATALOG_SHA256 and
                 source.get("rho_config_sha256") == spec.RHO_CONFIG_SHA256 and
                 source.get("caps") == {
                     "ic_seconds": spec.IC_CAP_SECONDS,
                     "rho_seconds": spec.RHO_CAP_SECONDS,
                     "global_seconds": spec.GLOBAL_CAP_SECONDS,
                     "rss_bytes": spec.RSS_CAP_BYTES} and
                 target_file.read_text() == point_line and
                 source.get("candidate_ids") ==
                 {str(k): v[0] for k, v in spec.CANDIDATES.items()} and
                 source.get("candidate_manifest_sha256") ==
                 {str(k): v[1] for k, v in spec.CANDIDATES.items()} and
                 source.get("protocol_sha256") == parent.digest(Path(spec.__file__)) and
                 source.get("runner_sha256") == parent.digest(Path(producer.__file__)))
    host_ok = (host.get("schema") == "n83.nearby-grid-tier1-host/v1" and
               producer.host_gate(host, require_current_run=False) and
               read(output / "preflight.json").get("status") ==
               "PASS_HOST_ENVELOPE" and
               (output / "host_telemetry_after.json").exists())
    children = []
    for event in schedule:
        folder = output / "runs" / event["name"]
        row = {"event": event, "status": "BOUNDED_UNKNOWN", "online_ms": None,
               "cold_process_ms": None, "resources": None}
        try:
            if not folder.exists():
                row["reason"] = "NOT_RUN"
            else:
                row["launch_verified"] = launch_gate(folder, event, output)
                row["resources"] = resource_gate(folder)
                if not row["launch_verified"]:
                    raise ValueError("launch flags, point or cap differ")
                if row["resources"]["child_status"] != "PASS_PRODUCER":
                    row["reason"] = row["resources"]["child_status"]
                else:
                    result = (replay_ic(folder, event["k"], point, scalar)
                              if event["kind"] == "ic" else
                              replay_rho(folder, event, point, scalar, target_file))
                    row.update(result)
                    row["cold_process_ms"] = (
                        result.get("cold_process_ms") if event["kind"] == "ic"
                        else row["resources"]["wall_seconds"] * 1000)
        except (OSError, ValueError, KeyError, TypeError, IndexError) as error:
            row["status"] = "VERIFICATION_FAILED"
            row["reason"] = str(error)
        children.append(row)
    complete = (len(children) == len(schedule) and source_ok and host_ok and
                all(row["status"] == "PASS_INDEPENDENT_REPLAY" and
                    row["resources"]["status"] == "PASS" for row in children))
    (blocks, noise_ok, medians, cold_medians, online_ratios,
     cold_ratios, status) = summarize_blocks(children, schedule, complete)
    result = {"schema": "n83.nearby-grid-tier1-replay/v1",
              "status": status, "source_verified": source_ok,
              "host_verified": host_ok, "noise_gate_passed": noise_ok,
              "point_index": 0, "blocks": blocks, "children": children,
              "median_online_ms": medians,
              "median_cold_process_ms": cold_medians,
              "median_paired_online_ratio_to_baseline": online_ratios,
              "median_paired_cold_ratio_to_baseline": cold_ratios,
              "descriptive_lowest_online_k":
                  int(min(online_ratios, key=online_ratios.get))
                  if online_ratios else None,
              "descriptive_lowest_cold_k":
                  int(min(cold_ratios, key=cold_ratios.get))
                  if cold_ratios else None,
              "absolute_factor_base_status": "BOUNDED_UNKNOWN"}
    parent.write_json(output / "independent_replay.json", result)
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    result = verify(args.output.resolve())
    print(json.dumps({key: result[key] for key in
                      ("status", "source_verified", "host_verified",
                       "noise_gate_passed", "median_online_ms",
                       "median_cold_process_ms")}, sort_keys=True))
    return 0 if result["status"] == "PASS_TIER1_GRID_SCREEN" else 2


if __name__ == "__main__":
    raise SystemExit(main())
