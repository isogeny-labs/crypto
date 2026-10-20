#!/usr/bin/env python3
"""Produce the conditional 1,536-point N83 factor-base tie-break screen."""

import argparse
import hashlib
import json
import shutil
import time
from pathlib import Path

from research.n83_highk_online_distribution_20261020 import arms
from research.n83_highk_online_distribution_20261020 import run as base
from research.n83_highk_online_tiebreak_20261020 import generate_fresh, select

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SOURCE_SHA256 = select.SOURCE_SHA256
SCREEN_TARGETS_SHA256 = select.TARGETS_SHA256
SCREEN_ANSWERS_SHA256 = "f17dcdfbf14b06c1cc708c8d0f362bf45c3b299156221be6eb9c1e281fb71bc7"
IC_CAP_SECONDS = dict(base.IC_CAP_SECONDS)
RHO_CAP_SECONDS = base.RHO_CAP_SECONDS
GLOBAL_PRODUCER_CAP_SECONDS = 4 * 3600
RSS_CAP_BYTES = base.RSS_CAP_BYTES
parent = base.parent
holdout_parent = base.holdout_parent
cold = base.cold
ic_environment = base.ic_environment


def fixture():
    hashes = base.finegrid.frozen_inputs()
    manifest = json.loads((HERE / "fresh_manifest.json").read_text())
    audit = json.loads((HERE / "fresh_exclusion_audit.json").read_text())
    target_path = HERE / "screen_targets.jsonl"
    answer_path = HERE / "screen_answers.jsonl"
    if (manifest.get("schema") != "n83.online-tiebreak-fixture/v1" or
            manifest.get("point_count") != select.POINT_COUNT or
            manifest.get("domain") != generate_fresh.DOMAIN.decode() or
            manifest.get("generator_sha256") != parent.digest(
                HERE / "generate_fresh.py") or
            manifest.get("targets_sha256") != SCREEN_TARGETS_SHA256 or
            manifest.get("answers_sha256") != SCREEN_ANSWERS_SHA256 or
            audit.get("schema") != "n83.online-tiebreak-exclusion/v1" or
            audit.get("status") != "PASS_ZERO_OVERLAP" or
            audit.get("prior_unique_point_count") != 31800 or
            audit.get("screen", {}).get("prior_overlap_count") != 0 or
            audit.get("screen", {}).get("targets_sha256") !=
            SCREEN_TARGETS_SHA256 or
            parent.digest(target_path) != SCREEN_TARGETS_SHA256 or
            parent.digest(answer_path) != SCREEN_ANSWERS_SHA256):
        raise ValueError("tie-break fixture or exclusion audit differs")
    lines = target_path.read_text().splitlines(keepends=True)
    answers = [json.loads(line) for line in answer_path.read_text().splitlines()]
    if (len(lines) != select.POINT_COUNT or
            len(answers) != select.POINT_COUNT or
            any(not line.endswith("\n") for line in lines)):
        raise ValueError("tie-break fixture row count differs")
    points = []
    for index, (line, answer) in enumerate(zip(lines, answers)):
        seed = hashlib.sha256(generate_fresh.DOMAIN +
                              index.to_bytes(4, "little")).digest()
        scalar = 1 + int.from_bytes(seed, "little") % (generate_fresh.group.R - 1)
        point = generate_fresh.group.point_from_json(json.loads(line))
        if (answer.get("fixture_index") != index or
                answer.get("seed_sha256") != seed.hex() or
                int(answer.get("scalar", "0")) != scalar or
                generate_fresh.group.multiply(generate_fresh.group.G, scalar) != point):
            raise ValueError(f"tie-break point {index} fails scalar replay")
        points.append(point)
    if len(set(points)) != select.POINT_COUNT:
        raise ValueError("tie-break fixture repeats a point")
    return hashes, lines


def validate_trigger(path):
    receipt = json.loads(path.read_text())
    if (receipt.get("schema") != "n83.online-tiebreak-trigger/v1" or
            receipt.get("status") != "PASS_ADMITTED_BOUNDED_UNKNOWN" or
            receipt.get("candidate_ids") != list(select.IDS) or
            receipt.get("prior_source_sha256") != SOURCE_SHA256 or
            receipt.get("prior_screen_targets_sha256") !=
            "0617ee4c55b08e3b4379d99490cad9c249c50ef2b0cc6ab321fe63a0bb1ba0b3" or
            any(not isinstance(receipt.get(key), str) or
                len(receipt[key]) != 64 or
                any(ch not in "0123456789abcdef" for ch in receipt[key])
                for key in ("trigger_aggregate_sha256", "trigger_archive_sha256"))):
        raise ValueError("tie-break trigger receipt differs")
    return receipt


def screen_schedule():
    events = []
    for block in range(2):
        ordered = arms.ARMS if block == 0 else tuple(reversed(arms.ARMS))
        for arm in ordered:
            name = arm[0]
            for repetition in range(3):
                events.append({"phase": "screen", "block": block,
                               "candidate": name, "kind": "calibration",
                               "name": f"b{block}_cal_before_{repetition}_{name}"})
            events.append({"phase": "screen", "block": block,
                           "candidate": name, "kind": "ic",
                           "name": f"b{block}_{name}"})
            for repetition in range(3):
                events.append({"phase": "screen", "block": block,
                               "candidate": name, "kind": "calibration",
                               "name": f"b{block}_cal_after_{repetition}_{name}"})
    if len(events) != 70:
        raise ValueError("tie-break event count differs")
    return events


def prepare(root, binary, rho_binary, trigger_path, started_unix):
    hashes, lines = fixture()
    trigger = validate_trigger(trigger_path)
    chosen, choice = arms.choose()
    if root.exists():
        raise ValueError("tie-break output directory already exists")
    root.mkdir(parents=True)
    group = parent.cgroup_root()
    host = parent.host_receipt(group)
    host["schema"] = "n83.online-tiebreak-host/v1"
    host["topology"] = holdout_parent.topology_receipt(
        host["visible_affinity"] or [])
    host["cpu_telemetry_before"] = holdout_parent.cpu_telemetry()
    parent.write_json(root / "host.json", host)
    memory_max = (host["cgroup_before"]["memory.max"] or "").strip()
    memory_too_small = (memory_max.isdigit() and int(memory_max) <
                        RSS_CAP_BYTES + 256 * 1024**2)
    if (host["mem_total_kib"] is None or
            host["mem_total_kib"] * 1024 < RSS_CAP_BYTES + 256 * 1024**2 or
            host["disk_free_bytes"] < 3 * 1024**3 or
            len(host["visible_affinity"] or []) < 4 or memory_too_small):
        parent.write_json(root / "preflight.json",
                          {"status": "REJECTED_HOST_CAPACITY", "host": host})
        return None
    parent.write_json(root / "preflight.json",
                      {"status": "PASS_CAPACITY_PREFLIGHT",
                       "rss_cap_bytes": RSS_CAP_BYTES})
    copies = root / "executed_bins"
    copies.mkdir()
    ic_copy, rho_copy = (copies / "koblitz_orbit_dlp_fast",
                         copies / "n83_rho_optimization_eval")
    shutil.copy2(binary, ic_copy)
    shutil.copy2(rho_binary, rho_copy)
    ic_copy.chmod(0o755)
    rho_copy.chmod(0o755)
    shutil.copy2(trigger_path, root / "trigger.json")
    parent.write_json(root / "source.json", {
        "schema": "n83.online-tiebreak-source/v1",
        "hashes": hashes, "source_sha256": SOURCE_SHA256,
        "binary_sha256": parent.digest(ic_copy),
        "rho_binary_sha256": parent.digest(rho_copy),
        "runner_sha256": parent.digest(Path(__file__)),
        "generator_sha256": parent.digest(HERE / "generate_fresh.py"),
        "selector_sha256": parent.digest(HERE / "select.py"),
        "trigger_guard_sha256": parent.digest(HERE / "check_trigger.py"),
        "pod_launcher_sha256": parent.digest(HERE / "launch_on_pod.sh"),
        "screen_targets_sha256": SCREEN_TARGETS_SHA256,
        "screen_answers_sha256": SCREEN_ANSWERS_SHA256,
        "fixture_manifest_sha256": parent.digest(HERE / "fresh_manifest.json"),
        "exclusion_audit_sha256": parent.digest(HERE / "fresh_exclusion_audit.json"),
        "trigger_receipt_sha256": parent.digest(root / "trigger.json"),
        "trigger_aggregate_sha256": trigger["trigger_aggregate_sha256"],
        "trigger_archive_sha256": trigger["trigger_archive_sha256"],
        "arms": chosen, "choice": choice,
        "screen_events": screen_schedule(),
        "calibration_seed": int(cold.RHO_SEED),
        "calibration_repetitions_per_side": 3,
        "calibration_comparison": "median_of_three_before_vs_after_20_percent",
        "calibration_target_sha256": parent.digest(cold.TARGETS),
        "calibration_answer_sha256": parent.digest(cold.ANSWERS),
        "global_producer_cap_seconds": GLOBAL_PRODUCER_CAP_SECONDS,
        "ic_cap_seconds": IC_CAP_SECONDS,
        "screen_mode": "secondary_shared_setup_per_point_intervals_no_rho",
        "started_unix": started_unix,
    })
    for block in range(2):
        path = root / "screen" / f"block{block}" / "targets.jsonl"
        path.parent.mkdir(parents=True)
        path.write_text("".join(lines[block*768:(block+1)*768]))
    return group, ic_copy, rho_copy, lines


def run_screen(root, binary, rho_binary, trigger_path):
    started_unix = time.time()
    prepared = prepare(root, binary, rho_binary, trigger_path, started_unix)
    if prepared is None:
        return 2
    group, ic_copy, rho_copy, lines = prepared
    for event in screen_schedule():
        cap = (IC_CAP_SECONDS[event["candidate"]] if event["kind"] == "ic"
               else RHO_CAP_SECONDS)
        if time.time() - started_unix + cap + 30 > GLOBAL_PRODUCER_CAP_SECONDS:
            parent.write_json(root / "producer_stop.json",
                              {"status": "GLOBAL_CAP_BEFORE_NEXT_EVENT",
                               "next_event": event})
            break
        base.run_event(root, group, ic_copy, rho_copy, event, {"screen": lines})
    parent.write_json(root / "screen" / "host_telemetry_after.json",
                      holdout_parent.cpu_telemetry())
    return 0


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--fixture-preflight", action="store_true")
    parser.add_argument("--binary", type=Path)
    parser.add_argument("--rho-binary", type=Path)
    parser.add_argument("--trigger-receipt", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    _, lines = fixture()
    if args.fixture_preflight:
        print(json.dumps({"screen_point_count": len(lines),
                          "screen_event_count": len(screen_schedule()),
                          "screen_targets_sha256": SCREEN_TARGETS_SHA256},
                         sort_keys=True))
        return 0
    if not all((args.binary, args.rho_binary, args.trigger_receipt,
                args.output)):
        parser.error("screen needs --binary, --rho-binary, --trigger-receipt and --output")
    return run_screen(args.output.resolve(), args.binary.resolve(),
                      args.rho_binary.resolve(), args.trigger_receipt.resolve())


if __name__ == "__main__":
    raise SystemExit(main())
