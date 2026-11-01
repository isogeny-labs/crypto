#!/usr/bin/env python3
"""Pin K992 and K1024 to the batched-root IC source used by the six-arm panel."""

import hashlib
import json
from copy import deepcopy
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
OLD = ROOT / "research/n83_k992_cold_followup_20261017"
CURRENT = ROOT / "research/n83_online_finalists_20261018"


def load_one(directory, k):
    matches = list(directory.glob(f"candidate_IC1N83Ckb1a1fb{166 * k}*.json"))
    if len(matches) != 1:
        raise ValueError(f"expected exactly one K{k} manifest in {directory}")
    return json.loads(matches[0].read_text())


def update(old, current):
    result = deepcopy(current)
    result["factor_base"] = deepcopy(old["factor_base"])
    result["point_decomposition"]["root_slot"] = deepcopy(
        old["point_decomposition"]["root_slot"])
    k = result["factor_base"]["effective_columns"]
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
    current = load_one(CURRENT, 1107)
    control = update(load_one(OLD, 1107), current)
    if control != current:
        raise ValueError("old-to-current K1107 transformation differs")
    for k in (992, 1024):
        manifest = update(load_one(OLD, k), current)
        path = HERE / f"candidate_{manifest['candidate_id']}.json"
        blob = json.dumps(manifest, indent=2, sort_keys=True) + "\n"
        if path.exists() and path.read_text() != blob:
            raise ValueError(f"sealed manifest differs: {path}")
        path.write_text(blob)
        print(json.dumps({"k": k, "candidate_id": manifest["candidate_id"]},
                         sort_keys=True))


if __name__ == "__main__":
    main()
