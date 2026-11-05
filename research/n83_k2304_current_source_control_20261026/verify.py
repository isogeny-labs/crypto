"""Independent artifact and group replay for the K2304 public-point control."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from research.n83_k2304_current_source_control_20261026 import run
from research.n83_k4405_linux_replay_20261026.verify import (
    candidate_digest, check, one_jsonl, read, replay_target, sha,
)


ROOT = Path(__file__).resolve().parents[2]
PREFIX = ROOT / "research/n83_k2304_current_source_prefix_20261025/prefix_receipt.json"
EXECUTED_SOURCE = Path(__file__).resolve().with_name(
    "koblitz_orbit_dlp_fast.executed-20261026.rs")
EXECUTED_FLAGS = {
    "KIC_INDEXED4_FIXED31": "0", "KIC_INDEXED5_FIXED30": "0",
    "KIC_INDEXED5_ID30": "0", "KIC_INDEXED5_LOAD75": "1",
    "KIC_INDEXED_ROOT4": "0", "KIC_INDEXED_ROOT5": "1",
    "KIC_INDEX_ROOT_BATCH": "1", "KIC_PACKED_ROOT_TABLE": "1",
    "KIC_PACKED_ROOT_TABLE_DENSE": "0",
    "KIC_PACKED_ROOT_TABLE_LOAD75": "0", "KIC_RANK_THREADS": "4",
    "KIC_ROOT_BLOOM": "0", "KIC_ROOT_ONLY11": "1",
    "KIC_ROOT_ONLY21": "0", "KIC_ROOT_ONLY22": "0",
    "KIC_SPARSE_CANONICAL": "1", "KIC_STATE26": "0",
    "KIC_STATE28": "0", "KIC_SYMMETRIC_S3": "1",
    "KIC_TARGET_CANCEL": "1", "KIC_TARGET_THREADS": "2",
}


def verify(output: Path, binary: Path) -> dict:
    config, receipt = read(output / "config.json"), read(output / "receipt.json")
    candidate, prefix = read(run.CANDIDATE), read(PREFIX)
    check(config["schema"] == "n83.k2304-current-source-local-control/v1" and
          receipt["schema"] == "n83.k2304-current-source-local-receipt/v1" and
          config["runner_sha256"] == sha(Path(run.__file__)) and
          config["source_sha256"] == run.SOURCE_SHA256 == sha(EXECUTED_SOURCE) and
          config["binary_sha256"] == run.BINARY_SHA256 == sha(binary) and
          config["candidate_manifest_sha256"] == sha(run.CANDIDATE) and
          config["public_targets_sha256"] == sha(run.PUBLIC) and
          config["answer_sidecar_sha256"] == sha(run.ANSWERS),
          "source, binary, candidate, or fixture custody differs")
    check(candidate["candidate_id"] == prefix["current_candidate_id"] and
          candidate["candidate_id"].endswith("h" + candidate_digest(candidate)) and
          prefix["status"] == "PASS_EXACT_BASE_IDENTITY" and
          prefix["current_ic_source_sha256"] == run.SOURCE_SHA256,
          "candidate or prefix identity differs")
    check({k: v for k, v in config["algorithm_flags"].items()
           if k != "KIC_DUMP_BASE"} ==
          EXECUTED_FLAGS and
          Path(config["algorithm_flags"]["KIC_DUMP_BASE"]).name == "base_dump.jsonl" and
          Path(config["command"][0]).name == binary.name.split(".")[0] ==
          "koblitz_orbit_dlp_fast" and
          config["command"][1] == "construct:83:1:2304" and
          Path(config["command"][2]).name == "target.jsonl" and
          config["command"][3] == "29" and
          Path(config["command"][4]).name == "targets_result.jsonl" and
          config["cap_seconds"] == 1200 and
          config["rss_cap_bytes"] == 12 * 1024**3 and
          config["initial_host_free_percent"] >= 50 and
          config["initial_host_swap_used_mib"] >= 0 and
          config["min_host_free_percent"] == 10 and
          config["max_host_swap_increase_mib"] == 1024,
          "K2304 flags or resource envelope differs")
    point = one_jsonl(output / "target.jsonl")
    answer = one_jsonl(run.ANSWERS)
    check(point == config["target"] == one_jsonl(run.PUBLIC) == answer["point"],
          "earlier public point differs")
    for key, name in (("samples_sha256", "samples.json"),
                      ("stdout_sha256", "stdout.jsonl"),
                      ("stderr_sha256", "stderr.log"),
                      ("target_sha256", "target.jsonl"),
                      ("base_dump_sha256", "base_dump.jsonl"),
                      ("targets_result_sha256", "targets_result.jsonl")):
        check(receipt[key] == sha(output / name), f"{name} hash differs")
    check(receipt["status"] == "PASS_PROCESS_EXIT" and receipt["exit_code"] == 0 and
          receipt["wall_seconds"] <= config["cap_seconds"] + 5 and
          receipt["base_dump_sha256"] == prefix["prefix_ordered_dump_sha256"],
          "process status or ordered base differs")
    samples = read(output / "samples.json")
    check(isinstance(samples, list) and samples and
          receipt["max_sampled_rss_bytes"] == max(
              (item["rss_bytes"] or 0 for item in samples), default=0) and
          all(item["rss_bytes"] is not None and
              item["rss_bytes"] <= config["rss_cap_bytes"] and
              (item["host_free_percent"] is None or
               item["host_free_percent"] >= config["min_host_free_percent"]) and
              (item["host_swap_used_mib"] is None or
               item["host_swap_used_mib"] - config["initial_host_swap_used_mib"] <=
               config["max_host_swap_increase_mib"])
              for item in samples), "sampled host resource gate differs")
    dump, summary, row = (read(output / "base_dump.jsonl"),
                          one_jsonl(output / "stdout.jsonl"),
                          one_jsonl(output / "targets_result.jsonl"))
    check(summary["base_hash"] == dump["base_hash"] ==
          prefix["prefix_base_hash_blake3"], "factor-base hash differs")
    target = replay_target(dump, summary, row, point, answer, k=2304)
    return {"schema": "n83.k2304-current-source-independent-replay/v1",
            "status": "PASS_INDEPENDENT_REPLAY",
            "candidate_id": candidate["candidate_id"],
            "base_dump_sha256": receipt["base_dump_sha256"], **target}


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("output", type=Path)
    parser.add_argument("--binary", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(verify(args.output.resolve(), args.binary.resolve()), sort_keys=True))
