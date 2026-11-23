#!/usr/bin/env bash
# r2audit: restore each curve25519_f1_walk_20261107 session's records.jsonl from the
# tarballs the study committed (hydrated from the artifact store), verify against
# records.sha256, and install the artifact-store manifests + ignore blocks.
# Run from the repository root. Needs store read credentials.
set -euo pipefail
study=research/curve25519_f1_walk_20261107
here=$(cd "$(dirname "$0")" && pwd)
python3 tools/artifacts/artifacts.py pull "$study/data"
for k in k20 k24; do tar -xzf "$study/data/sessions-$k-records.tar.gz" -C "$study"; done
(cd "$study/sessions" && sha256sum -c "$here/records.sha256")
for d in "$here"/k*-s*/; do n=$(basename "$d"); cp "$d.artifacts.json" "$d.gitignore" "$study/sessions/$n/"; done
echo "now: python3 tools/artifacts/artifacts.py push $study/sessions/*/records.jsonl  (uploads 16 objects; writes manifests)"
