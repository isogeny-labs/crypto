#!/usr/bin/env python3
"""Verify the corrected K4096 capacity panel before a second Docker launch."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import subprocess

import blake3


HERE = Path(__file__).resolve().parent
CHECKOUT = HERE.parents[3]
OBJECT_BLAKE3 = "a84572e6ca52efa7867082261bf0543723d52c0d136061a45c970970716aa779"
OBJECT_SHA256 = "ccc1196d75de406b1daba634b8011f08308c37bd1e674758f9efeabb0ff0bcad"
MANIFEST_SHA256 = "4557a25e4eed961025da19caab6175e601fa54d07193744384385837579bd961"
BINARY_SHA256 = "21817b5218b59b5227de5c705e44710583ef70bc8863a5c544d93853a98b5eed"
S3_URI = ("s3://crypto-autoresearcher/factor-bases/icv1/etc/"
          "koblitz_n83_factor_base_sweep_20261008/v2-size-frontier/a0/objects/"
          + OBJECT_BLAKE3 + ".jsonl.gz")
PRIOR_CONFIG = ("research/koblitz_n83_factor_base_sweep_20261008/verification/"
                "k2048-natural-highcap-20261018/evidence/run-01/config.json")


def require(condition: bool, reason: str) -> None:
    if not condition:
        raise ValueError(reason)


def sha(path: Path) -> str:
    state = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            state.update(chunk)
    return state.hexdigest()


def b3(path: Path) -> str:
    state = blake3.blake3()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            state.update(chunk)
    return state.hexdigest()


def check(panel: Path, fixtures: Path, binary: Path, head: Path,
          get: Path) -> dict:
    manifest_path = panel / "manifest.json"
    manifest = json.loads(manifest_path.read_text())
    require(sha(manifest_path) == MANIFEST_SHA256
            and manifest["schema"] == "n83.factor-base-panel/v2-size-frontier"
            and len(manifest["bases"]) == 1
            and manifest["bases"][0]["columns"] == 4096
            and manifest["bases"][0]["policy"] == "public_x_hash"
            and manifest["bases"][0]["seed"] == 2026100801
            and manifest["bases"][0]["compressed_blake3"] == OBJECT_BLAKE3
            and manifest["bases"][0]["s3_uri"] == S3_URI
            and manifest["selected_best_total_runtime"] is None,
            "K4096 manifest differs")
    object_path = panel / manifest["bases"][0]["object"]
    s3_head, s3_get = json.loads(head.read_text()), json.loads(get.read_text())
    require(object_path.stat().st_size == 20621466
            and sha(object_path) == OBJECT_SHA256
            and b3(object_path) == OBJECT_BLAKE3
            and s3_head["ContentLength"] == s3_get["ContentLength"] == 20621466
            and s3_head["ETag"] == s3_get["ETag"]
            and s3_head["Metadata"]["blake3"] == OBJECT_BLAKE3
            and s3_head["ServerSideEncryption"] == "AES256",
            "fresh K4096 S3 object or metadata differs")
    replay = json.loads((panel / "replay.json").read_text())
    upload = json.loads((panel / "upload-receipt.json").read_text())
    require(replay["status"] == "PASS"
            and replay["checks"] == [{"object": manifest["bases"][0]["object"],
                                       "points_checked": 679936,
                                       "representatives_checked": 4096,
                                       "status": "PASS"}]
            and upload["status"] == "PASS"
            and upload["objects"] == [{"s3_uri": S3_URI,
                                       "compressed_blake3": OBJECT_BLAKE3,
                                       "downloaded_hash_matches": True}],
            "generic replay or upload receipt differs")
    old = json.loads((CHECKOUT / PRIOR_CONFIG).read_text())
    require(sha(binary) == old["binary_sha256"] == BINARY_SHA256,
            "reused Linux worker differs")
    fixture_hashes = {}
    for name, expected in old["fixture_sha256"].items():
        value = sha(fixtures / name)
        require(value == expected, f"public fixture {name} differs")
        fixture_hashes[name] = value
    require((panel / "probe-corpus.json").is_file(),
            "capacity panel is missing the exact public corpus")
    require(sha(panel / "probe-corpus.json") == fixture_hashes["probe-corpus.json"],
            "capacity panel is missing the exact public corpus")
    corpus = json.loads((panel / "probe-corpus.json").read_text())
    require(corpus["schema"] == "n83.public-probe-corpus/v1"
            and sum(row.get("a") == 0 and row.get("fixture") == 0
                    for row in corpus["targets"]) == 1,
            "public fixture-zero target is absent or ambiguous")
    source_hashes = {}
    for name, expected in old["source_sha256"].items():
        if name == "attestation.json":
            continue
        value = sha(CHECKOUT / name)
        require(value == expected, f"compiled worker source {name} differs")
        source_hashes[name] = value
    require(not subprocess.check_output(
        ["git", "status", "--porcelain", "--untracked-files=normal"],
        cwd=CHECKOUT, text=True).strip(), "source checkout is not clean")
    commit = subprocess.check_output(["git", "rev-parse", "HEAD"],
                                     cwd=CHECKOUT, text=True).strip()
    return {
        "schema": "n83.k4096-orbit-capacity-corrected-preflight/v1",
        "status": "PASS_CORRECTED_INPUTS",
        "source_commit": commit,
        "s3_uri": S3_URI,
        "compressed_sha256": OBJECT_SHA256,
        "compressed_blake3": OBJECT_BLAKE3,
        "s3_etag": s3_head["ETag"],
        "binary_sha256": BINARY_SHA256,
        "panel_corpus_sha256": fixture_hashes["probe-corpus.json"],
        "fixture_sha256": fixture_hashes,
        "source_sha256": source_hashes,
        "selected_best_total_runtime": None,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--panel", type=Path, required=True)
    parser.add_argument("--fixtures", type=Path, required=True)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--head", type=Path, required=True)
    parser.add_argument("--get", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = check(args.panel.resolve(strict=True), args.fixtures.resolve(strict=True),
                   args.binary.resolve(strict=True), args.head.resolve(strict=True),
                   args.get.resolve(strict=True))
    with args.output.open("x") as stream:
        json.dump(result, stream, indent=2, sort_keys=True)
        stream.write("\n")
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
