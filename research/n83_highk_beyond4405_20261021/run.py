#!/usr/bin/env python3
"""Run the sealed fixed31 secondary screen on one Tier 1 allocation."""

import argparse
import hashlib
import json
import os
import shutil
import time
from pathlib import Path

from research.n83_highk_beyond4405_20261021 import generate_fresh, seal_dispatch
from research.n83_highk_online_distribution_20261020 import run as prior

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
parent = prior.parent
telemetry = prior.holdout_parent
cold = prior.cold
IC_CAP_SECONDS = 2400
RHO_CAP_SECONDS = 120
GLOBAL_CAP_SECONDS = 10 * 3600
RSS_CAP_BYTES = 26 * 1024**3
parent.RSS_CAP_BYTES = RSS_CAP_BYTES
cold.RSS_CAP_BYTES = RSS_CAP_BYTES

FLAG_ENV = {
    "packed_root_table": "KIC_PACKED_ROOT_TABLE",
    "dense_request": "KIC_PACKED_ROOT_TABLE_DENSE",
    "load75_request": "KIC_PACKED_ROOT_TABLE_LOAD75",
    "compact_state28": "KIC_STATE28",
    "compact_state26": "KIC_STATE26",
    "root_only22": "KIC_ROOT_ONLY22",
    "root_only21": "KIC_ROOT_ONLY21",
    "root_only11": "KIC_ROOT_ONLY11",
    "indexed_root5": "KIC_INDEXED_ROOT5",
    "indexed_root4": "KIC_INDEXED_ROOT4",
    "root_bloom": "KIC_ROOT_BLOOM",
    "symmetric_s3_states": "KIC_SYMMETRIC_S3",
    "indexed5_load75_request": "KIC_INDEXED5_LOAD75",
    "indexed5_load80_request": "KIC_INDEXED5_LOAD80",
    "indexed5_fixed30_request": "KIC_INDEXED5_FIXED30",
    "indexed5_fixed31_request": "KIC_INDEXED5_FIXED31",
    "indexed5_id30_request": "KIC_INDEXED5_ID30",
    "indexed5_id31_request": "KIC_INDEXED5_ID31",
    "indexed4_fixed31_request": "KIC_INDEXED4_FIXED31",
    "sparse_canonical": "KIC_SPARSE_CANONICAL",
    "target_cancel": "KIC_TARGET_CANCEL",
    "root_batch": "KIC_INDEX_ROOT_BATCH",
    "indexed5_split_shared_bloom": "KIC_INDEXED5_SPLIT_SHARED_BLOOM",
}


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def fixture() -> tuple[dict, dict[str, list[str]]]:
    dispatch = seal_dispatch.seal()
    manifest = json.loads((HERE / "fresh_manifest.json").read_text())
    if (digest(HERE / "generate_fresh.py") != manifest["generator_sha256"] or
            digest(HERE / "fresh_manifest.json") !=
            dispatch["fresh_manifest_sha256"]):
        raise ValueError("fixture generator or manifest differs")
    streams = {}
    seen = set()
    for kind, count in (("screen", 1536), ("holdout", 16)):
        target_path = HERE / f"{kind}_targets.jsonl"
        answer_path = HERE / f"{kind}_answers.jsonl"
        lines = target_path.read_text().splitlines(keepends=True)
        answers = [json.loads(line) for line in answer_path.read_text().splitlines()]
        if (len(lines) != count or len(answers) != count or
                any(not line.endswith("\n") for line in lines) or
                digest(target_path) != manifest[kind]["targets_sha256"] or
                digest(answer_path) != manifest[kind]["answers_sha256"]):
            raise ValueError(f"{kind} fixture count or digest differs")
        for index, (line, answer) in enumerate(zip(lines, answers)):
            seed = hashlib.sha256(
                generate_fresh.DOMAINS[kind] + index.to_bytes(4, "little")
            ).digest()
            scalar = 1 + int.from_bytes(seed, "little") % (generate_fresh.group.R - 1)
            point = generate_fresh.group.point_from_json(json.loads(line))
            if (answer["fixture_index"] != index or
                    answer["seed_sha256"] != seed.hex() or
                    int(answer["scalar"]) != scalar or
                    point in seen or
                    generate_fresh.group.multiply(generate_fresh.group.G, scalar) != point):
                raise ValueError(f"{kind} point {index} fails independent replay")
            seen.add(point)
        streams[kind] = lines
    return dispatch, streams


def events(dispatch: dict) -> list[dict]:
    result = []
    for block, row in enumerate(dispatch["screen_blocks"]):
        expected = [block * 768, block * 768 + 767]
        if row["indices"] != expected:
            raise ValueError("screen block differs")
        for candidate_id in row["candidate_ids"]:
            arm = next((a for a in dispatch["arms"]
                        if a["candidate_id"] == candidate_id), None)
            if arm is None:
                raise ValueError("unknown screen arm")
            name = f"b{block}_k{arm['k']}_{'fixed31' if arm['fixed31'] else 'load75'}"
            for side in ("before", "after"):
                if side == "after":
                    result.append({"kind": "ic", "block": block,
                                   "name": name, "candidate_id": candidate_id})
                for rep in range(3):
                    result.append({"kind": "calibration", "block": block,
                                   "name": f"{name}_cal_{side}_{rep}",
                                   "candidate_id": candidate_id})
    if len(result) != 56:
        raise ValueError("screen event count differs")
    return result


def ic_environment(folder: Path, arm: dict) -> dict[str, str]:
    path = HERE / "identities" / f"candidate_{arm['candidate_id']}.json"
    record = json.loads(path.read_text())
    if (digest(path) != arm["candidate_manifest_sha256"] or
            record["candidate_id"] != arm["candidate_id"]):
        raise ValueError("candidate manifest differs")
    flags = record["implementation"]["algorithm_flags"]
    env = {"PATH": os.environ.get("PATH", "/usr/bin:/bin")}
    for key, name in FLAG_ENV.items():
        if key not in flags or not isinstance(flags[key], bool):
            raise ValueError(f"missing Boolean flag {key}")
        env[name] = str(int(flags[key]))
    if flags["indexed5_split_at"] is not None:
        raise ValueError("fixed31 screen forbids split index")
    env.update({
        "KIC_ROOT_BLOOM_PROFILE": flags["root_bloom_profile"],
        "KIC_RANK_THREADS": str(flags["rank_threads"]),
        "KIC_TARGET_THREADS": str(flags["target_threads"]),
        "KIC_DUMP_BASE": str(folder / "base_dump.jsonl"),
    })
    return env


def prepare(output: Path, binary: Path, rho_binary: Path,
            dispatch: dict, streams: dict[str, list[str]]) -> tuple:
    if output.exists():
        raise ValueError("output already exists")
    output.mkdir(parents=True)
    group = parent.cgroup_root()
    host = parent.host_receipt(group)
    host["schema"] = "n83.fixed31-tier1-host/v1"
    host["topology"] = telemetry.topology_receipt(host["visible_affinity"] or [])
    host["cpu_telemetry_before"] = telemetry.cpu_telemetry()
    parent.write_json(output / "host.json", host)
    memory_max = (host["cgroup_before"]["memory.max"] or "").strip()
    quota, period = (host["cgroup_before"]["cpu.max"] or "").split()
    if (len(host["visible_affinity"] or []) != 4 or
            quota == "max" or int(quota) < 4 * int(period) or
            not memory_max.isdigit() or int(memory_max) < 32 * 1024**3 or
            host["mem_total_kib"] is None or
            host["mem_total_kib"] * 1024 < 32 * 1024**3 or
            host["disk_free_bytes"] < 3 * 1024**3):
        parent.write_json(output / "preflight.json",
                          {"status": "REJECTED_TIER1_CAPACITY", "host": host})
        return None
    parent.write_json(output / "preflight.json",
                      {"status": "PASS_TIER1_CAPACITY",
                       "rss_cap_bytes": RSS_CAP_BYTES})
    copies = output / "executed_bins"
    copies.mkdir()
    ic_copy = copies / "koblitz_orbit_dlp_fast"
    rho_copy = copies / "n83_rho_optimization_eval"
    shutil.copy2(binary, ic_copy)
    shutil.copy2(rho_binary, rho_copy)
    ic_copy.chmod(0o755)
    rho_copy.chmod(0o755)
    parent.write_json(output / "source.json", {
        "schema": "n83.fixed31-tier1-source/v1",
        "source_sha256": digest(ROOT / "examples/koblitz_orbit_dlp_fast.rs"),
        "rho_source_sha256": digest(ROOT / "examples/n83_rho_optimization_eval.rs"),
        "cargo_lock_sha256": digest(ROOT / "Cargo.lock"),
        "binary_sha256": digest(ic_copy),
        "rho_binary_sha256": digest(rho_copy),
        "runner_sha256": digest(Path(__file__)),
        "dispatch_sha256": digest(HERE / "dispatch.json"),
        "candidate_catalog_sha256": dispatch["candidate_catalog_sha256"],
        "screen_targets_sha256": dispatch["screen_targets_sha256"],
        "holdout_targets_sha256": dispatch["holdout_targets_sha256"],
        "screen_events": events(dispatch),
        "started_unix": time.time(),
    })
    for block in range(2):
        path = output / "screen" / f"block{block}" / "targets.jsonl"
        path.parent.mkdir(parents=True)
        path.write_text("".join(streams["screen"][block * 768:(block + 1) * 768]))
    return group, ic_copy, rho_copy


def run_screen(output: Path, binary: Path, rho_binary: Path) -> int:
    dispatch, streams = fixture()
    if digest(ROOT / "examples/koblitz_orbit_dlp_fast.rs") != dispatch["source_sha256"]:
        raise ValueError("source differs from frozen dispatch")
    prepared = prepare(output, binary, rho_binary, dispatch, streams)
    if prepared is None:
        return 2
    group, ic_copy, rho_copy = prepared
    started = time.time()
    for event in events(dispatch):
        cap = IC_CAP_SECONDS if event["kind"] == "ic" else RHO_CAP_SECONDS
        if time.time() - started + cap + 30 > GLOBAL_CAP_SECONDS:
            parent.write_json(output / "producer_stop.json",
                              {"status": "GLOBAL_CAP_BEFORE_NEXT_EVENT",
                               "next_event": event})
            break
        folder = output / "screen" / "runs" / event["name"]
        if event["kind"] == "ic":
            arm = next(a for a in dispatch["arms"]
                       if a["candidate_id"] == event["candidate_id"])
            target = output / "screen" / f"block{event['block']}" / "targets.jsonl"
            command = [str(ic_copy), f"construct:83:1:{arm['k']}",
                       str(target), "29", str(folder / "targets_result.jsonl")]
            env = ic_environment(folder, arm)
        else:
            command = [str(rho_copy), str(cold.TARGETS),
                       str(cold.RHO_SEED), str(cold.RHO_CONFIG),
                       str(folder / "receipt.json")]
            env = {"PATH": os.environ.get("PATH", "/usr/bin:/bin")}
        print(f"starting screen/{event['name']} at {parent.now()}", flush=True)
        usage = telemetry.run_with_telemetry(folder, command, env, cap, group)
        print(f"screen/{event['name']}: {usage['status']} after "
              f"{usage['wall_seconds']:.3f}s", flush=True)
    parent.write_json(output / "screen" / "host_telemetry_after.json",
                      telemetry.cpu_telemetry())
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--fixture-preflight", action="store_true")
    parser.add_argument("--binary", type=Path)
    parser.add_argument("--rho-binary", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    dispatch, streams = fixture()
    if args.fixture_preflight:
        print(json.dumps({"candidate_ids": [a["candidate_id"]
                                             for a in dispatch["arms"]],
                          "screen_point_count": len(streams["screen"]),
                          "holdout_point_count": len(streams["holdout"]),
                          "screen_event_count": len(events(dispatch))},
                         sort_keys=True))
        return 0
    if not all((args.binary, args.rho_binary, args.output)):
        parser.error("screen needs --binary, --rho-binary and --output")
    return run_screen(args.output.resolve(), args.binary.resolve(),
                      args.rho_binary.resolve())


if __name__ == "__main__":
    raise SystemExit(main())
