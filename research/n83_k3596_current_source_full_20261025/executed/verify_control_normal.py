#!/usr/bin/env python3
"""Independently replay the K3596 full-path current-source control."""

import copy
import hashlib
import json
import math
from pathlib import Path
import subprocess
import sys
import tarfile

ROOT = Path("/Volumes/SSD990/n83-confirm16-20261024")
OUT = Path("/Volumes/SSD990/n83-k3596-current-source-full-20261025/control2")
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT))

from research.n83_indexed5_20261013 import analyze as group
from research.n83_online_finalists_20261018.generate_candidates import canonical_id

SOURCE = ROOT / "examples/koblitz_orbit_dlp_fast.rs"
BIN = Path("/Volumes/SSD990-2/.codex/n83-fresh-cold-target/release/examples/koblitz_orbit_dlp_fast")
PUBLIC = ROOT / "research/n83_cold_factor_base_20261016/targets.jsonl"
ANSWERS = ROOT / "research/n83_cold_factor_base_20261016/answers.jsonl"
ARCHIVE = ROOT / "research/n83_root4_frontier_current_20261018/pilots/k3596.tar.gz"
OLD_MANIFEST = ROOT / ("research/n83_root4_frontier_current_20261018/"
                       "candidate_IC1N83Ckb1a1fb596936PDP4rootRCguided"
                       "LAgaussTDdirectISO0h3da4e8b31c3f.json")
SOURCE_COMMIT = "3ce5b531cacb1363c6bf1c8a2c4277bf534046d1"
OLD_SOURCE = "4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507"
NEW_SOURCE = "58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8"
BASE_DUMP_SHA256 = "1f3b323d6af8f16dbce51b4d7a3db15a27e68d3f7b72c604cdf147d6071b1473"
BASE_SET_SHA256 = "c86bd1ed9c983397bbe9641995ef99e13ec4daedea2d469b55f8836ced9cd777"
BASE_HASH = "9bdadb868c18ccaad3446a62316356e3fe21ecc76512c0c27ccc1fa24d661bc1"
PHASES = ("target_query_ms", "target_pdp_ms", "target_relation_check_ms",
          "target_descent_ms", "target_recovery_check_ms")


def sha(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def candidate():
    prior = json.loads(OLD_MANIFEST.read_text())
    if (prior["candidate_id"] != canonical_id(prior) or
            prior["implementation"]["source_sha256"]["ic"] != OLD_SOURCE or
            prior["factor_base"]["ordered_base_dump_sha256"] != BASE_DUMP_SHA256 or
            prior["factor_base"]["source_base_hash_blake3"] != BASE_HASH or
            prior["factor_base"]["enumerated_set_sha256"] != BASE_SET_SHA256 or
            prior["factor_base"]["actual_usable_points"] != 596936 or
            sha(SOURCE) != NEW_SOURCE or
            hashlib.sha256(subprocess.check_output(
                ["git", "show", f"{SOURCE_COMMIT}:examples/koblitz_orbit_dlp_fast.rs"],
                cwd=ROOT)).hexdigest() != NEW_SOURCE):
        raise ValueError("old candidate or frozen current source differs")
    record = copy.deepcopy(prior)
    record["implementation"]["source_commit"] = SOURCE_COMMIT
    record["implementation"]["source_sha256"]["ic"] = NEW_SOURCE
    for section, key in (("point_decomposition", "solver_source_sha256"),
                         ("relation_collection", "source_digest"),
                         ("relation_linear_algebra", "source_digest"),
                         ("target_descent", "source_digest")):
        if record[section][key] != OLD_SOURCE:
            raise ValueError(f"{section} source differs")
        record[section][key] = NEW_SOURCE
    record["candidate_id"] = canonical_id(record)
    return record


def read_one(path):
    lines = path.read_text().splitlines()
    if len(lines) != 1:
        raise ValueError(f"{path.name} must have one JSON row")
    return json.loads(lines[0])


def verify():
    launch = json.loads((OUT / "launch.json").read_text())
    receipt = json.loads((OUT / "receipt.json").read_text())
    record = {"schema": "n83.k3596-current-source-full-control-replay/v1",
              "status": "BOUNDED_UNKNOWN", "producer_status": receipt["status"],
              "candidate_id": None, "checks": {},
              "local_wall_seconds": receipt["wall_seconds"],
              "peak_sampled_rss_bytes": receipt["peak_sampled_rss_bytes"],
              "host_kind": launch["host_kind"]}
    if receipt["status"] != "PROCESS_EXITED":
        record["reason"] = "PRODUCER_DID_NOT_COMPLETE"
        return record
    current_candidate = candidate()
    summary = read_one(OUT / "stdout.jsonl")
    result = read_one(OUT / "targets_result.jsonl")
    target = read_one(OUT / "target.jsonl")
    answer = json.loads(ANSWERS.read_text().splitlines()[0])
    with tarfile.open(ARCHIVE, "r:gz") as bundle:
        stream = bundle.extractfile("pilot_work/launch.json")
        if stream is None:
            raise ValueError("historical K3596 launch missing")
        prior_launch = json.load(stream)
    expected_flags = dict(prior_launch["environment_flags"])
    expected_flags["KIC_DUMP_BASE"] = str(OUT / "base_dump.jsonl")
    dump_path = OUT / "base_dump.jsonl"
    dump = json.loads(dump_path.read_text())
    points = group.replay_base(dump, summary)
    canonical = json.dumps([[str(x), str(y)] for x, y in sorted(points)],
                           sort_keys=True, separators=(",", ":"),
                           ensure_ascii=False).encode()
    set_sha = hashlib.sha256(canonical).hexdigest()
    indices = result.get("point_indices")
    relation = None
    if (isinstance(indices, list) and len(indices) == 4 and
            all(type(i) is int and 0 <= i < len(points) for i in indices)):
        for i in indices:
            relation = group.add(relation, points[i])
    scalar = int(result["recovered_scalar"])
    point = group.point_from_json(target)
    phases = [result.get(name) for name in PHASES]
    timing = summary["timing_ms"]
    checks = {
        "source_and_binary": sha(SOURCE) == NEW_SOURCE and
                             sha(BIN) == launch["binary_sha256"] and
                             launch["source_sha256"] == NEW_SOURCE and
                             launch["script_sha256"] == sha(HERE / "run_control_normal.py"),
        "frozen_input": target == json.loads(PUBLIC.read_text().splitlines()[0]) ==
                        answer["point"] and answer["fixture_index"] == 0 and
                        launch["public_targets_sha256"] == sha(PUBLIC) and
                        launch["answer_sidecar_sha256"] == sha(ANSWERS) and
                        launch["target_sha256"] == sha(OUT / "target.jsonl"),
        "launch_flags": launch["environment_flags"] == expected_flags and
                        launch["binary_command"] == [
                            str(BIN), "construct:83:1:3596",
                            str(OUT / "target.jsonl"), "29",
                            str(OUT / "targets_result.jsonl")],
        "receipt_hashes": receipt["stdout_sha256"] == sha(OUT / "stdout.jsonl") and
                          receipt["stderr_sha256"] == sha(OUT / "stderr.log") and
                          receipt["base_dump_sha256"] == sha(dump_path) and
                          receipt["targets_result_sha256"] ==
                          sha(OUT / "targets_result.jsonl"),
        "base_identity": sha(dump_path) == BASE_DUMP_SHA256 and
                         set_sha == BASE_SET_SHA256 and
                         len(points) == 596936 and summary["base_hash"] == BASE_HASH,
        "rank_and_index": summary["rank"] == summary["orbit_columns"] == 3596 and
                          summary["factor_base_points"] == 596936 and
                          summary["root_table_layout"] == "indexed83_5byte_exact" and
                          summary["root_table_allocated_bytes"] == 8589934592 and
                          summary["root_table_entries"] > 0,
        "target_and_scalar": result["target"] == target and
                             scalar == int(answer["scalar"]) and
                             group.multiply(group.G, scalar) == point and
                             group.multiply(point, group.R) is None and
                             result["group_verified"] is True,
        "four_point_relation": relation == point,
        "online_phases": all(type(value) in (int, float) and
                             math.isfinite(value) and value >= 0
                             for value in phases) and
                         abs(math.fsum(phases) - result["target_ms"]) <= 1,
        "cold_accounting": timing["process_total"] >=
                           math.fsum([timing["setup_total"],
                                      timing["rank_stage"],
                                      timing["targets_total"]]) - 1 and
                           abs(timing["targets_total"] - result["target_ms"]) <= 2,
        "producer_success": receipt["exit_code"] == 0 and
                            result["exit_code"] == 0 and
                            summary["targets_solved"] == 1 and
                            summary["targets_failed"] == 0,
    }
    record.update({"checks": checks,
                   "candidate_id": current_candidate["candidate_id"],
                   "candidate_manifest_sha256": hashlib.sha256(json.dumps(
                       current_candidate, indent=2, sort_keys=True).encode() +
                       b"\n").hexdigest(),
                   "usable_points": len(points), "folded_columns": 3596,
                   "base_dump_sha256": sha(dump_path),
                   "enumerated_set_sha256": set_sha,
                   "root_table_entries": summary["root_table_entries"],
                   "root_table_allocated_bytes": summary["root_table_allocated_bytes"],
                   "rank": summary["rank"], "online_ms": result["target_ms"],
                   "cold_process_ms": timing["process_total"],
                   "online_phases_ms": dict(zip(PHASES, phases)),
                   "recovered_scalar": str(scalar),
                   "status": "PASS_INDEPENDENT_FULL_REPLAY"
                   if all(checks.values()) else "VERIFICATION_FAILED"})
    return record


if __name__ == "__main__":
    outcome = verify()
    (OUT / "independent_replay.json").write_text(
        json.dumps(outcome, sort_keys=True, indent=2) + "\n")
    print(json.dumps({key: outcome.get(key) for key in
                      ("status", "candidate_id", "online_ms", "cold_process_ms",
                       "peak_sampled_rss_bytes", "checks")}, sort_keys=True))
    sys.exit(0 if outcome["status"] == "PASS_INDEPENDENT_FULL_REPLAY" else 2)
