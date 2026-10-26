"""Audit the bounded K4405 local host-swap abort receipt."""

import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parent / "evidence-01"


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read(path: Path) -> dict:
    return json.loads(path.read_text())


def main() -> None:
    manifest = read(ROOT / "manifest.json")["sha256"]
    actual = {path.name: digest(path) for path in ROOT.iterdir()
              if path.is_file() and path.name != "manifest.json"}
    assert actual == manifest
    config = read(ROOT / "config.json")
    receipt = read(ROOT / "receipt.json")
    abort = read(ROOT / "manual_abort.json")
    pressure = read(ROOT / "host_pressure_observation.json")
    samples = read(ROOT / "samples.json")
    assert config["schema"] == "n83.k4405-current-source-local-control/v1"
    assert config["source_sha256"] == (
        "58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8")
    assert config["binary_sha256"] == (
        "07f7b5b424d0f1ecea1cd7e6564522c4d9c64557a820441c381e7474b13a388a")
    assert config["command"][1] == "construct:83:1:4405"
    assert config["algorithm_flags"]["KIC_ROOT_ONLY11"] == "1"
    assert config["algorithm_flags"]["KIC_INDEXED5_ID30"] == "1"
    assert config["algorithm_flags"]["KIC_ROOT_BLOOM_PROFILE"] == "wide3"
    assert config["algorithm_flags"]["KIC_TARGET_CANCEL"] == "1"
    assert config["algorithm_flags"]["KIC_TARGET_THREADS"] == "2"
    assert config["cap_seconds"] == 1200
    assert config["rss_cap_bytes"] == 24 * 1024**3
    assert config["initial_host_free_percent"] >= 50
    assert receipt["status"] == "PRODUCER_FAILURE_EXIT" and receipt["exit_code"] == -15
    assert abort["status"] == "BOUNDED_UNKNOWN_HOST_SWAP_ABORT"
    assert abort["signal"] == "SIGTERM" and abort["worker_pid"] > 0
    assert abort["receipt_sha256"] == digest(ROOT / "receipt.json")
    assert abort["runner_v1_sha256"] == digest(ROOT / "runner_v1.py")
    assert pressure["page_size_bytes"] == 16384
    assert pressure["pages_free"] == 3746
    assert pressure["pages_occupied_by_compressor"] == 1551744
    assert receipt["samples_sha256"] == digest(ROOT / "samples.json")
    assert receipt["stdout_sha256"] == digest(ROOT / "stdout.jsonl")
    assert receipt["stderr_sha256"] == digest(ROOT / "stderr.log")
    assert receipt["target_sha256"] == digest(ROOT / "target.jsonl")
    assert receipt["base_dump_sha256"] is None
    assert receipt["targets_result_sha256"] == digest(ROOT / "targets_result.jsonl")
    assert receipt["max_sampled_rss_bytes"] == max(
        (row["rss_bytes"] or 0 for row in samples), default=0)
    assert 0 < receipt["wall_seconds"] < config["cap_seconds"]
    assert not (ROOT / "stdout.jsonl").read_bytes()
    assert not (ROOT / "targets_result.jsonl").read_bytes()
    print(json.dumps({"status": "PASS_BOUNDED_HOST_SWAP_ABORT_RECEIPT",
                      "wall_seconds": receipt["wall_seconds"],
                      "max_sampled_rss_bytes": receipt["max_sampled_rss_bytes"]},
                     sort_keys=True))


if __name__ == "__main__":
    main()
