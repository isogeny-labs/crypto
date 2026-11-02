#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

source_revision=a34125841c2b696420bd97440d59b73c13355abc
binary=f3aab23417ece64cb3373e5c66d2c3be343e18c0e89347f626e0aea18ab1b774
image_digest=sha256:3f571ddab4df5624a48a8131b7c1c07ac9c11609120904954f5c2f4e67161e7e
owner=$(printf 'b%.0s' {1..64})
mkdir -p "$tmp/artifact/isolab-build"

jq -n --arg source "$source_revision" --arg binary "$binary" \
  --arg image "104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@$image_digest" \
  '{image_uri:$image,source_revision:$source,ecbench_sha256:$binary}' \
  > "$tmp/artifact/isolab-image.json"
jq -n --arg source "$source_revision" --arg binary "$binary" --arg owner "$owner" \
  '{source_repository:"https://github.com/aburan28/crypto",source_revision:$source,
    ecbench_sha256:$binary,fresh_owner_sha256:$owner,built_at:"2026-11-01T23:14:17Z",
    architecture:"linux/amd64",target:"x86_64-unknown-linux-musl",default_features:false,
    rustc:"rustc 1.99.1 (fixture 2026-10-01)\nbinary: rustc",rustflags:"-C target-cpu=x86-64"}' \
  > "$tmp/artifact/isolab-build/build-info.json"

"$here/verify-publication-receipt" "$tmp/artifact" "$tmp/receipt.json"
jq -e --arg source "$source_revision" --arg binary "$binary" --arg digest "$image_digest" \
  '.schema=="e2e.phase-ledger.publication-receipt/v1" and
   .publication_workflow_run=="41362392013" and .artifact_id==13106247069 and
   .source_revision==$source and .ecbench_sha256==$binary and .image_digest==$digest and
   .artifact_file_count==2' "$tmp/receipt.json" >/dev/null
! grep -q '104186811093' "$tmp/receipt.json"

cp -a "$tmp/artifact" "$tmp/wrong-binary"
jq '.ecbench_sha256="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"' \
  "$tmp/wrong-binary/isolab-image.json" > "$tmp/changed"
mv "$tmp/changed" "$tmp/wrong-binary/isolab-image.json"
if "$here/verify-publication-receipt" "$tmp/wrong-binary" "$tmp/rejected.json" 2>/dev/null; then
  echo 'wrong binary hash was accepted' >&2
  exit 1
fi

cp -a "$tmp/artifact" "$tmp/extra-file"
printf 'unexpected\n' > "$tmp/extra-file/private.log"
if "$here/verify-publication-receipt" "$tmp/extra-file" "$tmp/rejected.json" 2>/dev/null; then
  echo 'unexpected artifact file was accepted' >&2
  exit 1
fi

cp -a "$tmp/artifact" "$tmp/symlink"
ln -s isolab-image.json "$tmp/symlink/alias.json"
if "$here/verify-publication-receipt" "$tmp/symlink" "$tmp/rejected.json" 2>/dev/null; then
  echo 'artifact symlink was accepted' >&2
  exit 1
fi

cp -a "$tmp/artifact" "$tmp/wrong-digest"
jq '.image_uri |= sub("3f571d";"af571d")' "$tmp/wrong-digest/isolab-image.json" > "$tmp/changed"
mv "$tmp/changed" "$tmp/wrong-digest/isolab-image.json"
if "$here/verify-publication-receipt" "$tmp/wrong-digest" "$tmp/rejected.json" 2>/dev/null; then
  echo 'wrong image digest was accepted' >&2
  exit 1
fi

echo 'Publication receipt fixture accepted only the fixed source, binary, image, metadata and two-file artifact'
