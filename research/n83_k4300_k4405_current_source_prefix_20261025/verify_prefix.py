#!/usr/bin/env python3
"""Seal current-source K4300/K4405 candidates from the replayed K5500 dump."""

import argparse
import copy
import hashlib
import json
from pathlib import Path

from blake3 import blake3


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
PARENT_SHA256 = "823782912c75d782cdd7e6c9074c94282e28a686b1dd98ea978a367514d3acdd"
PARENT_BLAKE3 = "0789049953a67b51dfe23f6fa2788d11e0bb4b35d52180fc038a3ffd2c0efb5f"
SOURCE_SHA256 = "58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8"
CURRENT = ROOT / "research/n83_k3596_current_source_full_20261025/candidate_IC1N83Ckb1a1fb596936PDP4rootRCguidedLAgaussTDdirectISO0h2a3353097e81.json"
OLD = {
    4300: ROOT / "research/n83_root4_frontier_current_20261018/highk_20261019/identities/k4300_wide3_identity/candidate_IC1N83Ckb1a1fb713800PDP4rootRCguidedLAgaussTDdirectISO0h394bfa10df77.json",
    4405: ROOT / "research/n83_root4_frontier_current_20261018/highk_20261019/identities/k4405_wide3_identity/candidate_IC1N83Ckb1a1fb731230PDP4rootRCguidedLAgaussTDdirectISO0h2f81ad5da3e7.json",
}


def compact(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def require(actual, expected, label):
    if actual != expected:
        raise ValueError(f"{label}: {actual} != {expected}")


def candidate_id(record):
    stripped = {key: value for key, value in record.items() if key != "candidate_id"}
    digest = sha(compact(stripped))[:12]
    return f"IC1N83Ckb1a1fb{record['factor_base']['actual_usable_points']}PDP4rootRCguidedLAgaussTDdirectISO0h{digest}"


def verify(path):
    raw = path.read_bytes()
    require(sha(raw), PARENT_SHA256, "parent dump SHA256")
    parent = json.loads(raw)
    require(parent["base_hash"], PARENT_BLAKE3, "parent base BLAKE3")
    require(parent["orbit_columns"], 5500, "parent columns")
    require(parent["factor_base_points"], 913000, "parent points")
    coords = parent["factor_base_point_coordinates"]
    labels = parent["factor_base_point_labels"]
    reps = parent["factor_base_representatives"]
    require((len(coords), len(labels), len(reps)), (913000, 913000, 5500), "parent array counts")
    current = json.loads(CURRENT.read_bytes())
    require(candidate_id(current), current["candidate_id"], "current reference identity")
    require(current["implementation"]["source_sha256"]["ic"], SOURCE_SHA256, "current source")
    require(sha((ROOT / "examples/koblitz_orbit_dlp_fast.rs").read_bytes()), SOURCE_SHA256, "executed source")
    output = []
    for k, historical_path in OLD.items():
        historical = json.loads(historical_path.read_bytes())
        require(candidate_id(historical), historical["candidate_id"], "historical candidate identity")
        n = 166 * k
        h = blake3()
        h.update(b"compact-orbit-constructed-base-v1")
        for i, ((x, y), (column, coefficient)) in enumerate(zip(coords[:n], labels[:n])):
            require(column, i // 166, "contiguous orbit column")
            for word, width in ((x, 16), (y, 16), (column, 8), (coefficient, 8)):
                h.update(int(word).to_bytes(width, "little"))
        prefix_hash = h.hexdigest()
        prefix = dict(parent)
        prefix.update(base_hash=prefix_hash,
                      factor_base_point_coordinates=coords[:n],
                      factor_base_point_labels=labels[:n],
                      factor_base_points=n,
                      factor_base_representatives=reps[:k],
                      orbit_columns=k)
        dump_sha = sha(compact(prefix) + b"\n")
        points = [[str(x), str(y)] for x, y in sorted((int(x), int(y)) for x, y in coords[:n])]
        point_sha = sha(compact(points))
        base = historical["factor_base"]
        require((base["actual_usable_points"], base["effective_columns"]), (n, k), "base size")
        require(prefix_hash, base["source_base_hash_blake3"], "prefix BLAKE3")
        require(dump_sha, base["ordered_base_dump_sha256"], "prefix dump SHA256")
        require(point_sha, base["enumerated_set_sha256"], "point set SHA256")
        candidate = copy.deepcopy(historical)
        old_source = historical["implementation"]["source_sha256"]["ic"]
        require(candidate["implementation"]["source_sha256"]["cargo_lock"],
                current["implementation"]["source_sha256"]["cargo_lock"], "Cargo lock")
        candidate["implementation"]["source_commit"] = current["implementation"]["source_commit"]
        candidate["implementation"]["source_sha256"]["ic"] = SOURCE_SHA256
        for section, key in (("point_decomposition", "solver_source_sha256"),
                             ("relation_collection", "source_digest"),
                             ("relation_linear_algebra", "source_digest"),
                             ("target_descent", "source_digest")):
            require(candidate[section][key], old_source, f"historical {section} source")
            candidate[section][key] = SOURCE_SHA256
        candidate["candidate_id"] = candidate_id(candidate)
        result = {"schema": "n83.k4300-k4405-current-source-prefix/v1",
                  "status": "PASS_EXACT_BASE_IDENTITY",
                  "parent_dump_sha256": PARENT_SHA256,
                  "prefix_columns": k, "prefix_points": n,
                  "prefix_base_hash_blake3": prefix_hash,
                  "prefix_ordered_dump_sha256": dump_sha,
                  "prefix_point_set_sha256": point_sha,
                  "historical_candidate_id": historical["candidate_id"],
                  "current_ic_source_sha256": SOURCE_SHA256,
                  "current_candidate_id": candidate["candidate_id"],
                  "candidate_execution_status": "UNMEASURED_CURRENT_SOURCE"}
        output.append((result, candidate))
    return output


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("parent_dump", type=Path)
    parser.add_argument("--check-receipts", action="store_true")
    args = parser.parse_args()
    for result, candidate in verify(args.parent_dump):
        k = result["prefix_columns"]
        for path, value in ((HERE / f"k{k}_prefix_receipt.json", result),
                            (HERE / f"candidate_{candidate['candidate_id']}.json", candidate)):
            encoded = json.dumps(value, sort_keys=True, indent=2) + "\n"
            if args.check_receipts:
                require(path.read_text(), encoded, str(path))
            else:
                path.write_text(encoded)
        print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
