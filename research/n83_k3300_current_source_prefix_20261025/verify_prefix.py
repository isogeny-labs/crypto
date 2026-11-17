#!/usr/bin/env python3
"""Replay the K3300 prefix of the current-source K3596 base dump."""

import argparse
import copy
import hashlib
import json
from pathlib import Path

from blake3 import blake3


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
HISTORICAL = ROOT / "research/n83_final_extended_20261018/candidate_IC1N83Ckb1a1fb547800PDP4rootRCguidedLAgaussTDdirectISO0h8d1e886f8586.json"
CURRENT = ROOT / "research/n83_k3596_current_source_full_20261025/candidate_IC1N83Ckb1a1fb596936PDP4rootRCguidedLAgaussTDdirectISO0h2a3353097e81.json"
PARENT_SHA256 = "1f3b323d6af8f16dbce51b4d7a3db15a27e68d3f7b72c604cdf147d6071b1473"
K = 3300
ORBIT = 166


def compact(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()


def sha(value):
    return hashlib.sha256(value).hexdigest()


def check_equal(label, actual, expected):
    if actual != expected:
        raise ValueError(f"{label}: {actual} differs from {expected}")


def candidate_id(record):
    stripped = {key: value for key, value in record.items() if key != "candidate_id"}
    digest = sha(compact(stripped))[:12]
    return f"IC1N83Ckb1a1fb{record['factor_base']['actual_usable_points']}PDP4rootRCguidedLAgaussTDdirectISO0h{digest}"


def current_candidate(historical, current):
    check_equal("historical candidate ID", candidate_id(historical), historical["candidate_id"])
    check_equal("current candidate ID", candidate_id(current), current["candidate_id"])
    old_source = historical["implementation"]["source_sha256"]["ic"]
    new_source = current["implementation"]["source_sha256"]["ic"]
    check_equal("current IC source", sha((ROOT / "examples/koblitz_orbit_dlp_fast.rs").read_bytes()), new_source)
    check_equal("Cargo lock", historical["implementation"]["source_sha256"]["cargo_lock"],
                current["implementation"]["source_sha256"]["cargo_lock"])
    record = copy.deepcopy(historical)
    record["implementation"]["source_commit"] = current["implementation"]["source_commit"]
    record["implementation"]["source_sha256"]["ic"] = new_source
    for section, key in (("point_decomposition", "solver_source_sha256"),
                         ("relation_collection", "source_digest"),
                         ("relation_linear_algebra", "source_digest"),
                         ("target_descent", "source_digest")):
        check_equal(f"historical {section} source", record[section][key], old_source)
        record[section][key] = new_source
    record["candidate_id"] = candidate_id(record)
    return record


def verify(path):
    raw = path.read_bytes()
    check_equal("parent SHA256", sha(raw), PARENT_SHA256)
    parent = json.loads(raw)
    historical = json.loads(HISTORICAL.read_bytes())
    current = json.loads(CURRENT.read_bytes())
    coords = parent["factor_base_point_coordinates"]
    labels = parent["factor_base_point_labels"]
    reps = parent["factor_base_representatives"]
    check_equal("parent columns", parent["orbit_columns"], 3596)
    check_equal("parent points", parent["factor_base_points"], ORBIT * 3596)
    check_equal("coordinate count", len(coords), ORBIT * 3596)
    check_equal("label count", len(labels), len(coords))
    check_equal("representative count", len(reps), 3596)
    check_equal("parent BLAKE3", parent["base_hash"], current["factor_base"]["source_base_hash_blake3"])
    check_equal("parent manifest dump", sha(raw), current["factor_base"]["ordered_base_dump_sha256"])

    n = ORBIT * K
    h = blake3()
    h.update(b"compact-orbit-constructed-base-v1")
    for i, ((x, y), (column, coefficient)) in enumerate(zip(coords[:n], labels[:n])):
        check_equal("contiguous orbit column", column, i // ORBIT)
        for word, width in ((x, 16), (y, 16), (column, 8), (coefficient, 8)):
            h.update(int(word).to_bytes(width, "little"))
    prefix_hash = h.hexdigest()
    prefix = dict(parent)
    prefix.update(base_hash=prefix_hash,
                  factor_base_point_coordinates=coords[:n],
                  factor_base_point_labels=labels[:n],
                  factor_base_points=n,
                  factor_base_representatives=reps[:K],
                  orbit_columns=K)
    dump_sha = sha(compact(prefix) + b"\n")
    point_set = [[str(x), str(y)] for x, y in sorted((int(x), int(y)) for x, y in coords[:n])]
    set_sha = sha(compact(point_set))
    base = historical["factor_base"]
    check_equal("prefix columns", base["effective_columns"], K)
    check_equal("prefix points", base["actual_usable_points"], n)
    check_equal("prefix BLAKE3", prefix_hash, base["source_base_hash_blake3"])
    check_equal("prefix ordered dump", dump_sha, base["ordered_base_dump_sha256"])
    check_equal("prefix point set", set_sha, base["enumerated_set_sha256"])
    candidate = current_candidate(historical, current)
    result = {
        "schema": "n83.k3300-current-source-prefix/v1",
        "status": "PASS_EXACT_BASE_IDENTITY",
        "parent_dump_sha256": PARENT_SHA256,
        "prefix_columns": K,
        "prefix_points": n,
        "prefix_base_hash_blake3": prefix_hash,
        "prefix_ordered_dump_sha256": dump_sha,
        "prefix_point_set_sha256": set_sha,
        "historical_candidate_id": historical["candidate_id"],
        "historical_ic_source_sha256": historical["implementation"]["source_sha256"]["ic"],
        "current_ic_source_sha256": current["implementation"]["source_sha256"]["ic"],
        "current_candidate_id": candidate["candidate_id"],
        "candidate_execution_status": "UNMEASURED_CURRENT_SOURCE",
    }
    return result, candidate


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("parent_dump", type=Path)
    parser.add_argument("--check-receipt", action="store_true")
    args = parser.parse_args()
    result, candidate = verify(args.parent_dump)
    receipt = HERE / "prefix_receipt.json"
    candidate_path = HERE / f"candidate_{candidate['candidate_id']}.json"
    encoded = json.dumps(result, sort_keys=True, indent=2) + "\n"
    candidate_encoded = json.dumps(candidate, sort_keys=True, indent=2) + "\n"
    if args.check_receipt:
        check_equal("receipt", receipt.read_text(), encoded)
        check_equal("candidate manifest", candidate_path.read_text(), candidate_encoded)
    else:
        receipt.write_text(encoded)
        candidate_path.write_text(candidate_encoded)
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
