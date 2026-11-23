"""Retain full K1107 pair receipts without duplicating identical base dumps."""

import argparse
import json
import os
from pathlib import Path
import shutil

from research.n83_fourbyte_pair_20261027.run import ROOT, sha256
from research.n83_fourbyte_pair_20261027.verify import verify_arm, write_json

HERE = Path(__file__).resolve().parent
BASE = ROOT / "research/n83_fourbyte_id31_20261027/runs/k1107_full/base_dump.jsonl"
BINARY = ROOT / "research/n83_fourbyte_fp2_20261027/runs/k1107_full/executed_binary"
FILES = ("launch.json", "receipt.json", "stdout.jsonl", "stderr.log",
         "targets_result.jsonl")


def archive(source, name):
    replay = json.loads((source / "independent_replay.json").read_text())
    if replay["status"] != "PASS_PAIRED_FULL_REPLAY":
        raise ValueError("source pair did not pass independent replay")
    schedule = json.loads((source / "schedule.json").read_text())
    destination = HERE / "runs" / name
    destination.mkdir(parents=True, exist_ok=False)
    shutil.copy2(source / "schedule.json", destination / "schedule.json")
    for sequence, layout in enumerate(schedule["order"].split("-")):
        arm_source = source / f"{sequence}_{layout}"
        arm_destination = destination / arm_source.name
        arm_destination.mkdir()
        for filename in FILES:
            shutil.copy2(arm_source / filename, arm_destination / filename)
        for filename, canonical in (("base_dump.jsonl", BASE),
                                    ("executed_binary", BINARY)):
            if sha256(arm_source / filename) != sha256(canonical):
                raise ValueError(f"{filename} differs from retained canonical bytes")
            (arm_destination / filename).symlink_to(
                os.path.relpath(canonical, arm_destination))
        checked = verify_arm(arm_destination, layout, sequence, schedule)
        if checked["status"] != "PASS_INDEPENDENT_FULL_REPLAY":
            raise ValueError("materialized arm failed independent replay")
    write_json(destination / "independent_replay.json", replay)
    return destination


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("source", type=Path)
    parser.add_argument("name", choices=("plain_fp2", "fp2_plain"))
    args = parser.parse_args()
    print(archive(args.source.resolve(strict=True), args.name))


if __name__ == "__main__":
    main()
