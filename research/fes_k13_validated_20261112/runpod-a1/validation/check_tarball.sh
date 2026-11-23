#!/usr/bin/env bash
# Validator: bind the committed runpod-a1 copy to the pod's results tarball in S3.
# Usage: check_tarball.sh <empty scratch dir> <out.json>
# Downloads results.tgz into <scratch>/s3, extracts it into <scratch>/x (untrusted data, never executed),
# and compares every file byte for byte with research/fes_k13_validated_20261112/runpod-a1/.
set -euo pipefail
SCR=$1; OUT=$2
HERE=$(cd "$(dirname "$0")/.." && pwd)            # runpod-a1/
URI=s3://isolab-results-104186811093-us-east-2/runpod/fes-k13-validated-20261112-a1/launch2/results.tgz
WANT=cd2f7736839c650d0db2cf0ef18d80538cddece0268852f6282bf2f0698a3e69
mkdir -p "$SCR/s3" "$SCR/x"
aws s3 cp --quiet "$URI" "$SCR/s3/results.tgz"
GOT=$(sha256sum "$SCR/s3/results.tgz" | cut -d' ' -f1)
tar xzf "$SCR/s3/results.tgz" -C "$SCR/x" --no-same-owner
listing=$(aws s3 ls --recursive s3://isolab-results-104186811093-us-east-2/runpod/fes-k13-validated-20261112-a1/ | awk '{print $3"\t"$4}')
python3 -I - "$SCR/x/out" "$HERE" "$URI" "$WANT" "$GOT" "$listing" > "$OUT" <<'EOF'
import hashlib, json, os, sys
tdir, rdir, uri, want, got, listing = sys.argv[1:7]
def h(p): return hashlib.sha256(open(p, 'rb').read()).hexdigest()
files = []
for root, dirs, fs in os.walk(tdir):
    for f in fs:
        p = os.path.join(root, f); rel = os.path.relpath(p, tdir)
        q = os.path.join(rdir, rel)
        files.append({"path": rel, "bytes": os.path.getsize(p), "sha256": h(p),
                      "repo_copy_present": os.path.isfile(q),
                      "repo_copy_identical": os.path.isfile(q) and h(q) == h(p)})
files.sort(key=lambda e: e["path"])
tar_dirs = sorted(os.path.relpath(os.path.join(r, d), tdir) for r, ds, _ in os.walk(tdir) for d in ds)
repo_session = sorted(os.path.relpath(os.path.join(r, f), os.path.join(rdir, "session")) for r, _, fs in os.walk(os.path.join(rdir, "session")) for f in fs)
tar_session = sorted(e["path"][len("session/"):] for e in files if e["path"].startswith("session/"))
print(json.dumps({
  "schema": "validator-tarball-binding/v1",
  "s3_uri": uri, "expected_sha256": want, "downloaded_sha256": got, "sha256_matches": want == got,
  "tar_directories": tar_dirs,
  "files": files,
  "session_dir_identical": tar_session == repo_session and all(e["repo_copy_identical"] for e in files if e["path"].startswith("session/")),
  "repo_session_files": repo_session,
  "extra_files_in_repo_session": sorted(set(repo_session) - set(tar_session)),
  "tar_files_not_copied": [e["path"] for e in files if not e["repo_copy_present"]],
  "s3_prefix_listing": [l.split("\t") for l in listing.splitlines()],
}, indent=1))
EOF
echo "tarball sha256 $GOT (want $WANT)"
