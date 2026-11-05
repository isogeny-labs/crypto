#!/usr/bin/env bash
set -euo pipefail
D=/var/tmp/pencil-find-count-20261101-v1
m=$1
out=$(mktemp -d "$D/$m-attempt-XXXXXXXX")
cp "$D/$m.lean" "$out/"
lean=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean
export LEAN_SYSROOT=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain
export LEAN_PATH="$D:/var/tmp/pencil-edge-dispatch-20261101-v1:/var/tmp/pencil-union-return-20261101-v1:/var/tmp/pencil-component-insert-20261101-v1:/var/tmp/pencil-component-size-20261101-v1:/var/tmp/pencil-union-size-20261101-v1:/var/tmp/pencil-word-union-composition-20261101-v1:/var/tmp/pencil-word-union-20261101-v1:/var/tmp/pencil-quotient-storage-20261101-v1:/var/tmp/pencil-division-quotient-20261101-v1:$(cat "$D/dependencies.txt")"
{ find "$D" -maxdepth 1 -type f \( -name '*.lean' -o -name '*.olean' -o -name 'PLAN.md' -o -name '*.sh' \) ! -name "$m.olean"; printf '%s\n' "$lean" /home/ec2-user/crypto-pencil-signed-graph-20261029/src/cryptanalysis/three_leads.rs; IFS=: read -ra roots <<< "$LEAN_PATH"; for r in "${roots[@]:1}"; do find "$r" -maxdepth 1 -type f \( -name '*.lean' -o -name '*.olean' \); done; } | sort -u | xargs sha256sum > "$out/before.sha256"
cd "$D"
set +e
timeout 120 "$lean" -j1 -M4096 -o "$D/$m.olean" "$D/$m.lean" > "$out/stdout" 2> "$out/stderr"
code=$?
set -e
unchanged=true
sha256sum -c "$out/before.sha256" > "$out/bindings.stdout" 2> "$out/bindings.stderr" || { unchanged=false; code=126; }
printf '{"module":"%s","native_exit":%s,"bindings_unchanged":%s,"gate_exit":%s}\n' "$m" "$code" "$unchanged" "$code" > "$out/receipt.json"
cat "$out/stdout" "$out/stderr"
printf '%s\n' "$out"
exit "$code"
