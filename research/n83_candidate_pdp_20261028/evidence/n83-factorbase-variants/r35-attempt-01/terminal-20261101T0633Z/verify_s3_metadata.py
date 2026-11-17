#!/usr/bin/env python3
"""Restore and hash-check every non-shard payload from an immutable N83 S3 index."""
import concurrent.futures
import hashlib
import json
import subprocess
import sys
from pathlib import Path

index_path = Path(sys.argv[1])
out = Path(sys.argv[2])
index = json.loads(index_path.read_text())
files = index["files"]
selected = [row for row in files if not row["path"].endswith(".bin.zst")]
out.mkdir(parents=True, exist_ok=True)

def fetch(row):
    target = out / row["path"]
    target.parent.mkdir(parents=True, exist_ok=True)
    proc = subprocess.run(["aws", "s3", "cp", row["snapshot_key"], str(target), "--region", "us-east-2", "--only-show-errors"], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if proc.returncode:
        return {"path": row["path"], "ok": False, "error": proc.stderr.decode(errors="replace")[-1000:]}
    raw = target.read_bytes()
    got_sha = hashlib.sha256(raw).hexdigest()
    return {"path": row["path"], "ok": len(raw) == row["bytes"] and got_sha == row["sha256"], "bytes": len(raw), "sha256": got_sha, "expected_bytes": row["bytes"], "expected_sha256": row["sha256"]}

with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:
    restored = list(pool.map(fetch, selected))
failed = [row for row in restored if not row["ok"]]
summary = {"schema":"n83.factor-base-readback/v1","candidate_id":index.get("candidate_id"),"coordinate_bit":index.get("coordinate_bit"),"protocol_sha256":index.get("protocol_sha256"),"indexed_files":len(files),"compressed_shard_payloads":sum(row["path"].endswith(".bin.zst") for row in files),"metadata_payloads_selected":len(selected),"metadata_payloads_verified":len(restored)-len(failed),"metadata_bytes_verified":sum(row.get("bytes",0) for row in restored if row["ok"]),"failures":failed}

# Every manifest line must agree with the authoritative content-addressed index.
manifest_path=out/"manifest.sha256"
manifest_ok=None
if manifest_path.exists():
    by_path={row["path"]:row for row in files}
    checks=[]
    for line in manifest_path.read_text().splitlines():
        if not line.strip(): continue
        digest, rel = line.split(None,1)
        rel=rel.lstrip(" *./")
        row=by_path.get(rel)
        checks.append(bool(row and row["sha256"]==digest))
    manifest_ok=bool(checks) and all(checks)
summary["manifest_sha256_entries_match_index"]=manifest_ok

# Extract the structural controls for a human-readable completion record.
data_index_path=out/"data/index.json"
if data_index_path.exists():
    d=json.loads(data_index_path.read_text())
    summary["data_index"]={k:d.get(k) for k in ["schema","complete","candidate_id","coordinate_bit","candidate_x_count_per_curve","candidate_x_count","coordinate_bit_positions","same_base_for_m"] if k in d}
    fams=d.get("families",[])
    summary["family_count"]=len(fams)
    summary["families"]=[]
    for f in fams:
        summary["families"].append({k:f.get(k) for k in ["family","curve_id","point_count","candidate_x_count","shards","shard_count","same_base_for_m","complete"] if k in f})
# Extract shard metadata facts; preserve each path and digest binding in restored/.
shards=[]
for row in files:
    path=row["path"]
    if path.startswith("data/") and path.endswith(".json") and "/part-" in path:
        p=out/path
        if p.exists():
            try:
                d=json.loads(p.read_text())
                shards.append({"path":path,"family":path.split("/")[1],"metadata_sha256":row["sha256"],"fields":d})
            except Exception as exc:
                summary.setdefault("metadata_parse_failures",[]).append({"path":path,"error":str(exc)})
summary["shard_metadata_count"]=len(shards)
summary["shard_records"]=shards
summary["verified"]=not failed and manifest_ok is True and summary.get("data_index",{}).get("complete") is True and len(shards)==384
(out/"../metadata-readback-summary.json").write_text(json.dumps(summary,indent=2)+"\n")
print(json.dumps({k:v for k,v in summary.items() if k not in ("shard_records","failures")},indent=2))
if failed or not summary["verified"]: raise SystemExit(1)
