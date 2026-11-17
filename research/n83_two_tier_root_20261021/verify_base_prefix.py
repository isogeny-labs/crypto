"""Check an ordered N83 factor-base prefix against versioned base dumps."""

import argparse
import hashlib
import json
from pathlib import Path
import subprocess

if __package__:
    from .verify_base_stage import load_dump
else:
    from verify_base_stage import load_dump


REPO = Path(__file__).resolve().parents[2]
K4405_MANIFEST = (
    "research/n83_root4_frontier_current_20261018/highk_20261019/"
    "identities/k4405_wide3_identity/"
    "candidate_IC1N83Ckb1a1fb731230PDP4rootRCguidedLAgaussTDdirectISO0h2f81ad5da3e7.json"
)


def replay(small, large):
    small_verification = json.loads((small / "verification.json").read_text())
    large_verification = json.loads((large / "verification.json").read_text())
    if (small_verification["status"] != "PASS_INDEPENDENT_BASE_REPLAY" or
            large_verification["status"] != "PASS_INDEPENDENT_BASE_REPLAY"):
        raise ValueError("both bases need independent group replay")
    a, a_sha = load_dump(small)
    b, b_sha = load_dump(large)
    if (a_sha != small_verification["base_dump_sha256"] or
            b_sha != large_verification["base_dump_sha256"]):
        raise ValueError("ordered base dump digest differs from replay")
    k_small = small_verification["orbit_columns"]
    k_large = large_verification["orbit_columns"]
    if not 0 < k_small < k_large:
        raise ValueError("base sizes are not ordered")
    checks = {field: b[field][:count] == a[field] for field, count in (
        ("factor_base_point_coordinates", 166 * k_small),
        ("factor_base_point_labels", 166 * k_small),
        ("factor_base_representatives", k_small),
    )}
    manifest_path = None
    manifest_sha256 = None
    if k_small == 4405:
        manifest_path = K4405_MANIFEST
        manifest_bytes = subprocess.check_output(
            ["git", "show", "HEAD:" + manifest_path], cwd=REPO)
        manifest_sha256 = hashlib.sha256(manifest_bytes).hexdigest()
        manifest = json.loads(manifest_bytes)
        checks["frozen_k4405_manifest_hash"] = (
            a["base_hash"] == manifest["factor_base"]["source_base_hash_blake3"])
        checks["frozen_k4405_manifest_count"] = (
            a["factor_base_points"] == manifest["factor_base"]["actual_usable_points"])
    return {"schema": "n83.base-prefix-replay/v1",
            "status": "PASS_EXACT_PREFIX" if all(checks.values()) else "MISMATCH",
            "checks": checks, "small_k": k_small, "large_k": k_large,
            "small_base_hash": a["base_hash"], "large_base_hash": b["base_hash"],
            "small_base_dump_sha256": a_sha, "large_base_dump_sha256": b_sha,
            "frozen_manifest_path": manifest_path,
            "frozen_manifest_sha256": manifest_sha256}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--small", type=Path, required=True)
    parser.add_argument("--large", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = replay(args.small.resolve(strict=True), args.large.resolve(strict=True))
    args.output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps(result, sort_keys=True))
    return 0 if result["status"] == "PASS_EXACT_PREFIX" else 1


if __name__ == "__main__":
    raise SystemExit(main())
