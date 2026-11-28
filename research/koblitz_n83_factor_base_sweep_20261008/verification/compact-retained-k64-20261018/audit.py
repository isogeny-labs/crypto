#!/usr/bin/env python3
"""Independent fail-closed replay of the retained primary K64 compact probe."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
from pathlib import Path
import subprocess

import blake3

OBJECT_HASH = "1d0a6695564972f61db13c62c60a300da0cfa7bcd9b363c2e553a34345fae176"
ORDER = "2417851639230796216685689"
IMAGE = "sha256:0dd364ba7e10242f07755449e3a3d0e35f9efd987952737b90def6709ab0c5ce"
SOURCES = (
    "examples/koblitz_n83_factor_base_export.rs",
    "research/koblitz_n83_factor_base_sweep_20261008/primary_adapter.rs",
    "research/koblitz_n83_factor_base_sweep_20261008/compact_cold.rs",
)


def require(ok: bool, why: str) -> None:
    if not ok:
        raise ValueError(why)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def b3(path: Path) -> str:
    digest = blake3.blake3()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def finite_ms(value: object) -> bool:
    return type(value) in (int, float) and math.isfinite(value) and value >= 0


def audit(checkout: Path, panel: Path, run: Path, binary: Path) -> dict:
    original = json.loads((panel / "input.json").read_text())
    config = json.loads((run / "config.json").read_text())
    outer = json.loads((run / "outer.json").read_text())
    worker = json.loads((run / "worker.json").read_text())
    inspect = json.loads((run / "container-inspect.json").read_text())
    require(original.get("status") == "PASS_FRESH_EXACT_S3_INPUT"
            and original.get("object_blake3") == OBJECT_HASH
            and original.get("policy") == "public_x_hash"
            and original.get("seed") == 2026100801
            and original.get("columns") == 64
            and original.get("selected_best_total_runtime") is None,
            "frozen S3 K64 input differs")
    compressed = panel / "objects" / f"{OBJECT_HASH}.jsonl.gz"
    require(sha256(compressed) == original["object_sha256"]
            and b3(compressed) == OBJECT_HASH
            and compressed.stat().st_size == original["object_bytes"]
            and original["s3_get_etag"] == original["s3_head_etag"],
            "retained compressed S3 bytes differ")
    for name, digest in original["panel_files_sha256"].items():
        require(sha256(panel / name) == digest, f"panel {name} differs")
    manifest = json.loads((panel / "manifest.json").read_text())
    selected = [item for item in manifest["bases"] if item.get("a") == 0
                and item.get("columns") == 64 and item.get("policy") == "public_x_hash"
                and item.get("seed") == 2026100801]
    require(len(selected) == 1 and selected[0]["compressed_blake3"] == OBJECT_HASH,
            "selected K64 panel row differs")
    require(config["schema"] == "n83.primary-compact-probe-config/v1"
            and config["columns"] == 64 and config["policy"] == "public_x_hash"
            and config["seed"] == 2026100801 and config["pair_mode"] == "unordered"
            and config["max_candidate_states"] == 83 * 64 * 65 // 2
            and config["memory_cgroup_limit_bytes"] == 4 * 1024**3
            and config["memory_cgroup_swap_limit_bytes"] == 0
            and config["cpu_limit"] == 1.0 and config["wall_seconds"] == 900
            and config["container_image_id"] == IMAGE
            and config["binary_sha256"] == sha256(binary),
            "source, state or resource configuration differs")
    commit = config["source_commit"]
    require(commit == "2ae7aea81dc13b29cc8ea17918ccd3c0883f6e61"
            and outer["config_sha256"] == sha256(run / "config.json")
            and outer["source_changed"] is False
            and outer["config_changed"] is False
            and outer["binary_changed"] is False,
            "frozen commit or input mutation receipt differs")
    for relative in SOURCES:
        snapshot = run / "source" / relative
        committed = subprocess.check_output(["git", "show", f"{commit}:{relative}"],
                                            cwd=checkout)
        require(sha256(snapshot) == config["source_sha256"][relative]
                and hashlib.sha256(committed).hexdigest() == sha256(snapshot),
                f"compiled source snapshot {relative} differs")
    attestation = json.loads((run / "source/attestation.json").read_text())
    require(attestation == {"schema": "n83.primary-probe-source-attestation/v1",
                            "source_commit": commit, "status_clean": True}
            and sha256(run / "source/attestation.json")
            == config["source_sha256"]["attestation.json"],
            "source attestation differs")
    host = inspect["host_config"]
    require(inspect["schema"] == "n83.primary-compact-container-inspection/v1"
            and inspect["container_name"] == config["container_name"]
            and inspect["state_running_at_inspection"] is True
            and inspect["image_id"] == IMAGE
            and host["Memory"] == 4 * 1024**3
            and host["MemorySwap"] == host["Memory"]
            and host["NanoCpus"] == 1_000_000_000
            and host["NetworkMode"] == "none"
            and host["ReadonlyRootfs"] is True
            and host["CapDrop"] == ["ALL"]
            and host["SecurityOpt"] == ["no-new-privileges"]
            and host["PidsLimit"] == 64,
            "live Docker hard-limit inspection differs")
    mounts = {entry["Destination"]: entry for entry in inspect["mounts"]}
    require(set(mounts) == {"/panel", "/worker", "/source", "/out"}
            and all(mounts[name]["Type"] == "bind" for name in mounts)
            and all(mounts[name]["RW"] is False for name in ("/panel", "/worker", "/source"))
            and mounts["/out"]["RW"] is True,
            "Docker mount permissions differ")
    require(outer["schema"] == "n83.primary-compact-probe-outer/v1"
            and outer["status"] == "PASS_probe_miss"
            and outer["worker_exit_code"] == 0
            and outer["worker_error"] is None
            and outer["launch_error"] is None
            and outer["cleanup_error"] is None
            and outer["worker_sha256"] == sha256(run / "worker.json")
            and outer["stdout_sha256"] == sha256(run / "stdout.log")
            and outer["stderr_sha256"] == sha256(run / "stderr.log")
            and json.loads((run / "stdout.log").read_text()) == worker
            and (run / "stderr.log").read_bytes() == b"",
            "worker or outer raw disposition differs")
    require(worker["schema"] == "n83.primary-compact-s3-probe/v1"
            and worker["status"] == "MISS" and worker["curve_a"] == 0
            and worker["fixture"] == 0 and worker["orbit_columns"] == 64
            and worker["policy"] == "public_x_hash" and worker["seed"] == 2026100801
            and worker["object"] == original["object"]
            and worker["point_set_blake3"] == selected[0]["point_set_blake3"]
            and worker["panel_manifest_blake3"] == b3(panel / "manifest.json")
            and worker["source_commit"] == commit
            and worker["source_attestation_mode"] == "supervised_snapshot"
            and worker["relation_row"] is None
            and worker["relation_row_stage_executed"] is False
            and worker["rank_stage_executed"] is False
            and worker["column_log_verification"] is False
            and worker["total_index_calculus_runtime_ms"] is None
            and worker["selected_best_total_runtime"] is None,
            "primary worker status or claim boundary differs")
    probe = worker["probe"]
    require(probe["schema"] == "compact-four-sum-point-probe/v1"
            and probe["status"] == "MISS"
            and probe["n"] == 83 and probe["curve_a"] == 0
            and probe["orbit_columns"] == 64
            and probe["unordered_pairs"] is True
            and probe["candidate_states"] == 172640
            and probe["regular_states"] == 172640
            and 0 < probe["root_table_entries"] <= 2 * probe["regular_states"]
            and probe["point_indices"] is None
            and probe["rank_stage_executed"] is False
            and probe["total_index_calculus_runtime_ms"] is None,
            "compact root-index receipt differs")
    for key in ("base_import_ms", "target_validation_ms", "point_probe_ms",
                "relation_row_ms", "process_wall_ms"):
        require(finite_ms(worker.get(key)), f"worker {key} is invalid")
    require(finite_ms(outer.get("process_wall_ms"))
            and worker["process_wall_ms"] <= outer["process_wall_ms"] <= 900_000
            and worker["memory_cgroup_peak_bytes"] == outer["worker_cgroup_peak_bytes"]
            and 0 < worker["memory_cgroup_peak_bytes"] <= 4 * 1024**3,
            "resource or wall receipt differs")
    cleanup = subprocess.check_output(
        ["docker", "ps", "-aq", "--filter", f"name=^{config['container_name']}$"],
        text=True).strip()
    require(not cleanup, "compact probe container remains after completion")
    return {"schema": "n83.primary-compact-k64-audit/v1",
            "status": "PASS_CAPACITY_ONE_QUERY_MISS", "source_commit": commit,
            "object_blake3": OBJECT_HASH,
            "binary_sha256": sha256(binary),
            "candidate_states": probe["candidate_states"],
            "regular_states": probe["regular_states"],
            "root_table_entries": probe["root_table_entries"],
            "point_probe_ms": worker["point_probe_ms"],
            "worker_cgroup_peak_bytes": worker["memory_cgroup_peak_bytes"],
            "outer_process_wall_ms": outer["process_wall_ms"],
            "one_query_status": "MISS", "rank_stage_executed": False,
            "total_index_calculus_runtime_ms": None,
            "selected_best_total_runtime": None}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--checkout", type=Path, required=True)
    parser.add_argument("--panel", type=Path, required=True)
    parser.add_argument("--run", type=Path, required=True)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    row = audit(args.checkout, args.panel, args.run, args.binary)
    with args.output.open("x") as stream:
        json.dump(row, stream, indent=2, sort_keys=True)
        stream.write("\n")
    print(json.dumps(row, sort_keys=True))


if __name__ == "__main__":
    main()
