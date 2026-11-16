#!/usr/bin/env bash
set -euo pipefail
root=/var/tmp/pencil-kummer-row-fold-20261101-v1
lean=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean
export LEAN_SYSROOT=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain
export LEAN_PATH="$root/snapshot:/var/tmp/pencil-comparator-word-20261101/snapshot:/var/tmp/independent-product-review-20261101/snapshot:/var/tmp/pencil-mumford-validity-20261031-v1/dependency-union:/var/tmp/pencil-sameu-global-smooth-20261031-v1/dependency-union"
cd "$root/snapshot"
for m in "$@";do
 set +e
 timeout -k 5s 120s "$lean" -j1 -M4096 -o "$m.olean" "$m.lean" > "$root/$m.stdout" 2> "$root/$m.stderr"
 rc=$?
 set -e
 jq -n --arg m "$m" --argjson r "$rc" '{module:$m,native_exit:$r}' > "$root/$m.receipt.json"
 test "$rc" -eq 0 || exit "$rc"
done
