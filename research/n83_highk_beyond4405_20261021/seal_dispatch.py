#!/usr/bin/env python3
"""Seal the fixed31 screen and one-target holdout dispatch without allocation."""

from decimal import Decimal
import hashlib
import json
from pathlib import Path
import sys

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))
from research.n83_highk_beyond4405_20261021 import seal_workloads
SOURCE_SHA256 = "58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8"


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def canonical(value: object) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":"),
                      ensure_ascii=False).encode()


def seal() -> dict:
    seal_workloads.seal()
    catalog = json.loads((HERE / "identities/catalog.json").read_text())
    fixture = json.loads((HERE / "fresh_manifest.json").read_text())
    audit = json.loads((HERE / "fresh_exclusion_audit_v2.json").read_text())
    replay = json.loads((HERE / "base_stage_validation.json").read_text())
    if (digest(ROOT / "examples/koblitz_orbit_dlp_fast.rs") != SOURCE_SHA256 or
            catalog["source_sha256"] != SOURCE_SHA256 or
            replay["source_sha256"] != SOURCE_SHA256 or
            replay["status"] != "PASS_INDEPENDENT_BASE_REPLAY" or
            audit["status"] != "PASS_ZERO_OVERLAP" or
            audit["prior_unique_point_count"] != 33602):
        raise ValueError("source, base replay or exclusion gate differs")
    for kind, count in (("screen", 1536), ("holdout", 16)):
        if (fixture[kind]["point_count"] != count or
                fixture[kind]["targets_sha256"] !=
                digest(HERE / f"{kind}_targets.jsonl") or
                fixture[kind]["answers_sha256"] !=
                digest(HERE / f"{kind}_answers.jsonl") or
                audit[kind]["targets_sha256"] !=
                fixture[kind]["targets_sha256"] or
                audit[kind]["prior_overlap_count"] != 0 or
                audit[kind]["other_fresh_overlap_count"] != 0):
            raise ValueError(f"{kind} fixture differs")
    arms = []
    for entry in catalog["candidates"]:
        candidate_id = entry["candidate_id"]
        path = HERE / "identities" / f"candidate_{candidate_id}.json"
        record = json.loads(path.read_text())
        body = {key: value for key, value in record.items()
                if key != "candidate_id"}
        if (digest(path) != entry["manifest_sha256"] or
                candidate_id != record["candidate_id"] or
                not candidate_id.endswith("h" +
                    hashlib.sha256(canonical(body)).hexdigest()[:12]) or
                record["implementation"]["source_sha256"]["ic"] != SOURCE_SHA256 or
                record["factor_base"]["source_base_hash_blake3"] !=
                replay["arms"][f"k{entry['k']}"]["base_hash_blake3"]):
            raise ValueError(f"candidate {candidate_id} differs")
        arms.append({"candidate_id": candidate_id,
                     "candidate_manifest_sha256": digest(path),
                     "k": entry["k"],
                     "usable_points": record["factor_base"]["actual_usable_points"],
                     "fixed31": entry["fixed31"],
                     "base_hash_blake3": entry["base_hash"]})
    if [(row["k"], row["fixed31"]) for row in arms] != [
            (4405, False), (4405, True), (4500, True), (4549, True)]:
        raise ValueError("candidate order differs")
    hours = Decimal("10")
    rate = Decimal("0.20")
    reserve = Decimal("0.25")
    previous = Decimal("5.710216")
    incremental = hours * rate + reserve
    dispatch = {
        "schema": "n83.fixed31-tier1-dispatch/v1",
        "status": "AWAITING_OWNER_BUDGET_APPROVAL",
        "curve_id": fixture["curve_id"],
        "source_sha256": SOURCE_SHA256,
        "cargo_lock_sha256": digest(ROOT / "Cargo.lock"),
        "candidate_catalog_sha256": digest(HERE / "identities/catalog.json"),
        "base_replay_sha256": digest(HERE / "base_stage_validation.json"),
        "fresh_manifest_sha256": digest(HERE / "fresh_manifest.json"),
        "exclusion_audit_sha256": digest(HERE / "fresh_exclusion_audit_v2.json"),
        "screen_targets_sha256": fixture["screen"]["targets_sha256"],
        "holdout_targets_sha256": fixture["holdout"]["targets_sha256"],
        "workload_manifest_sha256": digest(HERE / "workloads.json"),
        "screen_target_count": 1536,
        "holdout_target_count": 16,
        "primary_holdout_index": 0,
        "arms": arms,
        "screen_blocks": [
            {"indices": [0, 767], "candidate_ids": [a["candidate_id"] for a in arms]},
            {"indices": [768, 1535],
             "candidate_ids": [a["candidate_id"] for a in reversed(arms)]},
        ],
        "ic_child_cap_seconds": 2400,
        "rho_calibration_repetitions_per_side": 3,
        "rho_aa_max_median_spread_fraction": "0.20",
        "tier": 1,
        "minimum_visible_cpus": 4,
        "minimum_cpu_quota_cores": 4,
        "minimum_memory_bytes": 32 * 1024**3,
        "child_rss_cap_bytes": 26 * 1024**3,
        "max_allocation_seconds": int(hours * 3600),
        "max_quoted_usd_per_hour": str(rate),
        "billing_reserve_usd": str(reserve),
        "incremental_upper_usd": str(incremental),
        "previous_campaign_upper_usd": str(previous),
        "combined_upper_usd": str(previous + incremental),
    }
    path = HERE / "dispatch.json"
    blob = (json.dumps(dispatch, indent=2, sort_keys=True) + "\n").encode()
    if path.exists() and path.read_bytes() != blob:
        raise ValueError("frozen dispatch differs")
    path.write_bytes(blob)
    return dispatch


if __name__ == "__main__":
    result = seal()
    print(json.dumps({"status": result["status"],
                      "candidate_ids": [a["candidate_id"] for a in result["arms"]],
                      "incremental_upper_usd": result["incremental_upper_usd"]},
                     sort_keys=True))
