"""Independent K4405 current-source Linux control replay.

This checks one earlier public point. Tier 1 same-point IC/rho admission is a
separate experiment after this control passes.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
from pathlib import Path

from research.n83_indexed5_20261013 import analyze as group
from research.n83_k4405_linux_control_20261026 import run


ROOT = Path(__file__).resolve().parents[2]
PREFIX = ROOT / "research/n83_k4300_k4405_current_source_prefix_20261025/k4405_prefix_receipt.json"
CANDIDATE = ROOT / "research/n83_k4300_k4405_current_source_prefix_20261025/candidate_IC1N83Ckb1a1fb731230PDP4rootRCguidedLAgaussTDdirectISO0h76ff6ddee28c.json"
PHASES = ("target_query_ms", "target_pdp_ms", "target_relation_check_ms",
          "target_descent_ms", "target_recovery_check_ms")


def check(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def sha(path: Path) -> str:
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def candidate_digest(candidate: dict) -> str:
    record = {key: value for key, value in candidate.items() if key != "candidate_id"}
    encoded = json.dumps(record, sort_keys=True, separators=(",", ":"),
                         ensure_ascii=False).encode()
    return hashlib.sha256(encoded).hexdigest()[:12]


def read(path: Path) -> dict:
    return json.loads(path.read_text())


def one_jsonl(path: Path) -> dict:
    rows = [json.loads(line) for line in path.read_text().splitlines() if line]
    check(len(rows) == 1, f"{path.name} must have one JSON row")
    return rows[0]


def counters(state: dict) -> tuple[int, int]:
    cpu = dict(line.split(maxsplit=1) for line in state["cpu_stat"].splitlines())
    events = dict(line.split(maxsplit=1) for line in state["memory_events"].splitlines())
    return int(cpu["nr_throttled"]), int(events["oom"]) + int(events["oom_kill"])


def replay_target(dump: dict, summary: dict, row: dict, point: dict, answer: dict,
                  *, k: int = 4405) -> dict:
    check(summary["n"] == row["n"] == 83 and row["a"] == 1,
          "curve parameters differ")
    check(summary["orbit_columns"] == summary["rank"] == k and
          summary["factor_base_points"] == len(dump["factor_base_point_coordinates"]) == 166*k and
          len(dump["factor_base_point_labels"]) == 166*k and
          len(dump["factor_base_representatives"]) == k,
          "factor base or rank differs")
    check(summary["targets"] == summary["targets_solved"] == 1 and
          summary["targets_failed"] == 0 and summary["rank_relations"] >= k,
          "producer rank or target coverage differs")
    check(row["target"] == point and row["published_q"] == point and
          row["fixture_index"] == answer["fixture_index"] == 0 and
          row["group_verified"] is True, "target fixture differs")
    q = group.point_from_json(point)
    scalar = row["recovered_scalar"]
    check(isinstance(scalar, int) and not isinstance(scalar, bool) and
          0 <= scalar < group.R and scalar == int(answer["scalar"]) and
          group.multiply(group.G, scalar) == q,
          "independent scalar replay differs")
    indices = row["point_indices"]
    check(isinstance(indices, list) and len(indices) == 4 and
          all(isinstance(i, int) and not isinstance(i, bool) and
              0 <= i < 166*k for i in indices), "four-point witness differs")
    relation = None
    for index in indices:
        relation = group.add(relation, group.point_from_json(
            dump["factor_base_point_coordinates"][index]))
    check(relation == q, "independent four-point relation replay differs")
    phases = [row[name] for name in PHASES]
    check(all(isinstance(value, (int, float)) and not isinstance(value, bool) and
              math.isfinite(value) and value >= 0 for value in phases) and
          abs(math.fsum(phases) - row["target_ms"]) < 1 and
          isinstance(row["probes"], int) and row["probes"] > 0,
          "online phase accounting differs")
    return {"rank": k, "recovered_scalar": scalar,
            "target_ms": row["target_ms"], "probes": row["probes"]}


def verify(output: Path, binary: Path) -> dict:
    config, receipt = read(output / "config.json"), read(output / "receipt.json")
    check(config["schema"] == "n83.k4405-current-source-linux-control/v1" and
          receipt["schema"] == "n83.k4405-current-source-linux-receipt/v1" and
          config["source_sha256"] == run.SOURCE_SHA256 == sha(run.SOURCE) and
          config["runner_sha256"] == sha(Path(run.__file__)) and
          config["binary_sha256"] == sha(binary) and
          config["source_flags_sha256"] == sha(run.SOURCE_FLAGS) and
          config["public_targets_sha256"] == sha(run.PUBLIC) and
          config["answer_sidecar_sha256"] == sha(run.ANSWERS),
          "source, binary, or input custody differs")
    expected = dict(read(run.SOURCE_FLAGS)["algorithm_flags"])
    expected.update({"KIC_DUMP_BASE": str(output / "base_dump.jsonl"),
                     "KIC_INDEX_ROOT_THREADS": "4"})
    check({k: v for k, v in config["algorithm_flags"].items()
           if k != "KIC_DUMP_BASE"} ==
          {k: v for k, v in expected.items() if k != "KIC_DUMP_BASE"} and
          Path(config["algorithm_flags"]["KIC_DUMP_BASE"]).name == "base_dump.jsonl" and
          config["command"][1] == "construct:83:1:4405" and
          Path(config["command"][2]).name == "target.jsonl" and
          config["command"][3] == "29" and
          Path(config["command"][4]).name == "targets_result.jsonl" and
          Path(config["command"][0]).name == binary.name == "koblitz_orbit_dlp_fast" and
          config["cap_seconds"] == 1200 and
          config["memory_stop_bytes"] == 48 * 1024**3,
          "K4405 algorithm or resource envelope differs")
    point = one_jsonl(output / "target.jsonl")
    answer = one_jsonl(run.ANSWERS)
    check(point == config["target"] == one_jsonl(run.PUBLIC) == answer["point"],
          "earlier public target differs")
    for key, filename in (("samples_sha256", "samples.json"),
                          ("stdout_sha256", "stdout.jsonl"),
                          ("stderr_sha256", "stderr.log"),
                          ("target_sha256", "target.jsonl")):
        check(receipt[key] == sha(output / filename), f"{filename} digest differs")
    check(receipt["status"] == "PASS_PROCESS_EXIT" and receipt["exit_code"] == 0,
          "producer did not finish under its resource cap")
    check(receipt["base_dump_sha256"] == sha(output / "base_dump.jsonl") and
          receipt["targets_result_sha256"] == sha(output / "targets_result.jsonl"),
          "producer base or target artifact digest differs")
    start, stop = config["initial_cgroup"], receipt["final_cgroup"]
    samples = read(output / "samples.json")
    check(isinstance(samples, list) and samples and
          all(item["memory_current"] is not None and
              item["memory_current"] < config["memory_stop_bytes"] and
              item["memory_swap_current"] is not None and
              item["memory_swap_current"] <= 2*1024**3
              for item in samples), "sampled cgroup resource gate differs")
    check(start["cgroup_interface"] == stop["cgroup_interface"] and
          start["memory_max"] is not None and
          56*1024**3 <= start["memory_max"] <= 128*1024**3 and
          stop["memory_current"] < config["memory_stop_bytes"] and
          stop["memory_peak"] is not None and
          stop["memory_peak"] < config["memory_stop_bytes"] and
          stop["memory_swap_current"] <= 2*1024**3 and
          counters(start) == counters(stop),
          "cgroup resource or noise gate differs")
    prefix = read(PREFIX)
    candidate = read(CANDIDATE)
    check(prefix["status"] == "PASS_EXACT_BASE_IDENTITY" and
          candidate["candidate_id"] == prefix["current_candidate_id"] and
          candidate["candidate_id"].endswith("h" + candidate_digest(candidate)) and
          prefix["current_ic_source_sha256"] == run.SOURCE_SHA256 and
          receipt["base_dump_sha256"] == prefix["prefix_ordered_dump_sha256"],
          "ordered K4405 base identity differs")
    dump, summary, row = (read(output / "base_dump.jsonl"),
                          one_jsonl(output / "stdout.jsonl"),
                          one_jsonl(output / "targets_result.jsonl"))
    check(summary["base_hash"] == dump["base_hash"] ==
          prefix["prefix_base_hash_blake3"], "factor-base hash differs")
    target = replay_target(dump, summary, row, point, answer)
    return {"schema": "n83.k4405-linux-independent-replay/v1",
            "status": "PASS_INDEPENDENT_REPLAY", "candidate_id": candidate["candidate_id"],
            "binary_sha256": config["binary_sha256"], "base_dump_sha256": receipt["base_dump_sha256"],
            **target}


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("output", type=Path)
    parser.add_argument("--binary", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(verify(args.output.resolve(), args.binary.resolve()), sort_keys=True))
