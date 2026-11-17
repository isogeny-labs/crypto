#!/usr/bin/env python3
"""Seal six exact N83 pipelines using the batched-root source snapshot."""

import copy
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
PRIOR_SOURCE = "8109fb33374ce2aa66c6a5a0b6e144d3b0143168ac756f9e0bd70af60a911829"
CURRENT_SOURCE = "4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507"
CURRENT_COMMIT = "ab9742b8969eb53caf4a0fcd4c42122faf8efdb6"
BRIDGE = ROOT / "research/n83_small_intermediate_bridge_20261018"
CROSS = ROOT / "research/n83_crossfamily_cancel_20261017"
BASES = {
    "k1107_root11": BRIDGE / "candidate_IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0hc54c7b61d40a.json",
    "k2304_root11": BRIDGE / "candidate_IC1N83Ckb1a1fb382464PDP4rootRCguidedLAgaussTDdirectISO0h7513107d16d1.json",
    "k2816_root11": CROSS / "candidate_IC1N83Ckb1a1fb467456PDP4rootRCguidedLAgaussTDdirectISO0h7a3cca2a9993.json",
    "k3072_root11": CROSS / "candidate_IC1N83Ckb1a1fb509952PDP4rootRCguidedLAgaussTDdirectISO0hde7207572a27.json",
    "k3450_wide3": CROSS / "candidate_IC1N83Ckb1a1fb572700PDP4rootRCguidedLAgaussTDdirectISO0hd7dca5070efb.json",
    "k3500_root4": CROSS / "candidate_IC1N83Ckb1a1fb581000PDP4rootRCguidedLAgaussTDdirectISO0h441d3898d220.json",
}
K = {name: int(name[1:].split("_")[0]) for name in BASES}


def canonical_id(record):
    stripped = {key: value for key, value in record.items() if key != "candidate_id"}
    digest = hashlib.sha256(json.dumps(
        stripped, sort_keys=True, separators=(",", ":"),
        ensure_ascii=False).encode()).hexdigest()[:12]
    return ("IC1N83Ckb1a1fb"
            f"{record['factor_base']['actual_usable_points']}"
            f"PDP4rootRCguidedLAgaussTDdirectISO0h{digest}")


def derive(name):
    source = json.loads(BASES[name].read_text())
    if (source["candidate_id"] != canonical_id(source) or
            source["factor_base"]["effective_columns"] != K[name] or
            source["factor_base"]["actual_usable_points"] != 166 * K[name] or
            source["implementation"]["source_sha256"]["ic"] != PRIOR_SOURCE):
        raise ValueError(f"{name}: source identity or factor base differs")
    record = copy.deepcopy(source)
    record["implementation"]["source_commit"] = CURRENT_COMMIT
    record["implementation"]["source_sha256"]["ic"] = CURRENT_SOURCE
    flags = record["implementation"]["algorithm_flags"]
    if (flags["target_threads"] != 2 or flags["target_cancel"] is not True or
            flags["indexed5_id30_request"] is not False):
        raise ValueError(f"{name}: target policy differs")
    flags["root_batch"] = True
    flags["root_batch_mode"] = "one_inversion_per_representative_pair"
    flags["root_batch_max_relative"] = 83
    for section, key in (("point_decomposition", "solver_source_sha256"),
                         ("relation_collection", "source_digest"),
                         ("relation_linear_algebra", "source_digest"),
                         ("target_descent", "source_digest")):
        if record[section][key] != PRIOR_SOURCE:
            raise ValueError(f"{name}: component source differs")
        record[section][key] = CURRENT_SOURCE
    record["point_decomposition"]["variant"] += "_batch_roots_per_pair"
    record["candidate_id"] = canonical_id(record)
    return record


def main():
    if hashlib.sha256((ROOT / "examples/koblitz_orbit_dlp_fast.rs").read_bytes()).hexdigest() != CURRENT_SOURCE:
        raise ValueError("current IC source differs")
    for name in BASES:
        record = derive(name)
        path = HERE / f"candidate_{record['candidate_id']}.json"
        blob = json.dumps(record, indent=2, sort_keys=True) + "\n"
        if path.exists() and path.read_text() != blob:
            raise ValueError(f"{name}: sealed candidate differs")
        path.write_text(blob)
        print(name, record["candidate_id"])


if __name__ == "__main__":
    main()
