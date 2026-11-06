#!/usr/bin/env python3
"""Independently replay high-K cancellation pilot bases and target relations."""

import argparse
import json
from pathlib import Path

from research.n83_highk_cancel_pilot_20261017 import pilot
from research.n83_root11_dynamic_20261016 import verify as dynamic


def read(path):
    return json.loads(path.read_text())


def verify(output):
    source = read(output / "source.json")
    host = read(output / "host.json")
    expected = pilot.preflight(output / "executed_bin")
    if (source.get("schema") != "n83.highk-cancel-pilot-source/v1" or
            any(source.get(key) != value for key, value in expected.items()) or
            source.get("runner_sha256") != pilot.sha256(Path(pilot.__file__)) or
            source.get("arms") != [list(arm) for arm in pilot.ARMS] or
            source.get("cap_seconds") != pilot.CAP_SECONDS):
        raise ValueError("frozen source, binary, target or run policy differs")
    dynamic.coldver.runner.parent.EXPECTED[
        dynamic.coldver.runner.parent.SOURCE] = pilot.EXPECTED[pilot.SOURCE]
    point, answer = dynamic.coldver.frozen_point()[1:]
    rows = []
    for name, k in pilot.ARMS:
        folder = output / "runs" / name
        launch = read(folder / "launch.json")
        usage = read(folder / "usage.json")
        flags = launch["environment_flags"]
        if (launch["command"][1:] !=
                [f"construct:83:1:{k}", str(pilot.TARGET), "29",
                 str(folder / "targets_result.jsonl")] or
                launch["cap_seconds"] != pilot.CAP_SECONDS or
                flags.get("KIC_TARGET_THREADS") != "2" or
                flags.get("KIC_TARGET_CANCEL") != "1"):
            raise ValueError(f"{name}: executed cancellation policy differs")
        if usage["status"] != "PASS_PRODUCER":
            rows.append({"arm": name, "k": k, "status": usage["status"],
                         "cap_seconds": launch["cap_seconds"],
                         "wall_seconds": usage["wall_seconds"],
                         "exit_status": usage["exit_status"],
                         "stop_reason": usage["stop_reason"],
                         "resource_status": "UNVERIFIED_MAC_RSS"})
            continue
        summary = read(folder / "stdout.jsonl")
        if (summary.get("target_threads") != 2 or
                summary.get("target_cancel") is not True):
            raise ValueError(f"{name}: producer worker policy differs")
        rows.append(dynamic.verify_ic(output, name, k, "root11_dynamic",
                                      point, answer))
    return {"schema": "n83.highk-cancel-pilot-analysis/v1",
            "source_sha256": source["source_sha256"],
            "target_sha256": source["target_sha256"],
            "binary_sha256": source["binary_sha256"],
            "resource_telemetry_status": (
                "UNVERIFIED_MAC_RSS" if host.get("platform", "").startswith("macOS")
                else "REVIEW_HOST_RECEIPT"),
            "arms": rows,
            "status": "PASS_INDEPENDENT_REPLAY" if all(
                row["status"] == "PASS_INDEPENDENT_REPLAY" for row in rows)
                else "INCOMPLETE_OR_FAILED"}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    print(json.dumps(verify(args.output.resolve()), indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
