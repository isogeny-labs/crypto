#!/usr/bin/env bash
set -euo pipefail
root=/var/tmp/independent-kummer-row-fold-review-20261101
out=$(mktemp -d "$root/replay-XXXXXXXX")
mkdir "$out/snapshot"
cp "$root/snapshot/"*.lean "$out/snapshot/"
lean=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean
export LEAN_SYSROOT=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain
export LEAN_PATH="$out/snapshot:/var/tmp/pencil-comparator-word-20261101/snapshot:/var/tmp/independent-product-review-20261101/snapshot:/var/tmp/pencil-mumford-validity-20261031-v1/dependency-union:/var/tmp/pencil-sameu-global-smooth-20261031-v1/dependency-union"
sha256sum "$out/snapshot/"*.lean "$lean" "$root/replay.sh" > "$out/before.sha256"
printf '%s\n' "$out" > "$root/current-replay"
cd "$out/snapshot"
for m in Product Comparator WordComparator Repair RowFold Controls Readback Reviewer ReviewerReadback;do
 set +e
 timeout -k 5s 120s "$lean" -j1 -M4096 -o "$m.olean" "$m.lean" > "$out/$m.stdout" 2> "$out/$m.stderr"
 rc=$?
 set -e
 printf '%s\n' "$rc" > "$out/$m.exit"
 test "$rc" -eq 0 || exit "$rc"
done
sha256sum -c "$out/before.sha256" > "$out/bindings-check.stdout"
printf '%s\n' "$out"
