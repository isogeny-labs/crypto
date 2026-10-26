#!/usr/bin/env python3
"""Independent arithmetic replay of the exposed-point K2816 thread4 pilot."""

import argparse
import json
from pathlib import Path

from research.n83_highk_cancel_pilot_20261017 import pilot as parent_pilot
from research.n83_root11_dynamic_20261016 import verify as dynamic
from research.n83_thread4_pilot_20261017.generate_candidate import candidate
from research.n83_thread4_pilot_20261017.pilot import preflight, sha256

HERE = Path(__file__).resolve().parent


def one_json(path):
    return json.loads(path.read_text())


def verify(output):
    record = candidate()
    source = one_json(output / "source.json")
    manifest = HERE / f"candidate_{record['candidate_id']}.json"
    expected = preflight(output / "executed_bin")
    if (source["schema"] != "n83.thread4-pilot-source/v1" or
            source["candidate_id"] != record["candidate_id"] or
            source["candidate_sha256"] != sha256(manifest) or
            source["source_sha256"] != expected["source_sha256"] or
            source["target_sha256"] != expected["target_sha256"] or
            source["answer_sha256"] != expected["answer_sha256"] or
            source["binary_sha256"] != expected["binary_sha256"] or
            source["runner_sha256"] != sha256(HERE / "pilot.py") or
            source["target_threads"] != 4 or
            source["target_cancel"] is not True):
        raise ValueError("source, input, binary or candidate identity differs")
    # The imported arithmetic verifier was frozen against an older IC source.
    # Bind its input check to this pilot's exact current-source receipt.
    dynamic.coldver.runner.parent.EXPECTED[
        dynamic.coldver.runner.parent.SOURCE] = parent_pilot.EXPECTED[
            parent_pilot.SOURCE]
    _, point, scalar = dynamic.coldver.frozen_point()
    result = dynamic.verify_ic(output, "k2816_thread4", 2816,
                               "root11_dynamic", point, scalar)
    if result["status"] != "PASS_INDEPENDENT_REPLAY":
        return {"schema": "n83.thread4-pilot-replay/v1",
                "status": "BOUNDED_UNKNOWN", "raw_replay": result}
    folder = output / "runs/k2816_thread4"
    summary = one_json(folder / "stdout.jsonl")
    launch = one_json(folder / "launch.json")
    usage = one_json(folder / "usage.json")
    if (summary["target_threads"] != 4 or
            summary["target_cancel"] is not True or
            summary["root_table_indexed5_id30_enabled"] is not False or
            launch["environment_flags"]["KIC_TARGET_THREADS"] != "4" or
            launch["environment_flags"]["KIC_TARGET_CANCEL"] != "1" or
            launch["environment_flags"]["KIC_INDEXED5_ID30"] != "0" or
            launch["cap_seconds"] != 900 or
            sha256(folder / "base_dump.jsonl") !=
            record["factor_base"]["ordered_base_dump_sha256"] or
            summary["base_hash"] !=
            record["factor_base"]["source_base_hash_blake3"]):
        raise ValueError("executed policy or exact factor base differs")
    return {"schema": "n83.thread4-pilot-replay/v1",
            "status": "PASS_INDEPENDENT_ARITHMETIC_REPLAY",
            "hardware_tier": "ordinary_host_exploratory",
            "candidate_id": record["candidate_id"],
            "one_target_online_ms": result["one_target_online_ms"],
            "cold_process_ms": result["cold_process_ms_exploratory"],
            "peak_rss_bytes": usage["peak_rss_bytes"],
            "resource_status": result["resources"]["status"],
            "raw_replay": result}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    output = args.output.resolve()
    result = verify(output)
    path = output / "replay.json"
    blob = json.dumps(result, indent=2, sort_keys=True) + "\n"
    if path.exists() and path.read_text() != blob:
        raise ValueError("sealed replay differs")
    path.write_text(blob)
    print(json.dumps({key: value for key, value in result.items()
                      if key != "raw_replay"}, sort_keys=True))
    return 0 if result["status"] == "PASS_INDEPENDENT_ARITHMETIC_REPLAY" else 2


if __name__ == "__main__":
    raise SystemExit(main())
