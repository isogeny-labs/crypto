#!/usr/bin/env python3
"""Derive the source-bound K1107 finalist identity for the fixed31 campaign."""

import copy
import hashlib
import json
from pathlib import Path
import subprocess

from research.n83_highk_beyond4405_20261021 import run_k1107_base_stage as base
from research.n83_online_finalists_20261018.generate_candidates import canonical_id

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
OLD_SOURCE = "4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507"
SOURCE_COMMIT = "3ce5b531cacb1363c6bf1c8a2c4277bf534046d1"
OLD_MANIFEST = ROOT / ("research/n83_online_finalists_20261018/"
                       "candidate_IC1N83Ckb1a1fb183762PDP4rootRCguided"
                       "LAgaussTDdirectISO0he423fcbc5668.json")


def derive() -> dict:
    prior = json.loads(OLD_MANIFEST.read_text())
    committed_source = subprocess.check_output(
        ["git", "show", f"{SOURCE_COMMIT}:examples/koblitz_orbit_dlp_fast.rs"],
        cwd=ROOT)
    if (prior["candidate_id"] != canonical_id(prior) or
            prior["factor_base"]["actual_usable_points"] != 183762 or
            prior["factor_base"]["effective_columns"] != 1107 or
            prior["factor_base"]["source_base_hash_blake3"] !=
            base.PRIOR_BASE_HASH or
            prior["implementation"]["source_sha256"]["ic"] != OLD_SOURCE or
            hashlib.sha256(committed_source).hexdigest() != base.SOURCE_SHA256 or
            base.sha256(base.SOURCE) != base.SOURCE_SHA256 or
            base.sha256(ROOT / "Cargo.lock") !=
            prior["implementation"]["source_sha256"]["cargo_lock"]):
        raise ValueError("old K1107 candidate or current source differs")
    record = copy.deepcopy(prior)
    record["implementation"]["source_commit"] = SOURCE_COMMIT
    record["implementation"]["source_sha256"]["ic"] = base.SOURCE_SHA256
    for section, key in (("point_decomposition", "solver_source_sha256"),
                         ("relation_collection", "source_digest"),
                         ("relation_linear_algebra", "source_digest"),
                         ("target_descent", "source_digest")):
        if record[section][key] != OLD_SOURCE:
            raise ValueError(f"{section} source differs")
        record[section][key] = base.SOURCE_SHA256
    record["candidate_id"] = canonical_id(record)
    return record


def main() -> None:
    record = derive()
    output = HERE / "identities" / f"candidate_{record['candidate_id']}.json"
    blob = json.dumps(record, indent=2, sort_keys=True) + "\n"
    if output.exists() and output.read_text() != blob:
        raise ValueError("sealed current K1107 identity differs")
    output.write_text(blob)
    print(json.dumps({"candidate_id": record["candidate_id"],
                      "manifest_sha256": hashlib.sha256(blob.encode()).hexdigest()},
                     sort_keys=True))


if __name__ == "__main__":
    main()
