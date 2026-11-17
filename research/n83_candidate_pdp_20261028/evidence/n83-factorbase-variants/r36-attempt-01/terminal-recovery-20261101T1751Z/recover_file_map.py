#!/usr/bin/env python3
"""Recover logical filenames for complete producer outputs in an incomplete S3 finalizer run."""
import concurrent.futures
import hashlib
import json
import subprocess
from pathlib import Path

root = Path(__file__).resolve().parent
listing = json.loads((root / "s3-object-listing.json").read_text())
objects = [x for x in listing["objects"] if "/attempt-01/snapshots/" in x["Key"]]
by_path_sha = {x["Key"].split("/")[4]: x for x in objects}
assert len(objects) == 784, len(objects)

recovered = root / "recovered"
data = recovered / "data"
index_doc = json.loads((data / "index.json").read_text())
assert index_doc["complete"] is True
assert index_doc["candidate_id"] == "spike36"
assert index_doc["candidate_x_count_per_curve"] == 1 << 27
assert index_doc["coordinate_bit_positions"] == list(range(26)) + [36]
assert len(index_doc["families"]) == 3

paths = {"data/index.json"}
family_docs = {}
shard_count = 0
for family in ("koblitz", "binary", "prime"):
    manifest_path = f"data/{family}/manifest.json"
    manifest = json.loads((data / family / "manifest.json").read_text())
    family_docs[family] = manifest
    assert manifest["candidate_id"] == "spike36"
    assert manifest["family"] == family
    assert manifest["candidate_x_count"] == 1 << 27
    assert manifest["reuse_for_m"] == [2, 3, 4, 5, 6, 7, 8, 9]
    assert len(manifest["shards"]) == 128
    paths.add(manifest_path)
    shards = sorted(manifest["shards"], key=lambda s: s["start"])
    cursor = 0
    for shard in shards:
        assert shard["verified_all_candidates"] is True
        assert shard["start"] == cursor
        assert shard["end"] - shard["start"] == 1 << 20
        cursor = shard["end"]
        data_file = shard["file"]
        paths.add(f"data/{family}/{data_file}")
        paths.add(f"data/{family}/{Path(data_file).name.split('.')[0]}.json")
        shard_count += 1
    assert cursor == 1 << 27

paths.update({
    "protocol.json", "FACTOR_BASE_VARIANT_SWEEP_DESIGN.md", "FORMAT.json",
    "RUN_OUTPUT_FORMAT.json", "runner.sh", "instrument", "host.txt",
    "selfcheck.json", "selfcheck.expected-negative-controls.log", "producer.log",
    "run-manifest.json", "index-validation.json",
})
assert len(paths) == 784, len(paths)
file_map = []
missing = []
for rel in sorted(paths):
    path_sha = hashlib.sha256(rel.encode()).hexdigest()
    obj = by_path_sha.get(path_sha)
    if obj is None:
        missing.append(rel)
        continue
    content_sha = obj["Key"].split("/")[5]
    file_map.append({"path": rel, "bytes": obj["Size"], "sha256": content_sha,
                     "snapshot_key": obj["Key"], "last_modified": obj["LastModified"]})
assert not missing, missing
assert len(file_map) == len(objects)

mapped = {x["path"]: x for x in file_map}
for family, manifest in family_docs.items():
    index_entry = next(x for x in index_doc["families"] if x["family"] == family)
    mf = mapped[f"data/{family}/manifest.json"]
    assert mf["sha256"] == index_entry["manifest_sha256"]
    assert mf["bytes"] == len((data / family / "manifest.json").read_bytes())
    total_compressed = 0
    for shard in manifest["shards"]:
        path = f"data/{family}/{shard['file']}"
        item = mapped[path]
        assert item["sha256"] == shard["sha256"], (path, item["sha256"], shard["sha256"])
        assert item["bytes"] == shard["bytes"], (path, item["bytes"], shard["bytes"])
        total_compressed += item["bytes"]
    assert total_compressed == index_entry["compressed_bytes"]

# Restore and hash-check all non-compressed payloads; the 384 large compressed
# shards remain in S3 and are checked against the hashes in the family manifests.
restore = root / "restored"
restore.mkdir(exist_ok=True)
local_files = [x for x in file_map if not x["path"].endswith(".bin.zst")]
def fetch(item):
    target = restore / item["path"]
    target.parent.mkdir(parents=True, exist_ok=True)
    uri = "s3://isolab-results-104186811093-us-east-2/" + item["snapshot_key"]
    r = subprocess.run(["aws", "s3", "cp", uri, str(target), "--region", "us-east-2", "--only-show-errors"], capture_output=True, text=True)
    if r.returncode:
        return {"path": item["path"], "error": r.stderr, "exit_code": r.returncode}
    raw = target.read_bytes()
    sha = hashlib.sha256(raw).hexdigest()
    if len(raw) != item["bytes"] or sha != item["sha256"]:
        return {"path": item["path"], "error": "size/hash mismatch", "actual_bytes": len(raw), "actual_sha256": sha}
    return None
with concurrent.futures.ThreadPoolExecutor(max_workers=16) as pool:
    failures = [x for x in pool.map(fetch, local_files) if x]
assert not failures, failures

# Reconstruct the final sha256sum manifest from each content-addressed object hash.
manifest_lines = [f"{x['sha256']}  ./{x['path']}" for x in file_map]
manifest_bytes = ("\n".join(manifest_lines) + "\n").encode()
(recovered / "manifest.sha256.reconstructed").write_bytes(manifest_bytes)

result = {
    "schema": "n83.factor-base-recovered-file-map/v1",
    "job_name": "n83-fb-variants-r36-20261031-r1",
    "producer_exit_code": 0,
    "enumeration_validation": "true",
    "mapped_output_files": len(file_map),
    "expected_shards": shard_count,
    "families": {f: {"shards": len(m["shards"]), "point_count": m["point_count"],
                       "liftable_x": m["liftable_x"], "compressed_bytes": m["compressed_bytes"],
                       "same_base_for_m": m["reuse_for_m"]} for f, m in family_docs.items()},
    "restored_noncompressed_files": len(local_files),
    "restored_noncompressed_bytes": sum((restore / x["path"]).stat().st_size for x in local_files),
    "compressed_shards_retained_in_s3_and_checked_against_family_manifests": sum(x["path"].endswith(".bin.zst") for x in file_map),
    "original_upload_history_and_producer_final_s3_index": "not emitted by failed finalizer; this mapping was reconstructed from path/content hashes and format contract",
    "files": file_map,
}
(root / "recovered-file-map.json").write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps({k: v for k, v in result.items() if k != "files"}, indent=2))
