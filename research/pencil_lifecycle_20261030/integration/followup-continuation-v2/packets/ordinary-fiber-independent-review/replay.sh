#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
p=$PWD
out=$p/replay1
mkdir "$out"
lean=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean
export LEAN_SYSROOT=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain
export LEAN_PATH="$p/source:$p/imports-a:$p/imports-b:/var/tmp/pencil-mumford-validity-20261031-v1/dependency-union:/var/tmp/pencil-sameu-global-smooth-20261031-v1/dependency-union"
"$lean" --version > "$out/lean-version.txt"
sha256sum "$lean" > "$out/toolchain.sha256"
for m in Comparator Repair SignedPair Fiber ReviewerControls ExpectedFailure; do
 set +e
 timeout -k 5s 120s "$lean" -j1 -M3500 -o "$p/source/$m.olean" "$p/source/$m.lean" > "$out/$m.stdout" 2> "$out/$m.stderr"
 code=$?
 set -e
 printf '%s\n' "$code" > "$out/$m.exit"
 if [ "$m" != ExpectedFailure ] && [ "$code" != 0 ]; then exit "$code"; fi
 if [ "$m" = ExpectedFailure ] && [ "$code" != 1 ]; then exit 10; fi
done
timeout -k 5s 30s rustc --edition=2024 producer-check.rs -o "$out/check-native"
"$out/check-native" "$out" > "$out/acceptance.json"
