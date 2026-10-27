#!/usr/bin/env python3
"""Register the K1107 fixed-root4 candidate after independent replay."""

import argparse
import copy
import hashlib
import json
from pathlib import Path

from research.n83_crossfamily_current_20261025 import arms
from research.n83_nearby_grid_tier1_20261024 import protocol as screen
from research.n83_root4_small_control_20261025 import run, verify


HERE = Path(__file__).resolve().parent
SMALL_DIR = run.ROOT / "research/n83_nearby_full_grid_20261024/identities"
PREFIX = "IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0h"


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"),
                      ensure_ascii=False).encode()


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def seal(output):
    replay = verify.verify(output)
    if replay["status"] != "PASS_INDEPENDENT_REPLAY":
        raise ValueError("fixed-root4 control has not passed independent replay")
    small_id, small_sha = screen.CANDIDATES[run.K]
    small_path = SMALL_DIR / f"candidate_{small_id}.json"
    if sha(small_path) != small_sha:
        raise ValueError("existing K1107 candidate manifest differs")
    small = json.loads(small_path.read_text())
    wide_path, wide = arms.manifest(arms.ARMS[2])
    if (small["factor_base"]["source_base_hash_blake3"] != run.BASE_HASH or
            small["factor_base"]["ordered_base_dump_sha256"] !=
            run.BASE_DUMP_SHA256 or
            replay["base_hash"] != run.BASE_HASH or
            replay["base_dump_sha256"] != run.BASE_DUMP_SHA256 or
            wide["implementation"]["source_sha256"]["ic"] != run.SOURCE_SHA256 or
            small["implementation"]["source_sha256"]["ic"] != run.SOURCE_SHA256):
        raise ValueError("base, source or control replay differs")
    record = copy.deepcopy(wide)
    record["factor_base"] = copy.deepcopy(small["factor_base"])
    record["relation_collection"]["stop_criterion"] = f"rank_{run.K}"
    record["relation_linear_algebra"]["columns"] = run.K
    record["relation_linear_algebra"]["rank_criterion"] = run.K
    record.pop("candidate_id")
    digest = hashlib.sha256(canonical(record)).hexdigest()[:12]
    candidate_id = PREFIX + digest
    record["candidate_id"] = candidate_id
    blob = json.dumps(record, sort_keys=True, indent=2) + "\n"
    destination = HERE / f"candidate_{candidate_id}.json"
    if destination.exists() and destination.read_text() != blob:
        raise ValueError("candidate manifest differs from sealed version")
    destination.write_text(blob)
    return {"candidate_id": candidate_id,
            "candidate_manifest_sha256": sha(destination),
            "small_base_manifest_sha256": small_sha,
            "root4_policy_manifest_sha256": sha(wide_path),
            "control_status": replay["status"],
            "scope": "verified_candidate_identity_after_existing_point_control"}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("control_output", type=Path)
    args = parser.parse_args()
    print(json.dumps(seal(args.control_output.resolve()), sort_keys=True))


if __name__ == "__main__":
    main()
