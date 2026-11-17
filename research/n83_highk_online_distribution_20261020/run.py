#!/usr/bin/env python3
"""Produce the frozen N83 secondary screen and conditional one-target holdout."""

import argparse
import importlib.util
import json
import os
import shutil
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
if sys.path and Path(sys.path[0]).resolve() == HERE:
    sys.path.pop(0)
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from research.n83_highk_online_distribution_20261020 import arms, generate_fresh
from research.n83_highk_online_distribution_20261020 import select as screen_selector

SOURCE_SHA256 = arms.SOURCE_SHA256
SCREEN_TARGETS_SHA256 = screen_selector.TARGETS_SHA256
SCREEN_ANSWERS_SHA256 = "bfca94938ee5e97c73580fe41ef6ca206b96189379d48cbc88a831eba5b1b527"
HOLDOUT_TARGETS_SHA256 = "e7dcb0fe5779f847d381017b3cb5fb7dd46b6a4a7e834611d200d7cbd6244f68"
HOLDOUT_ANSWERS_SHA256 = "e7af0217b400053e7abb3c9f714d910a72a2e26b66aa3fa3f14f0c729a845607"
RHO_SEED_BASE = 2026120200
IC_CAP_SECONDS = {"k3072_root11": 900, "k3500_root4": 1200,
                  "k3300_wide3": 1200, "k4300_wide3": 1500,
                  "k4405_wide3": 1500}
RHO_CAP_SECONDS = 120
GLOBAL_PRODUCER_CAP_SECONDS = 400 * 60
RSS_CAP_BYTES = 24 * 1024**3


def module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    value = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(value)
    return value


panel = module("n83_distribution_panel",
               ROOT / "research/n83_smallk_challenge_20261017/run.py")
online_run = module("n83_distribution_online_run",
                    ROOT / "research/n83_online_finalists_20261018/run.py")
finegrid = panel.finegrid
finegrid.screen.SOURCE_SHA256 = SOURCE_SHA256
finegrid.screen.dynamic.SOURCE_SHA256 = SOURCE_SHA256
finegrid.screen.parent.EXPECTED[finegrid.screen.SOURCE] = SOURCE_SHA256
holdout_parent = panel.holdout
cold = panel.cold
parent = panel.parent
cold.RSS_CAP_BYTES = RSS_CAP_BYTES
parent.RSS_CAP_BYTES = RSS_CAP_BYTES


def fixture():
    hashes = finegrid.frozen_inputs()
    manifest = json.loads((HERE / "fresh_manifest.json").read_text())
    audit = json.loads((HERE / "fresh_exclusion_audit.json").read_text())
    if (manifest.get("schema") != "n83.online-distribution-fixture/v1" or
            manifest.get("generator_sha256") != parent.digest(HERE / "generate_fresh.py") or
            audit.get("schema") != "n83.online-distribution-exclusion/v1" or
            audit.get("status") != "PASS_ZERO_OVERLAP"):
        raise ValueError("fixture manifest or exclusion audit differs")
    streams = {}
    all_points = set()
    for kind, count, target_sha, answer_sha in (
            ("screen", 192, SCREEN_TARGETS_SHA256, SCREEN_ANSWERS_SHA256),
            ("holdout", 8, HOLDOUT_TARGETS_SHA256, HOLDOUT_ANSWERS_SHA256)):
        target_path = HERE / f"{kind}_targets.jsonl"
        answer_path = HERE / f"{kind}_answers.jsonl"
        if (parent.digest(target_path) != target_sha or
                parent.digest(answer_path) != answer_sha or
                manifest[kind]["point_count"] != count or
                manifest[kind]["targets_sha256"] != target_sha or
                manifest[kind]["answers_sha256"] != answer_sha or
                audit[kind]["targets_sha256"] != target_sha):
            raise ValueError(f"{kind} target or answer digest differs")
        lines = target_path.read_text().splitlines(keepends=True)
        answers = [json.loads(line) for line in answer_path.read_text().splitlines()]
        if len(lines) != count or len(answers) != count or any(
                not line.endswith("\n") for line in lines):
            raise ValueError(f"{kind} row count differs")
        points = []
        for index, (line, answer) in enumerate(zip(lines, answers)):
            seed = generate_fresh.hashlib.sha256(
                generate_fresh.DOMAINS[kind] + index.to_bytes(4, "little")).digest()
            scalar = 1 + int.from_bytes(seed, "little") % (generate_fresh.group.R - 1)
            point = generate_fresh.group.point_from_json(json.loads(line))
            if (answer["fixture_index"] != index or
                    answer["seed_sha256"] != seed.hex() or
                    int(answer["scalar"]) != scalar or
                    generate_fresh.group.multiply(generate_fresh.group.G, scalar) != point):
                raise ValueError(f"{kind} point {index} fails scalar replay")
            points.append(point)
        if len(set(points)) != count or all_points.intersection(points):
            raise ValueError("fixture points repeat")
        all_points.update(points)
        streams[kind] = lines
    return hashes, streams


def ic_environment(folder, arm):
    name, _, layout, _ = arm
    if layout in ("root11_dynamic", "root4_standard"):
        return online_run.ic_environment(folder, layout, name)
    manifest = arms.manifest(arm)[1]
    profile = manifest["implementation"]["algorithm_flags"]
    mapping = {
        "KIC_PACKED_ROOT_TABLE": "packed_root_table",
        "KIC_PACKED_ROOT_TABLE_DENSE": "dense_request",
        "KIC_PACKED_ROOT_TABLE_LOAD75": "load75_request",
        "KIC_STATE28": "compact_state28", "KIC_STATE26": "compact_state26",
        "KIC_ROOT_ONLY22": "root_only22", "KIC_ROOT_ONLY21": "root_only21",
        "KIC_ROOT_ONLY11": "root_only11",
        "KIC_INDEXED_ROOT5": "indexed_root5", "KIC_INDEXED_ROOT4": "indexed_root4",
        "KIC_ROOT_BLOOM": "root_bloom", "KIC_SYMMETRIC_S3": "symmetric_s3_states",
        "KIC_INDEXED5_LOAD75": "indexed5_load75_request",
        "KIC_INDEXED5_FIXED30": "indexed5_fixed30_request",
        "KIC_INDEXED5_ID30": "indexed5_id30_request",
        "KIC_INDEXED4_FIXED31": "indexed4_fixed31_request",
        "KIC_SPARSE_CANONICAL": "sparse_canonical",
        "KIC_TARGET_CANCEL": "target_cancel", "KIC_INDEX_ROOT_BATCH": "root_batch",
    }
    flags = {"PATH": os.environ.get("PATH", "/usr/bin:/bin")}
    flags.update({env: str(int(profile[key])) for env, key in mapping.items()})
    flags.update({"KIC_ROOT_BLOOM_PROFILE": profile["root_bloom_profile"],
                  "KIC_RANK_THREADS": str(profile["rank_threads"]),
                  "KIC_TARGET_THREADS": str(profile["target_threads"]),
                  "KIC_DUMP_BASE": str(folder / "base_dump.jsonl")})
    return flags


def screen_schedule():
    result = []
    for block in range(2):
        ordered = arms.ARMS if block == 0 else tuple(reversed(arms.ARMS))
        for arm in ordered:
            name = arm[0]
            for repetition in range(3):
                result.append({"phase": "screen", "block": block,
                               "candidate": name, "kind": "calibration",
                               "name": f"b{block}_cal_before_{repetition}_{name}"})
            result.append({"phase": "screen", "block": block,
                           "candidate": name, "kind": "ic",
                           "name": f"b{block}_{name}"})
            for repetition in range(3):
                result.append({"phase": "screen", "block": block,
                               "candidate": name, "kind": "calibration",
                               "name": f"b{block}_cal_after_{repetition}_{name}"})
        for index in range(block * 96, (block + 1) * 96):
            result.append({"phase": "screen", "block": block,
                           "point_index": index, "kind": "rho",
                           "name": f"rho_p{index:03d}"})
    if len(result) != 262:
        raise ValueError("screen event count differs")
    return result


def holdout_schedule(candidate):
    if candidate not in {arm[3] for arm in arms.ARMS}:
        raise ValueError("holdout candidate differs")
    name = next(arm[0] for arm in arms.ARMS if arm[3] == candidate)
    result = []
    for index in range(8):
        for repetition in range(3):
            result.append({"phase": "holdout", "point_index": index,
                           "candidate": name, "kind": "calibration",
                           "name": f"p{index:02d}_cal_before_{repetition}"})
        kinds = ("ic", "rho") if index % 2 == 0 else ("rho", "ic")
        for kind in kinds:
            result.append({"phase": "holdout", "point_index": index,
                           "candidate": name, "kind": kind,
                           "name": f"p{index:02d}_{kind}"})
        for repetition in range(3):
            result.append({"phase": "holdout", "point_index": index,
                           "candidate": name, "kind": "calibration",
                           "name": f"p{index:02d}_cal_after_{repetition}"})
    if len(result) != 64:
        raise ValueError("holdout event count differs")
    return result


def prepare(root, binary, rho_binary, started_unix):
    hashes, streams = fixture()
    selected, choice = arms.choose()
    if root.exists():
        raise ValueError("output directory already exists")
    root.mkdir(parents=True)
    group = parent.cgroup_root()
    host = parent.host_receipt(group)
    host["schema"] = "n83.online-distribution-host/v1"
    host["topology"] = holdout_parent.topology_receipt(host["visible_affinity"] or [])
    host["cpu_telemetry_before"] = holdout_parent.cpu_telemetry()
    parent.write_json(root / "host.json", host)
    memory_max = (host["cgroup_before"]["memory.max"] or "").strip()
    memory_too_small = memory_max.isdigit() and int(memory_max) < RSS_CAP_BYTES
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
    copied = root / "executed_bins"
    copied.mkdir()
    ic_copy, rho_copy = (copied / "koblitz_orbit_dlp_fast",
                         copied / "n83_rho_optimization_eval")
    shutil.copy2(binary, ic_copy)
    shutil.copy2(rho_binary, rho_copy)
    ic_copy.chmod(0o755)
    rho_copy.chmod(0o755)
    parent.write_json(root / "source.json", {
        "schema": "n83.online-distribution-source/v1",
        "hashes": hashes, "source_sha256": SOURCE_SHA256,
        "binary_sha256": parent.digest(ic_copy),
        "rho_binary_sha256": parent.digest(rho_copy),
        "runner_sha256": parent.digest(Path(__file__)),
        "screen_targets_sha256": SCREEN_TARGETS_SHA256,
        "screen_answers_sha256": SCREEN_ANSWERS_SHA256,
        "holdout_targets_sha256": HOLDOUT_TARGETS_SHA256,
        "holdout_answers_sha256": HOLDOUT_ANSWERS_SHA256,
        "fixture_manifest_sha256": parent.digest(HERE / "fresh_manifest.json"),
        "exclusion_audit_sha256": parent.digest(HERE / "fresh_exclusion_audit.json"),
        "arms": selected, "choice": choice,
        "screen_events": screen_schedule(),
        "rho_seed_rule": "2026120200+point_index",
        "calibration_seed": int(cold.RHO_SEED),
        "calibration_repetitions_per_side": 3,
        "calibration_comparison": "median_of_three_before_vs_after_20_percent",
        "calibration_target_sha256": parent.digest(cold.TARGETS),
        "calibration_answer_sha256": parent.digest(cold.ANSWERS),
        "global_producer_cap_seconds": GLOBAL_PRODUCER_CAP_SECONDS,
        "ic_cap_seconds": IC_CAP_SECONDS,
        "screen_mode": "secondary_shared_setup_per_point_intervals",
        "started_unix": started_unix,
    })
    for block in range(2):
        path = root / "screen" / f"block{block}" / "targets.jsonl"
        path.parent.mkdir(parents=True)
        path.write_text("".join(streams["screen"][block*96:(block+1)*96]))
    return group, ic_copy, rho_copy, streams


def run_event(root, group, ic_copy, rho_copy, event, streams):
    phase, kind = event["phase"], event["kind"]
    folder = root / phase / "runs" / event["name"]
    if kind == "ic":
        arm = next(item for item in arms.ARMS
                   if item[0] == event["candidate"])
        _, k, _, _ = arm
        target = (root / "screen" / f"block{event['block']}" / "targets.jsonl"
                  if phase == "screen" else
                  root / "holdout" / "points" /
                  f"p{event['point_index']:02d}" / "target.jsonl")
        command = [str(ic_copy), f"construct:83:1:{k}", str(target), "29",
                   str(folder / "targets_result.jsonl")]
        environment = ic_environment(folder, arm)
        cap = IC_CAP_SECONDS[event["candidate"]]
    else:
        if kind == "calibration":
            target = cold.TARGETS
            seed = int(cold.RHO_SEED)
        else:
            index = event["point_index"]
            target = root / phase / "points" / f"p{index:02d}" / "target.jsonl"
            target.parent.mkdir(parents=True, exist_ok=True)
            expected = streams[phase][index]
            if target.exists() and target.read_text() != expected:
                raise ValueError("per-point rho target differs")
            target.write_text(expected)
            seed = RHO_SEED_BASE + index if phase == "screen" else RHO_SEED_BASE + 192 + index
        command = [str(rho_copy), str(target), str(seed),
                   str(cold.RHO_CONFIG), str(folder / "receipt.json")]
        environment = {"PATH": os.environ.get("PATH", "/usr/bin:/bin")}
        cap = RHO_CAP_SECONDS
    print(f"starting {phase}/{event['name']} at {parent.now()}", flush=True)
    usage = holdout_parent.run_with_telemetry(folder, command,
                                              environment, cap, group)
    print(f"{phase}/{event['name']}: {usage['status']} after "
          f"{usage['wall_seconds']:.3f}s", flush=True)
    return usage


def run_screen(root, binary, rho_binary):
    started_unix = time.time()
    prepared = prepare(root, binary, rho_binary, started_unix)
    if prepared is None:
        return 2
    group, ic_copy, rho_copy, streams = prepared
    for event in screen_schedule():
        cap = IC_CAP_SECONDS[event["candidate"]] if event["kind"] == "ic" else RHO_CAP_SECONDS
        if time.time() - started_unix + cap + 30 > GLOBAL_PRODUCER_CAP_SECONDS:
            parent.write_json(root / "producer_stop.json",
                              {"status": "GLOBAL_CAP_BEFORE_NEXT_EVENT",
                               "next_event": event})
            break
        run_event(root, group, ic_copy, rho_copy, event, streams)
    parent.write_json(root / "screen" / "host_telemetry_after.json",
                      holdout_parent.cpu_telemetry())
    return 0


def run_holdout(root, candidate):
    source = json.loads((root / "source.json").read_text())
    selected = json.loads((root / "screen_selection.json").read_text())
    if (source["schema"] != "n83.online-distribution-source/v1" or
            source["source_sha256"] != SOURCE_SHA256 or
            candidate not in source["choice"]["ordered_candidate_ids"] or
            selected.get("schema") != "n83.online-distribution-screen-selection/v1" or
            selected.get("admission_status") != "PASS" or
            selected.get("status") != "SELECTED" or
            selected.get("selected_candidate_id") != candidate or
            not isinstance(source.get("started_unix"), (int, float)) or
            (root / "holdout").exists()):
        raise ValueError("holdout source, candidate or output differs")
    _, streams = fixture()
    group = parent.cgroup_root()
    executed = root / "executed_bins"
    ic_copy, rho_copy = (executed / "koblitz_orbit_dlp_fast",
                         executed / "n83_rho_optimization_eval")
    if (parent.digest(ic_copy) != source["binary_sha256"] or
            parent.digest(rho_copy) != source["rho_binary_sha256"]):
        raise ValueError("executed binaries differ")
    events = holdout_schedule(candidate)
    parent.write_json(root / "holdout_host_telemetry_before.json",
                      holdout_parent.cpu_telemetry())
    parent.write_json(root / "holdout_source.json", {
        "schema": "n83.online-distribution-holdout-source/v1",
        "candidate_id": candidate,
        "screen_selection_sha256": parent.digest(root / "screen_selection.json"),
        "events": events, "source_sha256": SOURCE_SHA256,
        "targets_sha256": HOLDOUT_TARGETS_SHA256,
        "answers_sha256": HOLDOUT_ANSWERS_SHA256,
    })
    for event in events:
        index = event["point_index"]
        path = root / "holdout" / "points" / f"p{index:02d}" / "target.jsonl"
        path.parent.mkdir(parents=True, exist_ok=True)
        if path.exists() and path.read_text() != streams["holdout"][index]:
            raise ValueError("holdout target differs")
        path.write_text(streams["holdout"][index])
        cap = IC_CAP_SECONDS[event["candidate"]] if event["kind"] == "ic" else RHO_CAP_SECONDS
        if time.time() - source["started_unix"] + cap + 30 > GLOBAL_PRODUCER_CAP_SECONDS:
            parent.write_json(root / "producer_stop.json",
                              {"status": "GLOBAL_CAP_BEFORE_NEXT_HOLDOUT_EVENT",
                               "next_event": event})
            break
        run_event(root, group, ic_copy, rho_copy, event, streams)
    parent.write_json(root / "holdout" / "host_telemetry_after.json",
                      holdout_parent.cpu_telemetry())
    return 0


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--phase", choices=("screen", "holdout"), required=True)
    parser.add_argument("--fixture-preflight", action="store_true")
    parser.add_argument("--binary", type=Path)
    parser.add_argument("--rho-binary", type=Path)
    parser.add_argument("--output", type=Path)
    parser.add_argument("--candidate-id")
    args = parser.parse_args()
    hashes, streams = fixture()
    if args.fixture_preflight:
        print(json.dumps({"hashes": hashes,
                          "screen_point_count": len(streams["screen"]),
                          "holdout_point_count": len(streams["holdout"]),
                          "screen_event_count": len(screen_schedule())},
                         sort_keys=True))
        return 0
    if not args.output:
        parser.error("--output is required")
    if args.phase == "screen":
        if not args.binary or not args.rho_binary:
            parser.error("screen needs --binary and --rho-binary")
        return run_screen(args.output.resolve(), args.binary.resolve(),
                          args.rho_binary.resolve())
    if not args.candidate_id:
        parser.error("holdout needs --candidate-id")
    return run_holdout(args.output.resolve(), args.candidate_id)


if __name__ == "__main__":
    raise SystemExit(main())
