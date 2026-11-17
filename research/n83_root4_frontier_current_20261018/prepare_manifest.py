#!/usr/bin/env python3
"""Pin the K3596 root4 standard factor base to the batched-root source."""

import hashlib
import json
from copy import deepcopy
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
CURRENT = ROOT / "research/n83_online_finalists_20261018"
OLD_CONTROL = ROOT / "research/n83_crossfamily_cancel_20261017"
OLD_FRONTIER = ROOT / "research/n83_root4_memory_frontier_results_20261015"
SOURCE = "4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507"


def load_one(directory, usable_points):
    matches = list(directory.glob(f"candidate_IC1N83Ckb1a1fb{usable_points}*.json"))
    if len(matches) != 1:
        raise ValueError(f"expected one {usable_points}-point manifest in {directory}")
    return json.loads(matches[0].read_text())


def derive(old, current):
    result = deepcopy(current)
    if (old["point_decomposition"]["root_slot"] !=
            current["point_decomposition"]["root_slot"] or
            old["point_decomposition"]["root_bloom_filter"] !=
            current["point_decomposition"]["root_bloom_filter"]):
        raise ValueError("root4 standard index policy differs")
    result["factor_base"] = deepcopy(old["factor_base"])
    k = result["factor_base"]["effective_columns"]
    if result["factor_base"]["actual_usable_points"] != 166 * k:
        raise ValueError("factor-base orbit cardinality differs")
    result["relation_collection"]["stop_criterion"] = f"rank_{k}"
    result["relation_linear_algebra"]["columns"] = k
    result["relation_linear_algebra"]["rank_criterion"] = k
    without_id = {key: value for key, value in result.items()
                  if key != "candidate_id"}
    digest = hashlib.sha256(json.dumps(
        without_id, sort_keys=True, separators=(",", ":"),
        ensure_ascii=False).encode()).hexdigest()[:12]
    result["candidate_id"] = (
        f"IC1N83Ckb1a1fb{166 * k}PDP4rootRCguidedLAgaussTDdirectISO0h{digest}")
    return result


def main():
    source_path = ROOT / "examples/koblitz_orbit_dlp_fast.rs"
    if hashlib.sha256(source_path.read_bytes()).hexdigest() != SOURCE:
        raise ValueError("batched-root source differs")
    current = load_one(CURRENT, 581000)
    old_control = load_one(OLD_CONTROL, 581000)
    if derive(old_control, current) != current:
        raise ValueError("prior-to-current K3500 control differs")
    old = json.loads((OLD_FRONTIER / "candidate_K3596_root4_standard.json").read_text())
    if old["factor_base"]["effective_columns"] != 3596:
        raise ValueError("K3596 prior base differs")
    record = derive(old, current)
    path = HERE / f"candidate_{record['candidate_id']}.json"
    blob = json.dumps(record, indent=2, sort_keys=True) + "\n"
    if path.exists() and path.read_text() != blob:
        raise ValueError("sealed K3596 manifest differs")
    path.write_text(blob)
    print(json.dumps({"candidate_id": record["candidate_id"],
                      "actual_usable_points": record["factor_base"]["actual_usable_points"]},
                     sort_keys=True))


if __name__ == "__main__":
    main()
