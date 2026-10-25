#!/usr/bin/env python3
"""Replay the exact K3596 identity and both bounded local attempt receipts."""

import copy
import hashlib
import json
from pathlib import Path
import subprocess

from research.n83_online_finalists_20261018.generate_candidates import canonical_id

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
CANDIDATE_ID = "IC1N83Ckb1a1fb596936PDP4rootRCguidedLAgaussTDdirectISO0h2a3353097e81"
SOURCE_SHA256 = "58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8"
BINARY_SHA256 = "07f7b5b424d0f1ecea1cd7e6564522c4d9c64557a820441c381e7474b13a388a"
BASE_DUMP_SHA256 = "1f3b323d6af8f16dbce51b4d7a3db15a27e68d3f7b72c604cdf147d6071b1473"
BASE_SET_SHA256 = "c86bd1ed9c983397bbe9641995ef99e13ec4daedea2d469b55f8836ced9cd777"
OLD_SOURCE_SHA256 = "4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507"
SOURCE_COMMIT = "3ce5b531cacb1363c6bf1c8a2c4277bf534046d1"


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read(path):
    return json.loads(path.read_text())


def verify():
    path = HERE / f"candidate_{CANDIDATE_ID}.json"
    candidate = read(path)
    prior = read(ROOT / "research/n83_root4_frontier_current_20261018/"
                 "candidate_IC1N83Ckb1a1fb596936PDP4rootRCguided"
                 "LAgaussTDdirectISO0h3da4e8b31c3f.json")
    derived = copy.deepcopy(prior)
    if (canonical_id(prior) != prior["candidate_id"] or
            prior["implementation"]["source_sha256"]["ic"] != OLD_SOURCE_SHA256 or
            hashlib.sha256(subprocess.check_output(
                ["git", "show", f"{SOURCE_COMMIT}:examples/koblitz_orbit_dlp_fast.rs"],
                cwd=ROOT)).hexdigest() != SOURCE_SHA256):
        raise ValueError("historical candidate or source commit differs")
    derived["implementation"]["source_commit"] = SOURCE_COMMIT
    derived["implementation"]["source_sha256"]["ic"] = SOURCE_SHA256
    for section, key in (("point_decomposition", "solver_source_sha256"),
                         ("relation_collection", "source_digest"),
                         ("relation_linear_algebra", "source_digest"),
                         ("target_descent", "source_digest")):
        if derived[section][key] != OLD_SOURCE_SHA256:
            raise ValueError(f"historical {section} source differs")
        derived[section][key] = SOURCE_SHA256
    derived["candidate_id"] = canonical_id(derived)
    if (candidate["candidate_id"] != CANDIDATE_ID or
            candidate != derived or
            canonical_id(candidate) != CANDIDATE_ID or
            candidate["implementation"]["source_sha256"]["ic"] != SOURCE_SHA256 or
            candidate["factor_base"]["actual_usable_points"] != 596936 or
            candidate["factor_base"]["effective_columns"] != 3596 or
            candidate["factor_base"]["ordered_base_dump_sha256"] !=
            BASE_DUMP_SHA256 or
            candidate["factor_base"]["enumerated_set_sha256"] != BASE_SET_SHA256 or
            digest(ROOT / "examples/koblitz_orbit_dlp_fast.rs") != SOURCE_SHA256 or
            read(ROOT / "research/n83_highk_beyond4405_20261021/"
                 "source_binary_custody.json")["binary_sha256"] != BINARY_SHA256):
        raise ValueError("current-source K3596 candidate identity differs")
    public = json.loads((ROOT / "research/n83_cold_factor_base_20261016/"
                         "targets.jsonl").read_text().splitlines()[0])
    answer = json.loads((ROOT / "research/n83_cold_factor_base_20261016/"
                         "answers.jsonl").read_text().splitlines()[0])
    if public != answer["point"] or answer["fixture_index"] != 0:
        raise ValueError("earlier public point or answer differs")
    attempts = []
    for name, runner, verifier, expected_status in (
            ("priority10", "run_control.py", "verify_control.py", "PROCESS_FAILED"),
            ("normal", "run_control_normal.py", "verify_control_normal.py",
             "TIME_CAP")):
        folder = HERE / "runs" / name
        launch = read(folder / "launch.json")
        receipt = read(folder / "receipt.json")
        replay = read(folder / "independent_replay.json")
        if (launch["schema"] != "n83.k3596-current-source-full-control-launch/v1" or
                launch["source_sha256"] != SOURCE_SHA256 or
                launch["binary_sha256"] != BINARY_SHA256 or
                launch["script_sha256"] != digest(HERE / "executed" / runner) or
                launch["target_sha256"] != digest(folder / "target.jsonl") or
                json.loads((folder / "target.jsonl").read_text()) != public or
                launch["cap_seconds"] != 1200 or
                launch["rss_cap_bytes"] != 16 * 1024**3 or
                launch["min_host_free_percent"] != 10 or
                launch["host_kind"] != "contended_local_mac_correctness_control" or
                receipt["schema"] !=
                "n83.k3596-current-source-full-control-receipt/v1" or
                receipt["status"] != expected_status or
                receipt["exit_code"] != -15 or
                receipt["peak_sampled_rss_bytes"] <= 0 or
                receipt["peak_sampled_rss_bytes"] > launch["rss_cap_bytes"] or
                receipt["lowest_host_free_percent"] < 10 or
                receipt["stdout_sha256"] != digest(folder / "stdout.jsonl") or
                receipt["stderr_sha256"] != digest(folder / "stderr.log") or
                receipt["targets_result_sha256"] !=
                digest(folder / "targets_result.jsonl") or
                receipt["base_dump_sha256"] is not None or
                replay["status"] != "BOUNDED_UNKNOWN" or
                replay["producer_status"] != expected_status or
                replay["candidate_id"] is not None or
                replay.get("online_ms") is not None or
                replay.get("cold_process_ms") is not None):
            raise ValueError(f"{name} bounded receipt differs")
        digest(HERE / "executed" / verifier)
        if name == "priority10":
            stop = read(folder / "operator_stop.json")
            if (stop["reason"] != "severe_cpu_starvation_under_nice10" or
                    stop["signal"] != "SIGTERM"):
                raise ValueError("priority-10 operator stop differs")
        else:
            if not 1200 <= receipt["wall_seconds"] < 1210:
                raise ValueError("normal attempt did not reach frozen cap")
        attempts.append({"name": name, "status": replay["status"],
                         "producer_status": receipt["status"],
                         "wall_seconds": receipt["wall_seconds"],
                         "peak_sampled_rss_bytes": receipt["peak_sampled_rss_bytes"]})
    return {"schema": "n83.k3596-current-source-full-control-evidence/v1",
            "status": "PASS_BOUNDED_RECEIPT_REPLAY",
            "candidate_id": CANDIDATE_ID,
            "candidate_manifest_sha256": digest(path),
            "attempts": attempts,
            "full_target_result_status": "BOUNDED_UNKNOWN"}


if __name__ == "__main__":
    print(json.dumps(verify(), sort_keys=True, indent=2))
