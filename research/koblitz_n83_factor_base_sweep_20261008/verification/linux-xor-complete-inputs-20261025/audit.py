#!/usr/bin/env python3
"""Replay the exact S3 inputs needed by the primary-N83 external verifier."""

import argparse
import base64
import hashlib
import json
from pathlib import Path

import blake3


HERE = Path(__file__).resolve().parent
VERIFICATION = HERE.parent
ADMISSION = VERIFICATION / "linux-xor-panel-20261025/admission.json"
HEURISTIC = VERIFICATION / "external-xor-heuristic-panel-20261025/protocol.json"
SOURCE_RECEIPT = VERIFICATION / "external-xor-20261025/receipt.json"
SOURCE_COMMIT = "fd488218620e4dbacc42a7e6ea5812dd3e52a0f6"


def require(condition: bool, reason: str) -> None:
    if not condition:
        raise ValueError(reason)


def digest(path: Path, algorithm: str) -> str:
    hasher = blake3.blake3() if algorithm == "blake3" else hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            hasher.update(chunk)
    return hasher.hexdigest()


def audit(downloads: Path) -> dict:
    manifest_path = HERE / "manifest.json"
    manifest = json.loads(manifest_path.read_bytes())
    admission = json.loads(ADMISSION.read_bytes())
    heuristic = json.loads(HEURISTIC.read_bytes())
    source = json.loads(SOURCE_RECEIPT.read_bytes())
    require(manifest["schema"] == "n83.linux-xor-complete-inputs/v1"
            and manifest["source_commit"] == SOURCE_COMMIT
            and manifest["curve"] == admission["curve"] == source["curve_slug"]
            and manifest["bucket"] == admission["s3"]["bucket"]
            and manifest["selected_best_total_runtime"] is None,
            "manifest identity")
    prefix = admission["s3"]["prefix"] + "inputs/"
    entries = manifest["entries"]
    require(len(entries) == 11 and manifest["existing_xcnf_count"] == 4
            and manifest["new_asset_count"] == 7
            and len({entry["key"] for entry in entries}) == 11,
            "complete unique input set")
    expected_archives = {
        "base": (heuristic["base_object_blake3"], heuristic["base_object_sha256"]),
        **{policy: (heuristic["inputs"][f"{policy}_5_1"]["pool_object_blake3"],
                    heuristic["inputs"][f"{policy}_5_1"]["pool_object_sha256"])
           for policy in ("gray", "hash")},
    }
    by_kind = {kind: [entry for entry in entries if entry["kind"] == kind]
               for kind in ("xcnf", "archive", "export_metadata")}
    require(len(by_kind["xcnf"]) == 4 and len(by_kind["archive"]) == 3
            and len(by_kind["export_metadata"]) == 4,
            "input kinds")
    require({entry["name"] for entry in by_kind["xcnf"]}
            == set(admission["s3"]["inputs"]), "XCNF arms")
    require({entry["role"] for entry in by_kind["archive"]}
            == set(expected_archives), "archive roles")
    require({entry["name"] for entry in by_kind["export_metadata"]}
            == set(admission["s3"]["inputs"]), "metadata arms")
    source_arms = {arm["name"]: arm for arm in source["arms"]}
    checked = []
    for entry in entries:
        key = entry["key"]
        require(key.startswith(prefix) and ".." not in Path(key).parts,
                "S3 input prefix")
        relative = key[len(prefix):]
        path = downloads / relative
        require(path.is_file() and path.stat().st_size == entry["bytes"]
                and digest(path, "sha256") == entry["sha256"],
                f"{relative}: compressed/raw SHA-256 and length")
        if entry["kind"] == "xcnf":
            name = entry["name"]
            require(relative == f"{name}.xcnf"
                    and entry["sha256"] == admission["s3"]["inputs"][name]["sha256"]
                    == source_arms[name]["xcnf_sha256"],
                    f"{name}: original XCNF identity")
        elif entry["kind"] == "archive":
            role = entry["role"]
            b3, sha = expected_archives[role]
            require(relative == f"archives/s3-{b3}.jsonl.gz"
                    and entry["compressed_blake3"] == b3
                    and entry["sha256"] == sha
                    and digest(path, "blake3") == b3,
                    f"{role}: original archive identity")
        else:
            name = entry["name"]
            metadata = json.loads(path.read_bytes())
            policy, base_count, residual_count = name.split("_")
            require(relative == f"exports/{name}/export.json"
                    and metadata["schema"] == "n83.cross-stream-first-pattern-xcnf/v1"
                    and metadata["curve_slug"] == manifest["curve"]
                    and metadata["base_object_blake3"] == expected_archives["base"][0]
                    and metadata["pool_object_blake3"] == expected_archives[policy][0]
                    and [metadata["base_summands"], metadata["residual_summands"]]
                    == [int(base_count), int(residual_count)]
                    and metadata["xcnf_bytes"] == admission["s3"]["inputs"][name]["bytes"]
                    and metadata["xcnf_blake3"]
                    == digest(downloads / f"{name}.xcnf", "blake3")
                    and metadata["source_files_blake3"] == source["source_files_blake3"]
                    and metadata["identity_mask"] == 0
                    and metadata["new_n83_solver_search_executed"] is False
                    and metadata["selected_best_total_runtime"] is None,
                    f"{name}: exact export metadata")
        checked.append({"key": key, "bytes": entry["bytes"], "sha256": entry["sha256"]})
    receipt_path = HERE / "receipt.json"
    if receipt_path.exists():
        receipt = json.loads(receipt_path.read_bytes())
        require(receipt["schema"] == "n83.linux-xor-complete-input-publish/v1"
                and receipt["manifest_sha256"] == digest(manifest_path, "sha256")
                and receipt["status"] == "PASS_FRESH_S3_GET_REPLAY"
                and {row["key"] for row in receipt["objects"]}
                == {entry["key"] for entry in entries},
                "S3 receipt binding")
        for row in receipt["objects"]:
            matching = next(entry for entry in entries if entry["key"] == row["key"])
            require(row["bytes"] == matching["bytes"]
                    and row["checksum_sha256_base64"]
                    == base64.b64encode(bytes.fromhex(matching["sha256"])).decode()
                    and row["server_side_encryption"] == "AES256",
                    "S3 HEAD checksum and encryption")
    return {"schema": "n83.linux-xor-complete-input-replay/v1",
            "status": "PASS_COMPLETE_INPUT_REPLAY", "curve": manifest["curve"],
            "objects": checked, "selected_best_total_runtime": None}


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--downloads-dir", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(audit(args.downloads_dir), indent=2, sort_keys=True))
