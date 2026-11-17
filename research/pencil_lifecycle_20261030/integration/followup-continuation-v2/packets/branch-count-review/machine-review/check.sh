#!/usr/bin/env bash
set -euo pipefail
R=/var/tmp/pencil-branch-count-replay-20261102
D=$R/target
m=$1
out=$(mktemp -d "$R/$m-replay-XXXXXXXX")
cp "$D/$m.lean" "$out/"
lean=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean
export LEAN_SYSROOT=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain
export LEAN_PATH="$D:$(cat "$R/dependency-path.txt")"
{ find "$D" "$R/dependencies" -type f \( -name '*.lean' -o -name '*.olean*' -o -name '*.ir*' -o -name '*.json' -o -name '*.py' \) ! -path "$D/$m.olean"; printf '%s\n' "$lean" "$R/check.sh" "$R/review-plan.md" "$R/dependency-path.txt"; } | sort -u | xargs sha256sum > "$out/before.sha256"
printf '%s\n' "timeout 120 $lean -j1 -M4096 -o $D/$m.olean $D/$m.lean" > "$out/command.txt"
date -u +%FT%TZ > "$out/started.txt"
cd "$D"
set +e
timeout 120 "$lean" -j1 -M4096 -o "$D/$m.olean" "$D/$m.lean" > "$out/stdout" 2> "$out/stderr"
code=$?
set -e
unchanged=true
gate=$code
sha256sum -c "$out/before.sha256" > "$out/bindings.stdout" 2> "$out/bindings.stderr" || { unchanged=false; gate=126; }
date -u +%FT%TZ > "$out/ended.txt"
printf '{"module":"%s","native_exit":%s,"bindings_unchanged":%s,"gate_exit":%s}\n' "$m" "$code" "$unchanged" "$gate" > "$out/receipt.json"
cat "$out/stdout" "$out/stderr"
printf '%s\n' "$out"
exit "$gate"
