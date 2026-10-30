#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd);repo=$(git -C "$here" rev-parse --show-toplevel)
attempt=${1:?fresh attempt};raw="$here/runs/$attempt";target=/var/tmp/koblitz-r29-target
[ ! -e "$raw" ];mkdir -p "$raw/source/src"
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
(cd "$here";sha256sum -c inputs.sha256) > "$raw/input-verification.log"
cp "$here/native/src/main.rs" "$raw/source/src/"
cp "$here/native/Cargo.toml" "$here/native/Cargo.lock" "$raw/source/"
cp "$0" "$raw/runner.sh"
sha256sum "$raw/source/src/main.rs" "$raw/source/Cargo.toml" "$raw/source/Cargo.lock" "$raw/runner.sh" > "$raw/source.sha256"
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt"
set +e
(ulimit -v 4194304;timeout 1200 env CARGO_TARGET_DIR="$target" cargo build --offline --release -j1 --manifest-path "$raw/source/Cargo.toml") > "$raw/build.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/build.exit";set -e
[ "$rc" = 0 ]
sha256sum "$target/release/koblitz-modular-carriers" > "$raw/binary.sha256"
tar -cJf "$here/checkpoint-live.tar.xz" -C "$here" runs native
"$repo/scripts/checkpoint.sh" push > "$raw/runtime-checkpoint.log" 2>&1
set +e
date -u +%FT%TZ > "$raw/started-utc.txt"
(ulimit -v 4194304;timeout 1200 "$target/release/koblitz-modular-carriers" "$raw" "$here/../27/source-data" "$here/../28/runs/dual28-02") > "$raw/native.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt";date -u +%FT%TZ > "$raw/finished-utc.txt";set -e
tar -cJf "$here/checkpoint-live.tar.xz" -C "$here" runs native
[ "$rc" = 0 ];rg -qx JET29_ALL_CHECKS_PASS "$raw/native.log"
echo "PASS $attempt"
