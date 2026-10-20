#!/usr/bin/env python3
"""Independent arithmetic and resource replay shared by both N83 phases."""

import json
import math
import statistics
from pathlib import Path

from research.n83_highk_online_distribution_20261020 import arms, generate_fresh
from research.n83_highk_online_distribution_20261020 import run as runner

group = generate_fresh.group
RSS_CAP_BYTES = runner.RSS_CAP_BYTES
PHASE_KEYS = ("target_query_ms", "target_pdp_ms",
              "target_relation_check_ms", "target_descent_ms",
              "target_recovery_check_ms")


def read(path):
    return json.loads(path.read_text())


def counter(usage, section, key):
    def value(side):
        for line in (usage.get(side, {}).get(section) or "").splitlines():
            parts = line.split()
            if len(parts) == 2 and parts[0] == key:
                return int(parts[1])
        return None
    before, after = value("cgroup_before"), value("cgroup_after")
    return after - before if before is not None and after is not None else None


def v1_snapshot_consistent(snapshot):
    try:
        if snapshot["cgroup_interface"] != "v1":
            return False
        raw = snapshot["cgroup_v1_raw"]
        oom = dict(line.split(maxsplit=1) for line in
                   raw["memory.oom_control"].splitlines())
        quota = int(raw["cpu.cfs_quota_us"])
        period = int(raw["cpu.cfs_period_us"])
        failcnt = int(raw["memory.failcnt"])
        if (quota == -1 or quota < 4 * period or
                int(raw["memory.limit_in_bytes"]) <
                RSS_CAP_BYTES + 256 * 1024**2):
            return False
        return (snapshot["cpu.stat"] == raw["cpu.stat"] + "\n" and
                snapshot["cpu.max"] == f"{quota} {period}\n" and
                snapshot["memory.current"] == raw["memory.usage_in_bytes"] and
                snapshot["memory.peak"] == raw["memory.max_usage_in_bytes"] and
                snapshot["memory.max"] == raw["memory.limit_in_bytes"] and
                snapshot["memory.events"] ==
                f'oom {max(failcnt, int(oom["under_oom"]))}\n'
                f'oom_kill {int(oom["oom_kill"])}\n')
    except (KeyError, ValueError, TypeError, AttributeError):
        return False


def resources(usage):
    throttle = counter(usage, "cpu.stat", "nr_throttled")
    oom = counter(usage, "memory.events", "oom")
    kills = counter(usage, "memory.events", "oom_kill")
    snapshots = all(v1_snapshot_consistent(usage.get(name, {}))
                    for name in ("cgroup_before", "cgroup_after"))
    peak = usage.get("peak_rss_bytes")
    passed = (usage.get("status") == "PASS_PRODUCER" and snapshots and
              isinstance(peak, int) and 0 < peak <= RSS_CAP_BYTES and
              (throttle, oom, kills) == (0, 0, 0))
    return {"status": "PASS" if passed else "FAILED_OR_INCOMPLETE",
            "peak_rss_bytes": peak, "cpu_throttled_periods_delta": throttle,
            "oom_events_delta": oom, "oom_kills_delta": kills,
            "cgroup_v1_snapshots_consistent": snapshots}


def telemetry(phase_dir, name):
    path = phase_dir / "runs" / name / "host_telemetry.json"
    if not path.exists():
        return {"status": "MISSING"}
    receipt = read(path)
    before, after = receipt["before"], receipt["after"]
    valid = (len(before["cpu_jiffies"]) >= 8 and
             len(after["cpu_jiffies"]) >= 8 and
             before["steal_jiffies"] is not None and
             after["steal_jiffies"] is not None and
             after["steal_jiffies"] >= before["steal_jiffies"] and
             before["interrupts_total"] is not None and
             after["interrupts_total"] is not None and
             after["interrupts_total"] >= before["interrupts_total"] and
             before["frequency_khz_by_visible_cpu"] and
             set(before["frequency_khz_by_visible_cpu"]) ==
             set(after["frequency_khz_by_visible_cpu"]))
    return {"status": "PASS" if valid else "INVALID",
            "steal_jiffies_delta": (after["steal_jiffies"] -
                                    before["steal_jiffies"] if valid else None),
            "interrupts_delta": (after["interrupts_total"] -
                                  before["interrupts_total"] if valid else None)}


def replay_rho(folder, point, scalar, seed, target_file, source_hashes):
    try:
        if not (folder / "usage.json").exists():
            return {"status": "NOT_RUN"}
        usage = read(folder / "usage.json")
        if usage.get("status") != "PASS_PRODUCER":
            return {"status": usage.get("status", "FAILED"), "usage": usage}
        receipt = read(folder / "receipt.json")
        recovered = int(receipt["recovered_scalar"])
        online = float(receipt["online_ms"])
        if (receipt["status"] != "PASS_VERIFIED_PUBLIC_TARGET" or
                group.point_from_json(receipt["public_target_q"]) != point or
                recovered != scalar or receipt["walk_seed"] != seed or
                receipt["target_count"] != 1 or
                receipt["known_answer_input"] is not False or
                receipt["validation_sidecar_opened"] is not False or
                receipt["group_verified"] is not True or
                receipt["generic_scalar_replay_verified"] is not True or
                receipt["public_input_sha256"] != runner.parent.digest(target_file) or
                receipt["compiled_source_sha256"]["examples/n83_rho_optimization_eval.rs"] !=
                source_hashes["examples/n83_rho_optimization_eval.rs"] or
                group.multiply(group.G, recovered) != point or
                not math.isfinite(online) or online <= 0):
            raise ValueError("same-point rho scalar or source replay failed")
        return {"status": "PASS_INDEPENDENT_REPLAY", "online_ms": online,
                "resources": resources(usage),
                "full_child_wall_seconds": usage["wall_seconds"]}
    except Exception as error:
        return {"status": "VERIFICATION_FAILED", "error": str(error)}


def calibration(before, after):
    if len(before) != 3 or len(after) != 3:
        return {"status": "INCOMPLETE"}
    if any(item.get("status") != "PASS_INDEPENDENT_REPLAY" or
           item.get("resources", {}).get("status") != "PASS"
           for item in before + after):
        return {"status": "REPLAY_OR_RESOURCE_FAILED"}
    before_times = [item["online_ms"] for item in before]
    after_times = [item["online_ms"] for item in after]
    medians = (statistics.median(before_times),
               statistics.median(after_times))
    spread = abs(medians[0] - medians[1]) / statistics.mean(medians)
    return {"status": "PASS_20_PERCENT" if spread <= 0.20 else
            "NOISY_20_PERCENT", "before_ms": before_times,
            "after_ms": after_times, "relative_spread": spread}


def replay_ic(folder, arm, points, scalars):
    """Replay a 96-point screen child or a single-target holdout child."""
    try:
        if not (folder / "usage.json").exists():
            return {"status": "NOT_RUN"}
        usage = read(folder / "usage.json")
        if usage.get("status") != "PASS_PRODUCER":
            return {"status": usage.get("status", "FAILED"), "usage": usage}
        summary = read(folder / "stdout.jsonl")
        rows = [json.loads(line) for line in
                (folder / "targets_result.jsonl").read_text().splitlines() if line]
        name, k, layout, candidate_id = arm
        manifest_path, manifest = arms.manifest(arm)
        count = len(points)
        if (len(rows) != count or len(scalars) != count or
                summary.get("n") != 83 or summary.get("a") != 1 or
                summary.get("targets") != count or
                summary.get("targets_solved") != count or
                summary.get("targets_failed") != 0 or
                summary.get("orbit_columns") != k or
                summary.get("rank") != k or
                summary.get("factor_base_points") != 166 * k or
                summary.get("target_threads") != 2 or
                summary.get("target_cancel") is not True or
                summary.get("base_hash") !=
                manifest["factor_base"]["source_base_hash_blake3"] or
                runner.parent.digest(folder / "base_dump.jsonl") !=
                manifest["factor_base"]["ordered_base_dump_sha256"]):
            raise ValueError("exact base, rank or target count differs")
        slot = manifest["point_decomposition"]["root_slot"]
        bloom = manifest["point_decomposition"].get("root_bloom_filter")
        if "allocated_bytes" in slot:
            expected_table_bytes = slot["allocated_bytes"]
        else:
            rule = slot["request_rule"]
            if rule == "fixed_2pow29_request_2pow30_slots":
                expected_table_bytes = (1 << 30) * slot["bytes"]
            elif rule == "fixed_2pow30_request_2pow31_slots":
                expected_table_bytes = (1 << 31) * slot["bytes"]
            else:
                raise ValueError("unrecognized fixed root-table request rule")
        if (summary.get("root_table_slot_bytes") != slot["bytes"] or
                summary.get("root_table_allocated_bytes") != expected_table_bytes or
                summary.get("root_bloom_bytes") !=
                (bloom["allocated_bytes"] if bloom else 0)):
            raise ValueError("root index layout differs")
        base = group.replay_base(read(folder / "base_dump.jsonl"), summary)
        if len(base) != 166 * k:
            raise ValueError("factor-base replay count differs")
        checked = []
        for local, (row, point, scalar) in enumerate(zip(rows, points, scalars)):
            indices = row.get("point_indices")
            if (row.get("fixture_index") != local or
                    group.point_from_json(row["target"]) != point or
                    int(row["recovered_scalar"]) != scalar or
                    row.get("group_verified") is not True or
                    row.get("target_generation_ms_excluded") != 0.0 or
                    group.multiply(group.G, scalar) != point or
                    not isinstance(indices, list) or len(indices) != 4 or
                    any(not isinstance(i, int) or i < 0 or i >= len(base)
                        for i in indices)):
                raise ValueError(f"target row {local} scalar or indices differ")
            relation = None
            for index in indices:
                relation = group.add(relation, base[index])
            if relation != point:
                raise ValueError(f"target row {local} four-point sum differs")
            phases = {key: float(row[key]) for key in PHASE_KEYS}
            online = float(row["target_ms"])
            if (not math.isfinite(online) or online <= 0 or
                    any(not math.isfinite(value) or value < 0
                        for value in phases.values()) or
                    abs(math.fsum(phases.values()) - online) > 1):
                raise ValueError(f"target row {local} exclusive phases differ")
            checked.append({"status": "PASS_INDEPENDENT_REPLAY",
                            "candidate_id": candidate_id,
                            "one_target_online_ms": online,
                            "online_phases_ms": phases,
                            "target_probes": row["probes"]})
        timing = summary["timing_ms"]
        cold = float(timing["process_total"])
        setup = float(timing["setup_total"])
        rank = float(timing["rank_stage"])
        targets = float(timing["targets_total"])
        residual = cold - setup - rank - targets
        if (not all(math.isfinite(v) and v >= 0 for v in
                    (cold, setup, rank, targets)) or
                not 0 <= residual <= 1000 or
                not 0 <= targets - sum(x["one_target_online_ms"] for x in checked) <= 1000):
            raise ValueError("cold phase accounting differs")
        return {"status": "PASS_INDEPENDENT_REPLAY",
                "candidate_id": candidate_id, "k": k, "layout": layout,
                "manifest_sha256": arms.digest(manifest_path),
                "base_hash": summary["base_hash"],
                "root_table_entries": summary["root_table_entries"],
                "root_table_allocated_bytes": summary["root_table_allocated_bytes"],
                "cold_process_ms": cold, "setup_ms": setup,
                "rank_stage_ms": rank, "targets_total_ms": targets,
                "cold_unattributed_ms": residual,
                "resources": resources(usage), "targets": checked}
    except Exception as error:
        return {"status": "VERIFICATION_FAILED", "error": str(error)}
