#!/usr/bin/env python3
"""Seal same-source IC1 identities for four N83 cancellation factor bases."""

import copy
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
CURRENT_SOURCE = "8109fb33374ce2aa66c6a5a0b6e144d3b0143168ac756f9e0bd70af60a911829"
PRIOR_SOURCE = "8a2fe55dd90dc33892c197d27260182bf783fc8212fb485d8297daf2bbba8e12"
CURRENT_COMMIT = "c54d94c6e6db4978521b50208ce62bed24b580b4"
REFERENCE = ROOT / "research/n83_k3072_current_source_20261017"
PILOT = ROOT / "research/n83_highk_cancel_pilot_20261017"
BASES = {
    "k2816_root11": PILOT / "candidate_IC1N83Ckb1a1fb467456PDP4rootRCguidedLAgaussTDdirectISO0h7a3cca2a9993.json",
    "k3072_root11": PILOT / "candidate_IC1N83Ckb1a1fb509952PDP4rootRCguidedLAgaussTDdirectISO0hde7207572a27.json",
    "k3450_wide3": REFERENCE / "candidate_IC1N83Ckb1a1fb572700PDP4rootRCguidedLAgaussTDdirectISO0hbde4a785a36e.json",
    "k3500_root4": REFERENCE / "candidate_IC1N83Ckb1a1fb581000PDP4rootRCguidedLAgaussTDdirectISO0hb3a616c4e22f.json",
}
EXPECTED = {"k2816_root11": (2816, 467456),
            "k3072_root11": (3072, 509952),
            "k3450_wide3": (3450, 572700),
            "k3500_root4": (3500, 581000)}


def canonical_id(record):
    stripped = {key: value for key, value in record.items()
                if key != "candidate_id"}
    digest = hashlib.sha256(json.dumps(
        stripped, sort_keys=True, separators=(",", ":"),
        ensure_ascii=False).encode()).hexdigest()[:12]
    return ("IC1N83Ckb1a1fb"
            f"{record['factor_base']['actual_usable_points']}"
            f"PDP4rootRCguidedLAgaussTDdirectISO0h{digest}")


def derive(name):
    source = json.loads(BASES[name].read_text())
    if source["candidate_id"] != canonical_id(source):
        raise ValueError(f"{name}: original candidate identity differs")
    k, points = EXPECTED[name]
    factor = source["factor_base"]
    if (factor["effective_columns"] != k or
            factor["actual_usable_points"] != points or points != 166 * k):
        raise ValueError(f"{name}: enumerated factor base differs")
    record = copy.deepcopy(source)
    flags = record["implementation"]["algorithm_flags"]
    prior = name in ("k3450_wide3", "k3500_root4")
    expected_source = PRIOR_SOURCE if prior else CURRENT_SOURCE
    if record["implementation"]["source_sha256"]["ic"] != expected_source:
        raise ValueError(f"{name}: source snapshot differs")
    if prior:
        record["implementation"]["source_commit"] = CURRENT_COMMIT
        record["implementation"]["source_sha256"]["ic"] = CURRENT_SOURCE
        for section, key in (("point_decomposition", "solver_source_sha256"),
                             ("relation_collection", "source_digest"),
                             ("relation_linear_algebra", "source_digest"),
                             ("target_descent", "source_digest")):
            if record[section][key] != PRIOR_SOURCE:
                raise ValueError(f"{name}: source component differs")
            record[section][key] = CURRENT_SOURCE
        record["point_decomposition"]["variant"] += "_two_worker_cancel"
        record["target_descent"]["policy"] += "_two_worker_cancel"
        flags["target_threads"] = 2
        flags["target_cancel"] = True
        flags["indexed5_id30_request"] = False
    if (flags["target_threads"] != 2 or flags["target_cancel"] is not True or
            flags["indexed5_id30_request"] is not False):
        raise ValueError(f"{name}: cancellation policy differs")
    record["candidate_id"] = canonical_id(record)
    return record


def main():
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
