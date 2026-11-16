#!/usr/bin/env bash
set -euo pipefail
base=/var/tmp/pencil-graph-forest-review-20261101
out=$(mktemp -d "$base/native-verification/replay-XXXXXXXX")
snap="$out/snapshot"
mkdir "$snap"
cp "$base/subject/invocation-a7dpq5t0/snapshot/"*.lean "$snap/"
for f in /var/tmp/independent-graph-review-20261101/snapshot/*.olean; do cp "$f" "$snap/"; done
mods=(Find Cycles Union Forest Composition PriorControls ForestControls Readback Reviewer ReviewerReadback)
for m in "${mods[@]}"; do rm -f "$snap/$m.olean"; done
lean=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean
export LEAN_SYSROOT=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain
export LEAN_PATH="$snap:/var/tmp/pencil-mumford-validity-20261031-v1/dependency-union:/var/tmp/pencil-sameu-global-smooth-20261031-v1/dependency-union"
sha256sum "$snap/"*.lean "$snap/"*.olean "$base/native-verification/replay.sh" "$base/native-verification/src/main.rs" "$lean" > "$out/before.sha256"
printf '{"modules":[' > "$out/receipt.json"
cd "$snap"
sep=
gate=0
for m in "${mods[@]}"; do
 set +e
 timeout 120 "$lean" -j1 -M4096 -o "$snap/$m.olean" "$snap/$m.lean" > "$out/$m.stdout" 2> "$out/$m.stderr"
 code=$?
 set -e
 printf '%s{"module":"%s","native_exit":%s}' "$sep" "$m" "$code" >> "$out/receipt.json"
 sep=,
 if [ "$code" -ne 0 ]; then gate=$code; break; fi
done
unchanged=true
sha256sum -c "$out/before.sha256" > "$out/bindings-check.stdout" 2> "$out/bindings-check.stderr" || { unchanged=false; gate=126; }
printf '],"gate_exit":%s,"bindings_unchanged":%s}\n' "$gate" "$unchanged" >> "$out/receipt.json"
printf '%s\n' "$out"
exit "$gate"
