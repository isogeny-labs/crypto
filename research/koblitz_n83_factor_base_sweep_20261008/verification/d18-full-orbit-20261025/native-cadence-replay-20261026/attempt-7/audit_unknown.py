#!/usr/bin/env python3
"""Check the frozen D18 input and terminal natural conflict-cap receipt."""

import argparse
import base64
import hashlib
import json
from pathlib import Path
import subprocess


SOURCE = "45ce91d503ee0d0acc57e3d8d5cb49108c90ded6"
WORKER = "5f302b8cfd86470e4d08da34c972308f0c8b5dbc80422dc8b578625f8e075fd2"
MANIFEST = "1d3650eabc4662d8f2597498d7ac92b7cd0a4134ac5f8155aaa49c7af3407dce"
WRAPPER = "1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616"
OBJECT = "eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b"
OBJECT_NAME = "c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f.jsonl.gz"
POINT_SET = "31aa8e1c1340f4db4898144af59f6f34c87fa63e0d88b175b854f58ce9008a7d"
CORPUS = "760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd"


def digest(path):
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()


def require(condition, message):
    if not condition:
        raise ValueError(message)


def swapouts(path):
    return int(path.read_text().split("Swapouts:", 1)[1].split()[0].rstrip("."))


def load(path):
    return float(path.read_text().split()[1])


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("raw_root", type=Path)
    parser.add_argument("source_checkout", type=Path)
    parser.add_argument("worker_binary", type=Path)
    args = parser.parse_args()
    root = args.raw_root
    input_dir = root / "input"
    natural = root / "natural"
    expected = {
        "manifest.json": MANIFEST,
        "s3-manifest.json": WRAPPER,
        f"objects/{OBJECT_NAME}": OBJECT,
    }
    for name, wanted in expected.items():
        require(digest(input_dir / name) == wanted, f"input hash differs: {name}")
    for name, wanted, size in (("manifest-get-metadata.json", WRAPPER, 1311), ("object-get-metadata.json", OBJECT, 160747529)):
        metadata = json.loads((input_dir / name).read_text())
        encoded = base64.b64encode(bytes.fromhex(wanted)).decode()
        require(metadata.get("ChecksumType") == "FULL_OBJECT", f"provider checksum not full object: {name}")
        require(metadata.get("ChecksumSHA256") == encoded, f"provider checksum differs: {name}")
        require(metadata.get("ContentLength") == size, f"provider byte count differs: {name}")
    require(digest(args.worker_binary) == WORKER, "worker binary differs")
    commit = subprocess.check_output(["git", "-C", str(args.source_checkout), "rev-parse", "HEAD"], text=True).strip()
    require(commit == SOURCE, "source commit differs")
    dirty = subprocess.check_output(["git", "-C", str(args.source_checkout), "status", "--porcelain"], text=True)
    require(not dirty, "source checkout dirty")

    fixture = args.source_checkout / "research/koblitz_n83_factor_base_sweep_20261008/pilot-01/probe-corpus.json"
    require(digest(fixture) == CORPUS, "public fixture differs")
    target = json.loads(fixture.read_text())["targets"][0]
    require(target["a"] == 0 and target["fixture"] == 0, "first target identity differs")

    result_path = natural / "result.json"
    result = json.loads(result_path.read_text())
    require((natural / "result.json.exit.txt").read_text().strip() == "0", "terminal process exit differs")
    require(json.loads((natural / "stdout.txt").read_text()) == result, "stdout and result differ")
    required = {
        "schema": "n83.full-d18-m4-l0-natural/v1",
        "status": "UNKNOWN_search_cap",
        "mode": "natural",
        "source_commit": SOURCE,
        "binary_sha256": WORKER,
        "object": f"objects/{OBJECT_NAME}",
        "point_set_blake3": POINT_SET,
        "solver_answer": "Unknown",
        "solver_conflicts": 100000,
        "solver_limit_reason": "conflict_cap",
        "search_policy": "phase_zero_first_ordered_first_pair_xor_cadence32",
        "xor_decision_cadence": 32,
        "summands": 4,
        "curve_a": 0,
        "budget_seconds": 2400,
    }
    for key, wanted in required.items():
        require(result.get(key) == wanted, f"terminal field differs: {key}")
    require(result["target"] == target["point"], "natural target differs")
    require(result["phase_file"] == str(natural / "result.json.phases.jsonl"), "phase path differs")
    require(result["source_attestation_mode"] == "clean_checkout", "source attestation differs")
    require(result["relation"] is None and result["model_certificate"] is None, "unknown result contains relation or model")
    require(result["total_index_calculus_runtime_ms"] is None, "cold total unexpectedly set")
    require(result["selected_best_total_runtime"] is None, "selector unexpectedly set")
    require(0 < result["search_ms"] < result["process_wall_ms"] <= 2_400_000, "wall accounting differs")
    shape = result["shape"]
    require((shape["variables"], shape["clauses"], shape["xor_rows"]) == (329808, 2807501, 3901), "formula shape differs")
    phases = [json.loads(line) for line in (natural / "result.json.phases.jsonl").read_text().splitlines()]
    require([row["phase"] for row in phases] == ["started", "import_checked", "model_constructed", "solver_returned"], "phase order differs")
    require(phases[-1]["detail"]["answer"] == "Unknown", "terminal phase answer differs")
    require(phases[-1]["detail"]["conflicts"] == 100000, "terminal phase cap differs")
    require(phases[-1]["detail"]["limit_reason"] == "conflict_cap", "terminal phase reason differs")
    require(abs(phases[-1]["elapsed_process_wall_ms"] - result["process_wall_ms"]) < 100, "terminal phase wall differs")

    samples = (root / "host-preflight-natural.txt").read_text().splitlines()
    require(len(samples) == 12, "natural host preflight has wrong number of samples")
    sample_loads = [float(samples[index].split()[1]) for index in (1, 5, 9)]
    sample_swaps = [int(samples[index].split()[1].rstrip(".")) for index in (2, 6, 10)]
    require(all(value <= 24 for value in sample_loads), "preflight load gate failed")
    require(len(set(sample_swaps)) == 1, "preflight swapouts changed")
    require(samples[8] <= (natural / "start-utc.txt").read_text().strip(), "host samples follow launch")
    require((natural / "start-utc.txt").read_text().strip() < (natural / "end-utc.txt").read_text().strip(), "run time ordering differs")
    require(load(natural / "start-load.txt") <= 24, "launch load gate failed")
    require(swapouts(natural / "start-swapouts.txt") == sample_swaps[-1], "launch swapouts changed")
    require(swapouts(natural / "end-swapouts.txt") == sample_swaps[-1], "run swapouts changed")
    scratch = int((natural / "start-scratch.txt").read_text().splitlines()[1].split()[3])
    require(scratch >= 2 * 1024 * 1024, "scratch gate failed")

    print(json.dumps({
        "schema": "n83.d18-attempt7-natural-audit/v1",
        "status": "PASS_CENSORED_CONFLICT_CAP",
        "source_commit": SOURCE,
        "worker_binary_sha256": WORKER,
        "manifest_sha256": MANIFEST,
        "s3_manifest_sha256": WRAPPER,
        "object_sha256": OBJECT,
        "result_sha256": digest(result_path),
        "target": result["target"],
        "solver_conflicts": result["solver_conflicts"],
        "process_wall_ms": result["process_wall_ms"],
        "search_ms": result["search_ms"],
        "preflight_load_1m": sample_loads,
        "swapouts": sample_swaps[-1],
        "relation_count": 0,
        "selected_best_total_runtime": None,
    }, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
