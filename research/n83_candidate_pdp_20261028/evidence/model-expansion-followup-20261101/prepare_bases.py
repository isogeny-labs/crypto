#!/usr/bin/env python3
"""Assemble verified per-base metadata and immutable S3 shard indexes."""
import hashlib
import json
import shutil
import subprocess
from pathlib import Path

study = Path(__file__).resolve().parents[2]
repo = study.parent.parent
protocol_path = study / "protocol-model-full-domain-v2.json"
protocol = json.loads(protocol_path.read_text())
attempt = Path(__file__).resolve().parent / "attempt-05"
source_root = attempt / "base-sources"
if attempt.exists():
    raise SystemExit(f"fresh attempt required; preserving existing {attempt}")
source_root.mkdir(parents=True)
tmp_root = Path("/tmp/n83-full-domain-bases-attempt05")
tmp_root.mkdir(parents=True, exist_ok=True)

S3_INDEX_SOURCE = {
    27: "research/n83_candidate_pdp_20261028/evidence/n83-factorbase-variants/r27-attempt-07/terminal-20261031T0915Z/s3-file-index.json",
    29: "research/n83_candidate_pdp_20261028/evidence/n83-factorbase-variants/r29-attempt-02/factor-base-output/s3-file-index.json",
    30: "research/n83_candidate_pdp_20261028/evidence/n83-factorbase-variants/r30-attempt-01/terminal-20261031T1015Z/s3-file-index.json",
    31: "research/n83_candidate_pdp_20261028/evidence/n83-factorbase-variants/r31-attempt-01/terminal-20261031T1045Z/s3-file-index.json",
    32: "research/n83_candidate_pdp_20261028/evidence/n83-factorbase-variants/r32-attempt-01/terminal-20261031T1045Z/s3-file-index.json",
    33: "research/n83_candidate_pdp_20261028/evidence/n83-factorbase-variants/r33-attempt-01/terminal-20261031T1620Z/s3-file-index.json",
}

records = []
for source in protocol["design"]["factor_base_sources"]:
    bit = source["coordinate_bit"]
    candidate = source["candidate_id"]
    source_index = study / source["index_path"]
    raw_index = source_index.read_bytes()
    assert hashlib.sha256(raw_index).hexdigest() == source["index_sha256"]
    index = json.loads(raw_index)
    assert index["candidate_id"] == candidate and index["ell"] == 27
    assert index["candidate_x_count_per_curve"] == 1 << 27

    base_dir = tmp_root / candidate
    if base_dir.exists():
        raise SystemExit(f"preserving existing prepared base directory: {base_dir}")
    base_dir.mkdir(parents=True)
    (base_dir / "index.json").write_bytes(raw_index)
    saved_dir = source_root / candidate
    saved_dir.mkdir()
    (saved_dir / "index.json").write_bytes(raw_index)

    family_records = {}
    for family, expected_sha in source["family_manifests"].items():
        candidates = [
            source_index.parent / family / "manifest.json",
            source_index.parent / "manifests" / f"{family}-manifest.json",
            source_index.parent.parent / "manifests" / f"{family}-manifest.json",
            source_index.parent / f"{family}-manifest.json",
        ]
        variant_root = source_index
        while not variant_root.name.startswith(f"r{bit}-attempt-") and variant_root.parent != variant_root:
            variant_root = variant_root.parent
        candidates.extend(variant_root.rglob("*manifest.json"))
        manifest_path = next((p for p in candidates if p.is_file() and hashlib.sha256(p.read_bytes()).hexdigest() == expected_sha), None)
        if manifest_path is None:
            raise RuntimeError(f"could not locate exact {family} manifest {expected_sha} for {candidate}")
        raw_manifest = manifest_path.read_bytes()
        (base_dir / family).mkdir()
        (base_dir / family / "manifest.json").write_bytes(raw_manifest)
        (saved_dir / f"{family}-manifest.json").write_bytes(raw_manifest)
        family_records[family] = {
            "source_path": str(manifest_path.relative_to(repo)),
            "sha256": expected_sha,
            "bytes": len(raw_manifest),
        }

    if bit == 28:
        summary_path = repo / "research/n83_candidate_pdp_20261028/evidence/n83-factorbase-variants/r28-attempt-01/terminal-20261031T0933Z/summary.json"
        summary = json.loads(summary_path.read_text())
        key = summary["final_results_listing"]["s3_file_index"][0]["Key"]
        uri = f"s3://isolab-results-104186811093-us-east-2/{key}"
        s3_index = Path("/tmp/n83-r28-full-domain-s3-file-index.json")
        subprocess.run(["aws", "s3", "cp", uri, str(s3_index), "--region", "us-east-2", "--only-show-errors"], check=True)
    else:
        s3_index = repo / S3_INDEX_SOURCE[bit]
    raw_s3_index = s3_index.read_bytes()
    remote_index = json.loads(raw_s3_index)
    assert len(remote_index["files"]) >= 700
    (base_dir / "s3-file-index.json").write_bytes(raw_s3_index)
    (saved_dir / "s3-file-index.json").write_bytes(raw_s3_index)
    source_record = {
        "candidate_id": candidate,
        "coordinate_bit": bit,
        "index_sha256": source["index_sha256"],
        "index_bytes": len(raw_index),
        "family_manifests": family_records,
        "s3_file_index_sha256": hashlib.sha256(raw_s3_index).hexdigest(),
        "s3_file_index_bytes": len(raw_s3_index),
        "s3_file_count": len(remote_index["files"]),
        "s3_file_index_source": str(s3_index),
        "s3_file_index_schema": remote_index.get("schema"),
    }
    (saved_dir / "source-manifest.json").write_text(json.dumps(source_record, indent=2) + "\n")
    records.append(source_record)

manifest = {
    "schema": "n83.full-domain-factor-base-sources/v1",
    "protocol_sha256": hashlib.sha256(protocol_path.read_bytes()).hexdigest(),
    "prepared_bases": records,
    "layout": "base-sources/spikeNN/{index.json,s3-file-index.json,source-manifest.json,koblitz-manifest.json,binary-manifest.json,prime-manifest.json}",
    "shard_policy": "Only selector shards required by sampled inputs are fetched; each payload is SHA-256 and byte-count verified against the immutable S3 file index.",
}
(source_root / "source-index.json").write_text(json.dumps(manifest, indent=2) + "\n")
print(json.dumps({"attempt": str(attempt), "bases": len(records), "metadata_files": len(list(source_root.rglob("*"))), "protocol_sha256": manifest["protocol_sha256"]}, indent=2))
