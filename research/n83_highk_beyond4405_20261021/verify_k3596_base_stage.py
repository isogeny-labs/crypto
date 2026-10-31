#!/usr/bin/env python3
"""Replay the current-source K3596 base and its historical ordered identity."""

import argparse
import hashlib
import json
from pathlib import Path
import tarfile

from research.n83_highk_beyond4405_20261021 import run_k1107_base_stage as runner
from research.n83_indexed5_20261013 import analyze as group

ROOT = runner.ROOT
HISTORICAL = ROOT / "research/n83_root4_frontier_current_20261018"
ARCHIVE = HISTORICAL / "pilots/k3596.tar.gz"
MANIFEST = HISTORICAL / ("candidate_IC1N83Ckb1a1fb596936PDP4rootRCguided"
                         "LAgaussTDdirectISO0h3da4e8b31c3f.json")
K = 3596


def sha256_bytes(blob: bytes) -> str:
    return hashlib.sha256(blob).hexdigest()


def verify(stage: Path) -> dict:
    receipt = json.loads((stage / "receipt.json").read_text())
    summary = json.loads((stage / "stdout.jsonl").read_text())
    current = (stage / "base_dump.jsonl").read_bytes()
    dump = json.loads(current)
    manifest = json.loads(MANIFEST.read_text())
    previous = manifest["factor_base"]
    with tarfile.open(ARCHIVE, "r:gz") as bundle:
        member = bundle.extractfile("pilot_work/base_dump.jsonl")
        if member is None:
            raise ValueError("historical K3596 base dump absent")
        historical = member.read()
    source_binary = json.loads((runner.HERE / "source_binary_custody.json").read_text())
    expected_flags = {"KIC_BASE_BUILD_ONLY": "1",
                      "KIC_DUMP_BASE": str(stage / "base_dump.jsonl")}
    command = receipt.get("command", [])
    if (runner.sha256(runner.SOURCE) != runner.SOURCE_SHA256 or
            receipt.get("status") != "PROCESS_EXITED" or
            receipt.get("exit_code") != 0 or
            receipt.get("cap_seconds") != runner.CAP_SECONDS or
            receipt.get("source_sha256") != runner.SOURCE_SHA256 or
            receipt.get("binary_sha256") != source_binary["binary_sha256"] or
            source_binary.get("status") != "MATCHED_LOCAL_AND_FRESH_S3_DOWNLOAD" or
            len(command) != 5 or
            command[1:4] != [f"construct:83:1:{K}", str(runner.TARGET), "29"] or
            receipt.get("environment_flags") != expected_flags or
            receipt.get("target_sha256") != runner.sha256(runner.TARGET) or
            receipt.get("stdout_sha256") != runner.sha256(stage / "stdout.jsonl") or
            receipt.get("base_dump_sha256") != sha256_bytes(current) or
            receipt.get("summary") != summary or
            summary.get("kind") != "compact_orbit_base_stage" or
            summary.get("n") != group.N or summary.get("a") != 1 or
            summary.get("subgroup_order") != group.R or
            group.point_from_json(summary["generator"]) != group.G or
            summary.get("orbit_columns") != K or
            summary.get("factor_base_points") != 166 * K or
            summary.get("base_hash") != previous["source_base_hash_blake3"] or
            dump.get("base_hash") != previous["source_base_hash_blake3"] or
            sorted(summary["field_modulus_low_terms"]) !=
            [i for i in range(group.N) if group.MOD & (1 << i)]):
        raise ValueError("K3596 source, receipt, field or subgroup differs")
    if (current != historical or
            sha256_bytes(current) != previous["ordered_base_dump_sha256"]):
        raise ValueError("current K3596 dump differs from historical ordered base")
    points = group.replay_base(dump, summary)
    if len(points) != previous["actual_usable_points"]:
        raise ValueError("K3596 usable point count differs")
    ordered = [[str(x), str(y)] for x, y in sorted(points)]
    canonical = json.dumps(ordered, sort_keys=True, separators=(",", ":"),
                           ensure_ascii=False).encode()
    set_sha = sha256_bytes(canonical)
    if set_sha != previous["enumerated_set_sha256"]:
        raise ValueError("K3596 independently replayed point set differs")
    return {
        "schema": "n83.fixed31-k3596-base-replay/v1",
        "status": "PASS_INDEPENDENT_BASE_REPLAY",
        "source_sha256": runner.SOURCE_SHA256,
        "binary_sha256": receipt["binary_sha256"],
        "usable_points": len(points),
        "folded_columns": K,
        "base_hash_blake3": summary["base_hash"],
        "base_dump_sha256": sha256_bytes(current),
        "enumerated_set_sha256": set_sha,
        "historical_archive_sha256": runner.sha256(ARCHIVE),
        "historical_candidate_id": manifest["candidate_id"],
        "ordered_historical_identity": True,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--base-stage", type=Path, required=True)
    args = parser.parse_args()
    stage = args.base_stage.resolve(strict=True)
    result = verify(stage)
    output = stage / "independent_replay.json"
    blob = json.dumps(result, indent=2, sort_keys=True) + "\n"
    if output.exists() and output.read_text() != blob:
        raise ValueError("K3596 independent replay differs")
    output.write_text(blob)
    print(json.dumps({"status": result["status"],
                      "usable_points": result["usable_points"],
                      "ordered_historical_identity": True}, sort_keys=True))


if __name__ == "__main__":
    main()
