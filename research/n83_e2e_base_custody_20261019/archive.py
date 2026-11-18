#!/usr/bin/env python3
"""Bind replayed N83 cold bases to content-addressed S3 objects."""

import argparse
import gzip
import hashlib
import json
import re
import subprocess
import tempfile
import zipfile
from collections import defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SOURCE = ROOT / "research/n83_e2e_precision_20261016"
PROVIDER_SHA256 = "559578cd386693ce3681827ba3a9f192bd99be7ca72920fc35734cf515fd844d"
PREFIX = "s3://crypto-autoresearcher/factor-bases/icv1/etc/n83_e2e_cold_precision_20261019/a1/objects"
BASE_RE = re.compile(r"results/runs/p([1-9][0-9]*)_k(1104|1120)_([ab])/base_dump\.jsonl")
KS = (1104, 1120)
CANDIDATE_SHA256 = {
    1104: "1c2b0c54d04d086e5e402f2d11ad9923b992e2e3fb5f2e7aa3396ffb434fcdd7",
    1120: "053c054c040ad15e594089dbf62bcdfa408d172b48f41584f0e91b7cb173604d",
}
REPLAY_RECEIPT_SHA256 = "f8bd31d750e8109f115a42af94953a9aca706f946c8ac5dec080d176eb9bdae1"


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def aws(*args):
    result = subprocess.run(("aws", *args), capture_output=True, text=True)
    if result.returncode:
        raise RuntimeError(f"aws {' '.join(args[:2])} failed: {result.stderr.strip()}")
    return result.stdout


def head(uri):
    key = uri.removeprefix("s3://crypto-autoresearcher/")
    result = subprocess.run(("aws", "s3api", "head-object", "--bucket",
                             "crypto-autoresearcher", "--key", key,
                             "--output", "json"), capture_output=True, text=True)
    if result.returncode:
        if "Not Found" in result.stderr or "404" in result.stderr:
            return None
        raise RuntimeError(f"S3 HEAD failed: {result.stderr.strip()}")
    return json.loads(result.stdout)


def candidates():
    out = {}
    for k in KS:
        files = list(SOURCE.glob(f"candidate_*fb{166*k}PDP4rootRCguidedLAgaussTDdirectISO0h*.json"))
        if len(files) != 1:
            raise ValueError(f"expected one canonical K{k} manifest, got {len(files)}")
        if sha256(files[0].read_bytes()) != CANDIDATE_SHA256[k]:
            raise ValueError(f"K{k} candidate manifest SHA-256 mismatch")
        obj = json.loads(files[0].read_text())
        base = obj["factor_base"]
        if (base["effective_columns"] != k or
                base["actual_usable_points"] != 166 * k or
                obj["curve"]["curve_id"] != "EC1N83Ckb1a1haacb69ee9f37"):
            raise ValueError(f"K{k} candidate identity mismatch")
        out[k] = {"path": str(files[0].relative_to(ROOT)),
                  "sha256": sha256(files[0].read_bytes()),
                  "candidate_id": obj["candidate_id"],
                  "ordered_base_dump_sha256": base["ordered_base_dump_sha256"],
                  "source_base_hash_blake3": base["source_base_hash_blake3"],
                  "actual_usable_points": base["actual_usable_points"],
                  "effective_columns": k}
    return out


def collect(archive, expected):
    rows = defaultdict(list)
    selected = {}
    with zipfile.ZipFile(archive) as z:
        if z.testzip() is not None:
            raise ValueError("provider ZIP CRC failure")
        for name in z.namelist():
            m = BASE_RE.fullmatch(name)
            if m is None:
                continue
            point, k, repeat = map(str, m.groups())
            k = int(k)
            with z.open(name) as stream:
                raw = stream.read()
            digest = sha256(raw)
            if digest != expected[k]["ordered_base_dump_sha256"]:
                raise ValueError(f"{name} differs from canonical ordered base")
            rows[k].append({"point": int(point), "repeat": repeat,
                            "zip_member": name, "raw_sha256": digest})
            if int(point) == 1 and repeat == "a":
                selected[k] = raw
    for k in KS:
        seen = {(r["point"], r["repeat"]) for r in rows[k]}
        if seen != {(point, repeat) for point in range(1, 17)
                    for repeat in ("a", "b")} or k not in selected:
            raise ValueError(f"K{k} is missing a precision base dump")
        obj = json.loads(selected[k])
        if (obj["kind"] != "point_defined_factor_base" or
                obj["n"] != 83 or obj["orbit_columns"] != k or
                obj["base_hash"] != expected[k]["source_base_hash_blake3"] or
                len(obj["factor_base_point_coordinates"]) !=
                expected[k]["actual_usable_points"]):
            raise ValueError(f"K{k} base body identity mismatch")
    return rows, selected


def archive_one(k, raw, output, upload, expected):
    packed = gzip.compress(raw, compresslevel=9, mtime=0)
    digest = sha256(packed)
    path = output / f"{digest}.jsonl.gz"
    if path.exists() and path.read_bytes() != packed:
        raise ValueError("local content-addressed path conflicts")
    path.write_bytes(packed)
    if gzip.decompress(path.read_bytes()) != raw:
        raise ValueError("local gzip replay failed")
    uri = f"{PREFIX}/{path.name}"
    result = {"k": k, "s3_uri": uri, "gzip_sha256": digest,
              "gzip_bytes": len(packed), "raw_sha256": sha256(raw),
              "raw_bytes": len(raw), "roundtrip_status": "NOT_UPLOADED"}
    if upload:
        existing = head(uri)
        if existing is None:
            metadata = (f"raw-sha256={sha256(raw)},"
                        f"base-hash={expected['source_base_hash_blake3']},"
                        f"role=k{k}root11")
            aws("s3", "cp", str(path), uri, "--no-progress",
                "--only-show-errors", "--metadata", metadata)
        observed = head(uri)
        if (observed is None or observed["ContentLength"] != len(packed)
                or observed.get("Metadata", {}).get("raw-sha256") != sha256(raw)
                or observed.get("Metadata", {}).get("base-hash") !=
                expected["source_base_hash_blake3"]):
            raise ValueError(f"K{k} S3 HEAD metadata mismatch")
        with tempfile.TemporaryDirectory(dir=output) as folder:
            download = Path(folder) / "roundtrip.jsonl.gz"
            aws("s3", "cp", uri, str(download), "--no-progress",
                "--only-show-errors")
            if (sha256(download.read_bytes()) != digest or
                    sha256(gzip.decompress(download.read_bytes())) != sha256(raw)):
                raise ValueError(f"K{k} S3 downloaded bytes mismatch")
        result.update({"roundtrip_status": "PASS_DOWNLOADED_SHA256",
                       "s3_etag": observed["ETag"],
                       "s3_last_modified": observed["LastModified"]})
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--archive", type=Path, required=True)
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--receipt", type=Path, required=True)
    parser.add_argument("--upload", action="store_true")
    args = parser.parse_args()
    replay_path = SOURCE / "replay_receipt_38970960376.json"
    if sha256(replay_path.read_bytes()) != REPLAY_RECEIPT_SHA256:
        raise ValueError("independent replay receipt SHA-256 mismatch")
    replay = json.loads(replay_path.read_text())
    if (replay["run_id"] != 38970960376 or
            replay["run_head_sha"] != "d13e28e76d3709c19f15cf2c352d4ab912812335" or
            replay["provider_zip_sha256"] != PROVIDER_SHA256 or
            replay["evidence_status"] != "PASS_INDEPENDENT_REPLAY" or
            replay["ic_run_count"] != 64 or replay["candidate_count"] != 2):
        raise ValueError("precision replay receipt fails admission")
    if sha256(args.archive.read_bytes()) != PROVIDER_SHA256:
        raise ValueError("provider ZIP SHA-256 mismatch")
    expected = candidates()
    rows, selected = collect(args.archive, expected)
    args.output_dir.mkdir(parents=True, exist_ok=True)
    objects = [archive_one(k, selected[k], args.output_dir,
                           args.upload, expected[k]) for k in KS]
    manifest = {"schema": "n83.e2e-base-custody/v1",
                "status": "PASS_S3_ROUNDTRIP" if args.upload else "LOCAL_ONLY",
                "source_run_id": 38970960376,
                "source_head_sha": replay["run_head_sha"],
                "provider_zip_sha256": PROVIDER_SHA256,
                "independent_replay_receipt_sha256": sha256((SOURCE /
                    "replay_receipt_38970960376.json").read_bytes()),
                "destination_prefix": PREFIX,
                "candidates": expected,
                "observations": {str(k): sorted(rows[k], key=lambda r:
                    (r["point"], r["repeat"])) for k in KS},
                "objects": objects}
    serialized = json.dumps(manifest, indent=2, sort_keys=True) + "\n"
    if args.receipt.exists() and args.receipt.read_text() != serialized:
        raise ValueError("existing custody receipt differs")
    args.receipt.write_text(serialized)
    print(json.dumps({"status": manifest["status"],
                      "objects": objects}, sort_keys=True))


if __name__ == "__main__":
    main()
