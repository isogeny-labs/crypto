#!/usr/bin/env python3
"""Fail-closed independent custody and group replay for one K2048 search."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
from pathlib import Path
import subprocess
import sys

import blake3

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parents[1]))
import chain_search_supervisor as search  # noqa: E402

OBJECT_BLAKE3 = "82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851"
OBJECT_SHA256 = "aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a"


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def sha256(path: Path) -> str:
    state = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            state.update(block)
    return state.hexdigest()


def b3(path: Path) -> str:
    state = blake3.blake3()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            state.update(block)
    return state.hexdigest()


def read(path: Path) -> dict:
    return json.loads(path.read_text())


def audit(checkout: Path, frozen: Path, run: Path, binary: Path) -> dict:
    original = read(frozen / "input.json")
    panel, fixtures = frozen / "panel", frozen / "fixtures"
    object_path = panel / "objects" / f"{OBJECT_BLAKE3}.jsonl.gz"
    require(original["status"] == "PASS_FRESH_S3_INPUT"
            and original["object_blake3"] == OBJECT_BLAKE3
            and original["object_sha256"] == OBJECT_SHA256
            and original["head_etag"] == original["get_etag"]
            and sha256(object_path) == OBJECT_SHA256
            and b3(object_path) == OBJECT_BLAKE3,
            "frozen S3 K2048 object differs")
    for name, digest in original["panel_sha256"].items():
        require(sha256(panel / name) == digest, f"panel {name} differs")
    for name, digest in original["fixture_sha256"].items():
        require(sha256(fixtures / name) == digest, f"fixture {name} differs")
    manifest = read(panel / "manifest.json")
    row = manifest["bases"][0]
    require(len(manifest["bases"]) == 1 and row["a"] == 0
            and row["columns"] == 2048 and row["policy"] == "public_x_hash"
            and row["seed"] == 2026100801
            and row["compressed_blake3"] == OBJECT_BLAKE3
            and row["s3_uri"] == original["s3_uri"],
            "selected manifest row differs")
    config, outer = read(run / "config.json"), read(run / "outer.json")
    require(config["schema"] == "n83.chain-s3-search-config/v1"
            and config["columns"] == 2048
            and config["policy"] == "public_x_hash"
            and config["seed"] == 2026100801
            and config["summands"] == 5 and config["max_trials"] == 1
            and config["domain_encoding"] == "frobenius_orbit_phase"
            and config["max_variables"] == 150000
            and config["max_domain_clauses"] == 1000000
            and config["max_models"] == 32
            and config["conflict_budget"] == 100000
            and config["wall_seconds"] == 5400
            and config["worker_wall_seconds"] == 5400
            and config["memory_cgroup_limit_bytes"] == 4 * 1024**3
            and config["memory_cgroup_swap_limit_bytes"] == 0
            and config["cpu_limit"] == 1.0
            and config["selected_object_sha256"] == OBJECT_SHA256
            and config["selected_point_set_blake3"] == row["point_set_blake3"]
            and config["binary_sha256"] == sha256(binary),
            "source, base or search caps differ")
    for name, digest in config["panel_sha256"].items():
        require(sha256(panel / name) == digest, f"run panel {name} differs")
    for name, digest in config["fixture_sha256"].items():
        require(sha256(fixtures / name) == digest, f"run fixture {name} differs")
    commit = config["source_commit"]
    require(len(commit) == 40 and all(c in "0123456789abcdef" for c in commit)
            and outer["config_sha256"] == sha256(run / "config.json"),
            "source commit or configuration custody differs")
    for relative in search.SOURCES:
        snapshot = run / "source" / relative
        committed = subprocess.check_output(["git", "show", f"{commit}:{relative}"],
                                            cwd=checkout)
        require(hashlib.sha256(committed).hexdigest() == sha256(snapshot)
                == config["source_sha256"][relative],
                f"source snapshot {relative} differs")
    attestation = read(run / "source/attestation.json")
    require(attestation == {"schema": search.ATTESTATION_SCHEMA,
                            "source_commit": commit, "status_clean": True}
            and sha256(run / "source/attestation.json")
            == config["source_sha256"]["attestation.json"],
            "source attestation differs")
    live = read(run / "container-inspect.json")
    host = live["host_config"]
    require(live["schema"] == "n83.chain-natural-container-inspection/v1"
            and live["container_name"] == config["container_name"]
            and live["state_running_at_inspection"] is True
            and live["image_id"] == config["container_image_id"]
            and host["Memory"] == 4 * 1024**3
            and host["MemorySwap"] == host["Memory"]
            and host["NanoCpus"] == 1_000_000_000
            and host["NetworkMode"] == "none"
            and host["ReadonlyRootfs"] is True
            and host["CapDrop"] == ["ALL"]
            and host["SecurityOpt"] == ["no-new-privileges"]
            and host["PidsLimit"] == 64,
            "live Docker hard-limit inspection differs")
    mounts = {item["Destination"]: item for item in live["mounts"]}
    require(set(mounts) == {"/panel", "/fixtures", "/worker", "/source", "/out"}
            and all(item["Type"] == "bind" for item in mounts.values())
            and all(not mounts[name]["RW"] for name in
                    ("/panel", "/fixtures", "/worker", "/source"))
            and mounts["/out"]["RW"] is True,
            "Docker input mount permissions differ")
    require(outer["schema"] == "n83.chain-s3-search-outer/v1"
            and outer["source_changed"] is False
            and outer["config_changed"] is False
            and outer["binary_changed"] is False
            and outer["input_changed"] is False
            and outer["cleanup_error"] is None
            and outer["launch_error"] is None
            and outer["total_index_calculus_runtime_ms"] is None
            and outer["selected_best_total_runtime"] is None
            and outer["stdout_sha256"] == sha256(run / "stdout.log")
            and outer["stderr_sha256"] == sha256(run / "stderr.log")
            and math.isfinite(outer["process_wall_ms"])
            and 0 < outer["process_wall_ms"] <= 5_415_000,
            "outer raw custody or disposition differs")
    completed = outer["status"] in ("UNKNOWN_solver_cap", "UNKNOWN_trial_cap",
                                     "PASS_verified_target_only")
    worker = None
    replay = None
    if completed:
        worker_config = read(run / "run/config.json")
        worker = read(run / "run/summary.json")
        require(outer["worker_exit_code"] == 0
                and outer["parse_error"] is None
                and outer["worker_error"] is None
                and outer["group_replay_error"] is None
                and outer["worker_config_sha256"] == sha256(run / "run/config.json")
                and outer["worker_sha256"] == sha256(run / "run/summary.json")
                and outer["status"] == worker["status"]
                and outer["report"] == worker["report"]
                and search.receipt_error(worker, worker_config, config) is None,
                "completed worker receipt differs")
        replay = search.replay_group(worker, fixtures / "probe-corpus.json", object_path)
        require(replay == outer["group_replay"] and replay["status"] == "PASS",
                "independent public query or relation replay differs")
    else:
        require(outer["status"] == "UNKNOWN_wall_cap"
                and outer["worker_exit_code"] is not None
                and outer["group_replay"] is None,
                "noncompleted run is neither clean cap nor completed receipt")
    require(outer["worker_cgroup_peak_bytes"] is None or
            0 <= outer["worker_cgroup_peak_bytes"] <= 4 * 1024**3,
            "worker cgroup peak exceeds cap")
    remaining = subprocess.check_output(
        ["docker", "ps", "-aq", "--filter", f"name=^{config['container_name']}$"],
        text=True).strip()
    require(not remaining, "search container remains after completion")
    return {"schema": "n83.k2048-natural-highcap-audit/v1",
            "status": "PASS_CUSTODY_COMPLETED" if completed else "PASS_CUSTODY_CENSORED_WALL",
            "worker_status": outer["status"], "source_commit": commit,
            "object_blake3": OBJECT_BLAKE3, "binary_sha256": sha256(binary),
            "conflict_budget": 100000,
            "conflicts_observed": worker["report"]["sat_conflicts"] if worker else None,
            "models_observed": worker["report"]["sat_models"] if worker else None,
            "relations_observed": worker["report"]["relations"] if worker else None,
            "independent_relations_observed":
                worker["report"]["independent_relations"] if worker else None,
            "group_replay": replay,
            "outer_process_wall_ms": outer["process_wall_ms"],
            "worker_cgroup_peak_bytes": outer["worker_cgroup_peak_bytes"],
            "total_index_calculus_runtime_ms": None,
            "selected_best_total_runtime": None}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--checkout", type=Path, required=True)
    parser.add_argument("--frozen", type=Path, required=True)
    parser.add_argument("--run", type=Path, required=True)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = audit(args.checkout, args.frozen, args.run, args.binary)
    with args.output.open("x") as stream:
        json.dump(result, stream, indent=2, sort_keys=True)
        stream.write("\n")
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
