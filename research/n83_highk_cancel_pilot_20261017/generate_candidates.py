#!/usr/bin/env python3
"""Derive exact high-K join/cancel IC1 manifests from frozen source records."""

import copy
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SOURCE = "8109fb33374ce2aa66c6a5a0b6e144d3b0143168ac756f9e0bd70af60a911829"
BASE_MANIFESTS = {
    2816: ROOT / "research/n83_intermediate_factor_base_holdout_20261017/"
           "candidate_IC1N83Ckb1a1fb467456PDP4rootRCguidedLAgaussTDdirectISO0hf693b1b33c8b.json",
    3072: ROOT / "research/n83_smallk_challenge_20261017/"
           "candidate_IC1N83Ckb1a1fb509952PDP4rootRCguidedLAgaussTDdirectISO0hec7ae07d8b9a.json",
}
POLICY_MANIFESTS = {
    "join": ROOT / "research/n83_target_cancel_tier1_repeat_20261017/"
            "candidate_IC1N83Ckb1a1fb164672PDP4rootRCguidedLAgaussTDdirectISO0h33c9c3b2f933.json",
    "cancel": ROOT / "research/n83_target_cancel_tier1_repeat_20261017/"
              "candidate_IC1N83Ckb1a1fb164672PDP4rootRCguidedLAgaussTDdirectISO0h58ec337f71f7.json",
}


def canonical_id(record):
    stripped = {key: value for key, value in record.items()
                if key != "candidate_id"}
    digest = hashlib.sha256(json.dumps(
        stripped, sort_keys=True, separators=(",", ":"),
        ensure_ascii=False).encode()).hexdigest()[:12]
    factor = record["factor_base"]
    return ("IC1N83Ckb1a1fb"
            f"{factor['actual_usable_points']}PDP4rootRCguidedLAgaussTDdirectISO0h{digest}")


def read_checked(path):
    value = json.loads(path.read_text())
    if value["candidate_id"] != canonical_id(value):
        raise ValueError(f"noncanonical source candidate: {path}")
    return value


def generate(k, policy):
    base = read_checked(BASE_MANIFESTS[k])
    template = read_checked(POLICY_MANIFESTS[policy])
    if (base["factor_base"]["actual_usable_points"] != 166 * k or
            base["factor_base"]["effective_columns"] != k or
            template["implementation"]["source_sha256"]["ic"] != SOURCE or
            template["implementation"]["algorithm_flags"]["target_threads"] != 2 or
            template["implementation"]["algorithm_flags"]["target_cancel"]
            is not (policy == "cancel")):
        raise ValueError("frozen geometry or worker policy differs")
    result = copy.deepcopy(base)
    result["implementation"] = copy.deepcopy(template["implementation"])
    for section, keys in {
        "point_decomposition": ("solver_source_sha256", "variant"),
        "relation_collection": ("source_digest",),
        "relation_linear_algebra": ("source_digest",),
        "target_descent": ("policy", "source_digest"),
    }.items():
        for key in keys:
            result[section][key] = template[section][key]
    result["candidate_id"] = canonical_id(result)
    return result


def main():
    for k in BASE_MANIFESTS:
        for policy in POLICY_MANIFESTS:
            record = generate(k, policy)
            path = HERE / f"candidate_{record['candidate_id']}.json"
            blob = json.dumps(record, indent=2, sort_keys=True) + "\n"
            if path.exists() and path.read_text() != blob:
                raise ValueError(f"sealed candidate differs: {path}")
            path.write_text(blob)
            print(k, policy, record["candidate_id"])


if __name__ == "__main__":
    main()
