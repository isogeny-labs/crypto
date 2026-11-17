"""Metadata-only remote Git blob comparison; performs no curve arithmetic."""
import argparse
import json
from pathlib import Path
import subprocess

parser = argparse.ArgumentParser()
parser.add_argument("--revision", required=True)
parser.add_argument("--pr", required=True)
parser.add_argument("--out", type=Path, required=True)
parser.add_argument("--path", action="append", dest="paths")
args = parser.parse_args()
assert not args.out.exists(), "new publication receipt required"
study = "research/nist_cover_three_tests_20261116"
paths = [
    f"{study}/MAPS_STATUS.md", f"{study}/MAPS_STATUS.pdf",
    f"{study}/complete_maps.cpp", f"{study}/complete_km.cpp",
    f"{study}/transpose_maps.cpp", f"{study}/closed_transpose.cpp",
    f"{study}/moving_transpose.cpp",
    f"{study}/evidence/complete-km-v3/result.json",
    f"{study}/evidence/moving-transpose-v1/result.json",
    f"{study}/evidence/closed-transpose-v1/result.json",
    "pcc/claims/PCC-20261117-d7beff.json",
]
if args.paths:
    paths = args.paths
records = []
for path in paths:
    expected = subprocess.check_output(
        ["git", "rev-parse", f"{args.revision}:{path}"], text=True
    ).strip()
    remote = json.loads(subprocess.check_output([
        "gh", "api", f"repos/isogeny-labs/crypto/contents/{path}?ref={args.revision}",
        "--jq", "{path,sha,size}",
    ], text=True))
    assert remote["sha"] == expected, path
    records.append({"path": path, "git_blob_sha": expected, "remote_matches": True})
pr = json.loads(subprocess.check_output([
    "gh", "pr", "view", args.pr, "--json", "number,url,state,isDraft,headRefOid",
], text=True))
out = {"published_revision": args.revision, "pr": pr, "files": records}
args.out.write_text(json.dumps(out, indent=2) + "\n")
print(json.dumps({"revision": args.revision, "matched_files": len(records), "pr": pr}))
