#!/usr/bin/env python3
"""Seal the K3450 current-source root4 candidate from the K3596 base."""

import argparse
import copy
import hashlib
import json
from pathlib import Path

from blake3 import blake3


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
K = 3450
ORBIT = 166
PARENT_SHA256 = "1f3b323d6af8f16dbce51b4d7a3db15a27e68d3f7b72c604cdf147d6071b1473"
OLD = ROOT / "research/n83_online_finalists_20261018/candidate_IC1N83Ckb1a1fb572700PDP4rootRCguidedLAgaussTDdirectISO0hc7264b83bbd1.json"
CURRENT = ROOT / "research/n83_k3596_current_source_full_20261025/candidate_IC1N83Ckb1a1fb596936PDP4rootRCguidedLAgaussTDdirectISO0h2a3353097e81.json"


def compact(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()


def sha(blob):
    return hashlib.sha256(blob).hexdigest()


def same(label, value, expected):
    if value != expected:
        raise ValueError(f"{label}: {value} != {expected}")


def candidate_id(record):
    stripped = {key: value for key, value in record.items() if key != "candidate_id"}
    digest = sha(compact(stripped))[:12]
    return f"IC1N83Ckb1a1fb{166 * K}PDP4rootRCguidedLAgaussTDdirectISO0h{digest}"


def produce(path):
    blob = path.read_bytes()
    same("parent dump", sha(blob), PARENT_SHA256)
    parent = json.loads(blob)
    old = json.loads(OLD.read_bytes())
    current = json.loads(CURRENT.read_bytes())
    same("historical ID", candidate_id(old), old["candidate_id"])
    same("current K3596 ID", sha(compact({k: v for k, v in current.items() if k != "candidate_id"}))[:12], current["candidate_id"].split("h")[-1])
    same("source", sha((ROOT / "examples/koblitz_orbit_dlp_fast.rs").read_bytes()), current["implementation"]["source_sha256"]["ic"])
    same("parent base hash", parent["base_hash"], current["factor_base"]["source_base_hash_blake3"])
    same("parent manifest dump", sha(blob), current["factor_base"]["ordered_base_dump_sha256"])
    same("parent count", parent["factor_base_points"], ORBIT * 3596)
    same("parent columns", parent["orbit_columns"], 3596)
    coords = parent["factor_base_point_coordinates"]
    labels = parent["factor_base_point_labels"]
    reps = parent["factor_base_representatives"]
    same("parent coordinate count", len(coords), ORBIT * 3596)
    same("parent label count", len(labels), len(coords))
    same("parent representative count", len(reps), 3596)
    n = ORBIT * K
    h = blake3()
    h.update(b"compact-orbit-constructed-base-v1")
    for i, ((x, y), (column, coefficient)) in enumerate(zip(coords[:n], labels[:n])):
        same("orbit column", column, i // ORBIT)
        for word, width in ((x, 16), (y, 16), (column, 8), (coefficient, 8)):
            h.update(int(word).to_bytes(width, "little"))
    prefix = dict(parent)
    prefix.update(base_hash=h.hexdigest(), factor_base_point_coordinates=coords[:n],
                  factor_base_point_labels=labels[:n], factor_base_points=n,
                  factor_base_representatives=reps[:K], orbit_columns=K)
    ordered_sha = sha(compact(prefix) + b"\n")
    point_set = [[str(x), str(y)] for x, y in sorted((int(x), int(y)) for x, y in coords[:n])]
    set_sha = sha(compact(point_set))
    fb = old["factor_base"]
    same("point count", fb["actual_usable_points"], n)
    same("column count", fb["effective_columns"], K)
    same("base BLAKE3", h.hexdigest(), fb["source_base_hash_blake3"])
    same("ordered dump", ordered_sha, fb["ordered_base_dump_sha256"])
    same("point set", set_sha, fb["enumerated_set_sha256"])
    old_source = old["implementation"]["source_sha256"]["ic"]
    new_source = current["implementation"]["source_sha256"]["ic"]
    record = copy.deepcopy(old)
    same("cargo lock", record["implementation"]["source_sha256"]["cargo_lock"],
         current["implementation"]["source_sha256"]["cargo_lock"])
    record["implementation"]["source_commit"] = current["implementation"]["source_commit"]
    record["implementation"]["source_sha256"]["ic"] = new_source
    for section, key in (("point_decomposition", "solver_source_sha256"),
                         ("relation_collection", "source_digest"),
                         ("relation_linear_algebra", "source_digest"),
                         ("target_descent", "source_digest")):
        same(f"historical {section} source", record[section][key], old_source)
        record[section][key] = new_source
    record["candidate_id"] = candidate_id(record)
    receipt = {
        "schema": "n83.k3450-current-source-prefix/v1",
        "status": "PASS_EXACT_BASE_IDENTITY",
        "candidate_execution_status": "UNMEASURED_CURRENT_SOURCE",
        "current_candidate_id": record["candidate_id"],
        "historical_candidate_id": old["candidate_id"],
        "parent_dump_sha256": PARENT_SHA256,
        "prefix_columns": K, "prefix_points": n,
        "prefix_base_hash_blake3": h.hexdigest(),
        "prefix_ordered_dump_sha256": ordered_sha,
        "prefix_point_set_sha256": set_sha,
        "historical_ic_source_sha256": old_source,
        "current_ic_source_sha256": new_source,
    }
    return record, receipt


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("parent_dump", type=Path)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    candidate, receipt = produce(args.parent_dump)
    outputs = {
        HERE / f"candidate_{candidate['candidate_id']}.json": candidate,
        HERE / "k3450_prefix_receipt.json": receipt,
    }
    for path, value in outputs.items():
        encoded = json.dumps(value, sort_keys=True, indent=2) + "\n"
        if args.check:
            same(str(path), path.read_text(), encoded)
        else:
            path.write_text(encoded)
    print(json.dumps(receipt, sort_keys=True))


if __name__ == "__main__":
    main()
