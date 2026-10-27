#!/usr/bin/env python3
"""Derive the exact K2816 four-worker cancellation candidate identity."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
PARENT_ID = "IC1N83Ckb1a1fb467456PDP4rootRCguidedLAgaussTDdirectISO0h7a3cca2a9993"
PARENT = ROOT / "research/n83_highk_cancel_pilot_20261017" / f"candidate_{PARENT_ID}.json"


def candidate():
    record = json.loads(PARENT.read_text())
    assert record.pop("candidate_id") == PARENT_ID
    old = "two_worker_cancel"
    new = "four_worker_cancel"
    variant = record["point_decomposition"]["variant"]
    policy = record["target_descent"]["policy"]
    assert variant.endswith(old) and policy.endswith(old)
    record["point_decomposition"]["variant"] = variant[:-len(old)] + new
    record["target_descent"]["policy"] = policy[:-len(old)] + new
    flags = record["implementation"]["algorithm_flags"]
    assert flags["target_threads"] == 2 and flags["target_cancel"] is True
    flags["target_threads"] = 4
    digest = hashlib.sha256(json.dumps(record, sort_keys=True,
                                       separators=(",", ":"),
                                       ensure_ascii=False).encode()).hexdigest()[:12]
    record["candidate_id"] = PARENT_ID.rsplit("h", 1)[0] + "h" + digest
    return record


def main():
    record = candidate()
    path = HERE / f"candidate_{record['candidate_id']}.json"
    blob = json.dumps(record, indent=2, sort_keys=True) + "\n"
    if path.exists() and path.read_text() != blob:
        raise ValueError("candidate manifest differs")
    path.write_text(blob)
    print(record["candidate_id"])


if __name__ == "__main__":
    main()
