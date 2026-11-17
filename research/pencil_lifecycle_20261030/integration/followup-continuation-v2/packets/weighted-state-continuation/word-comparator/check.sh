#!/usr/bin/env bash
set -euo pipefail
root=/var/tmp/pencil-comparator-word-20261101
lean=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean
export LEAN_SYSROOT=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain
export LEAN_PATH="$root/snapshot:/var/tmp/independent-product-review-20261101/snapshot:/var/tmp/pencil-mumford-validity-20261031-v1/dependency-union:/var/tmp/pencil-sameu-global-smooth-20261031-v1/dependency-union"
cd "$root/snapshot"
module=$1
tag=$2
mkdir "$root/$tag"
cp "$module.lean" "$root/$tag/"
sha256sum ./*.lean ./*.txt "$root/PLAN.md" "$root/check.sh" "$lean" > "$root/$tag/before.sha256"
set +e
timeout -k 5s 120s "$lean" -j1 -M4096 -o "$module.olean" "$module.lean" > "$root/$tag/stdout" 2> "$root/$tag/stderr"
rc=$?
set -e
jq -n --arg m "$module" --argjson r "$rc" '{module:$m,native_exit:$r}' > "$root/$tag/receipt.json"
sha256sum -c "$root/$tag/before.sha256" > "$root/$tag/after.txt"
exit "$rc"
