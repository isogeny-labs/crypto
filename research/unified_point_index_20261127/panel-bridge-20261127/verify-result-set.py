#!/usr/bin/env python3
"""Check all three separately downloaded point-index panel repetitions."""

import argparse
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys
import tempfile


HERE = Path(__file__).resolve().parent
loader = importlib.util.spec_from_file_location("point_index_readback", HERE / "verify-result-readback.py")
readback = importlib.util.module_from_spec(loader)
loader.loader.exec_module(readback)


def paired_split(rows, panel_bin, split, expected_cases):
    with tempfile.TemporaryDirectory(prefix="point-index-split-") as temporary:
        path = Path(temporary, "rows.jsonl")
        with path.open("w") as stream:
            for slot, row in rows:
                if row["kind"] not in ("aa", "ab") or row["split"] != split:
                    continue
                if type(row["round"]) is not int or row["round"] not in range(5):
                    readback.fail("row has an unexpected paired round")
                copy = dict(row)
                copy["round"] += 10 * slot
                stream.write(json.dumps(copy, separators=(",", ":")) + "\n")
        rerun = subprocess.run([panel_bin, "analyze", str(path)], capture_output=True,
                               timeout=120, check=False)
        if rerun.returncode != 0:
            readback.fail(f"three-repetition {split} analysis failed")
        analysis = json.loads(rerun.stdout, object_pairs_hook=readback.unique_pairs)
    for kind in ("aa", "ab"):
        result = analysis[kind]
        if (result["complete_pairs"] != expected_cases * 15
                or result["excluded_pairs"] != 0
                or result["cases_in_interval"] != expected_cases
                or result["duplicate_member_rows"] != 0):
            readback.fail(f"incomplete three-repetition {split} {kind} pairs")
    return {kind: analysis[kind] for kind in ("aa", "ab")}


def verify(args):
    if len(args.raw) != 3 or len(args.s3_uri) != 3:
        readback.fail("exactly three result trees and S3 prefixes are required")
    if len({Path(raw).resolve() for raw in args.raw}) != 3 or len(set(args.s3_uri)) != 3:
        readback.fail("repeated result tree or S3 prefix")
    slots = {}
    for raw, uri in zip(args.raw, args.s3_uri):
        one = argparse.Namespace(**{**vars(args), "raw": raw, "s3_uri": uri})
        result = readback.verify(one)
        match = re.search(r"-s([012])-a0$", result["job_name"])
        if match is None:
            readback.fail("result lacks a schedule slot")
        slot = int(match.group(1))
        if slot in slots:
            readback.fail("repeated schedule slot")
        slots[slot] = (raw, result)
    if set(slots) != {0, 1, 2}:
        readback.fail("missing schedule repetition")
    output = {"schema": "point-index.result-set/v1", "scope": "structure_only" if args.structure_only
              else "complete_replay", "schedule_slots": [0, 1, 2],
              "jobs": [slots[slot][1]["job_name"] for slot in range(3)],
              "scientific_result": None}
    if args.structure_only:
        return output
    manifest = readback.read_json(Path(args.inputs, "manifest.json"))
    split_by_id = {case["id"]: case["split"] for case in manifest["cases"]}
    if (len(split_by_id) != 84 or list(split_by_id.values()).count("development") != 60
            or list(split_by_id.values()).count("holdout") != 24):
        readback.fail("unexpected public split dimensions")
    rows = []
    for slot in range(3):
        raw = Path(slots[slot][0])
        for line in (raw / "panel-run/rows.jsonl").read_text().splitlines():
            row = json.loads(line, object_pairs_hook=readback.unique_pairs)
            if split_by_id.get(row["case_id"]) != row["split"]:
                readback.fail("row split differs from public case")
            rows.append((slot, row))
    output["verified_rows"] = len(rows)
    output["fixed_target_strata"] = 6
    output["split_analysis"] = {
        "development": paired_split(rows, args.panel_bin, "development", 60),
        "holdout_internal_randomness": paired_split(rows, args.panel_bin, "holdout", 24),
    }
    return output


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--raw", action="append", required=True)
    parser.add_argument("--s3-uri", action="append", required=True)
    parser.add_argument("--image", required=True)
    parser.add_argument("--checkpoint", required=True)
    parser.add_argument("--inputs", required=True)
    parser.add_argument("--panel-bin")
    parser.add_argument("--baseline-bin")
    parser.add_argument("--candidate-bin")
    parser.add_argument("--structure-only", action="store_true")
    args = parser.parse_args()
    if not args.structure_only and not all((args.panel_bin, args.baseline_bin, args.candidate_bin)):
        parser.error("complete replay requires all three exact binaries")
    try:
        print(json.dumps(verify(args), sort_keys=True))
    except (ValueError, KeyError, TypeError, AttributeError, IndexError, OSError,
            subprocess.TimeoutExpired) as error:
        print(f"result set rejected: {error}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main())
