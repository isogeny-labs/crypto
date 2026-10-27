#!/usr/bin/env python3
"""Exploratory K2816 one-target thread-count control on an exposed point."""

import argparse
import hashlib
import json
import shutil
from pathlib import Path

from research.n83_highk_cancel_pilot_20261017 import pilot as parent_pilot
from research.n83_thread4_pilot_20261017.generate_candidate import PARENT_ID, candidate

ROOT = Path(__file__).resolve().parents[2]
CAP_SECONDS = 900


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def preflight(binary):
    source = parent_pilot.preflight(binary)
    record = candidate()
    manifest = Path(__file__).parent / f"candidate_{record['candidate_id']}.json"
    if json.loads(manifest.read_text()) != record:
        raise ValueError("four-worker candidate differs")
    source.update({"candidate_id": record["candidate_id"],
                   "candidate_sha256": sha256(manifest),
                   "parent_candidate_id": PARENT_ID,
                   "target_threads": 4, "target_cancel": True,
                   "cap_seconds": CAP_SECONDS})
    return source


def run(binary, output):
    source = preflight(binary)
    if output.exists():
        raise ValueError(f"output already exists: {output}")
    output.mkdir(parents=True)
    executed = output / "executed_bin"
    shutil.copy2(binary, executed)
    if sha256(executed) != source["binary_sha256"]:
        raise ValueError("copied binary differs")
    source.update({"schema": "n83.thread4-pilot-source/v1",
                   "runner_sha256": sha256(Path(__file__))})
    parent_pilot.prior.parent.write_json(output / "source.json", source)
    group = parent_pilot.prior.parent.cgroup_root()
    parent_pilot.prior.parent.write_json(
        output / "host.json", parent_pilot.prior.parent.host_receipt(group))
    folder = output / "runs" / "k2816_thread4"
    flags = parent_pilot.prior.ic_environment(folder, "root11_dynamic",
                                              "parallel_cancel")
    flags["KIC_TARGET_THREADS"] = "4"
    if flags["KIC_TARGET_CANCEL"] != "1" or flags["KIC_INDEXED5_ID30"] != "0":
        raise ValueError("target policy differs")
    command = [str(executed), "construct:83:1:2816",
               str(parent_pilot.TARGET), "29",
               str(folder / "targets_result.jsonl")]
    usage = parent_pilot.prior.parent.one_run(
        folder, command, flags, CAP_SECONDS, group)
    print(json.dumps({"candidate_id": source["candidate_id"],
                      "status": usage["status"],
                      "wall_seconds": usage["wall_seconds"]},
                     sort_keys=True), flush=True)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path)
    parser.add_argument("--preflight", action="store_true")
    args = parser.parse_args()
    binary = args.binary.resolve()
    if args.preflight:
        print(json.dumps(preflight(binary), sort_keys=True))
    elif args.output:
        run(binary, args.output.resolve())
    else:
        parser.error("--output is required without --preflight")


if __name__ == "__main__":
    main()
