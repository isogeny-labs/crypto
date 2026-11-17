#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
out=${1:?fresh receipt directory required}
mkdir "$out"
cp -r source "$out/source"
lean=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean
export LEAN_SYSROOT=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain
export LEAN_PATH=/var/tmp/pencil-mumford-validity-20261031-v1/dependency-union
sha256sum "$out"/source/*.lean check.rs run.sh "$lean" "$LEAN_PATH/Mathlib/LinearAlgebra/Span/Basic.olean" > "$out/before.sha256"
"$lean" --version > "$out/lean-version.txt"
for m in Information ExpectedFailure; do
 set +e
 timeout -k 5s 120s "$lean" -j1 -M4096 "$out/source/$m.lean" > "$out/$m.stdout" 2> "$out/$m.stderr"
 code=$?
 set -e
 printf '%s\n' "$code" > "$out/$m.exit"
done
rustc --edition=2024 check.rs -o "$out/check-native"
"$out/check-native" "$out" > "$out/acceptance.json"
sha256sum -c "$out/before.sha256" > "$out/bindings.txt"
