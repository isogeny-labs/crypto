#!/usr/bin/env python3
"""Replay the raw hosted six-summand access ladder, independently of its runner."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys

import blake3

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
STUDY = HERE.parents[1]
PROTOCOL = STUDY / "verification/hosted-m6-witness-ladder-20261017"
sys.path.insert(0, str(PROTOCOL))
sys.path.insert(0, str(STUDY))
import prepare  # noqa: E402
from chain_search_supervisor import field_mul  # noqa: E402

RUN_ID = "39062976388"
SOURCE = "3f43c2d16230b0b55722c01e7a6993b4c4b47b50"
FIXTURE_SHA256 = "76b78ec1a68807d130fc11e54e4220b5cde619d677316e313dc247869f727a9d"
IMAGE = "rust:1.98.1-bookworm@sha256:93ce27a88655056a51dbdd8f5f2d7ddc071c7b0070fb288a37b5a285fc83971e"
STAGES = (("all_x", 600, 5000, 10), ("summand_x", 900, 10000, 6),
          ("open", 1800, 20000, 0))


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read(path: Path) -> dict:
    result = json.loads(path.read_text())
    require(isinstance(result, dict), f"expected JSON object: {path}")
    return result


def same(row: dict, expected: dict, label: str) -> None:
    for key, value in expected.items():
        require(type(row.get(key)) is type(value) and row[key] == value,
                f"{label}: {key} differs")


def residual(x: int, y: int, z: int) -> int:
    xy = field_mul(x, y)
    pair = xy ^ field_mul(x, z) ^ field_mul(y, z)
    return field_mul(xy, z) ^ field_mul(pair, pair) ^ 1


def check_s3(arm: dict) -> None:
    xs = list(map(int, arm["ordered_xs"]))
    partial = list(map(int, arm["partial_xs"]))
    target = int(arm["target_x"])
    nodes = ((xs[0], xs[1], partial[0]),
             (partial[0], xs[2], partial[1]),
             (partial[1], xs[3], partial[2]),
             (partial[2], xs[4], partial[3]),
             (partial[3], xs[5], target))
    require(len(nodes) == 5 and all(residual(*node) == 0 for node in nodes),
            "original-field S3 witness differs")


def check_container(stage: dict, inspected: dict, role: str, name: str) -> None:
    command = stage["command"]
    require(command[:2] == ["docker", "run"] and command.count("--mount") == 1
            and command[-5:] == [IMAGE, "/worker", role, name,
                                  str(stage["conflict_cap"])]
            and "--network" in command and command[command.index("--network") + 1] == "none"
            and "--read-only" in command and "--cap-drop" in command
            and command[command.index("--cap-drop") + 1] == "ALL",
            "Docker command differs")
    mount = command[command.index("--mount") + 1]
    require(mount.startswith("type=bind,src=")
            and mount.endswith("/n83-m6-witness-setup/worker,dst=/worker,readonly"),
            "worker mount differs")
    require(isinstance(inspected, list) and len(inspected) == 1,
            "container inspection missing")
    container = inspected[0]
    host = container["HostConfig"]
    actual = [m for m in container["Mounts"] if m["Type"] == "bind"]
    require(len(actual) == 1 and actual[0]["Destination"] == "/worker"
            and actual[0]["RW"] is False
            and host["Memory"] == 4294967296
            and host["MemorySwap"] == 4294967296
            and host["NanoCpus"] == 1000000000
            and host["PidsLimit"] == 64
            and host["NetworkMode"] == "none"
            and host["ReadonlyRootfs"] is True
            and [c.upper() for c in host["CapDrop"]] == ["ALL"]
            and container["State"]["OOMKilled"] is False,
            "inspected limits or mount differ")


def audit(root: Path) -> dict:
    setup = root / "n83-m6-witness-setup"
    output = root / "n83-m6-witness"
    archives = root / "n83-archives"
    fixture = read(PROTOCOL / "fixture.json")
    require(digest(PROTOCOL / "fixture.json") == FIXTURE_SHA256,
            "frozen fixture differs")
    require((setup / "source-commit.txt").read_text().strip() == SOURCE
            and (setup / "source-status.txt").read_text() == "",
            "hosted source identity or cleanliness differs")
    lines = (setup / "sources.sha256").read_text().splitlines()
    require(len(lines) == 7, "source list count differs")
    for line in lines:
        expected, relative = line.split("  ", 1)
        require(not Path(relative).is_absolute(), "absolute source path")
        source = subprocess.check_output(["git", "show", f"{SOURCE}:{relative}"],
                                         cwd=ROOT)
        require(hashlib.sha256(source).hexdigest() == expected,
                f"pinned source differs: {relative}")
    config = read(output / "config.json")
    same(config, {"schema": "n83.m6-witness-ladder-config/v1",
                  "source_commit": SOURCE, "run_id": RUN_ID,
                  "fixture_sha256": FIXTURE_SHA256, "image": IMAGE,
                  "worker_sha256": digest(setup / "worker"),
                  "conservative_full_charge_seconds": 10800,
                  "selected_best_total_runtime": None}, "config")
    require((setup / "worker.sha256").read_text().startswith(
            config["worker_sha256"] + "  "), "worker hash receipt differs")
    require("test result: ok. 4084 passed; 0 failed; 117 ignored" in
            (setup / "release-lib.stdout").read_text()
            and "test result: ok. 0 passed; 0 failed" in
            (setup / "example-test.stdout").read_text(),
            "release validation logs differ")
    image_id = (setup / "image-identity.txt").read_text().strip().split(" ", 3)
    require(len(image_id) == 4 and image_id[1:3] == ["arm64", "linux"]
            and any(item.endswith("@" + IMAGE.split("@", 1)[1])
                    for item in json.loads(image_id[3])),
            "pinned image differs")
    regenerated = read(setup / "regeneration.json")
    require(regenerated["source_commit"] == SOURCE
            and regenerated["pilot_completed_base_count"] == 54,
            "pilot regeneration identity differs")
    archive_hashes = {}
    for arm in fixture["arms"]:
        check_s3(arm)
        for key in ("base_object_blake3", "pool_object_blake3"):
            identity = arm[key]
            path = archives / f"{identity}.jsonl.gz"
            require(blake3.blake3(path.read_bytes()).hexdigest() == identity,
                    "compressed base BLAKE3 differs")
            archive_hashes[path.name] = digest(path)
    require(regenerated["archive_sha256"] == archive_hashes,
            "regenerated archive hashes differ")
    require(prepare.build(archives) == fixture,
            "original-group base membership or planted fixture differs")
    observed = []
    for arm in fixture["arms"]:
        role = arm["role"]
        for name, wall_cap, conflict_cap, fixed in STAGES:
            directory = output / role / name
            stage = read(directory / "stage.json")
            worker = read(directory / "solve.stdout")
            same(stage, {"schema": "n83.m6-witness-ladder-stage/v1",
                         "role": role, "stage": name, "wall_cap_seconds": wall_cap,
                         "conflict_cap": conflict_cap, "exit_code": 0,
                         "timed_out": False, "inspect_exit_code": 0,
                         "cleanup_exit_code": 0}, "stage")
            require(stage["stdout_sha256"] == digest(directory / "solve.stdout")
                    and stage["stderr_sha256"] == digest(directory / "solve.stderr")
                    and 0 <= stage["elapsed_wall_seconds_L0"] <= wall_cap + 30,
                    "stage bytes or wall counter differ")
            check_container(stage, json.loads((directory / "container.inspect.json").read_text()),
                            role, name)
            expected_status = ("SAT_verified_planted_x" if name == "all_x"
                               else "UNKNOWN_conflict_cap")
            same(worker, {"schema": "n83.m6-planted-witness-control/v1",
                          "role": role, "stage": name, "status": expected_status,
                          "fixture_blake3": blake3.blake3((PROTOCOL / "fixture.json").read_bytes()).hexdigest(),
                          "base_object_blake3": arm["base_object_blake3"],
                          "pool_object_blake3": arm["pool_object_blake3"],
                          "target": arm["target"], "fixed_x_blocks": fixed,
                          "conflict_cap": conflict_cap, "identity_mask": 0,
                          "natural_relations": None,
                          "selected_best_total_runtime": None}, "worker")
            require(stage["status"] == expected_status
                    and worker["model_verified"] is (name == "all_x")
                    and 0 <= worker["conflicts"] <= conflict_cap
                    and (name == "all_x" or worker["conflicts"] == conflict_cap)
                    and worker["domain_clauses"] == (237280 if role == "gray_pool" else 323796),
                    "solver disposition or shape differs")
            observed.append({"role": role, "stage": name,
                             "status": expected_status, "conflicts": worker["conflicts"],
                             "elapsed_wall_seconds_L0": stage["elapsed_wall_seconds_L0"]})
    outer = read(output / "outer.json")
    same(outer, {"schema": "n83.m6-witness-ladder-outer/v1",
                 "status": "BOUNDED_PARTIAL_six_stage_access_ladder",
                 "source_commit": SOURCE, "run_id": RUN_ID,
                 "conservative_full_charge_seconds": 10800,
                 "natural_relations": None, "matrix_rank": None,
                 "total_index_calculus_runtime_ms": None,
                 "selected_best_total_runtime": None}, "outer")
    require([(s["role"], s["stage"], s["status"]) for s in outer["stages"]]
            == [(s["role"], s["stage"], s["status"]) for s in observed],
            "outer stage order differs")
    files = sorted(p for p in root.rglob("*") if p.is_file())
    tree = "".join(f"{p.relative_to(root)}  {digest(p)}\n" for p in files)
    return {"schema": "n83.m6-witness-ladder-audit/v1", "status": "PASS_BOUNDED_ACCESS_REPLAY",
            "source_commit": SOURCE, "run_id": RUN_ID, "file_count": len(files),
            "artifact_tree_sha256": hashlib.sha256(tree.encode()).hexdigest(),
            "fixture_sha256": FIXTURE_SHA256, "archive_sha256": archive_hashes,
            "stages": observed, "conservative_full_charge_seconds": 10800,
            "selected_best_total_runtime": None}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--artifact", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(audit(args.artifact), indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
