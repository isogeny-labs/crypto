#!/usr/bin/env python3
"""Fail-closed audit of the two capacity attempts and one bounded K4096 search."""

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
import chain_capacity_supervisor as capacity  # noqa: E402
import chain_search_supervisor as search  # noqa: E402
from preflight import (BINARY_SHA256, MANIFEST_SHA256, OBJECT_BLAKE3,
                       OBJECT_SHA256, S3_URI)  # noqa: E402

SOURCE_FIRST = "63c394a1be142f0a011d8b19e55243c663951316"
SOURCE_CORRECTED = "4ad69d5efb2fc6aec6082d30443ba18ba741d34f"
IMAGE = "sha256:0dd364ba7e10242f07755449e3a3d0e35f9efd987952737b90def6709ab0c5ce"
POINT_SET = "671c2d4d3e2f13276b474ae5e011d89c15084521eaa38df86ae989369395e3ba"


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def digest(path: Path, algorithm: str = "sha256") -> str:
    state = hashlib.sha256() if algorithm == "sha256" else blake3.blake3()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            state.update(block)
    return state.hexdigest()


def read(path: Path):
    return json.loads(path.read_text())


def check_source(checkout: Path, evidence: Path, run: Path, config: dict,
                 expected_commit: str) -> None:
    require(config["source_commit"] == expected_commit, "frozen source commit differs")
    attestation = read(run / "source/attestation.json")
    require(attestation == {"schema": capacity.ATTESTATION_SCHEMA,
                            "source_commit": expected_commit, "status_clean": True}
            and config["source_sha256"]["attestation.json"]
            == digest(run / "source/attestation.json"), "source attestation differs")
    for relative in capacity.SOURCES:
        snapshot = run / "source" / relative
        committed = subprocess.check_output(
            ["git", "show", f"{expected_commit}:{relative}"], cwd=checkout)
        expected = hashlib.sha256(committed).hexdigest()
        require(digest(snapshot) == expected == config["source_sha256"][relative]
                == read(evidence / "input-receipt.json")["source_sha256"][relative],
                f"source blob {relative} differs")


def check_container(evidence: Path, name: str, config: dict, search_run: bool) -> None:
    path = {"capacity-01": "capacity-live-inspect.json",
            "capacity-02": "capacity-02-live-inspect.json",
            "search-01": "search-live-inspect.json"}[name]
    inspected = read(evidence / path)
    require(isinstance(inspected, list) and len(inspected) == 1, "live inspection missing")
    item = inspected[0]
    host = item["HostConfig"]
    require(item["Name"] == "/" + config["container_name"]
            and item["State"]["Running"] is True
            and item["Image"] == config["container_image_id"] == IMAGE
            and host["Memory"] == host["MemorySwap"] == 4 * 1024**3
            and host["NanoCpus"] == 1_000_000_000
            and host["NetworkMode"] == "none"
            and host["ReadonlyRootfs"] is True
            and host["CapDrop"] == ["ALL"]
            and host["SecurityOpt"] == ["no-new-privileges"]
            and host["PidsLimit"] == 64,
            "Docker hard limits differ")
    mounts = {m["Destination"]: m for m in item["Mounts"]}
    expected = {"/panel", "/worker", "/source", "/out"}
    if search_run:
        expected.add("/fixtures")
    require(set(mounts) == expected
            and all(mounts[key]["RW"] is False for key in expected - {"/out"})
            and mounts["/out"]["RW"] is True,
            "Docker mount isolation differs")
    original_run = Path(config["panel_dir"]).parent / name
    sources = {"/panel": config["panel_dir"], "/worker": config["binary"],
               "/source": str(original_run / "source"), "/out": str(original_run)}
    if search_run:
        sources["/fixtures"] = config["fixture_dir"]
    require(all(mounts[target]["Source"] == source
                for target, source in sources.items()),
            "Docker mounted source paths differ from frozen configuration")


def check_run(checkout: Path, evidence: Path, name: str, source: str,
              binary: Path, object_path: Path) -> tuple[dict, dict | None]:
    run = evidence / name
    config, outer = read(run / "config.json"), read(run / "outer.json")
    is_search = name == "search-01"
    require(config["schema"] == ("n83.chain-s3-search-config/v1" if is_search
                                  else "n83.chain-s3-capacity-config/v1")
            and config["columns"] == 4096
            and config["policy"] == "public_x_hash"
            and config["seed"] == 2026100801
            and config["summands"] == 5
            and config["domain_encoding"] == "frobenius_orbit_phase"
            and config["max_variables"] == 150000
            and config["max_domain_clauses"] == 2000000
            and config["memory_cgroup_limit_bytes"] == 4 * 1024**3
            and config["memory_cgroup_swap_limit_bytes"] == 0
            and config["cpu_limit"] == 1.0
            and config["binary_sha256"] == digest(binary) == BINARY_SHA256
            and config["container_image_id"] == IMAGE
            and config["selected_best_total_runtime"] is None,
            f"{name} frozen configuration differs")
    if is_search:
        require(config["max_trials"] == 1 and config["max_models"] == 32
                and config["conflict_budget"] == 5000
                and config["wall_seconds"] == config["worker_wall_seconds"] == 1200
                and config["selected_object_sha256"] == OBJECT_SHA256
                and config["selected_point_set_blake3"] == POINT_SET,
                "search limits or base differ")
    else:
        require(config["identity_mask"] == 0 and config["wall_seconds"] == 900
                and config["solver_search_executed"] is False,
                "capacity limits differ")
    if is_search:
        for relative, expected in config["panel_sha256"].items():
            require(digest(evidence / "panel" / relative) == expected,
                    f"{name} panel {relative} differs")
        for relative, expected in config["fixture_sha256"].items():
            require(digest(evidence / "fixtures" / relative) == expected,
                    f"{name} fixture {relative} differs")
    check_source(checkout, evidence, run, config, source)
    check_container(evidence, name, config, is_search)
    require(outer["schema"] == ("n83.chain-s3-search-outer/v1" if is_search
                                else "n83.chain-s3-capacity-outer/v1")
            and outer["config_sha256"] == digest(run / "config.json")
            and outer["stdout_sha256"] == digest(run / "stdout.log")
            and outer["stderr_sha256"] == digest(run / "stderr.log")
            and all(outer[key] is False for key in
                    ("source_changed", "config_changed", "binary_changed"))
            and outer["launch_error"] is None and outer["cleanup_error"] is None
            and outer["total_index_calculus_runtime_ms"] is None
            and outer["selected_best_total_runtime"] is None
            and math.isfinite(outer["process_wall_ms"])
            and 0 < outer["process_wall_ms"] < config["wall_seconds"] * 1000 + 15000,
            f"{name} outer custody differs")
    worker = None
    if name == "capacity-01":
        require(outer["status"] == "PRODUCER_FAILURE_worker_exit"
                and outer["worker_exit_code"] == 1
                and outer["worker_sha256"] is None
                and not (run / "worker.json").exists()
                and "No such file or directory" in (run / "stderr.log").read_text(),
                "first producer failure was misclassified")
    elif name == "capacity-02":
        worker = read(run / "worker.json")
        require(outer["status"] == worker["status"] == "PASS_model_construction_only"
                and outer["worker_exit_code"] == 0
                and outer["worker_error"] is None
                and outer["worker_sha256"] == digest(run / "worker.json")
                and capacity.receipt_error(worker, config) is None
                and (worker["required_max_variables"], worker["required_domain_clauses"],
                     worker["sat_variables"], worker["sat_clauses"],
                     worker["sat_xor_rows"], worker["sat_and_gates"],
                     worker["sat_s3_nodes"]) ==
                    (131958, 1773110, 131958, 2021197, 4980, 82668, 4),
                "corrected model receipt differs")
    else:
        worker_config = read(run / "run/config.json")
        worker = read(run / "run/summary.json")
        report = worker["report"]
        require(outer["status"] == worker["status"] == "UNKNOWN_solver_cap"
                and outer["worker_exit_code"] == 0
                and all(outer[key] is None for key in
                        ("parse_error", "worker_error", "group_replay_error"))
                and outer["input_changed"] is False
                and outer["worker_config_sha256"] == digest(run / "run/config.json")
                and outer["worker_sha256"] == digest(run / "run/summary.json")
                and outer["report"] == report
                and search.receipt_error(worker, worker_config, config) is None
                and (report["sat_calls"], report["sat_conflicts"], report["sat_unknowns"],
                     report["sat_models"], report["relations"],
                     report["independent_relations"], report["matrix_rows"],
                     report["terminal_matrix_rank"]) == (1, 5000, 1, 0, 0, 0, 0, 0)
                and len(worker["attempt_records"]) == 1
                and worker["attempt_records"][0]["disposition"] == "Unknown"
                and worker["relation_matrix"] == []
                and worker["verified_log"] is None,
                "bounded search receipt differs")
        events = [json.loads(line)["event"] for line in
                  (run / "run/events.jsonl").read_text().splitlines()]
        require(len(events) == 8
                and events[0] == "FactorBaseReady { points: 679936, orbits: 4096 }"
                and events[2] == "RelationAttemptFinished { trial: 1, disposition: Unknown, conflicts: 5000, collected: 0 }"
                and events[-1] == "LinearAlgebraIncomplete",
                "bounded search event sequence differs")
        replay = search.replay_group(worker, evidence / "fixtures/probe-corpus.json",
                                     object_path)
        require(replay == outer["group_replay"] == {"queries": 1, "relations": 0,
                                                     "status": "PASS"},
                "independent public group replay differs")
    peak = outer["worker_cgroup_peak_bytes"]
    require(peak is None or 0 <= peak <= 4 * 1024**3, "cgroup peak exceeds limit")
    return outer, worker


def audit(checkout: Path, evidence: Path, object_path: Path) -> dict:
    inp, head, get = (read(evidence / name) for name in
                      ("input-receipt.json", "head.json", "get.json"))
    require(inp["status"] == "PASS_FRESH_S3_AND_SOURCE"
            and inp["s3_uri"] == S3_URI
            and inp["object_sha256"] == digest(object_path) == OBJECT_SHA256
            and inp["object_blake3"] == digest(object_path, "blake3") == OBJECT_BLAKE3
            and inp["object_bytes"] == object_path.stat().st_size == 20621466
            and head["ETag"] == get["ETag"] == inp["s3_etag"]
            and head["ContentLength"] == get["ContentLength"] == 20621466
            and head["Metadata"]["blake3"] == get["Metadata"]["blake3"] == OBJECT_BLAKE3
            and head["ServerSideEncryption"] == get["ServerSideEncryption"] == "AES256",
            "fresh S3 bytes or metadata differ")
    panel = evidence / "panel"
    manifest = read(panel / "manifest.json")
    row = manifest["bases"][0]
    require(digest(panel / "manifest.json") == MANIFEST_SHA256
            and len(manifest["bases"]) == 1
            and row["a"] == 0 and row["columns"] == 4096
            and row["points"] == 679936 and row["policy"] == "public_x_hash"
            and row["seed"] == 2026100801 and row["s3_uri"] == S3_URI
            and row["point_set_blake3"] == POINT_SET
            and row["compressed_blake3"] == OBJECT_BLAKE3
            and row["total_index_calculus_runtime_ms"] is None
            and manifest["selected_best_total_runtime"] is None,
            "K4096 panel manifest differs")
    for name, expected in inp["panel_sha256"].items():
        require(digest(panel / name) == expected, f"panel {name} differs")
    for name, expected in inp["fixture_sha256"].items():
        require(digest(evidence / "fixtures" / name) == expected,
                f"fixture {name} differs")
    require(digest(panel / "probe-corpus.json")
            == inp["fixture_sha256"]["probe-corpus.json"], "corrected panel corpus differs")
    corrected = read(evidence / "corrected-preflight.json")
    presearch = read(evidence / "presearch-preflight.json")
    require(corrected == presearch
            and corrected["source_commit"] == SOURCE_CORRECTED
            and corrected["status"] == "PASS_CORRECTED_INPUTS",
            "corrected input preflights differ")
    negative = read(evidence / "preflight-negative-receipt.json")
    require(negative["status"] == "PASS_MISSING_CORPUS_REJECTED"
            and negative["output_created"] is False
            and negative["stderr_sha256"] == digest(evidence / "preflight-negative.stderr")
            and "capacity panel is missing the exact public corpus" in
                (evidence / "preflight-negative.stderr").read_text(),
            "missing-corpus negative control differs")
    binary = evidence / "worker"
    require(digest(binary) == inp["binary_sha256"] == BINARY_SHA256,
            "retained worker differs")
    first, _ = check_run(checkout, evidence, "capacity-01", SOURCE_FIRST,
                         binary, object_path)
    second, capacity_worker = check_run(checkout, evidence, "capacity-02",
                                         SOURCE_CORRECTED, binary, object_path)
    searched, search_worker = check_run(checkout, evidence, "search-01",
                                        SOURCE_CORRECTED, binary, object_path)
    cleanup = read(evidence / "cleanup.json")
    names = [read(evidence / name / "config.json")["container_name"]
             for name in ("capacity-01", "capacity-02", "search-01")]
    require(cleanup["status"] == "PASS_NO_RETAINED_CONTAINERS"
            and cleanup["container_names"] == names
            and cleanup["selected_best_total_runtime"] is None,
            "container cleanup receipt differs")
    for name in names:
        remaining = subprocess.check_output(
            ["docker", "ps", "-aq", "--filter", f"name=^{name}$"], text=True)
        require(not remaining.strip(), "one or more experimental containers remain")
    return {
        "schema": "n83.k4096-orbit-model-audit/v1",
        "status": "PASS_CUSTODY_CENSORED_SEARCH",
        "first_capacity_status": first["status"],
        "corrected_capacity_status": second["status"],
        "search_status": searched["status"],
        "object_sha256": OBJECT_SHA256,
        "object_blake3": OBJECT_BLAKE3,
        "model_variables": capacity_worker["sat_variables"],
        "model_clauses": capacity_worker["sat_clauses"],
        "orbit_domain_clauses": capacity_worker["required_domain_clauses"],
        "search_conflicts": search_worker["report"]["sat_conflicts"],
        "search_relations": search_worker["report"]["relations"],
        "search_rank": search_worker["report"]["terminal_matrix_rank"],
        "observed_outer_wall_ms": {"capacity_01": first["process_wall_ms"],
                                   "capacity_02": second["process_wall_ms"],
                                   "search_01": searched["process_wall_ms"]},
        "conservative_new_stage_charge_seconds": 3300,
        "total_index_calculus_runtime_ms": None,
        "selected_best_total_runtime": None,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--checkout", type=Path, required=True)
    parser.add_argument("--evidence", type=Path, required=True)
    parser.add_argument("--object", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = audit(args.checkout, args.evidence, args.object)
    with args.output.open("x") as stream:
        json.dump(result, stream, indent=2, sort_keys=True)
        stream.write("\n")
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
