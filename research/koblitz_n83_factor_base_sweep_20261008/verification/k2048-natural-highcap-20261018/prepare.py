#!/usr/bin/env python3
"""Rehydrate and hash the exact retained primary K2048 search input."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess

import blake3

HERE = Path(__file__).resolve().parent
STUDY = HERE.parents[1]
OLD = HERE.parent / "matched-k1182-k2048-900s-20261011/evidence-01/panel-k2048"
BUCKET = "crypto-autoresearcher"
OBJECT_BLAKE3 = "82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851"
OBJECT_SHA256 = "aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a"
CORPUS_SHA256 = "760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd"
VALIDATION_SHA256 = "1c6d3a45c8b4654ff208751258fa2e34f0cc60e1c30ba48e9214db62be60472f"


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def b3(path: Path) -> str:
    digest = blake3.blake3()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def aws(*args: str) -> dict:
    call = subprocess.run(["aws", "s3api", *args, "--output", "json"],
                          check=True, capture_output=True, text=True, timeout=180)
    return json.loads(call.stdout)


def prepare(output: Path) -> dict:
    source_manifest = json.loads((OLD / "manifest.json").read_text())
    source_replay = json.loads((OLD / "replay.json").read_text())
    source_upload = json.loads((OLD / "upload-receipt.json").read_text())
    require(source_manifest["schema"] == "n83.factor-base-panel/v2-size-frontier"
            and source_manifest["status"] == "completed_factor_base_object"
            and source_replay["status"] == "PASS"
            and source_upload["status"] == "PASS"
            and len(source_manifest["bases"]) == 1, "retained panel receipts differ")
    row = source_manifest["bases"][0]
    name = f"objects/{OBJECT_BLAKE3}.jsonl.gz"
    uri = ("s3://crypto-autoresearcher/factor-bases/icv1/etc/"
           "koblitz_n83_factor_base_sweep_20261008/v2-size-frontier/a0/" + name)
    require((row["a"], row["columns"], row["policy"], row["seed"])
            == (0, 2048, "public_x_hash", 2026100801)
            and row["object"] == name and row["s3_uri"] == uri
            and row["compressed_blake3"] == OBJECT_BLAKE3
            and row["points"] == 339968 and row["bytes"] == 10313647
            and source_replay["checks"] == [{"object": name,
                  "points_checked": 339968, "representatives_checked": 2048,
                  "status": "PASS"}]
            and source_upload["objects"] == [{"compressed_blake3": OBJECT_BLAKE3,
                  "downloaded_hash_matches": True, "s3_uri": uri}],
            "selected K2048 source row or replay differs")
    output.mkdir(parents=True, exist_ok=False)
    panel, fixtures = output / "panel", output / "fixtures"
    panel.mkdir()
    fixtures.mkdir()
    for name_ in ("manifest.json", "replay.json", "upload-receipt.json"):
        shutil.copyfile(OLD / name_, panel / name_)
    for name_ in ("probe-corpus.json", "probe-validation.json"):
        shutil.copyfile(STUDY / "pilot-01" / name_, fixtures / name_)
    require(sha256(fixtures / "probe-corpus.json") == CORPUS_SHA256
            and sha256(fixtures / "probe-validation.json") == VALIDATION_SHA256,
            "public fixture differs")
    destination = panel / name
    destination.parent.mkdir()
    key = uri.split(f"s3://{BUCKET}/", 1)[1]
    head = aws("head-object", "--bucket", BUCKET, "--key", key)
    get = aws("get-object", "--bucket", BUCKET, "--key", key, str(destination))
    require(head["ContentLength"] == get["ContentLength"] == destination.stat().st_size
            == row["bytes"] and head["ETag"] == get["ETag"]
            and head["ServerSideEncryption"] == "AES256"
            and sha256(destination) == OBJECT_SHA256
            and b3(destination) == OBJECT_BLAKE3,
            "fresh S3 object bytes or metadata differ")
    receipt = {"schema": "n83.k2048-natural-highcap-input/v1",
               "status": "PASS_FRESH_S3_INPUT", "s3_uri": uri,
               "object_blake3": OBJECT_BLAKE3,
               "object_sha256": OBJECT_SHA256, "object_bytes": row["bytes"],
               "head_etag": head["ETag"], "get_etag": get["ETag"],
               "panel_sha256": {name_: sha256(panel / name_) for name_ in
                                 ("manifest.json", "replay.json", "upload-receipt.json")},
               "fixture_sha256": {name_: sha256(fixtures / name_) for name_ in
                                     ("probe-corpus.json", "probe-validation.json")},
               "selected_best_total_runtime": None}
    with (output / "input.json").open("x") as stream:
        json.dump(receipt, stream, indent=2, sort_keys=True)
        stream.write("\n")
    return receipt


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(prepare(args.output), sort_keys=True))


if __name__ == "__main__":
    main()
