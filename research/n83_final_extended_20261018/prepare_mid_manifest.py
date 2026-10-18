#!/usr/bin/env python3
"""Pin the historical K3300 wide3 base to the current batched-root source."""

import json
from pathlib import Path

from research.n83_final_extended_20261018.prepare_small_manifests import update

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
CURRENT = ROOT / "research/n83_online_finalists_20261018"
PRIOR = ROOT / "research/n83_best_e2e_results_20261015"


def main():
    current_path = next(CURRENT.glob("candidate_IC1N83Ckb1a1fb572700*.json"))
    prior_path = PRIOR / "candidate_K3300_root11_wide3.json"
    current = json.loads(current_path.read_text())
    prior = json.loads(prior_path.read_text())
    if (current["factor_base"]["effective_columns"] != 3450 or
            prior["factor_base"]["effective_columns"] != 3300 or
            prior["point_decomposition"]["root_slot"] !=
            current["point_decomposition"]["root_slot"] or
            prior["point_decomposition"]["root_bloom_filter"] !=
            current["point_decomposition"]["root_bloom_filter"]):
        raise ValueError("K3300 historical base or wide3 layout differs")
    record = update(prior, current)
    path = HERE / f"candidate_{record['candidate_id']}.json"
    blob = json.dumps(record, indent=2, sort_keys=True) + "\n"
    if path.exists() and path.read_text() != blob:
        raise ValueError("sealed K3300 manifest differs")
    path.write_text(blob)
    print(json.dumps({"candidate_id": record["candidate_id"],
                      "actual_usable_points": record["factor_base"]["actual_usable_points"]},
                     sort_keys=True))


if __name__ == "__main__":
    main()
