#!/usr/bin/env bash
set -euo pipefail
root=/var/tmp/independent-signed-pair-review-20261101
lean=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean
export LEAN_SYSROOT=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain
export LEAN_PATH="$root/snapshot:/var/tmp/independent-product-review-20261101/snapshot:/var/tmp/pencil-mumford-validity-20261031-v1/dependency-union:/var/tmp/pencil-sameu-global-smooth-20261031-v1/dependency-union"
cd "$root/snapshot"
sha256sum ./*.lean ./source/*.rs "$root/PLAN.md" "$root/run.sh" "$lean" > "$root/bindings-before.sha256"
"$lean" --version > "$root/lean-version.txt"
for m in SignedPair Controls Readback ReviewerControls ReviewerReadback; do
 set +e
 timeout -k 5s 120s "$lean" -j1 -M4096 -o "$m.olean" "$m.lean" > "$root/$m.stdout" 2> "$root/$m.stderr"
 rc=$?
 set -e
 jq -n --arg module "$m" --argjson exit "$rc" '{module:$module,native_exit:$exit}' > "$root/$m.receipt.json"
 test "$rc" -eq 0 || exit "$rc"
done
sha256sum -c "$root/bindings-before.sha256" > "$root/bindings-after.txt"
