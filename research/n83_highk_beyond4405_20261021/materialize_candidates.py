#!/usr/bin/env python3
"""Bind exact fixed31 N83 bases and source to immutable IC1 manifests."""

import argparse
import copy
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
TEMPLATE = (ROOT / "research/n83_root4_frontier_current_20261018/highk_20261019"
            / "identities/k4405_wide3_identity"
            / "candidate_IC1N83Ckb1a1fb731230PDP4rootRCguidedLAgaussTDdirectISO0h2f81ad5da3e7.json")
SOURCE_SHA256 = "58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8"
SOURCE_COMMIT = None
OLD_K4405_BASE_HASH = "d64fcee7ba4975873859d256302872681c82b99fbe687ff4fba0bf3ba3a4c8d2"
SLOTS = 1 << 31


def canonical(value: object) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":"),
                      ensure_ascii=False).encode()


def digest(value: object) -> str:
    return hashlib.sha256(canonical(value)).hexdigest()


def freeze(path: Path, value: object) -> None:
    blob = (json.dumps(value, indent=2, sort_keys=True,
                       ensure_ascii=False) + "\n").encode()
    if path.exists() and path.read_bytes() != blob:
        raise ValueError(f"candidate identity differs: {path}")
    path.write_bytes(blob)


def base_identity(k: int, stage: Path, original: dict) -> dict:
    receipt = json.loads((stage / f"k{k}" / "receipt.json").read_text())
    summary = receipt["summary"]
    if (receipt["status"] != "COMPLETE" or
            receipt["source_sha256"] != SOURCE_SHA256 or
            summary["orbit_columns"] != k or
            summary["factor_base_points"] != 166 * k):
        raise ValueError(f"K{k} base receipt differs")
    if k == 4405 and summary["base_hash"] != OLD_K4405_BASE_HASH:
        raise ValueError("K4405 prefix does not match archived base")
    dump_path = stage / f"k{k}" / "base_dump.jsonl"
    if hashlib.sha256(dump_path.read_bytes()).hexdigest() != receipt["base_dump_sha256"]:
        raise ValueError(f"K{k} base dump differs")
    dump = json.loads(dump_path.read_text())
    points = dump["factor_base_point_coordinates"]
    if (len(points) != 166 * k or
            len(dump["factor_base_representatives"]) != k or
            dump["base_hash"] != summary["base_hash"]):
        raise ValueError(f"K{k} base dump shape differs")
    ordered = sorted(points, key=lambda pair: (int(pair[0]), int(pair[1])))
    base = copy.deepcopy(original["factor_base"])
    base.update({
        "actual_usable_points": 166 * k,
        "effective_columns": k,
        "enumerated_set_sha256": digest(ordered),
        "ordered_base_dump_sha256": receipt["base_dump_sha256"],
        "source_base_hash_blake3": summary["base_hash"],
    })
    return base


def materialize(stage: Path) -> list[dict]:
    source = ROOT / "examples/koblitz_orbit_dlp_fast.rs"
    if hashlib.sha256(source.read_bytes()).hexdigest() != SOURCE_SHA256:
        raise ValueError("executed IC source differs")
    original = json.loads(TEMPLATE.read_text())
    out = HERE / "identities"
    out.mkdir(exist_ok=True)
    records = []
    for k, fixed31 in ((4405, False), (4405, True),
                       (4500, True), (4549, True)):
        record = copy.deepcopy(original)
        record["factor_base"] = base_identity(k, stage, original)
        implementation = record["implementation"]
        implementation["source_commit"] = SOURCE_COMMIT
        implementation["source_sha256"]["ic"] = SOURCE_SHA256
        flags = implementation["algorithm_flags"]
        flags["indexed5_fixed31_request"] = fixed31
        flags["indexed5_load75_request"] = not fixed31
        flags["indexed5_load80_request"] = False
        flags["indexed5_id31_request"] = False
        flags["indexed5_split_at"] = None
        flags["indexed5_split_shared_bloom"] = False
        pdp = record["point_decomposition"]
        pdp["solver_source_sha256"] = SOURCE_SHA256
        if fixed31:
            pdp["variant"] = pdp["variant"].replace(
                "dynamic_load75", "fixed31_upper80")
            pdp["root_slot"].update({
                "allocated_bytes": 5 * SLOTS,
                "request_entries": 1 << 30,
                "request_rule": "fixed_2^30_request_yields_2^31_slots_upper_load_at_most_80_percent",
            })
        record["relation_collection"]["stop_criterion"] = f"rank_{k}"
        record["relation_collection"]["source_digest"] = SOURCE_SHA256
        record["relation_linear_algebra"]["columns"] = k
        record["relation_linear_algebra"]["rank_criterion"] = k
        record["relation_linear_algebra"]["source_digest"] = SOURCE_SHA256
        record["target_descent"]["source_digest"] = SOURCE_SHA256
        body = {key: value for key, value in record.items()
                if key != "candidate_id"}
        candidate_id = (f"IC1N83Ckb1a1fb{166 * k}PDP4rootRCguidedLAgaussTDdirectISO0h"
                        + digest(body)[:12])
        record["candidate_id"] = candidate_id
        path = out / f"candidate_{candidate_id}.json"
        freeze(path, record)
        records.append({"k": k, "fixed31": fixed31,
                        "candidate_id": candidate_id,
                        "manifest_sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
                        "base_hash": record["factor_base"]["source_base_hash_blake3"]})
    freeze(out / "catalog.json", {"schema": "n83.fixed31-catalog/v1",
                                  "source_sha256": SOURCE_SHA256,
                                  "base_stage": "local_correctness_diagnostic",
                                  "candidates": records})
    return records


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--base-stage", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(materialize(args.base_stage.resolve()), sort_keys=True))
